#![forbid(unsafe_op_in_unsafe_fn)]

use abistr::cstr16;

use firehazard::*;

use winapi::shared::dxgi::*;
use winapi::shared::dxgiformat::*;
use winapi::shared::dxgitype::*;
use winapi::shared::minwindef::*;
use winapi::shared::windef::HWND;
use winapi::shared::windef::RECT;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d3d11::*;
use winapi::um::d3dcommon::*;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::winuser::*;
use winapi::Interface;

use std::mem::size_of;
use std::ptr::{null_mut, null};
use std::time::{Instant, Duration};



fn main() {
    let class = register_window_class();
    let hwnd = create_window(class);

    let timeout = std::env::var_os("WRITE_HANDLE") // detect max_sandbox
    .map(|_| Instant::now() + Duration::from_secs(1));

    let (swap_chain, _device, device_context, back_buffer_rtv) = create_d3d11(hwnd);

    output_debug_string_w(cstr16!("sandbox"));
    revert_to_self().unwrap();

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

        unsafe { device_context.ClearRenderTargetView(back_buffer_rtv.as_ptr(), &[0.1, 0.2, 0.3, 1.0]) };
        unsafe { swap_chain.Present(0, 0) }; // XXX

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
        lpszClassName:  cstr16!("ui_d3d11_window").as_ptr(),
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

fn create_d3d11(hwnd: HWND) -> (
    mcom::Rc<IDXGISwapChain>,
    mcom::Rc<ID3D11Device>,
    mcom::Rc<ID3D11DeviceContext>,
    mcom::Rc<ID3D11RenderTargetView>,
) {
    let (w, h) = {
        let mut rect : RECT = unsafe { core::mem::zeroed() };
        assert_ne!(0, unsafe { GetClientRect(hwnd, &mut rect) });
        ((rect.right - rect.left) as u32, (rect.bottom - rect.top) as u32)
    };

    let swap_chain_desc = DXGI_SWAP_CHAIN_DESC {
        BufferDesc: DXGI_MODE_DESC {
            Width:              w,
            Height:             h,
            RefreshRate:        DXGI_RATIONAL { Numerator: 60, Denominator: 1 },
            Format:             DXGI_FORMAT_R8G8B8A8_UNORM,
            ScanlineOrdering:   DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED,
            Scaling:            DXGI_MODE_SCALING_CENTERED,
        },
        SampleDesc:     DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
        BufferUsage:    DXGI_USAGE_RENDER_TARGET_OUTPUT,
        BufferCount:    1,
        OutputWindow:   hwnd,
        Windowed:       1,
        SwapEffect:     DXGI_SWAP_EFFECT_DISCARD,
        Flags:          0,
    };

    let mut swap_chain      = null_mut();
    let mut device          = null_mut();
    let mut device_context  = null_mut();
    let feature_level       = D3D_FEATURE_LEVEL_10_0;
    let feature_levels      = &[feature_level];
    assert!(SUCCEEDED(unsafe { D3D11CreateDeviceAndSwapChain(
        null_mut(), // adapter
        D3D_DRIVER_TYPE_HARDWARE,
        null_mut(), // software
        0, // flags
        feature_levels.as_ptr(),
        feature_levels.len().try_into().unwrap(),
        D3D11_SDK_VERSION, // SDK Version
        &swap_chain_desc,
        &mut swap_chain,
        &mut device,
        null_mut(), //&mut feature_level,
        &mut device_context
    )}));
    let swap_chain      = unsafe { mcom::Rc::from_raw(swap_chain    ) };
    let device          = unsafe { mcom::Rc::from_raw(device        ) };
    let device_context  = unsafe { mcom::Rc::from_raw(device_context) };

    let mut back_buffer = null_mut();
    assert!(SUCCEEDED(unsafe { swap_chain.GetBuffer(0, &ID3D11Texture2D::uuidof(), &mut back_buffer) }));
    let back_buffer = unsafe { mcom::Rc::from_raw(back_buffer as *mut ID3D11Resource) };

    let mut rtv = null_mut();
    assert!(SUCCEEDED(unsafe { device.CreateRenderTargetView(back_buffer.as_ptr(), null_mut(), &mut rtv) }));
    let rtv = unsafe { mcom::Rc::from_raw(rtv) };

    (swap_chain, device, device_context, rtv)
}
