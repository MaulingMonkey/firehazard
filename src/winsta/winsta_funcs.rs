use crate::prelude::*;
use winapi::ctypes::c_char;
use winapi::shared::minwindef::LPARAM;
use winapi::um::errhandlingapi::SetLastError;
use winapi::um::winuser::*;



#[doc(no_inline)] pub use create_window_station_w as create_window_station;



#[doc(alias = "CreateWindowStationA")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowstationa)\]
/// CreateWindowStationA
///
/// ### Example
/// ```
/// # use firehazard::*;
/// # use abistr::cstr;
/// # use winapi::shared::winerror::*;
/// let err = create_window_station_a(cstr!("WinSta0"), winsta::CWF_CREATE_ONLY, winsta::ALL_ACCESS, None).unwrap_err();
/// assert!(err == ERROR_ACCESS_DENIED || err == ERROR_ALREADY_EXISTS, "create_window_station_a(\"WinSta0\", ...) failed with error: {err:?}");
/// let winsta = create_window_station_a((), (), winsta::ALL_ACCESS, None).unwrap();
/// ```
///
/// ### Errors
/// *   `ERROR_ACCESS_DENIED`   if the window station already exists on Windows 10
/// *   `ERROR_ALREADY_EXISTS`  if the window station already exists on Windows Server 2019
///
pub fn create_window_station_a(
    winsta:         impl string::InOptionalNarrow,
    flags:          impl Into<winsta::CreateWindowFlags>,
    desired_access: impl Into<winsta::AccessRights>,
    sa:             Option<&security::Attributes>,
) -> firehazard::Result<winsta::OwnedHandle> {
    let flags           = flags.into().into();
    let desired_access  = desired_access.into().into();
    let sa              = sa.map_or(null(), |sa| sa) as *mut _;

    string::convert_to_cstr::<{limit::stack::WINSTA_NAME}, _, _>(winsta, |winsta| {
        let handle = unsafe { CreateWindowStationA(
            winsta.as_opt_cstr(), flags, desired_access, sa,
        )};
        firehazard::Error::get_last_if(handle.is_null())?;
        unsafe { winsta::OwnedHandle::from_raw(handle) }
    })?
}



#[doc(alias = "CreateWindowStation")]
#[doc(alias = "CreateWindowStationW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowstationw)\]
/// CreateWindowStationW
///
/// ### Example
/// ```
/// # use firehazard::*;
/// # use abistr::cstr16;
/// # use winapi::shared::winerror::*;
/// let err = create_window_station_w(cstr16!("WinSta0"), winsta::CWF_CREATE_ONLY, winsta::ALL_ACCESS, None).unwrap_err();
/// assert!(err == ERROR_ACCESS_DENIED || err == ERROR_ALREADY_EXISTS, "create_window_station_w(L\"WinSta0\", ...) failed with error: {err:?}");
/// let winsta = create_window_station_w((), (), winsta::ALL_ACCESS, None).unwrap();
/// ```
///
/// ### Errors
/// *   `ERROR_ACCESS_DENIED`   if the window station already exists on Windows 10
/// *   `ERROR_ALREADY_EXISTS`  if the window station already exists on Windows Server 2019
///
pub fn create_window_station_w(
    winsta:         impl string::InOptionalWide,
    flags:          impl Into<winsta::CreateWindowFlags>,
    desired_access: impl Into<winsta::AccessRights>,
    sa:             Option<&security::Attributes>,
) -> firehazard::Result<winsta::OwnedHandle> {
    let flags           = flags.into().into();
    let desired_access  = desired_access.into().into();
    let sa              = sa.map_or(null(), |sa| sa) as *mut _;

    string::convert_to_cstr::<{limit::stack::WINSTA_NAME}, _, _>(winsta, |winsta| {
        let handle = unsafe { CreateWindowStationW(
            winsta.as_opt_cstr(), flags, desired_access, sa,
        )};
        firehazard::Error::get_last_if(handle.is_null())?;
        unsafe { winsta::OwnedHandle::from_raw(handle) }
    })?
}



#[doc(alias = "CloseWindowStation")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-closewindowstation)\]
/// CloseWindowStation
///
/// ### Example
/// ```
/// # use abistr::cstr16;
/// # use firehazard::*;
/// let winsta = create_window_station_w((), (), winsta::ALL_ACCESS, None).unwrap();
/// close_window_station(winsta).unwrap();
/// ```
///
// ### Errors
// TODO
//
pub fn close_window_station(
    winsta:     impl Into<winsta::OwnedHandle>,
) -> firehazard::Result<()> {
    firehazard::Error::get_last_if(0 == unsafe { winapi::um::winuser::CloseWindowStation(
        core::mem::ManuallyDrop::new(winsta.into()).as_handle()
    )})
}



