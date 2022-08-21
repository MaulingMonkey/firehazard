use crate::*;
use crate::thread::Handle;

use winapi::um::handleapi::{DuplicateHandle, CloseHandle};
use winapi::um::winnt::*;

#[cfg(std)] use std::os::windows::io::IntoRawHandle;
#[cfg(std)] use std::thread::JoinHandle;

use core::fmt::{self, Debug, Formatter};
use core::ops::Deref;
use core::ptr::null_mut;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createthread)\]
/// `HANDLE` to a thread
#[repr(transparent)] pub struct OwnedHandle(HANDLE);

impl OwnedHandle {
    /// ### Safety
    /// `handle` must be a valid thread handle.
    ///
    /// This takes over ownership of `handle` and will `CloseHandle` it on drop.
    /// The caller must ensure no other code attempts to claim ownership over the same handle.
    pub unsafe fn from_raw_unchecked(handle: HANDLE) -> Self { Self(handle) }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-duplicatehandle)\]
    /// `DuplicateHandle`
    ///
    /// Shallow clones the thread handle (clone references the same underlying kernel object.)
    ///
    /// ### Safety
    /// The underlying `HANDLE` should be a valid thread when called.
    pub unsafe fn clone_from_raw(handle: HANDLE) -> Self {
        let process = get_current_process().as_handle();
        let mut new = null_mut();
        let success = 0 != unsafe { DuplicateHandle(process, handle, process, &mut new, 0, false as _, DUPLICATE_SAME_ACCESS) };
        assert!(success, "DuplicateHandle failed with {:?}", Error::get_last());
        // N.B. handle != new - this isn't refcounting per se

        Self(new)
    }
}

impl AsRef<Handle>  for OwnedHandle { fn as_ref(&self) -> &Handle { unsafe { core::mem::transmute(self) } } }
impl AsRef<HANDLE>  for OwnedHandle { fn as_ref(&self) -> &HANDLE { &self.0 } }
impl AsRef<handle::Owned> for OwnedHandle { fn as_ref(&self) -> &handle::Owned { unsafe { core::mem::transmute(self) } } }
impl Clone          for OwnedHandle { fn clone(&self) -> Self { unsafe { Self::clone_from_raw(self.0) } } }
impl Debug          for OwnedHandle { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "thread::OwnedHandle(0x{:08x})", self.0 as usize) } }
impl Deref          for OwnedHandle { type Target = Handle; fn deref(&self) -> &Self::Target { self.as_ref() } }
impl Drop           for OwnedHandle { fn drop(&mut self) { assert!(self.0.is_null() || (0 != unsafe { CloseHandle(self.0) }), "CloseHandle({:?}) failed with GetLastError()={:?}", self.0, Error::get_last()); } }
#[cfg(std)] impl<T> From<JoinHandle<T>> for OwnedHandle { fn from(jh: JoinHandle<T>) -> Self { unsafe { Self::from_raw_unchecked(jh.into_raw_handle().cast()) } } }

impl From<&OwnedHandle>  for HANDLE { fn from(thread: &OwnedHandle) -> Self { thread.0 } }
impl From<OwnedHandle> for handle::Owned { fn from(thread: OwnedHandle) -> Self { unsafe { core::mem::transmute(thread) } } }
