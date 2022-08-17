use crate::*;
use abistr::*;
use winapi::ctypes::c_char;
use winapi::shared::minwindef::{FALSE, LPARAM, BOOL, TRUE};
use winapi::shared::winerror::*;
use winapi::um::errhandlingapi::SetLastError;
use winapi::um::winuser::*;
use std::ptr::null_mut;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowstationa)\]
/// CreateWindowStationA
///
/// ### Example
/// ```
/// # use win32_security_playground::*;
/// # use abistr::cstr;
/// # use winapi::shared::winerror::*;
/// # use winapi::um::winnt::GENERIC_ALL;
/// # use winapi::um::winuser::CWF_CREATE_ONLY;
/// assert_eq!(ERROR_ACCESS_DENIED, create_window_station_a(cstr!("WinSta0"), CWF_CREATE_ONLY, GENERIC_ALL, None).unwrap_err());
/// let winsta = create_window_station_a((), 0, GENERIC_ALL, None).unwrap();
/// ```
pub fn create_window_station_a(
    winsta:         impl TryIntoAsOptCStr,
    flags:          u32,                                            // TODO: type
    desired_access: u32,                                            // TODO: type
    _sa:            impl Into<Option<std::convert::Infallible>>,    // TODO: type
) -> Result<winsta::OwnedHandle, LastError> {
    let winsta = winsta.try_into().map_err(|_| LastError(ERROR_INVALID_PARAMETER))?;
    let handle = unsafe { CreateWindowStationA(winsta.as_opt_cstr(), flags, desired_access, null_mut()) };
    LastError::get_if(handle.is_null())?;
    Ok(unsafe { winsta::OwnedHandle::from_raw_unchecked(handle) })
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowstationw)\]
/// CreateWindowStationW
///
/// ### Example
/// ```
/// # use win32_security_playground::*;
/// # use abistr::cstr16;
/// # use winapi::shared::winerror::*;
/// # use winapi::um::winnt::GENERIC_ALL;
/// # use winapi::um::winuser::CWF_CREATE_ONLY;
/// assert_eq!(ERROR_ACCESS_DENIED, create_window_station_w(cstr16!("WinSta0"), CWF_CREATE_ONLY, GENERIC_ALL, None).unwrap_err());
/// let winsta = create_window_station_w((), 0, GENERIC_ALL, None).unwrap();
/// ```
pub fn create_window_station_w(
    winsta:         impl TryIntoAsOptCStr<u16>,
    flags:          u32,                                            // TODO: type
    desired_access: u32,                                            // TODO: type
    _sa:            impl Into<Option<std::convert::Infallible>>,    // TODO: type
) -> Result<winsta::OwnedHandle, LastError> {
    let winsta = winsta.try_into().map_err(|_| LastError(ERROR_INVALID_PARAMETER))?;
    let handle = unsafe { CreateWindowStationW(winsta.as_opt_cstr(), flags, desired_access, null_mut()) };
    LastError::get_if(handle.is_null())?;
    Ok(unsafe { winsta::OwnedHandle::from_raw_unchecked(handle) })
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumwindow_stationsa)\]
/// EnumWindowStationsA
///
/// ### Example
/// ```
/// # use win32_security_playground::*;
/// let mut found_winsta0 = false;
/// enum_window_stations_a(|winsta|{
///     found_winsta0 |= winsta.to_string_lossy() == "WinSta0";
///     println!("{winsta:?}");
///     Ok(())
/// }).unwrap();
/// assert!(found_winsta0);
/// ```
///
/// ### Output
/// ```text
/// "WinSta0"
/// "Service-0x0-21cd8$"
/// ```
pub fn enum_window_stations_a<F: FnMut(CStrPtr) -> Result<(), LastError>>(mut enum_func: F) -> Result<(), LastError> {
    let enum_func : *mut F = &mut enum_func;
    LastError::get_if(FALSE == unsafe { EnumWindowStationsA(Some(fwd_enum_window_stations_a::<F>), enum_func as LPARAM) })
}

unsafe extern "system" fn fwd_enum_window_stations_a<F: FnMut(CStrPtr) -> Result<(), LastError>>(winsta: *mut c_char, param: LPARAM) -> BOOL {
    let winsta = unsafe { CStrPtr::from_ptr_unbounded(winsta) };
    let f = unsafe { &mut *(param as *mut F) };
    match f(winsta) {
        Ok(()) => TRUE,
        Err(e) => {
            unsafe { SetLastError(e.0) };
            FALSE
        },
    }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumwindow_stationsw)\]
