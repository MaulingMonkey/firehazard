/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadtoken)\]
/// SetThreadToken
///
/// ### Example
/// ```
/// use win32_security_playground::*;
/// use winapi::shared::winerror::ERROR_BAD_TOKEN_TYPE;
/// use winapi::um::winnt::{SecurityDelegation, TokenImpersonation};
/// let token = open_process_token::current_process();
/// let imp = unsafe { duplicate_token_ex(
///     &token, token::ALL_ACCESS, None, SecurityDelegation, token::Impersonation
/// )};
///
/// set_thread_token(None, &imp).unwrap();
/// set_thread_token(None, None).unwrap();
/// assert_eq!(ERROR_BAD_TOKEN_TYPE, set_thread_token(None, &token).unwrap_err());
/// ```
///
/// ### Errors
/// *   `ERROR_BAD_TOKEN_TYPE`  if `token` is a primary token instead of an impersonation token
pub fn set_thread_token<'t>(thread: impl Into<Option<std::convert::Infallible>>, token: impl Into<Option<&'t crate::token::Handle>>) -> Result<(), crate::error::LastError> {
    use winapi::um::processthreadsapi::SetThreadToken;
    use std::ptr::null_mut;

    // TODO: accept real thread handles
    //use std::os::windows::io::AsRawHandle;
    //let mut thread = thread.map(|t| t.as_raw_handle().cast());
    //let thread = thread.as_mut().map_or(null_mut(), |t| t);
    let _ = thread.into();
    let thread = null_mut();

    let token = token.into();
    let token = token.map_or(null_mut(), |t| t.as_handle());

    let success = 0 != unsafe { SetThreadToken(thread, token) };

    if success { Ok(()) } else { Err(crate::error::LastError::get()) }
}
