/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openthreadtoken)\] OpenThreadToken
///
/// ### Example
/// ```
/// # use win32_security_playground::*;
/// # use winapi::shared::winerror::*;
/// assert_eq!(open_thread_token(get_current_thread(), token::ALL_ACCESS, false).unwrap_err(), ERROR_NO_TOKEN);
/// // TODO: set/verify token and unwrap some
/// ```
///
/// ### Errors
/// *   `ERROR_NO_TOKEN`        if the thread isn't impersonating any token?
/// *   `ERROR_INVALID_HANDLE`  if `thread` wasn't a valid thread handle (maybe it was a process handle?)
/// *   `ERROR_ACCESS_DENIED`   if the current process/thread token lacks the rights to open the token with `rights` (Untrusted integrity, missing SIDs, blocked by DACL, etc.)
pub fn open_thread_token(thread: impl AsRef<crate::thread::Handle>, desired_access: impl Into<crate::token::AccessRights>, open_as_self: bool) -> Result<crate::token::Handle, crate::Error> {
    let mut h = std::ptr::null_mut();
    crate::Error::get_last_if(0 == unsafe { winapi::um::processthreadsapi::OpenThreadToken(thread.as_ref().as_handle(), desired_access.into().into(), open_as_self as _, &mut h) })?;
    if h.is_null() { return Err(crate::Error(winapi::shared::winerror::ERROR_INVALID_HANDLE)) }
    Ok(unsafe { crate::token::Handle::from_raw(h) })
}
