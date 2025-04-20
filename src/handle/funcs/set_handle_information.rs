#[doc(alias = "SetHandleInformation")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-sethandleinformation)\]
/// SetHandleInformation
///
/// ### Example
/// ```
/// # #[cfg(std)] {
/// # use firehazard::*;
/// # use winapi::shared::winerror::*;
/// let thread : thread::OwnedHandle = std::thread::spawn(||{}).into();
/// let info = get_handle_information(&thread).unwrap();
/// set_handle_information(&thread, !0, info).unwrap();
/// close_handle(thread).unwrap();
/// # }
/// ```
///
pub fn set_handle_information<'a>(
    object:     impl Into<handle::Borrowed<'a>>,
    mask:       impl Into<handle::FlagsMask>,
    flags:      impl Into<handle::Flags>,
) -> firehazard::Result<()> {
    firehazard::Error::get_last_if(0 == unsafe { winapi::um::handleapi::SetHandleInformation(
        object.into().as_handle(),
        mask.into().into(),
        flags.into().into(),
    )})
}
