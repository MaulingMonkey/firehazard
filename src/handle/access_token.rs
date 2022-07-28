use crate::error::{get_last_error, LastError};
use crate::From32;

use winapi::shared::winerror::*;

use winapi::um::handleapi::{DuplicateHandle, CloseHandle};
use winapi::um::processthreadsapi::{GetCurrentProcess, OpenProcessToken, OpenThreadToken, GetCurrentThread};
use winapi::um::securitybaseapi::{IsTokenRestricted, RevertToSelf, GetTokenInformation};
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
#[repr(transparent)] pub struct AccessToken(HANDLE);

impl AccessToken {
    /// ### Safety
    ///
    /// It's possible that some code will assume the underlying `HANDLE` remains valid for the lifetime of the `AccessToken`.
    /// Additionally, as this takes over ownership, the caller must ensure it does not permit another system to `CloseHandle(handle)`.
    #[allow(dead_code)] pub(crate) unsafe fn from_raw(handle: HANDLE) -> Self { Self(handle) }

    /// ### Safety
    ///
    /// The underlying `HANDLE` should be a valid access token when called.
    pub unsafe fn clone_from_raw(handle: HANDLE) -> Self {
        let process = unsafe { GetCurrentProcess() };
        assert!(!process.is_null(), "GetCurrentProcess");

        let mut new = null_mut();
        let success = 0 != unsafe { DuplicateHandle(process, handle, process, &mut new, 0, false as _, DUPLICATE_SAME_ACCESS) };
        assert!(success, "DuplicateHandle GetLastError()={}", get_last_error());
        // N.B. handle != new - this isn't refcounting per se

        Self(new)
    }

