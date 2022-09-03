/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openthreadtoken)\] OpenThreadToken
///
/// ### Example
/// ```
/// # use firehazard::*;
/// # use winapi::shared::winerror::*;
/// # let token = open_process_token(get_current_process(), token::ALL_ACCESS).unwrap();
/// # let token = duplicate_token_ex(&token, token::ALL_ACCESS, None, security::Delegation, token::Impersonation).unwrap();
/// let err = open_thread_token(get_current_thread(), token::ALL_ACCESS, false).unwrap_err();
/// assert_eq!(ERROR_NO_TOKEN, err);
///
/// set_thread_token(get_current_thread(), &token).unwrap();
/// let token = open_thread_token(get_current_thread(), token::ALL_ACCESS, false).unwrap();
///
/// set_thread_token(get_current_thread(), None).unwrap();
/// let err = open_thread_token(get_current_thread(), token::ALL_ACCESS, false).unwrap_err();
/// assert_eq!(ERROR_NO_TOKEN, err);
/// ```
///
/// ### Errors
/// *   `ERROR_NO_TOKEN`        if the thread isn't impersonating any token
/// *   `ERROR_INVALID_HANDLE`  if `thread` wasn't a valid thread handle (maybe it was a process handle?)
/// *   `ERROR_ACCESS_DENIED`   if the current process/thread token lacks the rights to open the token with `rights` (Untrusted integrity, missing SIDs, blocked by DACL, etc.)
pub fn open_thread_token<'a>(thread: impl AsRef<crate::thread::PsuedoHandle<'a>>, desired_access: impl Into<crate::token::AccessRights>, open_as_self: bool) -> Result<crate::token::OwnedHandle, crate::Error> {
    use crate::*;
    let mut h = core::ptr::null_mut();
    Error::get_last_if(0 == unsafe { winapi::um::processthreadsapi::OpenThreadToken(thread.as_ref().as_handle(), desired_access.into().into(), open_as_self as _, &mut h) })?;
    unsafe { token::OwnedHandle::from_raw(h) }
}
