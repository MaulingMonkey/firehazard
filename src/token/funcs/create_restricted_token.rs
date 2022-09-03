/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-createrestrictedtoken)\]
/// CreateRestrictedToken
pub fn create_restricted_token(
    existing_token_handle:  &crate::token::OwnedHandle,
    flags:                  impl Into<crate::token::RestrictedFlags>,
    sids_to_disable:        Option<&[crate::sid::AndAttributes]>,
    privileges_to_delete:   Option<&[crate::privilege::LuidAndAttributes]>,
    sids_to_restrict:       Option<&[crate::sid::AndAttributes]>,
) -> Result<crate::token::OwnedHandle, crate::Error> {
    use crate::*;
    use winapi::shared::winerror::*;
    use core::ptr::*;

    let mut new_handle = null_mut();
    Error::get_last_if(0 == unsafe { winapi::um::securitybaseapi::CreateRestrictedToken(
        existing_token_handle.as_handle(),
        flags.into().into(),
        u32::try_from(sids_to_disable.map_or(0, |s| s.len())).map_err(|_| Error(ERROR_INVALID_PARAMETER))?,
        sids_to_disable.map_or(null_mut(), |s| s.as_ptr() as *mut _),
        u32::try_from(privileges_to_delete.map_or(0, |s| s.len())).map_err(|_| Error(ERROR_INVALID_PARAMETER))?,
        privileges_to_delete.map_or(null_mut(), |s| s.as_ptr() as *mut _),
        u32::try_from(sids_to_restrict.map_or(0, |s| s.len())).map_err(|_| Error(ERROR_INVALID_PARAMETER))?,
        sids_to_restrict.map_or(null_mut(), |s| s.as_ptr() as *mut _),
        &mut new_handle
    )})?;
    unsafe { token::OwnedHandle::from_raw(new_handle) }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] GetTokenInformation(self, TokenGroupsAndPrivileges, ...) +<br>
/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-createrestrictedtoken)\] CreateRestrictedToken
pub fn create_restricted_token_filter(
    existing_token_handle:  &crate::token::OwnedHandle,
    flags:                  impl Into<crate::token::RestrictedFlags>,
    sids_to_disable:        impl FnMut(&crate::sid::AndAttributes           ) -> bool,
    privileges_to_delete:   impl FnMut(&crate::privilege::LuidAndAttributes ) -> bool,
    sids_to_restrict:       Option<&[crate::sid::AndAttributes]>,
) -> Result<crate::token::OwnedHandle, crate::Error> {
    use crate::*;

    let mut gap = existing_token_handle.groups_and_privileges()?;
    let sids  = partition::in_place_unstable(gap.sids_mut(), sids_to_disable);
    let privs = partition::in_place_unstable(gap.privileges_mut(), privileges_to_delete);
    let sids_to_disable         = &gap.sids()[..sids];
    let privileges_to_delete    = &gap.privileges()[..privs];

    create_restricted_token(existing_token_handle, flags, Some(sids_to_disable), Some(privileges_to_delete), sids_to_restrict)
}
