#[doc(alias = "CreateRestrictedToken")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-createrestrictedtoken)\]
/// CreateRestrictedToken
///
pub fn create_restricted_token(
    existing_token_handle:  &token::OwnedHandle,
    flags:                  impl Into<token::RestrictedFlags>,
    sids_to_disable:        Option<&[sid::AndAttributes]>,
    privileges_to_delete:   Option<&[privilege::LuidAndAttributes]>,
    sids_to_restrict:       Option<&[sid::AndAttributes]>,
) -> firehazard::Result<token::OwnedHandle> {
    let mut new_handle = null_mut();
    firehazard::Error::get_last_if(0 == unsafe { winapi::um::securitybaseapi::CreateRestrictedToken(
        existing_token_handle.as_handle(),
        flags.into().into(),
        u32::try_from(sids_to_disable.map_or(0, |s| s.len())).map_err(|_| ERROR_INVALID_PARAMETER)?,
        sids_to_disable.map_or(null_mut(), |s| s.as_ptr() as *mut _),
        u32::try_from(privileges_to_delete.map_or(0, |s| s.len())).map_err(|_| ERROR_INVALID_PARAMETER)?,
        privileges_to_delete.map_or(null_mut(), |s| s.as_ptr() as *mut _),
        u32::try_from(sids_to_restrict.map_or(0, |s| s.len())).map_err(|_| ERROR_INVALID_PARAMETER)?,
        sids_to_restrict.map_or(null_mut(), |s| s.as_ptr() as *mut _),
        &mut new_handle
    )})?;
    unsafe { token::OwnedHandle::from_raw(new_handle) }
}



#[doc(alias = "CreateRestrictedToken")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] GetTokenInformation(self, TokenGroupsAndPrivileges, ...) +<br>
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-createrestrictedtoken)\] CreateRestrictedToken
///
pub fn create_restricted_token_filter(
    existing_token_handle:  &token::OwnedHandle,
    flags:                  impl Into<token::RestrictedFlags>,
    sids_to_disable:        impl FnMut(&sid::AndAttributes           ) -> bool,
    privileges_to_delete:   impl FnMut(&privilege::LuidAndAttributes ) -> bool,
    sids_to_restrict:       Option<&[sid::AndAttributes]>,
) -> firehazard::Result<token::OwnedHandle> {
    let mut gap = existing_token_handle.groups_and_privileges()?;
    let sids  = partition::in_place_unstable(gap.sids_mut(), sids_to_disable);
    let privs = partition::in_place_unstable(gap.privileges_mut(), privileges_to_delete);
    let sids_to_disable         = &gap.sids()[..sids];
    let privileges_to_delete    = &gap.privileges()[..privs];

    create_restricted_token(existing_token_handle, flags, Some(sids_to_disable), Some(privileges_to_delete), sids_to_restrict)
}
