#[doc(alias = "GetHandleInformation")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-gethandleinformation)\]
/// GetHandleInformation
///
/// ### Example
/// ```
/// # #[cfg(std)] {
/// # use firehazard::*;
/// # use winapi::shared::winerror::*;
/// let thread : thread::OwnedHandle = std::thread::spawn(||{}).into();
/// let info = get_handle_information(&thread).unwrap();
/// assert_eq!(info, handle::Flags::default());
/// close_handle(thread).unwrap();
/// # }
/// ```
///
pub fn get_handle_information<'a>(
    object:     impl Into<handle::Borrowed<'a>>,
) -> firehazard::Result<handle::Flags> {
    let mut flags = 0;
    firehazard::Error::get_last_if(0 == unsafe { winapi::um::handleapi::GetHandleInformation(
        object.into().as_handle(),
        &mut flags,
    )})?;
    Ok(unsafe { handle::Flags::from_unchecked(flags) })
}
