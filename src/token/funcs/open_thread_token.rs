/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openthreadtoken)\] OpenThreadToken(GetCurrentThread(), TOKEN_ALL_ACCESS, ...)
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
pub fn open_thread_token(thread: impl crate::thread::AsHandle, desired_access: impl Into<crate::token::AccessRights>, open_as_self: bool) -> Result<crate::token::Handle, crate::error::LastError> {
    let mut h = std::ptr::null_mut();
    let success = 0 != unsafe { winapi::um::processthreadsapi::OpenThreadToken(thread.as_handle(), desired_access.into().into(), open_as_self as _, &mut h) };
    let h = unsafe { crate::token::Handle::from_raw(h) };

    if      !success                { Err(crate::error::LastError::get()) }
    else if h.as_handle().is_null() { Err(crate::error::LastError(winapi::shared::winerror::ERROR_NO_TOKEN)) }
    else                            { Ok(h) }
}