    #[inline(always)] pub fn as_handle(&self) -> HANDLE { self.0 }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenUser, ...)`
    pub fn get_token_user(&self) -> Result<impl Deref<Target = TOKEN_USER>, LastError> { unsafe { self.get_token_information_raw_header(TokenUser) } }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenGroups, ...)`
    pub fn get_token_groups(&self) -> Result<impl Deref<Target=[SID_AND_ATTRIBUTES]>, LastError> { unsafe { self.get_token_information_raw_u32_headers(TokenGroups) } }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenPrivileges, ...)`
    pub fn get_token_privileges(&self) -> Result<impl Deref<Target=[LUID_AND_ATTRIBUTES]>, LastError> { unsafe { self.get_token_information_raw_u32_headers(TokenPrivileges) } }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenOwner, ...)`
    pub fn get_token_owner(&self) -> Result<impl Deref<Target=TOKEN_OWNER>, LastError> { unsafe { self.get_token_information_raw_header(TokenOwner) } }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenPrimaryGroup, ...)`
    pub fn get_token_primary_group(&self) -> Result<impl Deref<Target=TOKEN_PRIMARY_GROUP>, LastError> { unsafe { self.get_token_information_raw_header(TokenPrimaryGroup) } }
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
    pub fn get_token_groups_and_privileges(&self) -> Result<impl Deref<Target=TOKEN_GROUPS_AND_PRIVILEGES>, LastError> { unsafe { self.get_token_information_raw_header(TokenGroupsAndPrivileges) } }
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
    pub fn get_token_integrity_level(&self) -> Result<impl Deref<Target=TOKEN_MANDATORY_LABEL>, LastError> { unsafe { self.get_token_information_raw_header(TokenIntegrityLevel) } }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenUIAccess, ...)`
    pub fn get_token_ui_access(&self) -> Result<bool, LastError> { unsafe { self.get_token_information_raw_bool(TokenUIAccess) } }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenMandatoryPolicy, ...)`
    pub fn get_token_mandatory_policy(&self) -> Result<TOKEN_MANDATORY_POLICY, LastError> { unsafe { self.get_token_information_raw_fixed(TokenMandatoryPolicy) } }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenLogonSid, ...)`
    pub fn get_token_login_sid(&self) -> Result<impl Deref<Target=[SID_AND_ATTRIBUTES]>, LastError> { unsafe { self.get_token_information_raw_u32_headers(TokenLogonSid) } }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenIsAppContainer, ...)`
    pub fn get_token_is_app_container(&self) -> Result<bool, LastError> { unsafe { self.get_token_information_raw_bool(TokenIsAppContainer) } }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenCapabilities, ...)`
    pub fn get_token_capabilities(&self) -> Result<impl Deref<Target=[SID_AND_ATTRIBUTES]>, LastError> { unsafe { self.get_token_information_raw_u32_headers(TokenCapabilities) } }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenAppContainerSid, ...)`
    pub fn get_token_app_container_sid(&self) -> Result<impl Deref<Target=TOKEN_APPCONTAINER_INFORMATION>, LastError> { unsafe { self.get_token_information_raw_header(TokenAppContainerSid) } }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenAppContainerNumber, ...)`
    pub fn get_token_app_container_number(&self) -> Result<u32, LastError> { unsafe { self.get_token_information_raw_fixed(TokenAppContainerNumber) } }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenUserClaimAttributes, ...)`
    pub fn get_token_user_claim_attributes(&self) -> Result<impl Deref<Target=CLAIM_SECURITY_ATTRIBUTES_INFORMATION>, LastError> { unsafe { self.get_token_information_raw_header(TokenUserClaimAttributes) } }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenDeviceClaimAttributes, ...)`
    pub fn get_token_device_claim_attributes(&self) -> Result<impl Deref<Target=CLAIM_SECURITY_ATTRIBUTES_INFORMATION>, LastError> { unsafe { self.get_token_information_raw_header(TokenDeviceClaimAttributes) } }
    // RESERVED: TokenRestrictedUserClaimAttributes
    // RESERVED: TokenRestrictedDeviceClaimAttributes
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\] `GetTokenInformation(self, TokenDeviceGroups, ...)`
    pub fn get_token_device_groups(&self) -> Result<impl Deref<Target=[SID_AND_ATTRIBUTES]>, LastError> { unsafe { self.get_token_information_raw_u32_headers(TokenDeviceGroups) } }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
    /// `GetTokenInformation(self, TokenRestrictedDeviceGroups, ...)`
    ///
    /// ### Errors
    /// *   `ERROR_INVALID_PARAMETER`   If `self` is a primary token instead of an impersonation token?
    pub fn get_token_restricted_device_groups(&self) -> Result<impl Deref<Target=[SID_AND_ATTRIBUTES]>, LastError> {
        // XXX: Return Option<...> instead?
        unsafe { self.get_token_information_raw_u32_headers(TokenRestrictedDeviceGroups) }
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

impl Debug for AccessToken {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "AccessToken({:08p})", self.0) }
}

impl Clone for AccessToken {
    fn clone(&self) -> Self { unsafe { Self::clone_from_raw(self.0) } }
}

#[test] fn clone_debug() {
    let p = crate::handle::get_current_process_token();
    let _p2 = dbg!(p.clone());
}

impl Drop for AccessToken {
    fn drop(&mut self) {
        let success = 0 != unsafe { CloseHandle(self.0) };
        assert!(success, "CloseHandle GetLastError()={}", get_last_error());
    }
}

impl From<&AccessToken> for HANDLE {
    fn from(token: &AccessToken) -> Self { token.0 }
}



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocesstoken)\] OpenProcessToken(GetCurrentProcess(), TOKEN_ALL_ACCESS, ...)
///
/// ### Example
/// ```
/// use win32_security_playground::handle::*;
/// let token : AccessToken = open_current_process_token();
/// ```
pub fn open_current_process_token() -> AccessToken {
    let process = unsafe { GetCurrentProcess() };
    assert!(!process.is_null(), "GetCurrentProcess");

    let mut h = null_mut();
    let success = 0 != unsafe { OpenProcessToken(process, TOKEN_ALL_ACCESS, &mut h) };
    assert!(success, "OpenProcessToken GetLastError()={}", get_last_error());
    assert!(!h.is_null(), "OpenProcessToken");

    AccessToken(h)
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openthreadtoken)\] OpenThreadToken(GetCurrentThread(), TOKEN_ALL_ACCESS, ...)
///
/// ### Example
/// ```
/// use win32_security_playground::handle::*;
/// assert!(open_current_thread_token(false).is_none());
/// // TODO: set/verify token and unwrap some
/// ```
///
/// ### Returns
/// * `None` if the current thread had no impersonation token set (e.g. OpenThreadToken failed with GetLastError() == ERROR_NO_TOKEN)
pub fn open_current_thread_token(as_self: bool) -> Option<AccessToken> {
    let thread = unsafe { GetCurrentThread() };
    assert!(!thread.is_null(), "GetCurrentThread");

    let mut h = null_mut();
    let success = 0 != unsafe { OpenThreadToken(thread, TOKEN_ALL_ACCESS, as_self as _, &mut h) };
    if !success {
        match get_last_error() {
            ERROR_NO_TOKEN  => return None,
            gle             => panic!("OpenThreadToken GetLastError()={gle}"),
        }
    } else {
        assert!(!h.is_null(), "OpenThreadToken");
        Some(AccessToken(h))
    }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-istokenrestricted)\] IsTokenRestricted