/// EnumWindowStationsW
///
/// ### Example
/// ```
/// # use win32_security_playground::*;
/// let mut found_winsta0 = false;
/// enum_window_stations_w(|winsta|{
///     found_winsta0 |= winsta.to_string_lossy() == "WinSta0";
///     println!("{winsta:?}");
///     Ok(())
/// }).unwrap();
/// assert!(found_winsta0);
/// ```
///
/// ### Output
/// ```text
/// "WinSta0"
/// "Service-0x0-21cd8$"
/// ```
pub fn enum_window_stations_w<F: FnMut(CStrPtr<u16>) -> Result<(), LastError>>(mut enum_func: F) -> Result<(), LastError> {
    let enum_func : *mut F = &mut enum_func;
    LastError::get_if(FALSE == unsafe { EnumWindowStationsW(Some(fwd_enum_window_stations_w::<F>), enum_func as LPARAM) })
}

unsafe extern "system" fn fwd_enum_window_stations_w<F: FnMut(CStrPtr<u16>) -> Result<(), LastError>>(winsta: *mut u16, param: LPARAM) -> BOOL {
    let winsta = unsafe { CStrPtr::<u16>::from_ptr_unbounded(winsta) };
    let f = unsafe { &mut *(param as *mut F) };
    match f(winsta) {
        Ok(()) => TRUE,
        Err(e) => {
            unsafe { SetLastError(e.0) };
            FALSE
        },
    }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getprocesswindowstation)
/// GetProcessWindowStation
///
/// ### Example
/// ```
/// # use win32_security_playground::*;
/// let winsta0 = get_process_window_station().unwrap();
/// ```
///
/// ### Errata
/// The docs for [`GetProcessWindowStation`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getprocesswindowstation) state:
/// >   Do not close the handle returned by this function.
///
/// A borrowed handle is super awkward here, so this function returns a *duplicated* handle that can be closed instead.
pub fn get_process_window_station() -> Result<winsta::OwnedHandle, LastError> {
    // "Do not close the handle returned by this function." - so we return a closeable clone instead
    let winsta = unsafe { GetProcessWindowStation() };
    LastError::get_if(winsta.is_null())?;
    Ok(unsafe { winsta::OwnedHandle::clone_from_raw(winsta) })
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-openwindowstationa)\]
/// OpenWindowStationA
///
/// ### Example
/// ```
/// # use win32_security_playground::*;
/// # use abistr::cstr;
/// # use winapi::um::winnt::GENERIC_ALL;
/// let winsta0 = open_window_station_a(cstr!("WinSta0"), false, GENERIC_ALL).unwrap();
/// ```
pub fn open_window_station_a(
    winsta:         impl TryIntoAsCStr,
    inherit:        bool,
    desired_access: u32,                                            // TODO: type
) -> Result<winsta::OwnedHandle, LastError> {
    let winsta = winsta.try_into().map_err(|_| LastError(ERROR_INVALID_PARAMETER))?;
    let handle = unsafe { OpenWindowStationA(winsta.as_cstr(), inherit as _, desired_access) };
    LastError::get_if(handle.is_null())?;
    Ok(unsafe { winsta::OwnedHandle::from_raw_unchecked(handle) })
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-openwindowstationw)\]
/// OpenWindowStationW
///
/// ### Example
/// ```
/// # use win32_security_playground::*;
/// # use abistr::cstr16;
/// # use winapi::um::winnt::GENERIC_ALL;
/// let winsta0 = open_window_station_w(cstr16!("WinSta0"), false, GENERIC_ALL).unwrap();
/// ```
pub fn open_window_station_w(
    winsta:         impl TryIntoAsCStr<u16>,
    inherit:        bool,
    desired_access: u32,                                            // TODO: type
) -> Result<winsta::OwnedHandle, LastError> {
    let winsta = winsta.try_into().map_err(|_| LastError(ERROR_INVALID_PARAMETER))?;
    let handle = unsafe { OpenWindowStationW(winsta.as_cstr(), inherit as _, desired_access) };
    LastError::get_if(handle.is_null())?;
    Ok(unsafe { winsta::OwnedHandle::from_raw_unchecked(handle) })
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setprocesswindowstation)\]
/// SetProcessWindowStation
// TODO: example?
pub fn set_process_window_station(winsta: &winsta::OwnedHandle) -> Result<(), LastError> {
    LastError::get_if(FALSE == unsafe { SetProcessWindowStation(winsta.as_handle()) })
}
