#[doc(alias = "OpenDesktopA")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-opendesktopa)\]
/// OpenDesktopA
///
/// Open an existing desktop.
///
/// ### Example
/// ```
/// # use firehazard::*;
/// # use firehazard::access::*;
/// # use abistr::cstr;
/// let desktop = open_desktop_a(cstr!("Default"), None, false, GENERIC_ALL).unwrap();
/// ```
///
pub fn open_desktop_a(
    desktop:        impl TryIntoAsCStr,
    flags:          impl Into<desktop::Flags>,
    inherit:        bool,
    desired_access: impl Into<desktop::AccessRights>,
) -> firehazard::Result<desktop::OwnedHandle> {
    let handle = unsafe { winapi::um::winuser::OpenDesktopA(
        desktop.try_into()?.as_cstr(),
        flags.into().into(),
        inherit as _,
        desired_access.into().into()
    )};
    firehazard::Error::get_last_if(handle.is_null())?;
    unsafe { desktop::OwnedHandle::from_raw(handle) }
}



#[doc(alias = "OpenDesktop")]
#[doc(alias = "OpenDesktopW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-opendesktopw)\]
/// OpenDesktopW
///
/// Open an existing desktop.
///
/// ### Example
/// ```
/// # use firehazard::*;
/// # use firehazard::access::*;
/// # use abistr::cstr16;
/// let desktop = open_desktop_w(cstr16!("Default"), None, false, GENERIC_ALL).unwrap();
/// ```
///
pub fn open_desktop_w(
    desktop:        impl TryIntoAsCStr<u16>,
    flags:          impl Into<desktop::Flags>,
    inherit:        bool,
    desired_access: impl Into<desktop::AccessRights>,
) -> firehazard::Result<desktop::OwnedHandle> {
    let handle = unsafe { winapi::um::winuser::OpenDesktopW(
        desktop.try_into()?.as_cstr(),
        flags.into().into(),
        inherit as _,
        desired_access.into().into()
    )};
    firehazard::Error::get_last_if(handle.is_null())?;
    unsafe { desktop::OwnedHandle::from_raw(handle) }
}
