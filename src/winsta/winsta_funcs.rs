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
pub fn get_process_window_station() -> Result<winsta::OwnedHandle, LastError> {
    // "Do not close the handle returned by this function." - so we return a closeable clone instead
    let winsta = unsafe { GetProcessWindowStation() };
    LastError::get_if(winsta.is_null())?;
    Ok(unsafe { winsta::OwnedHandle::clone_from_raw(winsta) })
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-openwindowstationa)\]
/// OpenWindowStationA
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
pub fn set_process_window_station(winsta: &winsta::OwnedHandle) -> Result<(), LastError> {
    LastError::get_if(FALSE == unsafe { SetProcessWindowStation(winsta.as_handle()) })
}
