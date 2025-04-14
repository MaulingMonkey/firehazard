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
    object:     impl AsRef<firehazard::handle::Borrowed<'a>>,
    mask:       impl Into<firehazard::handle::FlagsMask>,
    flags:      impl Into<firehazard::handle::Flags>,
) -> Result<(), firehazard::Error> {
    use firehazard::*;

    Error::get_last_if(0 == unsafe { winapi::um::handleapi::SetHandleInformation(
        object.as_ref().as_handle(),
        mask.into().into(),
        flags.into().into(),
    )})
}
