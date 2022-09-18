/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadtoken)\]
/// SetThreadToken
///
/// ### Example
/// ```
/// use firehazard::*;
/// use winapi::shared::winerror::ERROR_BAD_TOKEN_TYPE;
/// let token = open_process_token(get_current_process(), token::ALL_ACCESS).unwrap();
/// let imp = duplicate_token_ex(
///     &token, token::ALL_ACCESS, None, security::Delegation, token::Impersonation
/// ).unwrap();
///
/// set_thread_token(None,                  &imp).unwrap();
/// set_thread_token(get_current_thread(),  &imp).unwrap();
/// set_thread_token(None,                  None).unwrap();
/// set_thread_token(get_current_thread(),  None).unwrap();
/// assert_eq!(ERROR_BAD_TOKEN_TYPE, set_thread_token(None, &token).unwrap_err());
/// ```
///
/// ### Errors
/// *   `ERROR_BAD_TOKEN_TYPE`  if `token` is a primary token instead of an impersonation token
pub fn set_thread_token<'t>(thread: impl crate::thread::AsHandleOrNone, token: impl Into<Option<&'t crate::token::OwnedHandle>>) -> Result<(), crate::Error> {
    use crate::*;
    use winapi::um::processthreadsapi::SetThreadToken;
    use core::ptr::null_mut;

    let mut thread = thread.as_handle_or_none();
    let thread = thread.as_mut().map_or(null_mut(), |t| t);

    let token = token.into();
    let token = token.map_or(null_mut(), |t| t.as_handle());

    Error::get_last_if(0 == unsafe { SetThreadToken(thread, token) })
}
