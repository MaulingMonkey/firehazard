#[doc(no_inline)] pub use create_desktop_w as create_desktop;



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
/// let a = create_desktop_a(c"create_desktop_a", (), None, None, GENERIC_ALL, None).unwrap();
/// let b = create_desktop_a(c"create_desktop_a", (), None, None, GENERIC_ALL, None).unwrap();
/// ```
///
pub fn create_desktop_a(
    desktop:        impl string::InNarrow,
    device:         impl string::InOptionalNarrow,
    devmode:        Option<core::convert::Infallible>,
    flags:          impl Into<desktop::Flags>,
    desired_access: impl Into<desktop::AccessRights>,
    sa:             Option<&security::Attributes>,
) -> firehazard::Result<desktop::OwnedHandle> {
    let flags           = flags.into().into();
    let desired_access  = desired_access.into().into();
    let devmode         = none2null(devmode);
    let sa              = sa.map_or(null(), |sa| sa) as *mut _;

    string::convert_to_cstrnn::<{limit::stack::DESKTOP_NAME}, _, _>(desktop, |desktop| string::convert_to_cstr::<{limit::stack::DESKTOP_NAME}, _, _>(device, |device| {
        let handle = unsafe { winapi::um::winuser::CreateDesktopA(
            desktop.as_cstr(), device.as_opt_cstr(), devmode, flags, desired_access, sa
        )};
        firehazard::Error::get_last_if(handle.is_null())?;
        unsafe { desktop::OwnedHandle::from_raw(handle) }
    }))??
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
    desktop:        impl string::InWide,
    device:         impl string::InOptionalWide,
    devmode:        Option<core::convert::Infallible>,
    flags:          impl Into<desktop::Flags>,
    desired_access: impl Into<desktop::AccessRights>,
    sa:             Option<&security::Attributes>,
) -> firehazard::Result<desktop::OwnedHandle> {
    let flags           = flags.into().into();
    let desired_access  = desired_access.into().into();
    let devmode         = none2null(devmode);
    let sa              = sa.map_or(null(), |sa| sa) as *mut _;

    string::convert_to_cstrnn::<{limit::stack::DESKTOP_NAME}, _, _>(desktop, |desktop| string::convert_to_cstr::<{limit::stack::DESKTOP_NAME}, _, _>(device, |device| {
        let handle = unsafe { winapi::um::winuser::CreateDesktopW(
            desktop.as_cstr(), device.as_opt_cstr(), devmode, flags, desired_access, sa
        )};
        firehazard::Error::get_last_if(handle.is_null())?;
        unsafe { desktop::OwnedHandle::from_raw(handle) }
    }))??
}



// https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createdesktopexa
// TODO: CreateDesktopExA: adds heap size + reserved pvoid



// https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createdesktopexw
// TODO: CreateDesktopExW: adds heap size + reserved pvoid
