//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
//! GetTokenInformation

use crate::*;
use crate::error::{get_last_error, LastError};
use crate::token::*;

use winapi::shared::winerror::*;

use winapi::um::securitybaseapi::GetTokenInformation;
use winapi::um::winnt::*;

use std::marker::PhantomData;
use std::mem::{size_of, align_of};
use std::ops::Deref;
use std::ptr::null_mut;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenUser, ...)`
pub fn user(token: &token::Handle) -> Result<BoxTokenUser, LastError> { unsafe { Ok(BoxTokenUser::from_raw(raw_bytes(token, TokenUser)?)) } }
/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenGroups, ...)`
pub fn groups(token: &token::Handle) -> Result<BoxTokenGroups, LastError> { unsafe { Ok(BoxTokenGroups::from_raw(raw_bytes(token, TokenGroups)?)) } }
/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenPrivileges, ...)`
pub fn privileges(token: &token::Handle) -> Result<BoxTokenPrivileges, LastError> { unsafe { Ok(BoxTokenPrivileges::from_raw(raw_bytes(token, TokenPrivileges)?)) } }
/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenOwner, ...)`
pub fn owner(token: &token::Handle) -> Result<BoxTokenOwner, LastError> { unsafe { Ok(BoxTokenOwner::from_raw(raw_bytes(token, TokenOwner)?)) } }
/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenPrimaryGroup, ...)`
pub fn primary_group(token: &token::Handle) -> Result<BoxTokenPrimaryGroup, LastError> { unsafe { Ok(BoxTokenPrimaryGroup::from_raw(raw_bytes(token, TokenPrimaryGroup)?)) } }
/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenDefaultDacl, ...)`
pub fn default_dacl(token: &token::Handle) -> Result<impl Deref<Target=TOKEN_DEFAULT_DACL>, LastError> { unsafe { raw_header(token, TokenDefaultDacl) } }
/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenSource, ...)`
pub fn source(token: &token::Handle) -> Result<Source, LastError> { unsafe { raw_fixed(token, TokenSource) } }
/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenType, ...)`
pub fn r#type(token: &token::Handle) -> Result<token::Type, LastError> { unsafe { raw_fixed(token, TokenType) } }
pub use r#type as ty;

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
/// `GetTokenInformation(self, TokenImpersonationLevel, ...)`
///
/// ### Errors
/// *   `ERROR_INVALID_PARAMETER`   If `self` is a primary token instead of an impersonation token?
pub fn impersonation_level(token: &token::Handle) -> Result<SECURITY_IMPERSONATION_LEVEL, LastError> {
    // XXX: Return Option<SECURITY_IMPERSONATION_LEVEL> instead?
    unsafe { raw_fixed(token, TokenImpersonationLevel) }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenStatistics, ...)`
pub fn statistics(token: &token::Handle) -> Result<TOKEN_STATISTICS, LastError> { unsafe { raw_fixed(token, TokenStatistics) } }
// TODO: TokenRestrictedSids
/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenSessionId, ...)`
pub fn session_id(token: &token::Handle) -> Result<u32, LastError> { unsafe { raw_fixed(token, TokenSessionId) } }
/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenGroupsAndPrivileges, ...)`
pub fn groups_and_privileges(token: &token::Handle) -> Result<BoxTokenGroupsAndPrivileges, LastError> { unsafe { Ok(BoxTokenGroupsAndPrivileges::from_raw(raw_bytes(token, TokenGroupsAndPrivileges)?)) } }
// RESERVED: TokenSessionReference
/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenSandBoxInert, ...)`
pub fn sandbox_inert(token: &token::Handle) -> Result<bool, LastError> { unsafe { raw_bool(token, TokenSandBoxInert) } }
// RESERVED: TokenAuditPolicy
/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenOrigin, ...)`
pub fn origin(token: &token::Handle) -> Result<TOKEN_ORIGIN, LastError> { unsafe { raw_fixed(token, TokenOrigin) } }
/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenElevationType, ...)`
pub fn elevation_type(token: &token::Handle) -> Result<TOKEN_ELEVATION_TYPE, LastError> { unsafe { raw_fixed(token, TokenElevationType) } }
/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenLinkedToken, ...)`
pub fn linked_token(token: &token::Handle) -> Result<TOKEN_LINKED_TOKEN, LastError> { unsafe { raw_fixed(token, TokenLinkedToken) } }
/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenElevation, ...)`
pub fn elevation(token: &token::Handle) -> Result<TOKEN_ELEVATION, LastError> { unsafe { raw_fixed(token, TokenElevation) } }
/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenElevation, ...)`
pub fn is_elevated(token: &token::Handle) -> Result<bool, LastError> { elevation(token).map(|e| e.TokenIsElevated != 0) }
/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenHasRestrictions, ...)`
pub fn has_restrictions(token: &token::Handle) -> Result<bool, LastError> { unsafe { raw_bool(token, TokenHasRestrictions) } }
/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenAccessInformation, ...)`
pub fn access_information(token: &token::Handle) -> Result<impl Deref<Target=TOKEN_ACCESS_INFORMATION>, LastError> { unsafe { raw_header(token, TokenAccessInformation) } }
/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenVirtualizationAllowed, ...)`
pub fn virtualization_allowed(token: &token::Handle) -> Result<bool, LastError> { unsafe { raw_bool(token, TokenVirtualizationAllowed) } }
/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenVirtualizationEnabled, ...)`
pub fn virtualization_enabled(token: &token::Handle) -> Result<bool, LastError> { unsafe { raw_bool(token, TokenVirtualizationEnabled) } }
/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenIntegrityLevel, ...)`
pub fn integrity_level(token: &token::Handle) -> Result<BoxTokenMandatoryLabel, LastError> { unsafe { Ok(BoxTokenMandatoryLabel::from_raw(raw_bytes(token, TokenIntegrityLevel)?)) } }
/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenUIAccess, ...)`
pub fn ui_access(token: &token::Handle) -> Result<bool, LastError> { unsafe { raw_bool(token, TokenUIAccess) } }
/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenMandatoryPolicy, ...)`
pub fn mandatory_policy(token: &token::Handle) -> Result<TOKEN_MANDATORY_POLICY, LastError> { unsafe { raw_fixed(token, TokenMandatoryPolicy) } }
/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenLogonSid, ...)`
pub fn login_sid(token: &token::Handle) -> Result<BoxTokenGroups, LastError> { unsafe { Ok(BoxTokenGroups::from_raw(raw_bytes(token, TokenLogonSid)?)) } }
/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenIsAppContainer, ...)`
pub fn is_app_container(token: &token::Handle) -> Result<bool, LastError> { unsafe { raw_bool(token, TokenIsAppContainer) } }
/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenCapabilities, ...)`
pub fn capabilities(token: &token::Handle) -> Result<BoxTokenGroups, LastError> { unsafe { Ok(BoxTokenGroups::from_raw(raw_bytes(token, TokenCapabilities)?)) } }
/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenAppContainerSid, ...)`
pub fn app_container_sid(token: &token::Handle) -> Result<BoxTokenAppcontainerInformation, LastError> { unsafe { Ok(BoxTokenAppcontainerInformation::from_raw(raw_bytes(token, TokenAppContainerSid)?)) } }
/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenAppContainerNumber, ...)`
pub fn app_container_number(token: &token::Handle) -> Result<u32, LastError> { unsafe { raw_fixed(token, TokenAppContainerNumber) } }
/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenUserClaimAttributes, ...)`
pub fn user_claim_attributes(token: &token::Handle) -> Result<impl Deref<Target=CLAIM_SECURITY_ATTRIBUTES_INFORMATION>, LastError> { unsafe { raw_header(token, TokenUserClaimAttributes) } }
/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenDeviceClaimAttributes, ...)`
pub fn device_claim_attributes(token: &token::Handle) -> Result<impl Deref<Target=CLAIM_SECURITY_ATTRIBUTES_INFORMATION>, LastError> { unsafe { raw_header(token, TokenDeviceClaimAttributes) } }
// RESERVED: TokenRestrictedUserClaimAttributes
// RESERVED: TokenRestrictedDeviceClaimAttributes
/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenDeviceGroups, ...)`
pub fn device_groups(token: &token::Handle) -> Result<BoxTokenGroups, LastError> { unsafe { Ok(BoxTokenGroups::from_raw(raw_bytes(token, TokenDeviceGroups)?)) } }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
/// `GetTokenInformation(self, TokenRestrictedDeviceGroups, ...)`
///
/// ### Errors
/// *   `ERROR_INVALID_PARAMETER`   If `self` is a primary token instead of an impersonation token?
pub fn restricted_device_groups(token: &token::Handle) -> Result<BoxTokenGroups, LastError> {
    // XXX: Return Option<...> instead?
    unsafe { Ok(BoxTokenGroups::from_raw(raw_bytes(token, TokenRestrictedDeviceGroups)?)) }
}

