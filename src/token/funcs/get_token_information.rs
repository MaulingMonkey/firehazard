//! \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
//! GetTokenInformation

use crate::*;
use crate::alloc::*;
use crate::token::*;

use winapi::shared::winerror::*;

use winapi::um::securitybaseapi::GetTokenInformation;
use winapi::um::winnt::*;

use core::ops::Deref;
use core::ptr::null_mut;



#[doc(alias = "GetTokenInformation")]
#[doc(alias = "TokenUser")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
/// `GetTokenInformation(self, TokenUser, ...)`
///
pub fn user(token: &token::OwnedHandle) -> Result<BoxTokenUser, Error> { unsafe { Ok(BoxTokenUser::from_raw(raw_bytes(token, TokenUser)?)) } }



#[doc(alias = "GetTokenInformation")]
#[doc(alias = "TokenGroups")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
/// `GetTokenInformation(self, TokenGroups, ...)`
///
pub fn groups(token: &token::OwnedHandle) -> Result<BoxTokenGroups, Error> { unsafe { Ok(BoxTokenGroups::from_raw(raw_bytes(token, TokenGroups)?)) } }



#[doc(alias = "GetTokenInformation")]
#[doc(alias = "TokenPrivileges")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
/// `GetTokenInformation(self, TokenPrivileges, ...)`
///
pub fn privileges(token: &token::OwnedHandle) -> Result<BoxTokenPrivileges, Error> { unsafe { Ok(BoxTokenPrivileges::from_raw(raw_bytes(token, TokenPrivileges)?)) } }



#[doc(alias = "GetTokenInformation")]
#[doc(alias = "TokenOwner")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
/// `GetTokenInformation(self, TokenOwner, ...)`
///
pub fn owner(token: &token::OwnedHandle) -> Result<BoxTokenOwner, Error> { unsafe { Ok(BoxTokenOwner::from_raw(raw_bytes(token, TokenOwner)?)) } }



#[doc(alias = "GetTokenInformation")]
#[doc(alias = "TokenPrimaryGroup")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
/// `GetTokenInformation(self, TokenPrimaryGroup, ...)`
///
pub fn primary_group(token: &token::OwnedHandle) -> Result<BoxTokenPrimaryGroup, Error> { unsafe { Ok(BoxTokenPrimaryGroup::from_raw(raw_bytes(token, TokenPrimaryGroup)?)) } }



#[doc(alias = "GetTokenInformation")]
#[doc(alias = "TokenDefaultDacl")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
/// `GetTokenInformation(self, TokenDefaultDacl, ...)`
///
pub fn default_dacl(token: &token::OwnedHandle) -> Result<BoxTokenDefaultDacl, Error> { unsafe { Ok(BoxTokenDefaultDacl::from_raw(raw_bytes(token, TokenDefaultDacl)?)) } }



#[doc(alias = "GetTokenInformation")]
#[doc(alias = "TokenSource")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
/// `GetTokenInformation(self, TokenSource, ...)`
///
pub fn source(token: &token::OwnedHandle) -> Result<Source, Error> { unsafe { raw_fixed(token, TokenSource) } }



#[doc(alias = "GetTokenInformation")]
#[doc(alias = "TokenType")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
/// `GetTokenInformation(self, TokenType, ...)`
///
pub fn r#type(token: &token::OwnedHandle) -> Result<token::Type, Error> { unsafe { raw_fixed(token, TokenType) } }
pub use r#type as ty;



#[doc(alias = "GetTokenInformation")]
#[doc(alias = "TokenImpersonationLevel")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
/// `GetTokenInformation(self, TokenImpersonationLevel, ...)`
///
/// ### Errors
/// *   `ERROR_INVALID_PARAMETER`   If `self` is a primary token instead of an impersonation token?
pub fn impersonation_level(token: &token::OwnedHandle) -> Result<security::ImpersonationLevel, Error> { unsafe { raw_fixed(token, TokenImpersonationLevel) } }



#[doc(alias = "GetTokenInformation")]
#[doc(alias = "TokenStatistics")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
/// `GetTokenInformation(self, TokenStatistics, ...)`
///
pub fn statistics(token: &token::OwnedHandle) -> Result<TOKEN_STATISTICS, Error> { unsafe { raw_fixed(token, TokenStatistics) } }



