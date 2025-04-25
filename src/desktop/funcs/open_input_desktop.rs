#[doc(alias = "OpenInputDesktop")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-openinputdesktop)\]
/// OpenInputDesktop
///
/// ### Example
/// ```
/// # use firehazard::*;
/// # use firehazard::access::*;
/// let desktop = open_input_desktop(None, false, GENERIC_ALL).unwrap();
/// ```
///
pub fn open_input_desktop(
    flags:          impl Into<desktop::Flags>,
    inherit:        bool,
    desired_access: impl Into<desktop::AccessRights>,
) -> firehazard::Result<desktop::OwnedHandle> {
    let handle = unsafe { winapi::um::winuser::OpenInputDesktop(
        flags.into().into(),
        inherit as _,
        desired_access.into().into(),
    )};
    firehazard::Error::get_last_if(handle.is_null())?;
    unsafe { desktop::OwnedHandle::from_raw(handle) }
}
