use crate::*;
use crate::token::*;

use winapi::um::handleapi::{DuplicateHandle, CloseHandle};
use winapi::um::winnt::*;

use std::fmt::{self, Debug, Formatter};
use std::ptr::null_mut;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/secauthz/access-tokens)\]
/// `HANDLE` to an Access Token
#[repr(transparent)] pub struct OwnedHandle(HANDLE);

impl token::OwnedHandle {
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
        let process = get_current_process().as_handle();
        let mut new = null_mut();
        let success = 0 != unsafe { DuplicateHandle(process, handle, process, &mut new, 0, false as _, DUPLICATE_SAME_ACCESS) };
        assert!(success, "DuplicateHandle failed with {:?}", Error::get_last());
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
    pub unsafe fn clone_from_raw(handle: HANDLE, desired_access: impl Into<AccessRights>) -> Self {
        duplicate_token_ex(unsafe { Self::borrow_from_raw(&handle) }, desired_access, None, security::Delegation, token::Type::Primary).unwrap()
    }

    #[inline(always)] pub fn as_handle(&self) -> HANDLE { self.0 }
}

impl AsRef<HANDLE>  for OwnedHandle { fn as_ref(&self) -> &HANDLE { &self.0 } }
impl Clone          for OwnedHandle { fn clone(&self) -> Self { unsafe { Self::clone_from_raw(self.0, token::ALL_ACCESS) } } }
impl Debug          for OwnedHandle { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "token::OwnedHandle(0x{:08x})", self.0 as usize) } }
impl Drop           for OwnedHandle { fn drop(&mut self) { assert!(self.0.is_null() || (0 != unsafe { CloseHandle(self.0) }), "CloseHandle({:?}) failed with GetLastError()={:?}", self.0, Error::get_last()); } }

impl From<&OwnedHandle> for HANDLE { fn from(token: &OwnedHandle) -> Self { token.0 } }