#[doc(alias = "GetTokenInformation")]
#[doc(alias = "TokenRestrictedSids")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
/// `GetTokenInformation(self, TokenRestrictedSids, ...)`
///
pub fn restricted_sids(token: &token::OwnedHandle) -> Result<BoxTokenGroups, Error> { unsafe { Ok(BoxTokenGroups::from_raw(raw_bytes(token, TokenRestrictedSids)?)) } }



#[doc(alias = "GetTokenInformation")]
#[doc(alias = "TokenSessionId")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
/// `GetTokenInformation(self, TokenSessionId, ...)`
///
pub fn session_id(token: &token::OwnedHandle) -> Result<u32, Error> { unsafe { raw_fixed(token, TokenSessionId) } }



#[doc(alias = "GetTokenInformation")]
#[doc(alias = "TokenGroupsAndPrivileges")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
/// `GetTokenInformation(self, TokenGroupsAndPrivileges, ...)`
///
pub fn groups_and_privileges(token: &token::OwnedHandle) -> Result<BoxTokenGroupsAndPrivileges, Error> { unsafe { Ok(BoxTokenGroupsAndPrivileges::from_raw(raw_bytes(token, TokenGroupsAndPrivileges)?)) } }



// RESERVED: TokenSessionReference



#[doc(alias = "GetTokenInformation")]
#[doc(alias = "TokenSandBoxInert")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
/// `GetTokenInformation(self, TokenSandBoxInert, ...)`
///
pub fn sandbox_inert(token: &token::OwnedHandle) -> Result<bool, Error> { unsafe { raw_bool(token, TokenSandBoxInert) } }



// RESERVED: TokenAuditPolicy



#[doc(alias = "GetTokenInformation")]
#[doc(alias = "TokenOrigin")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
/// `GetTokenInformation(self, TokenOrigin, ...)`
///
pub fn origin(token: &token::OwnedHandle) -> Result<TOKEN_ORIGIN, Error> { unsafe { raw_fixed(token, TokenOrigin) } }



#[doc(alias = "GetTokenInformation")]
#[doc(alias = "TokenElevationType")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
/// `GetTokenInformation(self, TokenElevationType, ...)`
///
pub fn elevation_type(token: &token::OwnedHandle) -> Result<token::ElevationType, Error> { unsafe { raw_fixed(token, TokenElevationType) } }



#[doc(alias = "GetTokenInformation")]
#[doc(alias = "TokenLinkedToken")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
/// `GetTokenInformation(self, TokenLinkedToken, ...)`
///
pub fn linked_token(token: &token::OwnedHandle) -> Result<TOKEN_LINKED_TOKEN, Error> { unsafe { raw_fixed(token, TokenLinkedToken) } }



#[doc(alias = "GetTokenInformation")]
#[doc(alias = "TokenElevation")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
/// `GetTokenInformation(self, TokenElevation, ...)`
///
pub fn elevation(token: &token::OwnedHandle) -> Result<token::Elevation, Error> { unsafe { raw_fixed(token, TokenElevation) } }



#[doc(alias = "GetTokenInformation")]
#[doc(alias = "TokenElevation")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
/// `GetTokenInformation(self, TokenElevation, ...)`
///
pub fn is_elevated(token: &token::OwnedHandle) -> Result<bool, Error> { elevation(token).map(|e| !!e.token_is_elevated) }



#[doc(alias = "GetTokenInformation")]
#[doc(alias = "TokenHasRestrictions")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
/// `GetTokenInformation(self, TokenHasRestrictions, ...)`
///
pub fn has_restrictions(token: &token::OwnedHandle) -> Result<bool, Error> { unsafe { raw_bool(token, TokenHasRestrictions) } }



#[doc(alias = "GetTokenInformation")]
#[doc(alias = "TokenAccessInformation")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
/// `GetTokenInformation(self, TokenAccessInformation, ...)`
///
pub fn access_information(token: &token::OwnedHandle) -> Result<impl Deref<Target=TOKEN_ACCESS_INFORMATION>, Error> { unsafe { raw_header(token, TokenAccessInformation) } }



#[doc(alias = "GetTokenInformation")]
#[doc(alias = "TokenVirtualizationAllowed")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
/// `GetTokenInformation(self, TokenVirtualizationAllowed, ...)`
///
pub fn virtualization_allowed(token: &token::OwnedHandle) -> Result<bool, Error> { unsafe { raw_bool(token, TokenVirtualizationAllowed) } }



