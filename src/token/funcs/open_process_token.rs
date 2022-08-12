/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocesstoken)\] OpenProcessToken
///
/// ### Example
/// ```
/// # use win32_security_playground::*;
/// let token : token::Handle = open_process_token(get_current_process(), token::ALL_ACCESS).unwrap();
/// ```
///
/// ### Errors
/// *   `ERROR_INVALID_HANDLE`  if `process` wasn't a valid process handle (maybe it was a thread handle?)
/// *   `ERROR_ACCESS_DENIED`   if the current process/thread token lacks the rights to open the token with `desired_access` (Untrusted integrity, missing SIDs, blocked by DACL, etc.)
pub fn open_process_token(process: impl crate::process::AsHandle, desired_access: impl Into<crate::token::AccessRights>) -> Result<crate::token::Handle, crate::error::LastError> {
    let mut h = std::ptr::null_mut();
    let success = 0 != unsafe { winapi::um::processthreadsapi::OpenProcessToken(process.as_handle(), desired_access.into().into(), &mut h) };
    let h = unsafe { crate::token::Handle::from_raw(h) };

    if      !success                { Err(crate::error::LastError::get()) }
    else if h.as_handle().is_null() { Err(crate::error::LastError(winapi::shared::winerror::ERROR_INVALID_HANDLE)) }
    else                            { Ok(h) }
}
