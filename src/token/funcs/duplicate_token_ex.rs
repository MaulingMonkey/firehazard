#[doc(alias = "DuplicateTokenEx")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-duplicatetokenex)\]
/// DuplicateTokenEx
///
/// Deep clones the token handle, giving it it's own unique permissions list etc.
/// that can be modified without changing the permissions of the original `token`.
///
/// ### Example
/// ```
/// # use firehazard::*;
/// let tok : token::OwnedHandle = open_process_token(get_current_process(), token::ALL_ACCESS).unwrap();
///
/// let dup : token::OwnedHandle = duplicate_token_ex(
///     &tok, token::ALL_ACCESS, None, security::Delegation, token::Primary
/// ).unwrap();
///
/// assert_ne!(tok.as_handle(), dup.as_handle());
/// ```
///
pub fn duplicate_token_ex(
    token:                  &crate::token::OwnedHandle,
    desired_access:         impl Into<crate::token::AccessRights>,
    token_attributes:       Option<&crate::security::Attributes>,
    impersonation_level:    crate::security::ImpersonationLevel,
    token_type:             crate::token::Type,
) -> firehazard::Result<crate::token::OwnedHandle> {
    let mut new = null_mut();
    firehazard::Error::get_last_if(0 == unsafe { winapi::um::securitybaseapi::DuplicateTokenEx(
        token.as_handle(),
        desired_access.into().into(),
        token_attributes.map_or(null(), |a| a) as *mut _,
        impersonation_level.into(),
        token_type.into(),
        &mut new
    )})?;

    unsafe { token::OwnedHandle::from_raw(new) }
}