#[doc(alias = "EnumWindowStationsA")]
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
///
pub fn enum_window_stations_a<
    F: FnMut(CStrPtr) -> firehazard::Result<()>
>(
    mut enum_func: F,
) -> firehazard::Result<()> {
    let enum_func : *mut F = &mut enum_func;
    firehazard::Error::get_last_if(FALSE == unsafe { EnumWindowStationsA(
        Some(fwd_enum_window_stations_a::<F>),
        enum_func as LPARAM,
    )})
}

unsafe extern "system" fn fwd_enum_window_stations_a<
    F: FnMut(CStrPtr) -> firehazard::Result<()>
>(
    winsta:     *mut c_char,
    param:      LPARAM,
) -> BOOL {
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



#[doc(alias = "EnumWindowStations")]
#[doc(alias = "EnumWindowStationsW")]
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
///
pub fn enum_window_stations_w<
    F: FnMut(CStrPtr<u16>) -> firehazard::Result<()>,
>(
    mut enum_func: F,
) -> firehazard::Result<()> {
    let enum_func : *mut F = &mut enum_func;
    firehazard::Error::get_last_if(FALSE == unsafe { EnumWindowStationsW(
        Some(fwd_enum_window_stations_w::<F>),
        enum_func as LPARAM,
    )})
}

unsafe extern "system" fn fwd_enum_window_stations_w<
    F: FnMut(CStrPtr<u16>) -> firehazard::Result<()>
>(
    winsta:         *mut u16,
    param:          LPARAM,
) -> BOOL {
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



#[doc(alias = "GetProcessWindowStation")]
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
///
pub fn open_process_window_station() -> firehazard::Result<winsta::OwnedHandle> {
    // "Do not close the handle returned by this function." - so we return a closeable clone instead
    let winsta = unsafe { GetProcessWindowStation() };
    unsafe { winsta::OwnedHandle::borrow_from_raw(&winsta)? }.try_clone_to_owned()
}



#[doc(no_inline)] pub use open_window_station_w as open_window_station;



#[doc(alias = "OpenWindowStationA")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-openwindowstationa)\]
/// OpenWindowStationA
///
/// ### Example
/// ```
/// # use firehazard::*;
/// # use abistr::cstr;
/// let winsta0 = open_window_station_a(cstr!("WinSta0"), false, winsta::ALL_ACCESS).unwrap();
/// ```
///
pub fn open_window_station_a(
    winsta:         impl string::InNarrow,
    inherit:        bool,
    desired_access: impl Into<winsta::AccessRights>,
) -> firehazard::Result<winsta::OwnedHandle> {
    let inherit         = inherit as _;
    let desired_access  = desired_access.into().into();

    string::convert_to_cstrnn::<{limit::stack::WINSTA_NAME}, _, _>(winsta, |winsta| {
        let handle = unsafe { OpenWindowStationA(
            winsta.as_cstr(), inherit, desired_access,
        )};
        firehazard::Error::get_last_if(handle.is_null())?;
        unsafe { winsta::OwnedHandle::from_raw(handle) }
    })?
}



#[doc(alias = "OpenWindowStation")]
#[doc(alias = "OpenWindowStationW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-openwindowstationw)\]
/// OpenWindowStationW
///
/// ### Example
/// ```
/// # use firehazard::*;
/// # use abistr::cstr16;
/// let winsta0 = open_window_station_w(cstr16!("WinSta0"), false, winsta::ALL_ACCESS).unwrap();
/// ```
///
pub fn open_window_station_w(
    winsta:         impl string::InWide,
    inherit:        bool,
    desired_access: impl Into<winsta::AccessRights>,
) -> firehazard::Result<winsta::OwnedHandle> {
    let inherit         = inherit as _;
    let desired_access  = desired_access.into().into();

    string::convert_to_cstrnn::<{limit::stack::WINSTA_NAME}, _, _>(winsta, |winsta| {
        let handle = unsafe { OpenWindowStationW(
            winsta.as_cstr(), inherit, desired_access,
        )};
        firehazard::Error::get_last_if(handle.is_null())?;
        unsafe { winsta::OwnedHandle::from_raw(handle) }
    })?
}



#[doc(alias = "SetProcessWindowStation")]
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
///
pub fn set_process_window_station(winsta: &winsta::OwnedHandle) -> firehazard::Result<()> {
    firehazard::Error::get_last_if(FALSE == unsafe { SetProcessWindowStation(winsta.as_handle()) })
}
