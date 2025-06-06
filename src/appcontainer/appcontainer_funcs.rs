use crate::prelude::*;
use crate::alloc::LocalAllocFree;

use ialloc::allocator::adapt::DangleZst;
use ialloc::allocator::win32::Local;
use ialloc::vec::AVec;

use winapi::um::userenv::*;



// https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-checktokencapability
// CheckTokenCapability



#[doc(alias = "CreateAppContainerProfile")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/userenv/nf-userenv-createappcontainerprofile)\]
/// CreateAppContainerProfile
///
/// Creates a per-user, per-app profile.
///
/// ### Examples
/// ```
/// # use firehazard::*;
/// # use abistr::cstr16;
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
///
pub fn create_app_container_profile(
    app_container_name: impl string::InWide,
    display_name:       impl string::InWide,
    description:        impl string::InWide,
    capabilities:       &[sid::AndAttributes],
) -> firehazard::Result<sid::Box<alloc::FreeSid>> {
    let len32           = u32::try_from(capabilities.len()).map_err(|_| E_INVALIDARG)?;
    let capabilities    = if len32 == 0 { null_mut() } else { capabilities.as_ptr() as *mut _ };

    string::convert_to_cstrnn::<{limit::stack::APP_CONTAINER_NAME           }, _, _>(app_container_name,    |app_container_name|
    string::convert_to_cstrnn::<{limit::stack::APP_CONTAINER_DISPLAY_NAME   }, _, _>(display_name,          |display_name|
    string::convert_to_cstrnn::<{limit::stack::APP_CONTAINER_DESCRIPTION    }, _, _>(description,           |description|
    {
        let mut app_container_sid = null_mut();
        let hr = unsafe { CreateAppContainerProfile(
            app_container_name.as_cstr(),
            display_name.as_cstr(),
            description.as_cstr(),
            capabilities,
            len32,
            &mut app_container_sid,
        )};
        let app_container_sid = unsafe { sid::Box::from_raw(app_container_sid.cast()) }.ok_or(ERROR_INVALID_SID);
        if !SUCCEEDED(hr) { Err(hr)? }
        Ok(app_container_sid?)
    })))
    .map_err(hresultify)?
    .map_err(hresultify)?
    .map_err(hresultify)?
}

fn hresultify(e: firehazard::Error) -> firehazard::Error {
    use winresult::E;

    let hr =    if e == ERROR::ILLEGAL_CHARACTER { E::INVALIDARG    }
    else        if e == ERROR::NOT_ENOUGH_MEMORY { E::OUTOFMEMORY   }
    else        { return e };

    u32::from(hr).into()
}



#[doc(alias = "CreateAppContainerToken")]
/// \[<strike>microsoft.com</strike>\]
/// CreateAppContainerToken
///
/// An undocumented `kernelbase.dll` API for creating an app container token, used by Chromium etc.
///
/// ### Examples
/// ```
/// # use firehazard::*;
/// # use abistr::cstr16;
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
///
pub fn create_app_container_token<'a>(
    token:      impl Into<token::Handle<'a>>,
    security:   &security::Capabilities<'a>,
) -> firehazard::Result<token::OwnedHandle> {
    #[allow(non_snake_case)] let CreateAppContainerToken = *kernelbase::CreateAppContainerToken;

    let mut out_token = null_mut();
    firehazard::Error::get_last_if(0 == unsafe { CreateAppContainerToken(
        token.into().as_handle(),
        security.into(),
        &mut out_token,
    )})?;
    unsafe { token::OwnedHandle::from_raw(out_token) }
}



#[doc(alias = "DeleteAppContainerProfile")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/userenv/nf-userenv-deleteappcontainerprofile)\]
/// DeleteAppContainerProfile
///
/// Deletes the specified per-user, per-app profile.
/// Succeeds on non-existent profiles.
///
/// ### Examples
/// ```
/// # use firehazard::*;
/// # use abistr::cstr16;
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
///
pub fn delete_app_container_profile(
    app_container_name: impl string::InWide,
) -> firehazard::Result<()> {
    string::convert_to_cstrnn::<{limit::stack::APP_CONTAINER_NAME}, _, _>(app_container_name, |app_container_name| {
        let hr = unsafe { DeleteAppContainerProfile(app_container_name.as_cstr()) };
        if !SUCCEEDED(hr) { Err(hr)? }
        Ok(())
    }).map_err(hresultify)?
}



