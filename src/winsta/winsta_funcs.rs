use crate::*;
use abistr::*;
use winapi::ctypes::c_char;
use winapi::shared::minwindef::{FALSE, LPARAM, BOOL, TRUE};
use winapi::shared::ntdef::HANDLE;
use winapi::shared::winerror::*;
use winapi::um::errhandlingapi::SetLastError;
use winapi::um::handleapi::DuplicateHandle;
use winapi::um::winnt::DUPLICATE_SAME_ACCESS;
use winapi::um::winuser::*;
use core::ptr::null;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowstationa)\]
/// CreateWindowStationA
///
/// ### Example
/// ```
/// # use firehazard::*;
/// # use abistr::cstr;
/// # use winapi::shared::winerror::*;
/// assert_eq!(ERROR_ACCESS_DENIED, create_window_station_a(cstr!("WinSta0"), winsta::CWF_CREATE_ONLY, winsta::ALL_ACCESS, None).unwrap_err());
/// let winsta = create_window_station_a((), (), winsta::ALL_ACCESS, None).unwrap();
/// ```
pub fn create_window_station_a(
    winsta:         impl TryIntoAsOptCStr,
    flags:          impl Into<winsta::CreateWindowFlags>,
    desired_access: impl Into<winsta::AccessRights>,
    sa:             Option<&security::Attributes>,
) -> Result<winsta::OwnedHandle, Error> {
    let handle = unsafe { CreateWindowStationA(
        winsta.try_into().map_err(|_| Error(E_STRING_NOT_NULL_TERMINATED as _))?.as_opt_cstr(),
        flags.into().into(),
        desired_access.into().into(),
        sa.map_or(null(), |sa| sa) as *mut _
    )};
    Error::get_last_if(handle.is_null())?;
    unsafe { winsta::OwnedHandle::from_raw(handle) }
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowstationw)\]
/// CreateWindowStationW
///
/// ### Example
/// ```
/// # use firehazard::*;
/// # use abistr::cstr16;
/// # use winapi::shared::winerror::*;
/// assert_eq!(ERROR_ACCESS_DENIED, create_window_station_w(cstr16!("WinSta0"), winsta::CWF_CREATE_ONLY, winsta::ALL_ACCESS, None).unwrap_err());
/// let winsta = create_window_station_w((), (), winsta::ALL_ACCESS, None).unwrap();
/// ```
pub fn create_window_station_w(
    winsta:         impl TryIntoAsOptCStr<u16>,
    flags:          impl Into<winsta::CreateWindowFlags>,
    desired_access: impl Into<winsta::AccessRights>,
    sa:             Option<&security::Attributes>,
) -> Result<winsta::OwnedHandle, Error> {
    let handle = unsafe { CreateWindowStationW(
        winsta.try_into().map_err(|_| Error(E_STRING_NOT_NULL_TERMINATED as _))?.as_opt_cstr(),
        flags.into().into(),
        desired_access.into().into(),
        sa.map_or(null(), |sa| sa) as *mut _
    )};
    Error::get_last_if(handle.is_null())?;
    unsafe { winsta::OwnedHandle::from_raw(handle) }
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumwindowstationsa)\]
/// EnumWindowStationsA
///
/// ### Example
/// ```
/// # #[cfg(feature = "std")] {
/// # use firehazard::*;
/// let mut found_winsta0 = false;
/// enum_window_stations_a(|winsta|{
///     found_winsta0 |= winsta.to_string_lossy() == "WinSta0";
///     println!("{winsta:?}");
///     Ok(())
/// }).unwrap();
/// assert!(found_winsta0);
/// # }
/// ```
///
/// ### Output
/// ```text
/// "WinSta0"
/// "Service-0x0-21cd8$"
/// ```
pub fn enum_window_stations_a<F: FnMut(CStrPtr) -> Result<(), Error>>(mut enum_func: F) -> Result<(), Error> {
    let enum_func : *mut F = &mut enum_func;
    Error::get_last_if(FALSE == unsafe { EnumWindowStationsA(Some(fwd_enum_window_stations_a::<F>), enum_func as LPARAM) })
}

