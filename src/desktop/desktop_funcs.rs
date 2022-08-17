use crate::*;
use abistr::*;
use winapi::ctypes::c_char;
use winapi::shared::minwindef::{FALSE, LPARAM, BOOL, TRUE};
use winapi::shared::winerror::*;
use winapi::um::errhandlingapi::SetLastError;
use winapi::um::winuser::*;
use std::ptr::null_mut;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createdesktopa)\]
/// CreateDesktopA
///
/// ### Example
/// ```
/// # use win32_security_playground::*;
/// # use abistr::cstr;
/// # use winapi::um::winnt::GENERIC_ALL;
/// let winsta = create_desktop_a(cstr!("PlaygroundDesktop"), (), None, 0, GENERIC_ALL, None).unwrap();
/// ```
pub fn create_desktop_a(
    desktop:        impl TryIntoAsCStr,
    device:         impl TryIntoAsOptCStr,
    devmode:        Option<std::convert::Infallible>,
    flags:          u32,                                // TODO: type
    desired_access: u32,                                // TODO: type
    _sa:            Option<std::convert::Infallible>,   // TODO: type
) -> Result<desktop::OwnedHandle, LastError> {
    let desktop = desktop.try_into().map_err(|_| LastError(ERROR_INVALID_PARAMETER))?;
    let device = device.try_into().map_err(|_| LastError(ERROR_INVALID_PARAMETER))?;
    let _ = devmode; let devmode = null_mut();
    let handle = unsafe { CreateDesktopA(desktop.as_cstr(), device.as_opt_cstr(), devmode, flags, desired_access, null_mut()) };
    LastError::get_if(handle.is_null())?;
    Ok(unsafe { desktop::OwnedHandle::from_raw_unchecked(handle) })
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowstationw)\]
/// CreateDesktopW
///
/// ### Example
/// ```
/// # use win32_security_playground::*;
/// # use abistr::cstr16;
/// # use winapi::um::winnt::GENERIC_ALL;
/// let winsta = create_desktop_w(cstr16!("PlaygroundDesktop"), (), None, 0, GENERIC_ALL, None).unwrap();
/// ```
pub fn create_desktop_w(
    desktop:        impl TryIntoAsCStr<u16>,
    device:         impl TryIntoAsOptCStr<u16>,
    devmode:        Option<std::convert::Infallible>,
    flags:          u32,                                // TODO: type
    desired_access: u32,                                // TODO: type
    _sa:            Option<std::convert::Infallible>,   // TODO: type
) -> Result<desktop::OwnedHandle, LastError> {
    let desktop = desktop.try_into().map_err(|_| LastError(ERROR_INVALID_PARAMETER))?;
    let device = device.try_into().map_err(|_| LastError(ERROR_INVALID_PARAMETER))?;
    let _ = devmode; let devmode = null_mut();
    let handle = unsafe { CreateDesktopW(desktop.as_cstr(), device.as_opt_cstr(), devmode, flags, desired_access, null_mut()) };
    LastError::get_if(handle.is_null())?;
    Ok(unsafe { desktop::OwnedHandle::from_raw_unchecked(handle) })
}

// https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createdesktopexa
// CreateDesktopExA: adds heap size + reserved pvoid
// https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createdesktopexw
// CreateDesktopExW: adds heap size + reserved pvoid

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumdesktopsa)\]
/// EnumDesktopsA
///
/// ### Example
/// ```
/// # use win32_security_playground::*;
/// let winsta = get_process_window_station().unwrap();
/// enum_desktops_a(&winsta, |desktop| {
///     println!("{desktop:?}");
///     Ok(())
/// }).unwrap();
/// ```
///
/// ### Output
/// ```text
/// "Default"
/// ```
///
/// ### Errata
/// The docs for [`EnumDesktopsA`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumdesktopsa) state:
/// >   If this parameter (`winsta`) is NULL, the current window station is used.
///
/// However, in my testing (Windows 10.0.19043.1889), this appears to be a lie: if the parameter is NULL, this enumerates
/// *window stations* instead of enumerating *desktops of said window station*.  As such, I've made `winsta` a non-optional
/// type in this function.
pub fn enum_desktops_a<F: FnMut(CStrPtr) -> Result<(), LastError>>(
    winsta:         &winsta::OwnedHandle,
    mut enum_func:  F,
) -> Result<(), LastError> {
    let enum_func : *mut F = &mut enum_func;
    LastError::get_if(FALSE == unsafe { EnumDesktopsA(winsta.as_handle(), Some(fwd_enum_desktops_a::<F>), enum_func as LPARAM) })
}

unsafe extern "system" fn fwd_enum_desktops_a<F: FnMut(CStrPtr) -> Result<(), LastError>>(desktop: *mut c_char, param: LPARAM) -> BOOL {
    let desktop = unsafe { CStrPtr::from_ptr_unbounded(desktop) };
    let f = unsafe { &mut *(param as *mut F) };
    match f(desktop) {
        Ok(()) => TRUE,
        Err(e) => {
            unsafe { SetLastError(e.0) };
            FALSE
        },
    }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumdesktopsw)\]
