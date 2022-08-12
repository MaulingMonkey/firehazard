/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-reverttoself)\] RevertToSelf
///
/// ### Example
/// ```
/// use win32_security_playground::*;
/// use winapi::shared::winerror::ERROR_BAD_TOKEN_TYPE;
/// use winapi::um::winnt::{SecurityDelegation, TokenImpersonation};
/// let token = open_process_token::current_process(token::ALL_ACCESS).unwrap();
/// let imp = unsafe { duplicate_token_ex(
///     &token, token::ALL_ACCESS, None, SecurityDelegation, token::Impersonation
/// )};
///
/// assert!(open_thread_token::current_thread(false).is_none());
/// revert_to_self().unwrap(); // wasn't impersonating, should succeed anyways
/// set_thread_token(None, &imp).unwrap();
/// assert!(open_thread_token::current_thread(false).is_some());
/// revert_to_self().unwrap(); // was impersonating
/// assert!(open_thread_token::current_thread(false).is_none());
/// revert_to_self().unwrap(); // wasn't impersonating, should succeed anyways
/// assert!(open_thread_token::current_thread(false).is_none());
/// ```
pub fn revert_to_self() -> Result<(), crate::error::LastError> {
    let success = 0 != unsafe { winapi::um::securitybaseapi::RevertToSelf() };
    if success { Ok(()) } else { Err(crate::error::LastError::get()) }
}
