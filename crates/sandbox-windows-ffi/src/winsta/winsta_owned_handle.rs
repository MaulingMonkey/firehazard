use crate::*;

use winapi::shared::minwindef::HWINSTA;
use winapi::um::handleapi::DuplicateHandle;
use winapi::um::winnt::*;
use winapi::um::winuser::CloseWindowStation;

use core::fmt::{self, Debug, Formatter};
use core::ptr::null_mut;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowstationa)\]
/// `HWINSTA` to a window station
#[repr(transparent)] pub struct OwnedHandle(HWINSTA);

impl OwnedHandle {
    /// ### Safety
    /// `handle` must be a valid window station handle.
    ///
    /// This takes over ownership of `handle` and will `CloseHandle` it on drop.
    /// The caller must ensure no other code attempts to claim ownership over the same handle.
    pub unsafe fn from_raw_unchecked(handle: HWINSTA) -> Self { Self(handle) }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-duplicatehandle)\]
    /// `DuplicateHandle`
    ///
    /// Shallow clones the window station handle (clone references the same underlying kernel object.)
    ///
    /// ### Safety
    /// The underlying `HWINSTA` should be a valid window station when called.
    pub unsafe fn clone_from_raw(handle: HWINSTA) -> Self {
        let process = get_current_process().as_handle();
        let mut new = null_mut();
        let success = 0 != unsafe { DuplicateHandle(process, handle.cast(), process, &mut new, 0, false as _, DUPLICATE_SAME_ACCESS) };
        assert!(success, "DuplicateHandle failed with {:?}", Error::get_last());
        // N.B. handle != new - this isn't refcounting per se

        Self(new.cast())
    }

    pub fn as_handle(&self) -> HWINSTA { self.0 }
}

impl AsRef<HWINSTA> for OwnedHandle { fn as_ref(&self) -> &HWINSTA { &self.0 } }
impl AsRef<handle::Owned> for OwnedHandle { fn as_ref(&self) -> &handle::Owned { unsafe { core::mem::transmute(self) } } }
impl Clone          for OwnedHandle { fn clone(&self) -> Self { unsafe { Self::clone_from_raw(self.0) } } }
impl Debug          for OwnedHandle { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "winsta::OwnedHandle(0x{:08x})", self.0 as usize) } }
impl Drop           for OwnedHandle { fn drop(&mut self) { assert!(self.0.is_null() || (0 != unsafe { CloseWindowStation(self.0) }), "CloseWindowStation({:?}) failed with GetLastError()={:?}", self.0, Error::get_last()); } }

impl From<&OwnedHandle>  for HWINSTA { fn from(winsta: &OwnedHandle) -> Self { winsta.0 } }
impl From<OwnedHandle> for handle::Owned { fn from(winsta: OwnedHandle) -> Self { unsafe { core::mem::transmute(winsta) } } }
