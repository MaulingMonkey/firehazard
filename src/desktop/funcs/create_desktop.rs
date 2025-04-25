#[doc(alias = "CreateDesktopA")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createdesktopa)\]
/// CreateDesktopA
///
/// Create or open an existing desktop.
///
/// ### Example
/// ```
/// # use firehazard::*;
/// # use firehazard::access::*;
/// # use abistr::cstr;
/// let a = create_desktop_a(cstr!("create_desktop_a"), (), None, None, GENERIC_ALL, None).unwrap();
/// let b = create_desktop_a(cstr!("create_desktop_a"), (), None, None, GENERIC_ALL, None).unwrap();
/// ```
///
pub fn create_desktop_a(
    desktop:        impl TryIntoAsCStr,
    device:         impl TryIntoAsOptCStr,
    devmode:        Option<core::convert::Infallible>,
    flags:          impl Into<desktop::Flags>,
    desired_access: impl Into<desktop::AccessRights>,
    sa:             Option<&security::Attributes>,
) -> firehazard::Result<desktop::OwnedHandle> {
    let handle = unsafe { winapi::um::winuser::CreateDesktopA(
        desktop.try_into()?.as_cstr(),
        device.try_into()?.as_opt_cstr(),
        none2null(devmode),
        flags.into().into(),
        desired_access.into().into(),
        sa.map_or(null(), |sa| sa) as *mut _
    )};
    firehazard::Error::get_last_if(handle.is_null())?;
    unsafe { desktop::OwnedHandle::from_raw(handle) }
}



#[doc(alias = "CreateDesktop")]
#[doc(alias = "CreateDesktopW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createdesktopw)\]
/// CreateDesktopW
///
/// Create or open an existing desktop.
///
/// ### Example
/// ```
/// # use firehazard::*;
/// # use firehazard::access::*;
/// # use abistr::cstr16;
/// let a = create_desktop_w(cstr16!("create_desktop_w"), (), None, None, GENERIC_ALL, None).unwrap();
/// # let b = create_desktop_w(cstr16!("create_desktop_w"), (), None, None, GENERIC_ALL, None).unwrap();
/// ```
///
pub fn create_desktop_w(
    desktop:        impl TryIntoAsCStr<u16>,
    device:         impl TryIntoAsOptCStr<u16>,
    devmode:        Option<core::convert::Infallible>,
    flags:          impl Into<desktop::Flags>,
    desired_access: impl Into<desktop::AccessRights>,
    sa:             Option<&security::Attributes>,
) -> firehazard::Result<desktop::OwnedHandle> {
    let handle = unsafe { winapi::um::winuser::CreateDesktopW(
        desktop.try_into()?.as_cstr(),
        device.try_into()?.as_opt_cstr(),
        none2null(devmode),
        flags.into().into(),
        desired_access.into().into(),
        sa.map_or(null(), |sa| sa) as *mut _
    )};
    firehazard::Error::get_last_if(handle.is_null())?;
    unsafe { desktop::OwnedHandle::from_raw(handle) }
}



// https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createdesktopexa
// TODO: CreateDesktopExA: adds heap size + reserved pvoid



// https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createdesktopexw
// TODO: CreateDesktopExW: adds heap size + reserved pvoid
