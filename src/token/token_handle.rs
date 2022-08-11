use crate::*;
use crate::error::get_last_error;
use crate::token::*;

use winapi::um::handleapi::{DuplicateHandle, CloseHandle};
use winapi::um::processthreadsapi::GetCurrentProcess;
use winapi::um::winnt::*;

use std::fmt::{self, Debug, Formatter};
use std::ptr::null_mut;



/// An Access Token HANDLE belonging to the current process.
///
/// ### References
/// *   <https://docs.microsoft.com/en-us/windows/win32/secauthz/access-tokens>
#[repr(transparent)] pub struct Handle(HANDLE);

impl Handle {
    /// ### Safety
    /// `handle` must be a valid access token handle.
    ///
    /// This takes over ownership of `handle` and will `CloseHandle` it on drop.
    /// The caller must ensure no other code attempts to claim ownership over the same handle.
    pub unsafe fn from_raw(handle: HANDLE) -> Self { Self(handle) }

    /// ### Safety
    /// `handle` must be a valid access token handle.
    ///
    /// This borrows ownership of `handle`.
    /// The caller must ensure no other code attempts to release ownership over the same handle for the duration of the borrow.
    pub unsafe fn borrow_from_raw(handle: &HANDLE) -> &Self { unsafe { std::mem::transmute(handle) } }

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
        unsafe { duplicate_token_ex(Self::borrow_from_raw(&handle), desired_access, None, SecurityDelegation, token::Type::Primary) }
    }

    #[inline(always)] pub fn as_handle(&self) -> HANDLE { self.0 }
}

impl Debug for Handle {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "token::Handle({:08p})", self.0) }
}

impl Clone for Handle {
    fn clone(&self) -> Self { unsafe { Self::clone_from_raw(self.0, token::ALL_ACCESS) } }
}

impl Drop for Handle {
    fn drop(&mut self) {
        let success = self.0.is_null() || (0 != unsafe { CloseHandle(self.0) });
        assert!(success, "CloseHandle({:?}) GetLastError()={}", self.0, get_last_error());
    }
}

impl From<&Handle> for HANDLE {
    fn from(token: &Handle) -> Self { token.0 }
}