#[doc(alias = "GetTokenInformation")]
#[doc(alias = "TokenVirtualizationEnabled")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
/// `GetTokenInformation(self, TokenVirtualizationEnabled, ...)`
///
pub fn virtualization_enabled(token: &token::OwnedHandle) -> Result<bool, Error> { unsafe { raw_bool(token, TokenVirtualizationEnabled) } }



#[doc(alias = "GetTokenInformation")]
#[doc(alias = "TokenIntegrityLevel")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
/// `GetTokenInformation(self, TokenIntegrityLevel, ...)`
///
pub fn integrity_level(token: &token::OwnedHandle) -> Result<BoxTokenMandatoryLabel, Error> { unsafe { Ok(BoxTokenMandatoryLabel::from_raw(raw_bytes(token, TokenIntegrityLevel)?)) } }



#[doc(alias = "GetTokenInformation")]
#[doc(alias = "TokenUIAccess")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
/// `GetTokenInformation(self, TokenUIAccess, ...)`
///
pub fn ui_access(token: &token::OwnedHandle) -> Result<bool, Error> { unsafe { raw_bool(token, TokenUIAccess) } }



#[doc(alias = "GetTokenInformation")]
#[doc(alias = "TokenMandatoryPolicy")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
/// `GetTokenInformation(self, TokenMandatoryPolicy, ...)`
///
pub fn mandatory_policy(token: &token::OwnedHandle) -> Result<token::MandatoryPolicy, Error> { unsafe { raw_fixed(token, TokenMandatoryPolicy) } }



#[doc(alias = "GetTokenInformation")]
#[doc(alias = "TokenLogonSid")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
/// `GetTokenInformation(self, TokenLogonSid, ...)`
///
pub fn logon_sid(token: &token::OwnedHandle) -> Result<BoxTokenGroups, Error> { unsafe { Ok(BoxTokenGroups::from_raw(raw_bytes(token, TokenLogonSid)?)) } }



#[doc(alias = "GetTokenInformation")]
#[doc(alias = "TokenIsAppContainer")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
/// `GetTokenInformation(self, TokenIsAppContainer, ...)`
///
pub fn is_app_container(token: &token::OwnedHandle) -> Result<bool, Error> { unsafe { raw_bool(token, TokenIsAppContainer) } }



#[doc(alias = "GetTokenInformation")]
#[doc(alias = "TokenCapabilities")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
/// `GetTokenInformation(self, TokenCapabilities, ...)`
///
pub fn capabilities(token: &token::OwnedHandle) -> Result<BoxTokenGroups, Error> { unsafe { Ok(BoxTokenGroups::from_raw(raw_bytes(token, TokenCapabilities)?)) } }



#[doc(alias = "GetTokenInformation")]
#[doc(alias = "TokenAppContainerSid")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
/// `GetTokenInformation(self, TokenAppContainerSid, ...)`
///
pub fn app_container_sid(token: &token::OwnedHandle) -> Result<BoxTokenAppcontainerInformation, Error> { unsafe { Ok(BoxTokenAppcontainerInformation::from_raw(raw_bytes(token, TokenAppContainerSid)?)) } }



#[doc(alias = "GetTokenInformation")]
#[doc(alias = "TokenAppContainerNumber")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
/// `GetTokenInformation(self, TokenAppContainerNumber, ...)`
///
pub fn app_container_number(token: &token::OwnedHandle) -> Result<u32, Error> { unsafe { raw_fixed(token, TokenAppContainerNumber) } }



#[doc(alias = "GetTokenInformation")]
#[doc(alias = "TokenUserClaimAttributes")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
/// `GetTokenInformation(self, TokenUserClaimAttributes, ...)`
///
pub fn user_claim_attributes(token: &token::OwnedHandle) -> Result<impl Deref<Target=CLAIM_SECURITY_ATTRIBUTES_INFORMATION>, Error> { unsafe { raw_header(token, TokenUserClaimAttributes) } }



#[doc(alias = "GetTokenInformation")]
#[doc(alias = "TokenDeviceClaimAttributes")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
/// `GetTokenInformation(self, TokenDeviceClaimAttributes, ...)`
///
pub fn device_claim_attributes(token: &token::OwnedHandle) -> Result<impl Deref<Target=CLAIM_SECURITY_ATTRIBUTES_INFORMATION>, Error> { unsafe { raw_header(token, TokenDeviceClaimAttributes) } }



