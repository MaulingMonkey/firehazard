#[doc(alias = "IsTokenRestricted")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-istokenrestricted)\]
/// IsTokenRestricted
///
/// ### Example
/// ```
/// # use firehazard::*;
/// let token : token::OwnedHandle = open_process_token(get_current_process(), token::ALL_ACCESS).unwrap();
/// assert!(!is_token_restricted(&token));
/// ```
///
pub fn is_token_restricted(token: &crate::token::OwnedHandle) -> bool {
    0 != unsafe { winapi::um::securitybaseapi::IsTokenRestricted(token.as_handle()) }
}
