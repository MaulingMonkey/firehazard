use crate::*;
use crate::alloc::{CVec, LocalAllocFree};

use winapi::shared::winerror::*;
use winapi::um::userenv::*;

use abistr::*;

use core::ptr::null_mut;



// https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-checktokencapability
// CheckTokenCapability

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/userenv/nf-userenv-createappcontainerprofile)\]
/// CreateAppContainerProfile
///
/// Creates a per-user, per-app profile.
///
/// ### Examples
/// ```
/// # use firehazard::*;
/// # use abistr::*;
/// let app_container_name = cstr16!("firehazard create_app_container_profile doc example");
/// # let _ = delete_app_container_profile(app_container_name);
/// let app_container_sid = create_app_container_profile(
///     app_container_name,
///     cstr16!("firehazard: docs: create_app_container_profile example"),
///     cstr16!("Deletable temporary app container for create_app_container_profile example"),
///     &[] // no capabilities
/// ).expect("create_app_container_profile");
///
/// assert_eq!(
///     "S-1-15-2-382764940-2791294323-2722217349-2365663742-3629851628-1828535050-2293899398",
///     format!("{app_container_sid:?}")
/// );
///
/// delete_app_container_profile(app_container_name).expect("delete_app_container_profile");
/// ```
///
/// ### Arguments
/// *   `app_container_name`    - The application name.  Should contain the publisher name as well for uniqueness sake.<br>
///                               Can be up to 64 characters in length, matching the regular expression `[-_. A-Za-z0-9]+`.
/// *   `display_name`          - A friendly display name, up to 512 characters in length.
/// *   `description`           - A friendly description of the app, up to 2048 characters in length.
/// *   `capabilities`          - SIDs describing the capabilities of an app.  Attributes should probably be all `0`.
///
/// ### Errors
/// *   `E_ACCESSDENIED`                            - The caller does not have permission to create the profile.
/// *   `E_INVALIDARG`                              - If an argument is not valid (illegal chars? exceeds length limits?)
/// *   `HRESULT_FROM_WIN32(ERROR_ALREADY_EXISTS)`  - The application data store already exists.
pub fn create_app_container_profile(
    app_container_name: impl TryIntoAsCStr<u16>,
    display_name:       impl TryIntoAsCStr<u16>,
    description:        impl TryIntoAsCStr<u16>,
    capabilities:       &[sid::AndAttributes],
) -> Result<sid::Box<alloc::FreeSid>, Error> {
    let len32 : u32 = capabilities.len().try_into().map_err(|_| E_INVALIDARG)?;
    let mut app_container_sid = null_mut();
    let hr = unsafe { CreateAppContainerProfile(
        app_container_name.try_into().map_err(|_| E_INVALIDARG)?.as_cstr(),
        display_name.try_into().map_err(|_| E_INVALIDARG)?.as_cstr(),
        description.try_into().map_err(|_| E_INVALIDARG)?.as_cstr(),
        if len32 == 0 { null_mut() } else { capabilities.as_ptr() as *mut _ },
        len32,
        &mut app_container_sid,
    )};
    let app_container_sid = unsafe { sid::Box::from_raw(app_container_sid.cast()) }.ok_or(ERROR_INVALID_SID);
    if !SUCCEEDED(hr) { Err(hr)? }
    Ok(app_container_sid?)
}