// RESERVED: TokenRestrictedUserClaimAttributes
// RESERVED: TokenRestrictedDeviceClaimAttributes



#[doc(alias = "GetTokenInformation")]
#[doc(alias = "TokenDeviceGroups")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
/// `GetTokenInformation(self, TokenDeviceGroups, ...)`
///
pub fn device_groups(token: &token::OwnedHandle) -> Result<BoxTokenGroups, Error> { unsafe { Ok(BoxTokenGroups::from_raw(raw_bytes(token, TokenDeviceGroups)?)) } }



#[doc(alias = "GetTokenInformation")]
#[doc(alias = "TokenRestrictedDeviceGroups")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
/// `GetTokenInformation(self, TokenRestrictedDeviceGroups, ...)`
///
/// ### Errors
/// *   `ERROR_INVALID_PARAMETER`   If `self` is a primary token instead of an impersonation token?
///
pub fn restricted_device_groups(token: &token::OwnedHandle) -> Result<BoxTokenGroups, Error> {
    // XXX: Return Option<...> instead?
    unsafe { Ok(BoxTokenGroups::from_raw(raw_bytes(token, TokenRestrictedDeviceGroups)?)) }
}



// RESERVED: TokenSecurityAttributes
// RESERVED: TokenIsRestricted
// https://learn.microsoft.com/en-us/windows/win32/api/winnt/ne-winnt-token_information_class
// UNDOCUMENTED?: TokenProcessTrustLevel
// UNDOCUMENTED?: TokenPrivateNameSpace
// UNDOCUMENTED?: TokenSingletonAttributes
// UNDOCUMENTED?: TokenBnoIsolation
// UNDOCUMENTED?: TokenChildProcessFlags
// UNDOCUMENTED?: TokenIsLessPrivilegedAppContainer



impl token::OwnedHandle {
    #[doc(alias = "GetTokenInformation")]
    #[doc(alias = "TokenUser")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
    /// `GetTokenInformation(self, TokenUser, ...)`
    ///
    pub fn user(&self) -> Result<BoxTokenUser, Error> { user(self) }



    #[doc(alias = "GetTokenInformation")]
    #[doc(alias = "TokenGroups")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
    /// `GetTokenInformation(self, TokenGroups, ...)`
    ///
    pub fn groups(&self) -> Result<BoxTokenGroups, Error> { groups(self) }



    #[doc(alias = "GetTokenInformation")]
    #[doc(alias = "TokenPrivileges")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
    /// `GetTokenInformation(self, TokenPrivileges, ...)`
    ///
    pub fn privileges(&self) -> Result<BoxTokenPrivileges, Error> { privileges(self) }



    #[doc(alias = "GetTokenInformation")]
    #[doc(alias = "TokenOwner")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
    /// `GetTokenInformation(self, TokenOwner, ...)`
    ///
    pub fn owner(&self) -> Result<BoxTokenOwner, Error> { owner(self) }



    #[doc(alias = "GetTokenInformation")]
    #[doc(alias = "TokenPrimaryGroup")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
    /// `GetTokenInformation(self, TokenPrimaryGroup, ...)`
    ///
    pub fn primary_group(&self) -> Result<BoxTokenPrimaryGroup, Error> { primary_group(self) }



    #[doc(alias = "GetTokenInformation")]
    #[doc(alias = "TokenDefaultDacl")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
    /// `GetTokenInformation(self, TokenDefaultDacl, ...)`
    ///
    pub fn default_dacl(&self) -> Result<BoxTokenDefaultDacl, Error> { default_dacl(self) }



    #[doc(alias = "GetTokenInformation")]
    #[doc(alias = "TokenSource")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
    /// `GetTokenInformation(self, TokenSource, ...)`
    ///
    pub fn source(&self) -> Result<Source, Error> { source(self) }



    #[doc(alias = "GetTokenInformation")]
    #[doc(alias = "TokenType")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
    /// `GetTokenInformation(self, TokenType, ...)`
    ///
    pub fn r#type(&self) -> Result<token::Type, Error> { r#type(self) }



    #[doc(alias = "GetTokenInformation")]
    #[doc(alias = "TokenType")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
    /// `GetTokenInformation(self, TokenType, ...)`
    ///
    pub fn ty(&self) -> Result<token::Type, Error> { ty(self) }