unsafe extern "system" fn fwd_enum_window_stations_a<F: FnMut(CStrPtr) -> Result<(), Error>>(winsta: *mut c_char, param: LPARAM) -> BOOL {
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

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumwindowstationsw)\]
/// EnumWindowStationsW
///
/// ### Example
/// ```
/// # #[cfg(feature = "std")] {
/// # use firehazard::*;
/// let mut found_winsta0 = false;
/// enum_window_stations_w(|winsta|{
///     found_winsta0 |= winsta.to_string_lossy() == "WinSta0";
///     println!("{winsta:?}");
///     Ok(())
/// }).unwrap();
/// assert!(found_winsta0);
/// # }
/// ```
///
/// ### Output
/// ```text
/// "WinSta0"
/// "Service-0x0-21cd8$"
/// ```
pub fn enum_window_stations_w<F: FnMut(CStrPtr<u16>) -> Result<(), Error>>(mut enum_func: F) -> Result<(), Error> {
    let enum_func : *mut F = &mut enum_func;
    Error::get_last_if(FALSE == unsafe { EnumWindowStationsW(Some(fwd_enum_window_stations_w::<F>), enum_func as LPARAM) })
}

unsafe extern "system" fn fwd_enum_window_stations_w<F: FnMut(CStrPtr<u16>) -> Result<(), Error>>(winsta: *mut u16, param: LPARAM) -> BOOL {
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

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getprocesswindowstation)\]
/// GetProcessWindowStation + DuplicateHandle
///
/// ### Example
/// ```
/// # use firehazard::*;
/// let winsta0 = open_process_window_station().unwrap();
/// ```
///
/// ### Errata
/// The docs for [`GetProcessWindowStation`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getprocesswindowstation) state:
/// >   Do not close the handle returned by this function.
///
/// A borrowed handle is super awkward here, so this function returns a *duplicated* handle that can be closed instead.
pub fn open_process_window_station() -> Result<winsta::OwnedHandle, Error> {
    // "Do not close the handle returned by this function." - so we return a closeable clone instead
    let mut winsta : HANDLE = unsafe { GetProcessWindowStation() }.cast();
    Error::get_last_if(winsta.is_null())?;
    let process = get_current_process().as_handle();
    Error::get_last_if(FALSE == unsafe { DuplicateHandle(process, winsta, process, &mut winsta, 0, 0, DUPLICATE_SAME_ACCESS) })?;
    unsafe { winsta::OwnedHandle::from_raw(winsta.cast()) }
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-openwindowstationa)\]
/// OpenWindowStationA
///
/// ### Example
/// ```
/// # use firehazard::*;
/// # use abistr::cstr;
/// let winsta0 = open_window_station_a(cstr!("WinSta0"), false, winsta::ALL_ACCESS).unwrap();
/// ```
pub fn open_window_station_a(
    winsta:         impl TryIntoAsCStr,
    inherit:        bool,
    desired_access: impl Into<winsta::AccessRights>,
) -> Result<winsta::OwnedHandle, Error> {
    let handle = unsafe { OpenWindowStationA(
        winsta.try_into().map_err(|_| Error(E_STRING_NOT_NULL_TERMINATED as _))?.as_cstr(),
        inherit as _,
        desired_access.into().into()
    )};
    Error::get_last_if(handle.is_null())?;
    unsafe { winsta::OwnedHandle::from_raw(handle) }
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-openwindowstationw)\]
/// OpenWindowStationW
///
/// ### Example
/// ```
/// # use firehazard::*;
/// # use abistr::cstr16;
/// let winsta0 = open_window_station_w(cstr16!("WinSta0"), false, winsta::ALL_ACCESS).unwrap();
/// ```
pub fn open_window_station_w(
    winsta:         impl TryIntoAsCStr<u16>,
    inherit:        bool,
    desired_access: impl Into<winsta::AccessRights>,
) -> Result<winsta::OwnedHandle, Error> {
    let handle = unsafe { OpenWindowStationW(
        winsta.try_into().map_err(|_| Error(E_STRING_NOT_NULL_TERMINATED as _))?.as_cstr(),
        inherit as _,
        desired_access.into().into()
    )};
    Error::get_last_if(handle.is_null())?;
    unsafe { winsta::OwnedHandle::from_raw(handle) }
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setprocesswindowstation)\]
/// SetProcessWindowStation
///
/// ### Example
/// ```
/// # use firehazard::*;
/// # let winsta = open_process_window_station().unwrap();
/// set_process_window_station(&winsta).unwrap();
/// # std::mem::forget(winsta); // will ERROR_BUSY otherwise
/// ```
pub fn set_process_window_station(winsta: &winsta::OwnedHandle) -> Result<(), Error> {
    Error::get_last_if(FALSE == unsafe { SetProcessWindowStation(winsta.as_handle()) })
}
