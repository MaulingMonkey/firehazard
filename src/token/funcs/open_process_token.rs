#[doc(alias = "OpenProcessToken")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocesstoken)\]
/// OpenProcessToken
///
/// ### Example
/// ```
/// # use firehazard::*;
/// let token = open_process_token(get_current_process(), token::ALL_ACCESS).unwrap();
/// ```
///
/// ### Errors
/// *   `ERROR_INVALID_HANDLE`  if `process` wasn't a valid process handle (maybe it was a thread handle?)
/// *   `ERROR_ACCESS_DENIED`   if the current process/thread token lacks the rights to open the token with `desired_access` (Untrusted integrity, missing SIDs, blocked by DACL, etc.)
///
pub fn open_process_token<'a>(
    process:            impl Into<crate::process::PseudoHandle<'a>>,
    desired_access:     impl Into<crate::token::AccessRights>,
) -> Result<crate::token::OwnedHandle, crate::Error> {
    use crate::*;
    let mut h = core::ptr::null_mut();
    Error::get_last_if(0 == unsafe { winapi::um::processthreadsapi::OpenProcessToken(
        process.into().as_handle(),
        desired_access.into().into(),
        &mut h,
    )})?;
    unsafe { token::OwnedHandle::from_raw(h) }
}
