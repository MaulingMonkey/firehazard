/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-duplicatetokenex)\]
/// DuplicateTokenEx
///
/// Deep clones the token handle, giving it it's own unique permissions list etc.
/// that can be modified without changing the permissions of the original `token`.
///
/// ### Safety
/// *   ~~`desired_access`      might need to be valid access rights~~ (already enforced by type?)
/// *   `impersontation_level`  might need to be a valid impresonation level
/// *   `token_type`            might need to be a valid token type
///
/// ### Example
/// ```
/// # use win32_security_playground::*;
/// use winapi::um::winnt::SecurityDelegation;
/// let tok : token::OwnedHandle = open_process_token(get_current_process(), token::ALL_ACCESS).unwrap();
///
/// let dup : token::OwnedHandle = unsafe { duplicate_token_ex(
///     &tok, token::ALL_ACCESS, None, SecurityDelegation, token::Primary
/// )}.unwrap();
///
/// assert_ne!(tok.as_handle(), dup.as_handle());
/// ```
pub unsafe fn duplicate_token_ex(
    token:                  &crate::token::OwnedHandle,
    desired_access:         impl Into<crate::token::AccessRights>,
    token_attributes:       Option<&crate::security::Attributes>,
    impersonation_level:    winapi::um::winnt::SECURITY_IMPERSONATION_LEVEL,    // TODO: type wrapper?
    token_type:             crate::token::Type,
) -> Result<crate::token::OwnedHandle, crate::Error> {
    use std::ptr::{null, null_mut};

    let mut new = null_mut();
    crate::Error::get_last_if(0 == unsafe { winapi::um::securitybaseapi::DuplicateTokenEx(
        token.as_handle(),
        desired_access.into().into(),
        token_attributes.map_or(null(), |a| a) as *mut _,
        impersonation_level,
        token_type.into(),
        &mut new
    )})?;

    Ok(unsafe { crate::token::OwnedHandle::from_raw(new) })
}