    #[doc(alias = "GetTokenInformation")]
    #[doc(alias = "TokenImpersonationLevel")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
    /// `GetTokenInformation(self, TokenImpersonationLevel, ...)`
    ///
    ///
    /// ### Errors
    /// *   `ERROR_INVALID_PARAMETER`   If `self` is a primary token instead of an impersonation token?
    ///
    pub fn impersonation_level(&self) -> Result<security::ImpersonationLevel, Error> { impersonation_level(self) }



    #[doc(alias = "GetTokenInformation")]
    #[doc(alias = "TokenStatistics")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
    /// `GetTokenInformation(self, TokenStatistics, ...)`
    ///
    pub fn statistics(&self) -> Result<TOKEN_STATISTICS, Error> { statistics(self) }



    #[doc(alias = "GetTokenInformation")]
    #[doc(alias = "TokenRestrictedSids")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
    /// `GetTokenInformation(self, TokenRestrictedSids, ...)`
    ///
    pub fn restricted_sids(&self) -> Result<BoxTokenGroups, Error> { restricted_sids(self) }



    #[doc(alias = "GetTokenInformation")]
    #[doc(alias = "TokenSessionId")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
    /// `GetTokenInformation(self, TokenSessionId, ...)`
    ///
    pub fn session_id(&self) -> Result<u32, Error> { session_id(self) }



    #[doc(alias = "GetTokenInformation")]
    #[doc(alias = "TokenGroupsAndPrivileges")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
    /// `GetTokenInformation(self, TokenGroupsAndPrivileges, ...)`
    ///
    pub fn groups_and_privileges(&self) -> Result<BoxTokenGroupsAndPrivileges, Error> { groups_and_privileges(self) }



    // RESERVED: TokenSessionReference



    #[doc(alias = "GetTokenInformation")]
    #[doc(alias = "TokenSandBoxInert")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
    /// `GetTokenInformation(self, TokenSandBoxInert, ...)`
    ///
    pub fn sandbox_inert(&self) -> Result<bool, Error> { sandbox_inert(self) }



    // RESERVED: TokenAuditPolicy



    #[doc(alias = "GetTokenInformation")]
    #[doc(alias = "TokenOrigin")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
    /// `GetTokenInformation(self, TokenOrigin, ...)`
    ///
    pub fn origin(&self) -> Result<TOKEN_ORIGIN, Error> { origin(self) }



    #[doc(alias = "GetTokenInformation")]
    #[doc(alias = "TokenElevationType")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
    /// `GetTokenInformation(self, TokenElevationType, ...)`
    ///
    pub fn elevation_type(&self) -> Result<token::ElevationType, Error> { elevation_type(self) }



    #[doc(alias = "GetTokenInformation")]
    #[doc(alias = "TokenLinkedToken")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
    /// `GetTokenInformation(self, TokenLinkedToken, ...)`
    ///
    pub fn linked_token(&self) -> Result<TOKEN_LINKED_TOKEN, Error> { linked_token(self) }



    #[doc(alias = "GetTokenInformation")]
    #[doc(alias = "TokenElevation")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
    /// `GetTokenInformation(self, TokenElevation, ...)`
    ///
    pub fn elevation(&self) -> Result<token::Elevation, Error> { elevation(self) }



    #[doc(alias = "GetTokenInformation")]
    #[doc(alias = "TokenElevation")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
    /// `GetTokenInformation(self, TokenElevation, ...)`
    ///
    pub fn is_elevated(&self) -> Result<bool, Error> { is_elevated(self) }



    #[doc(alias = "GetTokenInformation")]
    #[doc(alias = "TokenHasRestrictions")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
    /// `GetTokenInformation(self, TokenHasRestrictions, ...)`
    ///
    pub fn has_restrictions(&self) -> Result<bool, Error> { has_restrictions(self) }



    #[doc(alias = "GetTokenInformation")]
    #[doc(alias = "TokenAccessInformation")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
    /// `GetTokenInformation(self, TokenAccessInformation, ...)`
    ///
    pub fn access_information(&self) -> Result<impl Deref<Target=TOKEN_ACCESS_INFORMATION>, Error> { access_information(self) }



    #[doc(alias = "GetTokenInformation")]
    #[doc(alias = "TokenVirtualizationAllowed")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
    /// `GetTokenInformation(self, TokenVirtualizationAllowed, ...)`
    ///
    pub fn virtualization_allowed(&self) -> Result<bool, Error> { virtualization_allowed(self) }