/// EnumDesktopsW
///
/// ### Example
/// ```
/// # use win32_security_playground::*;
/// let winsta = get_process_window_station().unwrap();
/// enum_desktops_w(&winsta, |desktop| {
///     println!("{desktop:?}");
///     Ok(())
/// }).unwrap();
/// ```
///
/// ### Output
/// ```text
/// "Default"
/// ```
///
/// ### Errata
/// The docs for [`EnumDesktopsW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumdesktopsw) state:
/// >   If this parameter (`winsta`) is NULL, the current window station is used.
///
/// However, in my testing (Windows 10.0.19043.1889), this appears to be a lie: if the parameter is NULL, this enumerates
/// *window stations* instead of enumerating *desktops of said window station*.  As such, I've made `winsta` a non-optional
/// type in this function.
pub fn enum_desktops_w<F: FnMut(CStrPtr<u16>) -> Result<(), LastError>>(
    winsta:         &winsta::OwnedHandle,
    mut enum_func:  F,
) -> Result<(), LastError> {
    let enum_func : *mut F = &mut enum_func;
    LastError::get_if(FALSE == unsafe { EnumDesktopsW(winsta.as_handle(), Some(fwd_enum_desktops_w::<F>), enum_func as LPARAM) })
}

unsafe extern "system" fn fwd_enum_desktops_w<F: FnMut(CStrPtr<u16>) -> Result<(), LastError>>(desktop: *mut u16, param: LPARAM) -> BOOL {
    let desktop = unsafe { CStrPtr::<u16>::from_ptr_unbounded(desktop) };
    let f = unsafe { &mut *(param as *mut F) };
    match f(desktop) {
        Ok(()) => TRUE,
        Err(e) => {
            unsafe { SetLastError(e.0) };
            FALSE
        },
    }
}

// https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumdesktopwindows
// EnumDesktopWindows

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getthreaddesktop)\]
/// GetThreadDesktop
///
/// ### Example
/// ```
/// # use win32_security_playground::*;
/// let desktop = get_thread_desktop(get_current_thread_id()).unwrap();
/// ```
///
/// ### Errata
/// The docs for [`GetThreadDesktop`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getthreaddesktop) state:
/// >   You do not need to call the CloseDesktop function to close the returned handle.
///
/// A borrowed handle is super awkward here, so this function returns a *duplicated* handle that can be closed instead.
pub fn get_thread_desktop(thread_id: thread::Id) -> Result<desktop::OwnedHandle, LastError> {
    let desktop = unsafe { GetThreadDesktop(thread_id) };
    LastError::get_if(desktop.is_null())?;
    Ok(unsafe { desktop::OwnedHandle::clone_from_raw(desktop) })
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-openwindowstationa)\]
/// OpenDesktopA
///
/// ### Example
/// ```
/// # use win32_security_playground::*;
/// # use abistr::cstr;
/// # use winapi::um::winnt::GENERIC_ALL;
/// let desktop = open_desktop_a(cstr!("Default"), 0, false, GENERIC_ALL).unwrap();
/// ```
pub fn open_desktop_a(
    desktop:        impl TryIntoAsCStr,
    flags:          u32,    // TODO: type
    inherit:        bool,
    desired_access: u32,    // TODO: type
) -> Result<desktop::OwnedHandle, LastError> {
    let desktop = desktop.try_into().map_err(|_| LastError(ERROR_INVALID_PARAMETER))?;
    let handle = unsafe { OpenDesktopA(desktop.as_cstr(), flags, inherit as _, desired_access) };
    LastError::get_if(handle.is_null())?;
    Ok(unsafe { desktop::OwnedHandle::from_raw_unchecked(handle) })
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-openwindowstationw)\]
/// OpenDesktopW
///
/// ### Example
/// ```
/// # use win32_security_playground::*;
/// # use abistr::cstr16;
/// # use winapi::um::winnt::GENERIC_ALL;
/// let desktop = open_desktop_w(cstr16!("Default"), 0, false, GENERIC_ALL).unwrap();
/// ```
pub fn open_desktop_w(
    desktop:        impl TryIntoAsCStr<u16>,
    flags:          u32,    // TODO: type
    inherit:        bool,
    desired_access: u32,    // TODO: type
) -> Result<desktop::OwnedHandle, LastError> {
    let desktop = desktop.try_into().map_err(|_| LastError(ERROR_INVALID_PARAMETER))?;
    let handle = unsafe { OpenDesktopW(desktop.as_cstr(), flags, inherit as _, desired_access) };
    LastError::get_if(handle.is_null())?;
    Ok(unsafe { desktop::OwnedHandle::from_raw_unchecked(handle) })
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-openinputdesktop)\]
/// OpenInputDesktop
///
/// ### Example
/// ```
/// # use win32_security_playground::*;
/// # use winapi::um::winnt::GENERIC_ALL;
/// let desktop = open_input_desktop(0, false, GENERIC_ALL).unwrap();
/// ```
pub fn open_input_desktop(
    flags:          u32,    // TODO: type
    inherit:        bool,
    desired_access: u32,    // TODO: type
) -> Result<desktop::OwnedHandle, LastError> {
    let handle = unsafe { OpenInputDesktop(flags, inherit as _, desired_access) };
    LastError::get_if(handle.is_null())?;
    Ok(unsafe { desktop::OwnedHandle::from_raw_unchecked(handle) })
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setthreaddesktop)\]
/// SetThreadDesktop
// TODO: example?
pub fn set_thread_desktop(desktop: &desktop::OwnedHandle) -> Result<(), LastError> {
    LastError::get_if(FALSE == unsafe { SetThreadDesktop(desktop.as_handle()) })
}
