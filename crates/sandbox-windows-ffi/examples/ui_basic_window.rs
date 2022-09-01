#![forbid(unsafe_op_in_unsafe_fn)]

use sandbox_windows_ffi::*;

use abistr::*;

use winapi::shared::minwindef::*;
use winapi::shared::windef::HWND;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::wingdi::{CreateSolidBrush, RGB};
use winapi::um::winuser::*;

use std::mem::{zeroed, size_of};
use std::ptr::{null_mut, null};
use std::time::{Instant, Duration};



fn main() {
    output_debug_string_w(cstr16!("sandbox"));
    revert_to_self().unwrap();

    let class = register_window_class();
    let _hwnd = create_window(class);

    let timeout = std::env::var_os("WRITE_HANDLE") // detect max_sandbox
        .map(|_| Instant::now() + Duration::from_secs(1));

    loop {
        let mut msg : MSG = unsafe { zeroed() };
        while unsafe { PeekMessageW(&mut msg, null_mut(), 0, 0, PM_REMOVE) } != 0 {
            match msg.message {
                WM_QUIT => std::process::exit(msg.wParam as _),
                _ => {},
            }
            unsafe { TranslateMessage(&msg) };
            unsafe { DispatchMessageW(&msg) };
        }

        if let Some(timeout) = timeout {
            let now = Instant::now();
            if now >= timeout { return }
            sleep_ms(100);
        } else {
            unsafe { WaitMessage() };
        }
    }
}

fn get_exe_hinstance() -> HINSTANCE {
    let hinstance = unsafe { GetModuleHandleW(null()) };
    assert!(!hinstance.is_null());
    hinstance
}

fn create_window(atom: ATOM) -> HWND {
    let hwnd = unsafe { CreateWindowExW(
        0, atom as usize as _, cstr16!("Example Window").as_ptr(), WS_OVERLAPPEDWINDOW,
        CW_USEDEFAULT, CW_USEDEFAULT, 400, 300,
        null_mut(), null_mut(), get_exe_hinstance(), null_mut()
    )};
    assert!(!hwnd.is_null());
    // update overlapped?
    let _was_already_visible = unsafe { ShowWindow(hwnd, SW_SHOWDEFAULT) };
    hwnd
}

fn register_window_class() -> ATOM {
    let cursor_arrow = unsafe { LoadCursorW(null_mut(), IDC_ARROW) };
    assert!(!cursor_arrow.is_null());

    let bg = unsafe { CreateSolidBrush(RGB(0x11, 0x22, 0x33)) };

    let atom = unsafe { RegisterClassExW(&WNDCLASSEXW {
        cbSize:         size_of::<WNDCLASSEXW>() as _,
        hInstance:      get_exe_hinstance(),
        hCursor:        cursor_arrow,
        lpszClassName:  cstr16!("ui_basic_window").as_ptr(),
        lpfnWndProc:    Some(wndproc),
        hbrBackground:  bg,
        ..Default::default()
    })};
    assert!(atom != 0, "RegisterClassExW failed with GetLastError()={:?}", Error::get_last());

    atom
}

unsafe extern "system" fn wndproc(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_DESTROY => unsafe {
            PostQuitMessage(0);
            DefWindowProcW(hwnd, msg, wparam, lparam)
        }
        _ => unsafe {
            DefWindowProcW(hwnd, msg, wparam, lparam)
        }
    }
}