///
/// ### Example
/// ```
/// use win32_security_playground::handle::*;
/// let token : AccessToken = open_current_process_token();
/// assert!(!is_token_restricted(&token));
/// ```
pub fn is_token_restricted(token: &AccessToken) -> bool {
    0 != unsafe { IsTokenRestricted(token.0) }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-reverttoself)\] RevertToSelf
///
/// ### Example
/// ```
/// use win32_security_playground::handle::*;
/// // TODO: set/reset/verify thread token
/// revert_to_self().unwrap();
/// ```
pub fn revert_to_self() -> Result<(), LastError> {
    let success = 0 != unsafe { RevertToSelf() };
    if success { Ok(()) } else { Err(LastError::get()) }
}



impl AccessToken {
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

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
    /// `GetTokenInformation(self, class, &mut result, size_of_val(&result), ...)`
    ///
    /// ### Safety
    /// *   `class` might need to be a valid token information class?
    /// *   `R` should probably be bytemuck::Zeroable or equivalent
    unsafe fn get_token_information_raw_u32_headers<H: Copy>(&self, class: TOKEN_INFORMATION_CLASS) -> Result<impl Deref<Target = [H]>, LastError> {
        let bytes = unsafe { self.get_token_information_raw_bytes(class)? };
        if bytes.len() < size_of::<u32>() { return Err(LastError(ERROR_INSUFFICIENT_BUFFER)) }
        #[inline(always)] fn len_from_bytes(bytes: &[u8]) -> usize { usize::from32(u32::from_ne_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])) }

        let len = len_from_bytes(&*bytes);
        let slice_offset = size_of::<u32>().max(align_of::<H>());
        assert!(len <= (bytes.len()-slice_offset)/size_of::<H>());
        let h_start : *const H = bytes[slice_offset..].as_ptr().cast();
        assert!(h_start as usize % align_of::<u32>().max(align_of::<H>()) == 0);

        struct R<H>(Box<[u8]>, PhantomData<H>);
        impl<H> Deref for R<H> {
            type Target = [H];
            fn deref(&self) -> &Self::Target {
                let bytes = &*self.0;
                let len = len_from_bytes(bytes);
                let slice_offset = size_of::<u32>().max(align_of::<H>());
                unsafe { std::slice::from_raw_parts(bytes[slice_offset..].as_ptr().cast(), len) }
            }
        }
        Ok(R(bytes, PhantomData))
    }
}
