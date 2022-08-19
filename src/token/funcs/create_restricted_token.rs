/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-createrestrictedtoken)\]
/// CreateRestrictedToken
///
/// ### Safety
/// *   `flags` might need to be valid?
/// *   excessive slice lengths might cause buffer overflows?
pub unsafe fn create_restricted_token(
    existing_token_handle:  &crate::token::OwnedHandle,
    flags:                  u32,
    sids_to_disable:        Option<&[crate::sid::AndAttributes]>,
    privileges_to_delete:   Option<&[crate::privilege::LuidAndAttributes]>,
    sids_to_restrict:       Option<&[crate::sid::AndAttributes]>,
) -> Result<crate::token::OwnedHandle, crate::Error> {
    use crate::*;
    use winapi::shared::winerror::*;
    use std::ptr::*;

    let mut new_handle = null_mut();
    Error::get_last_if(0 == unsafe { winapi::um::securitybaseapi::CreateRestrictedToken(
        existing_token_handle.as_handle(),
        flags,
        u32::try_from(sids_to_disable.map_or(0, |s| s.len())).map_err(|_| Error(ERROR_INVALID_PARAMETER))?,
        sids_to_disable.map_or(null_mut(), |s| s.as_ptr() as *mut _),
        u32::try_from(privileges_to_delete.map_or(0, |s| s.len())).map_err(|_| Error(ERROR_INVALID_PARAMETER))?,
        privileges_to_delete.map_or(null_mut(), |s| s.as_ptr() as *mut _),
        u32::try_from(sids_to_restrict.map_or(0, |s| s.len())).map_err(|_| Error(ERROR_INVALID_PARAMETER))?,
        sids_to_restrict.map_or(null_mut(), |s| s.as_ptr() as *mut _),
        &mut new_handle
    )})?;
    Ok(unsafe { token::OwnedHandle::from_raw(new_handle) })
}