    #[doc(alias = "GetTokenInformation")]
    #[doc(alias = "TokenVirtualizationEnabled")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
    /// `GetTokenInformation(self, TokenVirtualizationEnabled, ...)`
    ///
    pub fn virtualization_enabled(&self) -> Result<bool, Error> { virtualization_enabled(self) }



    #[doc(alias = "GetTokenInformation")]
    #[doc(alias = "TokenIntegrityLevel")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
    /// `GetTokenInformation(self, TokenIntegrityLevel, ...)`
    ///
    pub fn integrity_level(&self) -> Result<BoxTokenMandatoryLabel, Error> { integrity_level(self) }



    #[doc(alias = "GetTokenInformation")]
    #[doc(alias = "TokenUIAccess")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
    /// `GetTokenInformation(self, TokenUIAccess, ...)`
    ///
    pub fn ui_access(&self) -> Result<bool, Error> { ui_access(self) }



    #[doc(alias = "GetTokenInformation")]
    #[doc(alias = "TokenMandatoryPolicy")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
    /// `GetTokenInformation(self, TokenMandatoryPolicy, ...)`
    ///
    pub fn mandatory_policy(&self) -> Result<token::MandatoryPolicy, Error> { mandatory_policy(self) }



    #[doc(alias = "GetTokenInformation")]
    #[doc(alias = "TokenLogonSid")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
    /// `GetTokenInformation(self, TokenLogonSid, ...)`
    ///
    pub fn logon_sid(&self) -> Result<BoxTokenGroups, Error> { logon_sid(self) }



    #[doc(alias = "GetTokenInformation")]
    #[doc(alias = "TokenIsAppContainer")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
    /// `GetTokenInformation(self, TokenIsAppContainer, ...)`
    ///
    pub fn is_app_container(&self) -> Result<bool, Error> { is_app_container(self) }



    #[doc(alias = "GetTokenInformation")]
    #[doc(alias = "TokenCapabilities")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
    /// `GetTokenInformation(self, TokenCapabilities, ...)`
    ///
    pub fn capabilities(&self) -> Result<BoxTokenGroups, Error> { capabilities(self) }



    #[doc(alias = "GetTokenInformation")]
    #[doc(alias = "TokenAppContainerSid")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
    /// `GetTokenInformation(self, TokenAppContainerSid, ...)`
    ///
    pub fn app_container_sid(&self) -> Result<BoxTokenAppcontainerInformation, Error> { app_container_sid(self) }



    #[doc(alias = "GetTokenInformation")]
    #[doc(alias = "TokenAppContainerNumber")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
    /// `GetTokenInformation(self, TokenAppContainerNumber, ...)`
    ///
    pub fn app_container_number(&self) -> Result<u32, Error> { app_container_number(self) }



    #[doc(alias = "GetTokenInformation")]
    #[doc(alias = "TokenUserClaimAttributes")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
    /// `GetTokenInformation(self, TokenUserClaimAttributes, ...)`
    ///
    pub fn user_claim_attributes(&self) -> Result<impl Deref<Target=CLAIM_SECURITY_ATTRIBUTES_INFORMATION>, Error> { user_claim_attributes(self) }



    #[doc(alias = "GetTokenInformation")]
    #[doc(alias = "TokenDeviceClaimAttributes")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
    /// `GetTokenInformation(self, TokenDeviceClaimAttributes, ...)`
    ///
    pub fn device_claim_attributes(&self) -> Result<impl Deref<Target=CLAIM_SECURITY_ATTRIBUTES_INFORMATION>, Error> { device_claim_attributes(self) }



    // RESERVED: TokenRestrictedUserClaimAttributes
    // RESERVED: TokenRestrictedDeviceClaimAttributes



    #[doc(alias = "GetTokenInformation")]
    #[doc(alias = "TokenDeviceGroups")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
    /// `GetTokenInformation(self, TokenDeviceGroups, ...)`
    ///
    pub fn device_groups(&self) -> Result<BoxTokenGroups, Error> { device_groups(self) }



    #[doc(alias = "GetTokenInformation")]
    #[doc(alias = "TokenRestrictedDeviceGroups")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
    /// `GetTokenInformation(self, TokenRestrictedDeviceGroups, ...)`
    ///
    /// ### Errors
    /// *   `ERROR_INVALID_PARAMETER`   If `self` is a primary token instead of an impersonation token?
    ///
    pub fn restricted_device_groups(&self) -> Result<BoxTokenGroups, Error> { restricted_device_groups(self) }



