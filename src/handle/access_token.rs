use crate::error::{get_last_error, LastError};

use winapi::shared::winerror::*;

use winapi::um::handleapi::{DuplicateHandle, CloseHandle};
use winapi::um::processthreadsapi::{GetCurrentProcess, OpenProcessToken, OpenThreadToken, GetCurrentThread};
use winapi::um::securitybaseapi::{IsTokenRestricted, RevertToSelf};
use winapi::um::winnt::{DUPLICATE_SAME_ACCESS, TOKEN_ALL_ACCESS, HANDLE};

use std::fmt::{self, Debug, Formatter};
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
        let success = 0 != unsafe { DuplicateHandle(process, handle, process, &mut new, DUPLICATE_SAME_ACCESS, false as _, 0) };
        assert!(success, "DuplicateHandle GetLastError()={}", get_last_error());
        // N.B. handle != new - this isn't refcounting per se

        Self(new)
    }

    #[inline(always)] pub fn as_handle(&self) -> HANDLE { self.0 }
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
