use crate::*;
use crate::error::LastError;

use winapi::um::handleapi::{DuplicateHandle, CloseHandle};
use winapi::um::winnt::*;

use std::fmt::{self, Debug, Formatter};
use std::os::windows::io::IntoRawHandle;
use std::process::Child;
use std::ptr::null_mut;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessw)\]
/// `HANDLE` to a process
#[repr(transparent)] pub struct OwnedHandle(HANDLE);

impl OwnedHandle {
    /// ### Safety
    /// `handle` must be a valid process handle.
    ///
    /// This takes over ownership of `handle` and will `CloseHandle` it on drop.
    /// The caller must ensure no other code attempts to claim ownership over the same handle.
    pub unsafe fn from_raw_unchecked(handle: HANDLE) -> Self { Self(handle) }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-duplicatehandle)\]
    /// `DuplicateHandle`
    ///
    /// Shallow clones the process handle (clone references the same underlying kernel object.)
    ///
    /// ### Safety
    /// The underlying `HANDLE` should be a valid process when called.
    pub unsafe fn clone_from_raw(handle: HANDLE) -> Self {
        let process = get_current_process().as_handle();
        let mut new = null_mut();
        let success = 0 != unsafe { DuplicateHandle(process, handle, process, &mut new, 0, false as _, DUPLICATE_SAME_ACCESS) };
        assert!(success, "DuplicateHandle failed with {:?}", LastError::get());
        // N.B. handle != new - this isn't refcounting per se

        Self(new)
    }

    pub fn as_handle(&self) -> HANDLE { self.0 }
}

impl AsRef<HANDLE>  for OwnedHandle { fn as_ref(&self) -> &HANDLE { &self.0 } }
impl Clone          for OwnedHandle { fn clone(&self) -> Self { unsafe { Self::clone_from_raw(self.0) } } }
impl Debug          for OwnedHandle { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "process::OwnedHandle(0x{:08x})", self.0 as usize) } }
impl Drop           for OwnedHandle { fn drop(&mut self) { assert!(self.0.is_null() || (0 != unsafe { CloseHandle(self.0) }), "CloseHandle({:?}) failed with GetLastError()={:?}", self.0, LastError::get()); } }
impl From<Child>    for OwnedHandle { fn from(c: Child) -> Self { unsafe { Self::from_raw_unchecked(c.into_raw_handle().cast()) } } }

impl From<&OwnedHandle>  for HANDLE { fn from(process: &OwnedHandle) -> Self { process.0 } }