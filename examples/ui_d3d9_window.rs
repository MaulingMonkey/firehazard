#![forbid(unsafe_op_in_unsafe_fn)]

use abistr::cstr16;

use bytemuck::*;

use firehazard::*;

use thindx::SafeHWND;
use thindx::d3d::*;
use thindx::d3d9::*;

use winapi::shared::minwindef::*;
use winapi::shared::windef::HWND;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::winuser::*;

use std::mem::size_of;
use std::ptr::{null_mut, null};
use std::time::{Instant, Duration};



fn main() {
    let d3d = unsafe { Direct3D::create(SdkVersion::DEFAULT) }.unwrap();
    output_debug_string_w(cstr16!("sandbox"));
    revert_to_self().unwrap();

    let class = register_window_class();
    let hwnd = create_window(class);

    let timeout = std::env::var_os("WRITE_HANDLE") // detect max_sandbox
        .map(|_| Instant::now() + Duration::from_secs(1));

    let device = unsafe { d3d.create_device(
        0, DevType::HAL, None, Create::FpuPreserve | Create::HardwareVertexProcessing | Create::NoWindowChanges,
        &mut PresentParameters {
            swap_effect:            SwapEffect::Discard,
            device_window:          SafeHWND::assert_unbounded(hwnd),
            windowed:               true.into(),
            presentation_interval:  Present::IntervalOne,
            .. Zeroable::zeroed()
        }
    )}.unwrap();

    loop {
        let mut msg : MSG = Default::default();
        while unsafe { PeekMessageW(&mut msg, null_mut(), 0, 0, PM_REMOVE) } != 0 {
            match msg.message {
                WM_QUIT => std::process::exit(msg.wParam as _),
                _ => {},
            }
            unsafe { TranslateMessage(&msg) };
            unsafe { DispatchMessageW(&msg) };
        }

        device.clear(None, Some(Color::argb(0xFF112233)), None, None).unwrap();
        device.present(.., .., (), None).unwrap();

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

    let atom = unsafe { RegisterClassExW(&WNDCLASSEXW {
        cbSize:         size_of::<WNDCLASSEXW>() as _,
        hInstance:      get_exe_hinstance(),
        hCursor:        cursor_arrow,
        lpszClassName:  cstr16!("ui_d3d9_window").as_ptr(),
        lpfnWndProc:    Some(wndproc),
        .. Default::default()
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
