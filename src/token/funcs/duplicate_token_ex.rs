/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-duplicatetokenex)\]
/// DuplicateTokenEx
///
/// Deep clones the token handle, giving it it's own unique permissions list etc.
/// that can be modified without changing the permissions of the original `token`.
///
/// ### Safety
/// *   ~~`desired_access`      might need to be valid access rights~~ (already enforced by type?)
/// *   `token_attributes`      , if some, might need to contain a valid `SECURITY_DESCRIPTOR`
/// *   `impersontation_level`  might need to be a valid impresonation level
/// *   `token_type`            might need to be a valid token type
///
/// ### Example
/// ```
/// # use win32_security_playground::*;
/// use winapi::um::winnt::{SecurityDelegation, TokenPrimary, TOKEN_ALL_ACCESS};
/// let tok : token::Handle = open_process_token::current_process();
///
/// let dup : token::Handle = unsafe { duplicate_token_ex(
///     &tok, token::ALL_ACCESS, None, SecurityDelegation, TokenPrimary
/// )};
///
/// assert_ne!(tok.as_handle(), dup.as_handle());
/// ```
pub unsafe fn duplicate_token_ex(
    token:                  &crate::token::Handle,
    desired_access:         crate::token::AccessRights,
    _token_attributes:      Option<std::convert::Infallible>,
    impersonation_level:    winapi::um::winnt::SECURITY_IMPERSONATION_LEVEL,    // TODO: type wrapper?
    token_type:             winapi::um::winnt::TOKEN_TYPE,                      // TODO: type wrapper?
) -> crate::token::Handle {
    use crate::error::get_last_error;
    use std::ptr::null_mut;

    let mut new = null_mut();
    let success = 0 != unsafe { winapi::um::securitybaseapi::DuplicateTokenEx(token.as_handle(), desired_access.as_u32(), null_mut(), impersonation_level, token_type, &mut new) };
    assert!(success, "DuplicateTokenEx GetLastError()={}", get_last_error());

    unsafe { crate::token::Handle::from_raw(new) }
}
