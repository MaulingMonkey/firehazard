use firehazard::*;

use winapi::shared::minwindef::FALSE;
use winapi::shared::winerror::ERROR_INVALID_HANDLE;
use winapi::um::errhandlingapi::{GetLastError, SetLastError};
use winapi::um::fileapi::ReadFile;
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};

use std::os::windows::io::AsRawHandle;

use core::ptr::null_mut;



const ENABLE_STRICT_HANDLE_CHECK_POLICY : &'static str = "ENABLE_STRICT_HANDLE_CHECK_POLICY";
struct Test { name: &'static str, f: fn(), relaxed_exit_code: u32, strict_exit_code: u32 }
macro_rules! tests { ( $($ident:ident => $relaxed:expr, $strict:expr),* $(,)? ) => {
    const TESTS : &[Test] = &[$(Test { name: stringify!($ident), f: $ident, relaxed_exit_code: $relaxed, strict_exit_code: $strict },)*];
}}
tests![
    // name                 => relaxed, strict exit codes
    panic                       => 101, 101, // https://rust-cli.github.io/book/in-depth/exit-code.html

    close_handle_null           => 0, 0,
    close_handle_invalid        => 0, 0,
    close_handle_dangling       => 0, 0, // These don't trigger strict handle checks? How unnerving.
    close_handle_never_valid    => 0, 0, // These don't trigger strict handle checks? How unnerving.

    read_file_null              => 0, 0,
    read_file_invalid           => 0, 0,
    read_file_dangling          => 0, 0xC0000008, // STATUS_INVALID_HANDLE
    read_file_never_valid       => 0, 0xC0000008, // STATUS_INVALID_HANDLE
];

fn panic() {
    panic!("test panic please ignore");
}



fn close_handle_null() {
    assert!(FALSE == unsafe { CloseHandle(null_mut()) });
    assert_eq!(ERROR_INVALID_HANDLE, unsafe { GetLastError() });
}

fn close_handle_invalid() {
    // CloseHandle(INVALID_HANDLE_VALUE) "succeeds"
    let clear_error = 0;

    unsafe { SetLastError(clear_error) };
    assert!(0 != unsafe { CloseHandle(INVALID_HANDLE_VALUE) });
    assert_eq!(clear_error, unsafe { GetLastError() });

    unsafe { SetLastError(clear_error) };
    assert!(0 != unsafe { CloseHandle(INVALID_HANDLE_VALUE) });
    assert_eq!(clear_error, unsafe { GetLastError() });
}

fn close_handle_dangling() {
    let file = std::fs::File::open("Readme.md").unwrap();
    let handle = file.as_raw_handle();
    drop(file);
    assert!(FALSE == unsafe { CloseHandle(handle.cast()) });
    assert_eq!(ERROR_INVALID_HANDLE, unsafe { GetLastError() });
}

fn close_handle_never_valid() {
    assert!(FALSE == unsafe { CloseHandle(0x12345678_usize as *mut _) });
    assert_eq!(ERROR_INVALID_HANDLE, unsafe { GetLastError() });
}



fn read_file_null() {
    let (mut buffer, mut read) = ([0u8; 1024], 0);
    assert!(FALSE == unsafe { ReadFile(null_mut(), buffer.as_mut_ptr().cast(), 1024, &mut read, null_mut()) });
    assert_eq!(ERROR_INVALID_HANDLE, unsafe { GetLastError() });
}

fn read_file_invalid() {
    let (mut buffer, mut read) = ([0u8; 1024], 0);
    assert!(FALSE == unsafe { ReadFile(INVALID_HANDLE_VALUE, buffer.as_mut_ptr().cast(), 1024, &mut read, null_mut()) });
    assert_eq!(ERROR_INVALID_HANDLE, unsafe { GetLastError() });
}

fn read_file_dangling() {
    let file = std::fs::File::open("Readme.md").unwrap();
    let handle = file.as_raw_handle();
    drop(file);
    let (mut buffer, mut read) = ([0u8; 1024], 0);
    assert!(FALSE == unsafe { ReadFile(handle.cast(), buffer.as_mut_ptr().cast(), 1024, &mut read, null_mut()) });
    assert_eq!(ERROR_INVALID_HANDLE, unsafe { GetLastError() }); // unreached in strict mode: ReadFile throws STATUS_INVALID_HANDLE (0xC0000008)
}

fn read_file_never_valid() {
    let (mut buffer, mut read) = ([0u8; 1024], 0);
    assert!(FALSE == unsafe { ReadFile(0x12345678_usize as *mut _, buffer.as_mut_ptr().cast(), 1024, &mut read, null_mut()) });
    assert_eq!(ERROR_INVALID_HANDLE, unsafe { GetLastError() }); // unreached in strict mode: ReadFile throws STATUS_INVALID_HANDLE (0xC0000008)
}



fn main() {
    let mut args = std::env::args();
    let _exe = args.next();
    let test = args.next();
    let test = test.as_deref().unwrap_or("*");

    if test == "*" {
        let exe = std::env::current_exe().expect("unable to determine current exe to launch subprocesses");

        for test in TESTS.iter() {
            for (strict,    expected_exit_code      ) in [
                ("0",       test.relaxed_exit_code  ),
                ("1",       test.strict_exit_code   ),
            ].iter().copied() {
                let actual_exit_code = std::process::Command::new(&exe).arg(test.name).env(ENABLE_STRICT_HANDLE_CHECK_POLICY, strict).status().unwrap().code().unwrap() as u32;
                assert!(expected_exit_code == actual_exit_code, "expected exit code 0x{expected_exit_code:08X}, got 0x{actual_exit_code:08X}");
            }
        }
    } else if let Some(test) = TESTS.iter().find(|t| t.name == test) {
        let strict = std::env::var_os(ENABLE_STRICT_HANDLE_CHECK_POLICY).map_or(false, |value| !"0 false FALSE".split(' ').any(|v| value == v));
        firehazard::set_process_mitigation_policy(firehazard::process::mitigation::StrictHandleCheckPolicy {
            handle_exceptions_permanently_enabled:          strict,
            raise_exception_on_invalid_handle_reference:    strict,
            .. Default::default()
        }).unwrap();

        println!(
            "examples\\strict_handle_check_policy: running test {test} in {strict} mode",
            test = test.name,
            strict = if strict { "strict" } else { "relaxed" },
        );
        (test.f)();
    } else {
        eprintln!("examples\\strict_handle_check_policy: failed to find test {test:?}");
        std::process::exit(1);
    }
}
