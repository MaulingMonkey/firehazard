/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-istokenrestricted)\] IsTokenRestricted
///
/// ### Example
/// ```
/// # use win32_security_playground::*;
/// let token : token::Handle = open_process_token::current_process().unwrap();
/// assert!(!is_token_restricted(&token));
/// ```
pub fn is_token_restricted(token: &crate::token::Handle) -> bool {
    0 != unsafe { winapi::um::securitybaseapi::IsTokenRestricted(token.as_handle()) }
}
