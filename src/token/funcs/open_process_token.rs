/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocesstoken)\] OpenProcessToken
///
/// ### Example
/// ```
/// # use win32_security_playground::*;
/// let token : token::OwnedHandle = open_process_token(get_current_process(), token::ALL_ACCESS).unwrap();
/// ```
///
/// ### Errors
/// *   `ERROR_INVALID_HANDLE`  if `process` wasn't a valid process handle (maybe it was a thread handle?)
/// *   `ERROR_ACCESS_DENIED`   if the current process/thread token lacks the rights to open the token with `desired_access` (Untrusted integrity, missing SIDs, blocked by DACL, etc.)
pub fn open_process_token(process: impl AsRef<crate::process::Handle>, desired_access: impl Into<crate::token::AccessRights>) -> Result<crate::token::OwnedHandle, crate::Error> {
    let mut h = std::ptr::null_mut();
    crate::Error::get_last_if(0 == unsafe { winapi::um::processthreadsapi::OpenProcessToken(process.as_ref().as_handle(), desired_access.into().into(), &mut h) })?;
    if h.is_null() { return Err(crate::Error(winapi::shared::winerror::ERROR_INVALID_HANDLE)) }
    Ok(unsafe { crate::token::OwnedHandle::from_raw(h) })
}