// RESERVED: TokenSecurityAttributes
// RESERVED: TokenIsRestricted
// https://docs.microsoft.com/en-us/windows/win32/api/winnt/ne-winnt-token_information_class
// UNDOCUMENTED?: TokenProcessTrustLevel
// UNDOCUMENTED?: TokenPrivateNameSpace
// UNDOCUMENTED?: TokenSingletonAttributes
// UNDOCUMENTED?: TokenBnoIsolation
// UNDOCUMENTED?: TokenChildProcessFlags
// UNDOCUMENTED?: TokenIsLessPrivilegedAppContainer



impl token::Handle {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenUser, ...)`
    pub fn user(&self) -> Result<BoxTokenUser, LastError> { user(self) }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenGroups, ...)`
    pub fn groups(&self) -> Result<BoxTokenGroups, LastError> { groups(self) }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenPrivileges, ...)`
    pub fn privileges(&self) -> Result<BoxTokenPrivileges, LastError> { privileges(self) }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenOwner, ...)`
    pub fn owner(&self) -> Result<BoxTokenOwner, LastError> { owner(self) }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenPrimaryGroup, ...)`
    pub fn primary_group(&self) -> Result<BoxTokenPrimaryGroup, LastError> { primary_group(self) }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenDefaultDacl, ...)`
    pub fn default_dacl(&self) -> Result<impl Deref<Target=TOKEN_DEFAULT_DACL>, LastError> { default_dacl(self) }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenSource, ...)`
    pub fn source(&self) -> Result<Source, LastError> { source(self) }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenType, ...)`
    pub fn r#type(&self) -> Result<token::Type, LastError> { r#type(self) }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenType, ...)`
    pub fn ty(&self) -> Result<token::Type, LastError> { ty(self) }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
    /// `GetTokenInformation(self, TokenImpersonationLevel, ...)`
    ///
    /// ### Errors
    /// *   `ERROR_INVALID_PARAMETER`   If `self` is a primary token instead of an impersonation token?
    pub fn impersonation_level(&self) -> Result<SECURITY_IMPERSONATION_LEVEL, LastError> { impersonation_level(self) }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenStatistics, ...)`
    pub fn statistics(&self) -> Result<TOKEN_STATISTICS, LastError> { statistics(self) }
    // TODO: TokenRestrictedSids
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenSessionId, ...)`
    pub fn session_id(&self) -> Result<u32, LastError> { session_id(self) }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenGroupsAndPrivileges, ...)`
    pub fn groups_and_privileges(&self) -> Result<BoxTokenGroupsAndPrivileges, LastError> { groups_and_privileges(self) }
    // RESERVED: TokenSessionReference
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenSandBoxInert, ...)`
    pub fn sandbox_inert(&self) -> Result<bool, LastError> { sandbox_inert(self) }
    // RESERVED: TokenAuditPolicy
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenOrigin, ...)`
    pub fn origin(&self) -> Result<TOKEN_ORIGIN, LastError> { origin(self) }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenElevationType, ...)`
    pub fn elevation_type(&self) -> Result<TOKEN_ELEVATION_TYPE, LastError> { elevation_type(self) }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenLinkedToken, ...)`
    pub fn linked_token(&self) -> Result<TOKEN_LINKED_TOKEN, LastError> { linked_token(self) }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenElevation, ...)`
    pub fn elevation(&self) -> Result<TOKEN_ELEVATION, LastError> { elevation(self) }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenElevation, ...)`
    pub fn is_elevated(&self) -> Result<bool, LastError> { is_elevated(self) }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenHasRestrictions, ...)`
    pub fn has_restrictions(&self) -> Result<bool, LastError> { has_restrictions(self) }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenAccessInformation, ...)`
    pub fn access_information(&self) -> Result<impl Deref<Target=TOKEN_ACCESS_INFORMATION>, LastError> { access_information(self) }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenVirtualizationAllowed, ...)`
    pub fn virtualization_allowed(&self) -> Result<bool, LastError> { virtualization_allowed(self) }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenVirtualizationEnabled, ...)`
    pub fn virtualization_enabled(&self) -> Result<bool, LastError> { virtualization_enabled(self) }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenIntegrityLevel, ...)`
    pub fn integrity_level(&self) -> Result<BoxTokenMandatoryLabel, LastError> { integrity_level(self) }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenUIAccess, ...)`
    pub fn ui_access(&self) -> Result<bool, LastError> { ui_access(self) }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenMandatoryPolicy, ...)`
    pub fn mandatory_policy(&self) -> Result<TOKEN_MANDATORY_POLICY, LastError> { mandatory_policy(self) }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenLogonSid, ...)`
    pub fn login_sid(&self) -> Result<BoxTokenGroups, LastError> { login_sid(self) }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenIsAppContainer, ...)`
    pub fn is_app_container(&self) -> Result<bool, LastError> { is_app_container(self) }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenCapabilities, ...)`
    pub fn capabilities(&self) -> Result<BoxTokenGroups, LastError> { capabilities(self) }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenAppContainerSid, ...)`
    pub fn app_container_sid(&self) -> Result<BoxTokenAppcontainerInformation, LastError> { app_container_sid(self) }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenAppContainerNumber, ...)`
    pub fn app_container_number(&self) -> Result<u32, LastError> { app_container_number(self) }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenUserClaimAttributes, ...)`
    pub fn user_claim_attributes(&self) -> Result<impl Deref<Target=CLAIM_SECURITY_ATTRIBUTES_INFORMATION>, LastError> { user_claim_attributes(self) }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenDeviceClaimAttributes, ...)`
    pub fn device_claim_attributes(&self) -> Result<impl Deref<Target=CLAIM_SECURITY_ATTRIBUTES_INFORMATION>, LastError> { device_claim_attributes(self) }
    // RESERVED: TokenRestrictedUserClaimAttributes
    // RESERVED: TokenRestrictedDeviceClaimAttributes
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenDeviceGroups, ...)`
    pub fn device_groups(&self) -> Result<BoxTokenGroups, LastError> { device_groups(self) }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
    /// `GetTokenInformation(self, TokenRestrictedDeviceGroups, ...)`
    ///
    /// ### Errors
    /// *   `ERROR_INVALID_PARAMETER`   If `self` is a primary token instead of an impersonation token?
    pub fn restricted_device_groups(&self) -> Result<BoxTokenGroups, LastError> { restricted_device_groups(self) }

    // RESERVED: TokenSecurityAttributes
    // RESERVED: TokenIsRestricted
    // https://docs.microsoft.com/en-us/windows/win32/api/winnt/ne-winnt-token_information_class
    // UNDOCUMENTED?: TokenProcessTrustLevel
    // UNDOCUMENTED?: TokenPrivateNameSpace
    // UNDOCUMENTED?: TokenSingletonAttributes
    // UNDOCUMENTED?: TokenBnoIsolation
    // UNDOCUMENTED?: TokenChildProcessFlags
    // UNDOCUMENTED?: TokenIsLessPrivilegedAppContainer
}



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
/// `GetTokenInformation(self, class, NULL, 0, &mut result)`
///
/// Get the length/size, in bytes, of the buffer size required to get token information.
///
/// ### Safety
/// *   `class` might need to be a valid token information class?
unsafe fn raw_len(token: &token::Handle, class: TOKEN_INFORMATION_CLASS) -> Result<u32, LastError> {
    let mut size = 0;
    let success = 0 != unsafe { GetTokenInformation(token.as_handle(), class, null_mut(), 0, &mut size) };
    if !success {
        match get_last_error() {
            ERROR_INSUFFICIENT_BUFFER   => {}, // normal
            error                       => return Err(LastError(error)),
        }
    }
    Ok(size)
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
/// `GetTokenInformation(self, class, &mut result[..], size_of_val(&result), ...)`
///
/// Get the token information as a raw byte buffer.
///
/// Note that the resulting buffer often contains self-referential pointers, do *not* try to apply any kind of
/// on-stack buffer optimizations unless you *know* the resulting type has no such pointers - in which case you
/// can probably use [raw_fixed] or [raw_u32_headers] instead.
///
/// ### Safety
/// *   `class` might need to be a valid token information class?
unsafe fn raw_bytes(token: &token::Handle, class: TOKEN_INFORMATION_CLASS) -> Result<Box<[u8]>, LastError> {
    let mut size = 0;
    let r_size = unsafe { raw_len(token, class)? };
    let mut result = Box::<[u8]>::from(vec![0u8; usize::from32(r_size)]);
    let success = 0 != unsafe { GetTokenInformation(token.as_handle(), class, result.as_mut_ptr().cast(), r_size, &mut size) };
    if !success { return Err(LastError::get()) }
    if size != r_size { return Err(LastError(ERROR_INCORRECT_SIZE)) }
    Ok(result)
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
/// `GetTokenInformation(self, class, &mut result, size_of_val(&result), ...)`
///
/// ### Safety
/// *   `class` might need to be a valid token information class?
/// *   `R` should probably be bytemuck::Zeroable or equivalent
///
/// ### Errors
/// *   `ERROR_INVALID_PARAMETER` / `87` - handle is wrong token type for query?
/// *   `ERROR_INSUFFICIENT_BUFFER` / `122` - `R` is wrong type? or merely a header for a longer blob of data?
unsafe fn raw_fixed<R: Copy>(token: &token::Handle, class: TOKEN_INFORMATION_CLASS) -> Result<R, LastError> {
    let mut size = 0;
    let mut r = unsafe { std::mem::zeroed::<R>() };
    let r_size = u32::try_from(std::mem::size_of_val(&r)).map_err(|_| LastError(ERROR_INSUFFICIENT_BUFFER))?;
    let success = 0 != unsafe { GetTokenInformation(token.as_handle(), class, &mut r as *mut _ as *mut _, r_size, &mut size) };
    if !success { return Err(LastError::get()) }
    if size != r_size { return Err(LastError(ERROR_INCORRECT_SIZE)) }
    Ok(r)
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
/// `GetTokenInformation(self, class, &mut result, 4, ...)`
///
/// Retrieve a boolean value about a token.
///
/// These are generally documented as being DWORDs that are zero or non-zero.
/// However, it's worth noting that the documentation arguably lies at times - e.g. I've observed `TokenHasRestrictions` being 1 byte despite documentation stating it's a DWORD.
/// We can still use a pointer to a DWORD as the destination value to handle this case, but it's important to zero-init, and to not assume a full overwrite by the win32 API.
///
/// ### Safety
/// *   `class` might need to be a valid token information class?
///
/// ### Errors
/// *   `ERROR_INVALID_PARAMETER` / `87` - handle is wrong token type for query?
/// *   `ERROR_INSUFFICIENT_BUFFER` / `122` - `DWORD` is wrong type?
unsafe fn raw_bool(token: &token::Handle, class: TOKEN_INFORMATION_CLASS) -> Result<bool, LastError> {
    let mut size = 0;
    let mut result = 0u32;
    let success = 0 != unsafe { GetTokenInformation(token.as_handle(), class, &mut result as *mut _ as *mut _, 4, &mut size) };
    if !success { return Err(LastError::get()) }
    Ok(result != 0)
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
/// `GetTokenInformation(self, class, &mut result, size_of_val(&result), ...)`
///
/// ### Safety
/// *   `class` might need to be a valid token information class?
/// *   `R` should probably be bytemuck::Zeroable or equivalent
unsafe fn raw_header<H: Copy>(token: &token::Handle, class: TOKEN_INFORMATION_CLASS) -> Result<impl Deref<Target = H>, LastError> {
    let bytes = unsafe { raw_bytes(token, class)? };
    if bytes.len() < size_of::<H>() { return Err(LastError(ERROR_INSUFFICIENT_BUFFER)) }
    assert_eq!(0, (bytes.as_ptr() as usize) % align_of::<H>());

    struct R<H>(Box<[u8]>, PhantomData<H>);
    impl<H> Deref for R<H> {
        type Target = H;
        fn deref(&self) -> &Self::Target { unsafe { &*(self.0.as_ptr() as *const H) } }
    }
    Ok(R(bytes, PhantomData))
}
