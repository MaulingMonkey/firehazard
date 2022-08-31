#![forbid(unsafe_op_in_unsafe_fn)]

use sandbox_windows_ffi::*;

use abistr::*;

use std::io::Write;

fn main() {
    // Pulls in for DoS resistant hash seeding:
    //  BCryptGenRandom                     (bcrypt.dll)
    //  SystemFunction036 aka RtlGenRandom  (cryptbase.dll) as a fallback
    // Which can cause permissions problems if permissions lock the process down too much:
    //  1. loading dlls         (can fail to even read the DLL into memory)
    //  2. initializing dlls    (can fail to finish DllMain)
    //  3. executing dlls       (can fail on bellow line due to internal access denied panic)
    dbg!(std::collections::HashMap::<u32, u32>::new());
    sandbox();
    dbg!(std::collections::HashMap::<u32, u32>::new());
}

fn sandbox() {
    println!("stdout");
    output_debug_string_a(cstr!("sandbox"));
    eprintln!("stderr");
    let mut write_handle        : io::WriteHandle<'static>  = unsafe { env_var_handle("%WRITE_HANDLE%"          ) };
    let read_handle_noinherit   : io::ReadHandle<'static>   = unsafe { env_var_handle("%READ_HANDLE_NOINHERIT%" ) };
    let _ = dbg!(get_handle_information(write_handle));

    write_handle.write_all(b"explicit handle i/o\n").unwrap();

    if false {
        // XXX: This kills the process if:
        //  1. strict handle checks are enabled
        //  2. READ_HANDLE_NOINHERIT is indeed not inherited by this process (e.g. `process::ThreadAttributeRef::handle_list` was used to only inherit WRITE_HANDLE)
        // The documentation states that enabling strict handle checks causes invalid handles to throw an exception.
        // These might be unhandleable?  I don't even see them in the debugger events stream: only the immediate fatal exit.
        // At the very least, this catch_unwind doesn't do anything useful.
        let _ = dbg!(std::panic::catch_unwind(|| get_handle_information(read_handle_noinherit)));
    }
}

unsafe fn env_var_handle<H: FromLocalHandle>(name: &str) -> H {
    let inner = name.strip_prefix("%").and_then(|name| name.strip_suffix("%")).unwrap();
    let handle = std::env::var(inner).expect(name);
    let handle = handle.parse::<usize>().expect(name);
    let handle = unsafe { H::from_raw(handle as _) }.expect(name);
    handle
}