#[doc(alias = "DeriveAppContainerSidFromAppContainerName")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/userenv/nf-userenv-deriveappcontainersidfromappcontainername)\]
/// DeriveAppContainerSidFromAppContainerName
///
/// Determine the AppContainer SID from it's name.
///
/// ### Examples
/// ```
/// # use firehazard::*;
/// # use abistr::cstr16;
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
///
pub fn derive_app_container_sid_from_app_container_name(
    app_container_name: impl string::InWide,
) -> firehazard::Result<sid::Box<alloc::FreeSid>> {
    string::convert_to_cstrnn::<{limit::stack::APP_CONTAINER_NAME}, _, _>(app_container_name, |app_container_name| {
        let mut app_container_sid = null_mut();
        let hr = unsafe { DeriveAppContainerSidFromAppContainerName(
            app_container_name.as_cstr(),
            &mut app_container_sid,
        )};
        let app_container_sid = unsafe { sid::Box::from_raw(app_container_sid.cast()) }.ok_or(ERROR_INVALID_SID);
        if !SUCCEEDED(hr) { Err(hr)? }
        Ok(app_container_sid?)
    }).map_err(hresultify)?
}



#[doc(alias = "DeriveCapabilitySidsFromName")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-derivecapabilitysidsfromname)\]
/// DeriveCapabilitySidsFromName
///
/// ### Examples
/// ```
/// # use firehazard::*;
/// # use abistr::cstr16;
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
/// *   `ERROR_ILLEGAL_CHARACTER`       - if `cap_name` contains interior `\0`s
///
/// ### Errata
/// There appears to be no static linking against this symbol.
/// While the docs say the fn lives in `kernel32.dll`, this is a lie - it lives in `kernelbase.dll`.
/// `kernel32.dll` doesn't even re-export the symbol.
/// I checked.
///
pub fn derive_capability_sids_from_name(
    cap_name:       impl string::InWide,
) -> firehazard::Result<(AVec<sid::Box<LocalAllocFree>, DangleZst<Local>>, AVec<sid::Box<LocalAllocFree>, DangleZst<Local>>)> {
    use winapi::shared::minwindef::*;

    string::convert_to_cstrnn::<{limit::stack::CAPABILITY_NAME}, _, _>(cap_name, |cap_name| {
        #[allow(non_snake_case)] let DeriveCapabilitySidsFromName = *kernelbase::DeriveCapabilitySidsFromName;

        let mut n_group_sids = 0;
        let mut   group_sids = null_mut();
        let mut n_sids = 0;
        let mut   sids = null_mut();

        firehazard::Error::get_last_if(FALSE == unsafe { DeriveCapabilitySidsFromName(
            cap_name.as_cstr(),
            &mut group_sids,
            &mut n_group_sids,
            &mut sids,
            &mut n_sids,
        )})?;
        let n_group_sids = usize::from32(n_group_sids);
        let n_sids       = usize::from32(n_sids);
        let group_sids = NonNull::new(group_sids).map_or(AVec::new(), |nn| unsafe { AVec::from_raw_parts(nn.cast(), n_group_sids, n_group_sids) });
        let       sids = NonNull::new(      sids).map_or(AVec::new(), |nn| unsafe { AVec::from_raw_parts(nn.cast(),       n_sids,       n_sids) });

        Ok((group_sids, sids))
    })? // TODO: hresultify?
}



#[doc(alias = "DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/userenv/nf-userenv-deriverestrictedappcontainersidfromappcontainersidandrestrictedname)\]
/// DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName
///
#[allow(dead_code)] // "DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName is reserved for future use."
fn derive_restricted_app_container_sid_from_app_container_sid_and_restricted_name(
    app_container_sid:              &sid::Value,
    restricted_app_container_name:  impl string::InWide,
) -> firehazard::Result<sid::Box<alloc::FreeSid>> {
    string::convert_to_cstrnn::<{limit::stack::APP_CONTAINER_NAME}, _, _>(restricted_app_container_name, |restricted_app_container_name| {
        let mut restricted_app_container_sid = null_mut();
        let hr = unsafe { DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName(
            app_container_sid.as_psid(),
            restricted_app_container_name.as_cstr(),
            &mut restricted_app_container_sid,
        )};
        let restricted_app_container_sid = unsafe { sid::Box::from_raw(restricted_app_container_sid.cast()) }.ok_or(ERROR_INVALID_SID);
        if !SUCCEEDED(hr) { Err(hr)? }
        Ok(restricted_app_container_sid?)
    }).map_err(hresultify)?
}



#[doc(alias = "GetAppContainerFolderPath")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/userenv/nf-userenv-getappcontainerfolderpath)\]
/// GetAppContainerFolderPath
///
#[cfg(not_yet)] // someone gave GetAppContainerFolderPath a *string* based SID parameter? verify that they actually meant to require a SID and not a name before exposing
#[cfg(std)] // std::path::PathBuf
pub fn get_app_container_folder_path(
    app_container_sid:              &sid::Value,
) -> firehazard::Result<std::path::PathBuf> {
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



#[doc(alias = "GetAppContainerRegistryLocation")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/userenv/nf-userenv-getappcontainerregistrylocation)
/// GetAppContainerRegistryLocation
///
/// Gets the registry location associated with an AppContainer.
/// Must be called from within the context of the AppContainer.
///
#[cfg(not_yet)] // missing types, use case
pub fn get_app_container_registry_location(desired_access: ()) -> Result<!, Error> { // missing type wrapper for HKEY as well
    todo!()
}
