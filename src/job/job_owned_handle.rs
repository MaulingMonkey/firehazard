use crate::*;

use winapi::um::handleapi::{CloseHandle, DuplicateHandle};
use winapi::um::winnt::*;

use std::fmt::{self, Debug, Formatter};
use std::ptr::null_mut;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createdesktopa)\]
/// `HANDLE` to a job
#[repr(transparent)] pub struct OwnedHandle(HANDLE);

impl OwnedHandle {
    /// ### Safety
    /// `handle` must be a valid job handle.
    ///
    /// This takes over ownership of `handle` and will `CloseHandle` it on drop.
    /// The caller must ensure no other code attempts to claim ownership over the same handle.
    pub unsafe fn from_raw_unchecked(handle: HANDLE) -> Self { Self(handle) }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-duplicatehandle)\]
    /// `DuplicateHandle`
    ///
    /// Shallow clones the job handle (clone references the same underlying kernel object.)
    ///
    /// ### Safety
    /// The underlying `HANDLE` should be a valid job when called.
    pub unsafe fn clone_from_raw(handle: HANDLE) -> Self {
        let process = get_current_process().as_handle();
        let mut new = null_mut();
        let success = 0 != unsafe { DuplicateHandle(process, handle, process, &mut new, 0, false as _, DUPLICATE_SAME_ACCESS) };
        assert!(success, "DuplicateHandle failed with {:?}", Error::get_last());
        // N.B. handle != new - this isn't refcounting per se

        Self(new)
    }

    pub fn as_handle(&self) -> HANDLE { self.0 }
}

impl AsRef<HANDLE>  for OwnedHandle { fn as_ref(&self) -> &HANDLE { &self.0 } }
impl AsRef<handle::Owned> for OwnedHandle { fn as_ref(&self) -> &handle::Owned { unsafe { std::mem::transmute(self) } } }
impl Clone          for OwnedHandle { fn clone(&self) -> Self { unsafe { Self::clone_from_raw(self.0) } } }
impl Debug          for OwnedHandle { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "job::OwnedHandle(0x{:08x})", self.0 as usize) } }
impl Drop           for OwnedHandle { fn drop(&mut self) { assert!(self.0.is_null() || (0 != unsafe { CloseHandle(self.0) }), "CloseHandle({:?}) failed with GetLastError()={:?}", self.0, Error::get_last()); } }

impl From<&OwnedHandle>  for HANDLE { fn from(job: &OwnedHandle) -> Self { job.0 } }
impl From<OwnedHandle> for handle::Owned { fn from(job: OwnedHandle) -> Self { unsafe { std::mem::transmute(job) } } }