/// \[<strike>microsoft.com</strike>\] CreateAppContainerToken
///
/// An undocumented `kernelbase.dll` API for creating an app container token, used by Chromium etc.
///
/// ### Examples
/// ```
/// # use firehazard::*;
/// # use abistr::*;
/// let process_token = open_process_token(get_current_process(), token::ALL_ACCESS).unwrap();
///
/// let app_container_sid = sid::NULL; // would cause ERROR_NOT_APPCONTAINER
/// let app_container_sid = derive_app_container_sid_from_app_container_name(
///     cstr16!("firehazard create_app_container_token example")
/// ).unwrap();
/// let capabilities : &[sid::AndAttributes] = &[
///     //sid::AndAttributes::new(sid::NULL, None), // would cause ERROR_INVALID_PARAMETER
///     // TODO: example of a valid capability sid
/// ];
/// let capabilities = security::Capabilities::from((&*app_container_sid, capabilities));
///
/// let appcontainer_token = create_app_container_token(&process_token, &capabilities).unwrap();
/// # if std::env::var_os("CI").is_none() { // Seems to fail on Windows Server 2019
/// assert!(appcontainer_token.has_restrictions().unwrap());
/// # }
/// assert!(0 != appcontainer_token.app_container_number().unwrap());
/// ```
///
/// ### Errors
/// *   `ERROR_CALL_NOT_IMPLEMENTED`    - if `kernelbase.dll` failed to load
/// *   `ERROR_CALL_NOT_IMPLEMENTED`    - if `kernelbase.dll` was missing `CreateAppContainerToken`
/// *   `ERROR_INVALID_HANDLE`          - if `token` is invalid (or wrong type of token?)
/// *   `ERROR_INVALID_PARAMETER`       - if `security.capabilities()` contains non-capability SIDs
/// *   `ERROR_NOT_APPCONTAINER`        - if `security.app_container_sid()` is not an app container SID
/// *   ???
///
/// ### References
/// *   <https://github.com/chromium/chromium/commit/9d4152381ceebbd1445489daa4f45f3d728a213c>
#[cfg(std)] // minidl requires std for now: https://github.com/MaulingMonkey/minidl/issues/1
pub fn create_app_container_token<'a>(
    token:      impl AsRef<token::Handle<'a>>,
    security:   &security::Capabilities<'a>,
) -> Result<token::OwnedHandle, Error> {
    use winapi::shared::minwindef::*;
    use winapi::um::winnt::*;

    lazy_static::lazy_static! {
        static ref CREATE_APP_CONTAINER_TOKEN : Option<unsafe extern "system" fn(token_handle: HANDLE, security_capabilities: *const SECURITY_CAPABILITIES, out_token: PHANDLE) -> BOOL> = {
            minidl::Library::load("kernelbase.dll").ok().and_then(|lib| unsafe { lib.sym_opt("CreateAppContainerToken\0") })
        };
    }
    #[allow(non_snake_case)] let CreateAppContainerToken = (*CREATE_APP_CONTAINER_TOKEN).ok_or(ERROR_CALL_NOT_IMPLEMENTED)?;

    let mut out_token = null_mut();
    Error::get_last_if(0 == unsafe { CreateAppContainerToken(
        token.as_ref().as_handle(),
        security.into(),
        &mut out_token,
    )})?;
    unsafe { token::OwnedHandle::from_raw(out_token) }
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/userenv/nf-userenv-deleteappcontainerprofile)\]
/// DeleteAppContainerProfile
///
/// Deletes the specified per-user, per-app profile.
/// Succeeds on non-existent profiles.
///
/// ### Examples
/// ```
/// # use firehazard::*;
/// # use abistr::*;
/// let app_container_name = cstr16!("firehazard delete_app_container_profile doc example");
/// # let _ = delete_app_container_profile(app_container_name);
///
/// create_app_container_profile(
///     app_container_name,
///     cstr16!("firehazard: docs: delete_app_container_profile example"),
///     cstr16!("Deletable temporary app container for delete_app_container_profile example"),
///     &[] // no capabilities
/// ).unwrap();
///
/// delete_app_container_profile(app_container_name).expect("delete_app_container_profile #1");
/// delete_app_container_profile(app_container_name).expect("delete_app_container_profile #2");
/// ```
///
/// ### Errors
/// *   `E_INVALIDARG`                              - If `app_container_name` is not valid (illegal chars? exceeds 64 codepoints?)
/// *   `HRESULT_FROM_WIN32(ERROR_NOT_SUPPORTED)`   - If called from within an app container.
pub fn delete_app_container_profile(
    app_container_name: impl TryIntoAsCStr<u16>,
) -> Result<(), Error> {
    let hr = unsafe { DeleteAppContainerProfile(app_container_name.try_into().map_err(|_| E_INVALIDARG)?.as_cstr()) };
    if !SUCCEEDED(hr) { Err(hr)? }
    Ok(())
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/userenv/nf-userenv-deriveappcontainersidfromappcontainername)\]
/// DeriveAppContainerSidFromAppContainerName
///
/// Determine the AppContainer SID from it's name.
///
/// ### Examples
/// ```
/// # use firehazard::*;
/// # use abistr::*;
/// let app_container_sid = derive_app_container_sid_from_app_container_name(
///     cstr16!("firehazard appcontainer")
/// ).expect("derive_app_container_sid_from_app_container_name");
///
/// assert_eq!(
///     "S-1-15-2-1616271714-4138532714-4113762788-3985685701-3505203873-1678203538-1574651687",
///     format!("{app_container_sid:?}")
/// );
/// ```
///
/// ### Errors
/// *   `E_INVALIDARG`  - If `app_container_name` is not valid (illegal chars? exceeds 64 codepoints?)
pub fn derive_app_container_sid_from_app_container_name(
    app_container_name: impl TryIntoAsCStr<u16>,
) -> Result<sid::Box<alloc::FreeSid>, Error> {
    let mut app_container_sid = null_mut();
    let hr = unsafe { DeriveAppContainerSidFromAppContainerName(
        app_container_name.try_into().map_err(|_| E_INVALIDARG)?.as_cstr(),
        &mut app_container_sid,
    )};
    let app_container_sid = unsafe { sid::Box::from_raw(app_container_sid.cast()) }.ok_or(ERROR_INVALID_SID);
    if !SUCCEEDED(hr) { Err(hr)? }
    Ok(app_container_sid?)
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-derivecapabilitysidsfromname)\]
/// DeriveCapabilitySidsFromName
///
/// ### Examples
/// ```
/// # use firehazard::*;
/// # use abistr::*;
/// let (group_sids, sids) = derive_capability_sids_from_name(cstr16!("internetClient")).unwrap();
/// assert_eq!(1, group_sids.len());
/// assert_eq!(1, sids.len());
/// assert_eq!(*group_sids[0],  *sid!(S-1-5-32-2779705173-1925339129-2667939958-2414465498-3395756507-4015878651-158944808-788332705));
/// assert_eq!(*sids[0],        *sid!(S-1-15-3-1)); // aka S - 1 - SECURITY_APP_PACKAGE_AUTHORITY - SECURITY_CAPABILITY_BASE_RID - SECURITY_CAPABILITY_INTERNET_CLIENT
///
/// let (group_sids, sids) = derive_capability_sids_from_name(cstr16!("activateAsUser")).unwrap();
/// assert_eq!(1, group_sids.len());
/// assert_eq!(1, sids.len());
/// assert_eq!(*group_sids[0],  *sid!(S-1-5-32-1619559953-3382903645-900470658-1831728285-1265525240-911141481-3610949621-2233473754));
/// assert_eq!(*sids[0],   *sid!(S-1-15-3-1024-1619559953-3382903645-900470658-1831728285-1265525240-911141481-3610949621-2233473754));
/// ```
///
/// ### Errors
/// *   `ERROR_CALL_NOT_IMPLEMENTED`    - if `kernelbase.dll` failed to load
/// *   `ERROR_CALL_NOT_IMPLEMENTED`    - if `kernelbase.dll` was missing `DeriveCapabilitySidsFromName`
/// *   `ERROR_INVALID_PARAMETER`       - if `cap_name` contains interior `\0`s
///
/// ### Errata
/// There appears to be no static linking against this symbol.
/// While the docs say the fn lives in `kernel32.dll`, this is a lie - it lives in `kernelbase.dll`.
/// `kernel32.dll` doesn't even re-export the symbol.
/// I checked.
#[cfg(std)] // minidl requires std for now
pub fn derive_capability_sids_from_name(cap_name: impl TryIntoAsCStr<u16>) -> Result<(CVec<sid::Box<LocalAllocFree>, LocalAllocFree>, CVec<sid::Box<LocalAllocFree>, LocalAllocFree>), Error> {
    use winapi::shared::minwindef::*;
    use winapi::um::winnt::*;

    let cap_name = cap_name.try_into().map_err(|_| ERROR_INVALID_PARAMETER)?;

    lazy_static::lazy_static! {
        static ref DERIVE_CAPABILITY_SIDS_FROM_NAME : Option<unsafe extern "system" fn(
            CapName:                    LPCWSTR,
            CapabilityGroupSids:        *mut *mut PSID,
            CapabilityGroupSidCount:    *mut DWORD,
            CapabilitySids:             *mut *mut PSID,
            CapabilitySidCount:         *mut DWORD,
        ) -> BOOL> = {
            minidl::Library::load("kernelbase.dll").ok().and_then(|lib| unsafe { lib.sym_opt("DeriveCapabilitySidsFromName\0") })
        };
    }
    #[allow(non_snake_case)] let DeriveCapabilitySidsFromName = (*DERIVE_CAPABILITY_SIDS_FROM_NAME).ok_or(ERROR_CALL_NOT_IMPLEMENTED)?;

    let mut n_group_sids = 0;
    let mut   group_sids = null_mut();
    let mut n_sids = 0;
    let mut   sids = null_mut();

    Error::get_last_if(FALSE == unsafe { DeriveCapabilitySidsFromName(cap_name.as_cstr(), &mut group_sids, &mut n_group_sids, &mut sids, &mut n_sids) })?;
    let n_group_sids = usize::from32(n_group_sids);
    let n_sids       = usize::from32(n_sids);
    let group_sids = unsafe { CVec::from_raw_parts(group_sids.cast(), n_group_sids, n_group_sids) };
    let       sids = unsafe { CVec::from_raw_parts(      sids.cast(),       n_sids,       n_sids) };

    Ok((group_sids, sids))
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/userenv/nf-userenv-deriverestrictedappcontainersidfromappcontainersidandrestrictedname)\]
/// DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName
#[allow(dead_code)] // "DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName is reserved for future use."
fn derive_restricted_app_container_sid_from_app_container_sid_and_restricted_name(
    app_container_sid:              &sid::Value,
    restricted_app_container_name:  impl TryIntoAsCStr<u16>,
) -> Result<sid::Box<alloc::FreeSid>, Error> {
    let mut restricted_app_container_sid = null_mut();
    let hr = unsafe { DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName(
        app_container_sid.as_psid(),
        restricted_app_container_name.try_into().map_err(|_| E_INVALIDARG)?.as_cstr(),
        &mut restricted_app_container_sid,
    )};
    let restricted_app_container_sid = unsafe { sid::Box::from_raw(restricted_app_container_sid.cast()) }.ok_or(ERROR_INVALID_SID);
    if !SUCCEEDED(hr) { Err(hr)? }
    Ok(restricted_app_container_sid?)
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/userenv/nf-userenv-getappcontainerfolderpath)\]
/// GetAppContainerFolderPath
#[cfg(not_yet)] // someone gave GetAppContainerFolderPath a *string* based SID parameter? verify that they actually meant to require a SID and not a name before exposing
#[cfg(std)]
pub fn get_app_container_folder_path(
    app_container_sid:              &sid::Value,
) -> Result<std::path::PathBuf, Error> {
    use std::os::windows::prelude::*;

    let mut path = null_mut();
    let hr = unsafe { GetAppContainerFolderPath(
        app_container_sid.as_psid(),
        &mut path,
    )};
    let path = unsafe { alloc::CBox::<u16, alloc::CoTaskMem>::from_raw_ptr(path) };
    if !SUCCEEDED(hr) { Err(hr)? }
    let path = path.ok_or(E_INVALIDARG)?;
    let path = unsafe { CStrNonNull::<u16>::from_ptr_unchecked_unbounded(path.as_ptr()) }.ok_or(E_INVALIDARG)?;
    Ok(std::path::PathBuf::from(std::ffi::OsString::from_wide(path.units())))
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/userenv/nf-userenv-getappcontainerregistrylocation)
/// GetAppContainerRegistryLocation
///
/// Gets the registry location associated with an AppContainer.
/// Must be called from within the context of the AppContainer.
#[cfg(not_yet)] // missing types, use case
pub fn get_app_container_registry_location(desired_access: ()) -> Result<!, Error> { // missing type wrapper for HKEY as well
    todo!()
}
