use crate::*;
use crate::error::{get_last_error, LastError};
use crate::token::*;

use winapi::shared::winerror::*;

use winapi::um::handleapi::{DuplicateHandle, CloseHandle};
use winapi::um::processthreadsapi::GetCurrentProcess;
use winapi::um::securitybaseapi::{GetTokenInformation, DuplicateTokenEx};
use winapi::um::winnt::*;

use std::fmt::{self, Debug, Formatter};
use std::marker::PhantomData;
use std::mem::{size_of, align_of};
use std::ops::Deref;
use std::ptr::null_mut;



/// An Access Token HANDLE belonging to the current process.
///
/// ### References
/// *   <https://docs.microsoft.com/en-us/windows/win32/secauthz/access-tokens>
#[repr(transparent)] pub struct Handle(HANDLE);

impl Handle {
    /// ### Safety
    ///
    /// It's possible that some code will assume the underlying `HANDLE` remains valid for the lifetime of the `Handle`.
    /// Additionally, as this takes over ownership, the caller must ensure it does not permit another system to `CloseHandle(handle)`.
    #[allow(dead_code)] pub(crate) unsafe fn from_raw(handle: HANDLE) -> Self { Self(handle) }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-duplicatehandle)\] `DuplicateHandle`
    ///
    /// Shallow clones the token handle, causing it to share permissions lists etc.
    /// that will modify the original `handle` if changed through the resulting clone.
    ///
    /// ### Safety
    ///
    /// The underlying `HANDLE` should be a valid access token when called.
    pub unsafe fn shallow_clone_from_raw(handle: HANDLE) -> Self {
        let process = unsafe { GetCurrentProcess() };
        assert!(!process.is_null(), "GetCurrentProcess");

        let mut new = null_mut();
        let success = 0 != unsafe { DuplicateHandle(process, handle, process, &mut new, 0, false as _, DUPLICATE_SAME_ACCESS) };
        assert!(success, "DuplicateHandle GetLastError()={}", get_last_error());
        // N.B. handle != new - this isn't refcounting per se

        Self(new)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-duplicatetokenex)\] `DuplicateTokenEx`
    ///
    /// Deep clones the token handle, giving it it's own unique permissions list etc.
    /// that can be modified without changing the permissions of the original `handle`.
    ///
    /// ### Safety
    ///
    /// The underlying `HANDLE` should be a valid access token when called.
    pub unsafe fn clone_from_raw(handle: HANDLE, desired_access: AccessRights) -> Self {
        let process = unsafe { GetCurrentProcess() };
        assert!(!process.is_null(), "GetCurrentProcess");

        let mut new = null_mut();
        let success = 0 != unsafe { DuplicateTokenEx(handle, desired_access.as_u32(), null_mut(), SecurityDelegation, TokenPrimary, &mut new) };
        assert!(success, "DuplicateTokenEx GetLastError()={}", get_last_error());

        Self(new)
    }

    #[inline(always)] pub fn as_handle(&self) -> HANDLE { self.0 }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenUser, ...)`
    pub fn get_token_user(&self) -> Result<BoxTokenUser, LastError> { unsafe { Ok(BoxTokenUser::from_raw(self.get_token_information_raw_bytes(TokenUser)?)) } }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenGroups, ...)`
    pub fn get_token_groups(&self) -> Result<BoxTokenGroups, LastError> { unsafe { Ok(BoxTokenGroups::from_raw(self.get_token_information_raw_bytes(TokenGroups)?)) } }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenPrivileges, ...)`
    pub fn get_token_privileges(&self) -> Result<BoxTokenPrivileges, LastError> { unsafe { Ok(BoxTokenPrivileges::from_raw(self.get_token_information_raw_bytes(TokenPrivileges)?)) } }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenOwner, ...)`
    pub fn get_token_owner(&self) -> Result<BoxTokenOwner, LastError> { unsafe { Ok(BoxTokenOwner::from_raw(self.get_token_information_raw_bytes(TokenOwner)?)) } }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenPrimaryGroup, ...)`
    pub fn get_token_primary_group(&self) -> Result<BoxTokenPrimaryGroup, LastError> { unsafe { Ok(BoxTokenPrimaryGroup::from_raw(self.get_token_information_raw_bytes(TokenPrimaryGroup)?)) } }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenDefaultDacl, ...)`
    pub fn get_token_default_dacl(&self) -> Result<impl Deref<Target=TOKEN_DEFAULT_DACL>, LastError> { unsafe { self.get_token_information_raw_header(TokenDefaultDacl) } }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenSource, ...)`
    pub fn get_token_source(&self) -> Result<TOKEN_SOURCE, LastError> { unsafe { self.get_token_information_raw_fixed(TokenSource) } }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenType, ...)`
    pub fn get_token_type(&self) -> Result<TOKEN_TYPE, LastError> { unsafe { self.get_token_information_raw_fixed(TokenType) } }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
    /// `GetTokenInformation(self, TokenImpersonationLevel, ...)`
    ///
    /// ### Errors
    /// *   `ERROR_INVALID_PARAMETER`   If `self` is a primary token instead of an impersonation token?
    pub fn get_token_impersonation_level(&self) -> Result<SECURITY_IMPERSONATION_LEVEL, LastError> {
        // XXX: Return Option<SECURITY_IMPERSONATION_LEVEL> instead?
        unsafe { self.get_token_information_raw_fixed(TokenImpersonationLevel) }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenStatistics, ...)`
    pub fn get_token_statistics(&self) -> Result<TOKEN_STATISTICS, LastError> { unsafe { self.get_token_information_raw_fixed(TokenStatistics) } }
    // TODO: TokenRestrictedSids
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenSessionId, ...)`
    pub fn get_token_session_id(&self) -> Result<u32, LastError> { unsafe { self.get_token_information_raw_fixed(TokenSessionId) } }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenGroupsAndPrivileges, ...)`
    pub fn get_token_groups_and_privileges(&self) -> Result<BoxTokenGroupsAndPrivileges, LastError> { unsafe { Ok(BoxTokenGroupsAndPrivileges::from_raw(self.get_token_information_raw_bytes(TokenGroupsAndPrivileges)?)) } }
    // RESERVED: TokenSessionReference
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenSandBoxInert, ...)`
    pub fn get_token_sandbox_inert(&self) -> Result<bool, LastError> { unsafe { self.get_token_information_raw_bool(TokenSandBoxInert) } }
    // RESERVED: TokenAuditPolicy
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenOrigin, ...)`
    pub fn get_token_origin(&self) -> Result<TOKEN_ORIGIN, LastError> { unsafe { self.get_token_information_raw_fixed(TokenOrigin) } }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenElevationType, ...)`
    pub fn get_token_elevation_type(&self) -> Result<TOKEN_ELEVATION_TYPE, LastError> { unsafe { self.get_token_information_raw_fixed(TokenElevationType) } }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenLinkedToken, ...)`
    pub fn get_token_linked_token(&self) -> Result<TOKEN_LINKED_TOKEN, LastError> { unsafe { self.get_token_information_raw_fixed(TokenLinkedToken) } }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenElevation, ...)`
    pub fn get_token_elevation(&self) -> Result<TOKEN_ELEVATION, LastError> { unsafe { self.get_token_information_raw_fixed(TokenElevation) } }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenElevation, ...)`
    pub fn get_token_is_elevated(&self) -> Result<bool, LastError> { self.get_token_elevation().map(|e| e.TokenIsElevated != 0) }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenHasRestrictions, ...)`
    pub fn get_token_has_restrictions(&self) -> Result<bool, LastError> { unsafe { self.get_token_information_raw_bool(TokenHasRestrictions) } }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenAccessInformation, ...)`
    pub fn get_token_access_information(&self) -> Result<impl Deref<Target=TOKEN_ACCESS_INFORMATION>, LastError> { unsafe { self.get_token_information_raw_header(TokenAccessInformation) } }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenVirtualizationAllowed, ...)`
    pub fn get_token_virtualization_allowed(&self) -> Result<bool, LastError> { unsafe { self.get_token_information_raw_bool(TokenVirtualizationAllowed) } }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenVirtualizationEnabled, ...)`
    pub fn get_token_virtualization_enabled(&self) -> Result<bool, LastError> { unsafe { self.get_token_information_raw_bool(TokenVirtualizationEnabled) } }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenIntegrityLevel, ...)`
    pub fn get_token_integrity_level(&self) -> Result<BoxTokenMandatoryLabel, LastError> { unsafe { Ok(BoxTokenMandatoryLabel::from_raw(self.get_token_information_raw_bytes(TokenIntegrityLevel)?)) } }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenUIAccess, ...)`
    pub fn get_token_ui_access(&self) -> Result<bool, LastError> { unsafe { self.get_token_information_raw_bool(TokenUIAccess) } }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenMandatoryPolicy, ...)`
    pub fn get_token_mandatory_policy(&self) -> Result<TOKEN_MANDATORY_POLICY, LastError> { unsafe { self.get_token_information_raw_fixed(TokenMandatoryPolicy) } }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenLogonSid, ...)`
    pub fn get_token_login_sid(&self) -> Result<BoxTokenGroups, LastError> { unsafe { Ok(BoxTokenGroups::from_raw(self.get_token_information_raw_bytes(TokenLogonSid)?)) } }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenIsAppContainer, ...)`
    pub fn get_token_is_app_container(&self) -> Result<bool, LastError> { unsafe { self.get_token_information_raw_bool(TokenIsAppContainer) } }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenCapabilities, ...)`
    pub fn get_token_capabilities(&self) -> Result<BoxTokenGroups, LastError> { unsafe { Ok(BoxTokenGroups::from_raw(self.get_token_information_raw_bytes(TokenCapabilities)?)) } }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenAppContainerSid, ...)`
    pub fn get_token_app_container_sid(&self) -> Result<BoxTokenAppcontainerInformation, LastError> { unsafe { Ok(BoxTokenAppcontainerInformation::from_raw(self.get_token_information_raw_bytes(TokenAppContainerSid)?)) } }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenAppContainerNumber, ...)`
    pub fn get_token_app_container_number(&self) -> Result<u32, LastError> { unsafe { self.get_token_information_raw_fixed(TokenAppContainerNumber) } }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenUserClaimAttributes, ...)`
    pub fn get_token_user_claim_attributes(&self) -> Result<impl Deref<Target=CLAIM_SECURITY_ATTRIBUTES_INFORMATION>, LastError> { unsafe { self.get_token_information_raw_header(TokenUserClaimAttributes) } }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenDeviceClaimAttributes, ...)`
    pub fn get_token_device_claim_attributes(&self) -> Result<impl Deref<Target=CLAIM_SECURITY_ATTRIBUTES_INFORMATION>, LastError> { unsafe { self.get_token_information_raw_header(TokenDeviceClaimAttributes) } }
    // RESERVED: TokenRestrictedUserClaimAttributes
    // RESERVED: TokenRestrictedDeviceClaimAttributes
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenDeviceGroups, ...)`
    pub fn get_token_device_groups(&self) -> Result<BoxTokenGroups, LastError> { unsafe { Ok(BoxTokenGroups::from_raw(self.get_token_information_raw_bytes(TokenDeviceGroups)?)) } }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
    /// `GetTokenInformation(self, TokenRestrictedDeviceGroups, ...)`
    ///
    /// ### Errors
    /// *   `ERROR_INVALID_PARAMETER`   If `self` is a primary token instead of an impersonation token?
    pub fn get_token_restricted_device_groups(&self) -> Result<BoxTokenGroups, LastError> {
        // XXX: Return Option<...> instead?
        unsafe { Ok(BoxTokenGroups::from_raw(self.get_token_information_raw_bytes(TokenRestrictedDeviceGroups)?)) }
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
}

impl Debug for Handle {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "token::Handle({:08p})", self.0) }
}

impl Clone for Handle {
    fn clone(&self) -> Self { unsafe { Self::clone_from_raw(self.0, token::ALL_ACCESS) } }
}

impl Drop for Handle {
    fn drop(&mut self) {
        let success = 0 != unsafe { CloseHandle(self.0) };
        assert!(success, "CloseHandle GetLastError()={}", get_last_error());
    }
}

impl From<&Handle> for HANDLE {
    fn from(token: &Handle) -> Self { token.0 }
}



impl Handle {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
    /// `GetTokenInformation(self, class, NULL, 0, &mut result)`
    ///
    /// Get the length/size, in bytes, of the buffer size required to get token information.
    ///
    /// ### Safety
    /// *   `class` might need to be a valid token information class?
    unsafe fn get_token_information_raw_len(&self, class: TOKEN_INFORMATION_CLASS) -> Result<u32, LastError> {
        let mut size = 0;
        let success = 0 != unsafe { GetTokenInformation(self.0, class, null_mut(), 0, &mut size) };
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
    /// can probably use [get_token_information_raw_fixed] or [get_token_information_raw_u32_headers] instead.
    ///
    /// ### Safety
    /// *   `class` might need to be a valid token information class?
    unsafe fn get_token_information_raw_bytes(&self, class: TOKEN_INFORMATION_CLASS) -> Result<Box<[u8]>, LastError> {
        let mut size = 0;
        let r_size = unsafe { self.get_token_information_raw_len(class)? };
        let mut result = Box::<[u8]>::from(vec![0u8; usize::from32(r_size)]);
        let success = 0 != unsafe { GetTokenInformation(self.0, class, result.as_mut_ptr().cast(), r_size, &mut size) };
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
    unsafe fn get_token_information_raw_fixed<R: Copy>(&self, class: TOKEN_INFORMATION_CLASS) -> Result<R, LastError> {
        let mut size = 0;
        let mut r = unsafe { std::mem::zeroed::<R>() };
        let r_size = u32::try_from(std::mem::size_of_val(&r)).map_err(|_| LastError(ERROR_INSUFFICIENT_BUFFER))?;
        let success = 0 != unsafe { GetTokenInformation(self.0, class, &mut r as *mut _ as *mut _, r_size, &mut size) };
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
    unsafe fn get_token_information_raw_bool(&self, class: TOKEN_INFORMATION_CLASS) -> Result<bool, LastError> {
        let mut size = 0;
        let mut result = 0u32;
        let success = 0 != unsafe { GetTokenInformation(self.0, class, &mut result as *mut _ as *mut _, 4, &mut size) };
        if !success { return Err(LastError::get()) }
        Ok(result != 0)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
    /// `GetTokenInformation(self, class, &mut result, size_of_val(&result), ...)`
    ///
    /// ### Safety
    /// *   `class` might need to be a valid token information class?
    /// *   `R` should probably be bytemuck::Zeroable or equivalent
    unsafe fn get_token_information_raw_header<H: Copy>(&self, class: TOKEN_INFORMATION_CLASS) -> Result<impl Deref<Target = H>, LastError> {
        let bytes = unsafe { self.get_token_information_raw_bytes(class)? };
        if bytes.len() < size_of::<H>() { return Err(LastError(ERROR_INSUFFICIENT_BUFFER)) }
        assert_eq!(0, (bytes.as_ptr() as usize) % align_of::<H>());

        struct R<H>(Box<[u8]>, PhantomData<H>);
        impl<H> Deref for R<H> {
            type Target = H;
            fn deref(&self) -> &Self::Target { unsafe { &*(self.0.as_ptr() as *const H) } }
        }
        Ok(R(bytes, PhantomData))
    }
}
