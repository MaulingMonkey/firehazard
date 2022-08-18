/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-reverttoself)\] RevertToSelf
///
/// ### Example
/// ```
/// use win32_security_playground::*;
/// use winapi::shared::winerror::ERROR_BAD_TOKEN_TYPE;
/// use winapi::um::winnt::{SecurityDelegation, TokenImpersonation};
/// let token = open_process_token(get_current_process(), token::ALL_ACCESS).unwrap();
/// let imp = unsafe { duplicate_token_ex(
///     &token, token::ALL_ACCESS, None, SecurityDelegation, token::Impersonation
/// )};
///
/// open_thread_token(get_current_thread(), token::ALL_ACCESS, true).expect_err("no token was set, shouldn't have been able to open_thread_token");
/// revert_to_self().expect("wasn't impersonating, but revert_to_self should succeed anyways");
/// set_thread_token(None, &imp).unwrap();
/// open_thread_token(get_current_thread(), token::ALL_ACCESS, true).expect("token was set, should've read back successfully");
/// revert_to_self().expect("was impersonating, revert_to_self should've succeeded");
/// open_thread_token(get_current_thread(), token::ALL_ACCESS, true).expect_err("reverted token, shouldn't have been able to open_thread_token");
/// revert_to_self().expect("wasn't impersonating, should succeed anyways");
/// open_thread_token(get_current_thread(), token::ALL_ACCESS, true).expect_err("reverted token, shouldn't have been able to open_thread_token");
/// ```
pub fn revert_to_self() -> Result<(), crate::Error> {
    crate::Error::get_last_if(0 == unsafe { winapi::um::securitybaseapi::RevertToSelf() })
}