    // RESERVED: TokenSecurityAttributes
    // RESERVED: TokenIsRestricted
    // https://learn.microsoft.com/en-us/windows/win32/api/winnt/ne-winnt-token_information_class
    // UNDOCUMENTED?: TokenProcessTrustLevel
    // UNDOCUMENTED?: TokenPrivateNameSpace
    // UNDOCUMENTED?: TokenSingletonAttributes
    // UNDOCUMENTED?: TokenBnoIsolation
    // UNDOCUMENTED?: TokenChildProcessFlags
    // UNDOCUMENTED?: TokenIsLessPrivilegedAppContainer
}



#[doc(alias = "GetTokenInformation")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
/// `GetTokenInformation(self, class, NULL, 0, &mut result)`
///
/// Get the length/size, in bytes, of the buffer size required to get token information.
///
/// ### Safety
/// *   `class` might need to be a valid token information class?
///
unsafe fn raw_len(token: &token::OwnedHandle, class: TOKEN_INFORMATION_CLASS) -> Result<u32, Error> {
    let mut size = 0;
    Error::get_last_if(0 == unsafe { GetTokenInformation(token.as_handle(), class, null_mut(), 0, &mut size) }).unerr(ERROR_INSUFFICIENT_BUFFER, ())?;
    Ok(size)
}



#[doc(alias = "GetTokenInformation")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
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
///
unsafe fn raw_bytes<T: Default>(token: &token::OwnedHandle, class: TOKEN_INFORMATION_CLASS) -> Result<CBoxSized<T>, Error> {
    let mut size = 0;
    let r_size = unsafe { raw_len(token, class)? };
    let mut result = CBoxSized::<T>::new_oversized(T::default(), usize::from32(r_size));
    Error::get_last_if(0 == unsafe { GetTokenInformation(token.as_handle(), class, result.as_mut_ptr().cast(), r_size, &mut size) })?;
    if size != r_size { return Err(Error(ERROR_INCORRECT_SIZE)) }
    Ok(result)
}



#[doc(alias = "GetTokenInformation")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
/// `GetTokenInformation(self, class, &mut result, size_of_val(&result), ...)`
///
/// ### Safety
/// *   `class` might need to be a valid token information class?
/// *   `R` should probably be bytemuck::Zeroable or equivalent
///
/// ### Errors
/// *   `ERROR_INVALID_PARAMETER` / `87` - handle is wrong token type for query?
/// *   `ERROR_INSUFFICIENT_BUFFER` / `122` - `R` is wrong type? or merely a header for a longer blob of data?
///
unsafe fn raw_fixed<R: Copy + Default>(token: &token::OwnedHandle, class: TOKEN_INFORMATION_CLASS) -> Result<R, Error> {
    let mut size = 0;
    let mut r = R::default();
    let r_size = u32::try_from(core::mem::size_of_val(&r)).map_err(|_| ERROR_INSUFFICIENT_BUFFER)?;
    Error::get_last_if(0 == unsafe { GetTokenInformation(token.as_handle(), class, &mut r as *mut _ as *mut _, r_size, &mut size) })?;
    if size != r_size { return Err(Error(ERROR_INCORRECT_SIZE)) }
    Ok(r)
}



#[doc(alias = "GetTokenInformation")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
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
///
unsafe fn raw_bool(token: &token::OwnedHandle, class: TOKEN_INFORMATION_CLASS) -> Result<bool, Error> {
    let mut size = 0;
    let mut result = 0u32;
    Error::get_last_if(0 == unsafe { GetTokenInformation(token.as_handle(), class, &mut result as *mut _ as *mut _, 4, &mut size) })?;
    Ok(result != 0)
}



#[doc(alias = "GetTokenInformation")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
/// `GetTokenInformation(self, class, &mut result, size_of_val(&result), ...)`
///
/// ### Safety
/// *   `class` might need to be a valid token information class?
/// *   `R` should probably be bytemuck::Zeroable or equivalent
///
unsafe fn raw_header<H: Copy + Default>(token: &token::OwnedHandle, class: TOKEN_INFORMATION_CLASS) -> Result<impl Deref<Target = H>, Error> {
    let cbs = unsafe { raw_bytes::<H>(token, class)? };
    Ok(CBox::from(cbs))
}
