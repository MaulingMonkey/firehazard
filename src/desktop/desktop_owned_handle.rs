use crate::*;

use winapi::shared::windef::HDESK;
use winapi::um::handleapi::DuplicateHandle;
use winapi::um::winnt::*;
use winapi::um::winuser::CloseDesktop;

use std::fmt::{self, Debug, Formatter};
use std::ptr::null_mut;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createdesktopa)\]
/// `HDESK` to a desktop
#[repr(transparent)] pub struct OwnedHandle(HDESK);

impl OwnedHandle {
    /// ### Safety
    /// `handle` must be a valid desktop handle.
    ///
    /// This takes over ownership of `handle` and will `CloseHandle` it on drop.
    /// The caller must ensure no other code attempts to claim ownership over the same handle.
    pub unsafe fn from_raw_unchecked(handle: HDESK) -> Self { Self(handle) }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-duplicatehandle)\]
    /// `DuplicateHandle`
    ///
    /// Shallow clones the desktop handle (clone references the same underlying kernel object.)
    ///
    /// ### Safety
    /// The underlying `HDESK` should be a valid desktop when called.
    pub unsafe fn clone_from_raw(handle: HDESK) -> Self {
        let process = get_current_process().as_handle();
        let mut new = null_mut();
        let success = 0 != unsafe { DuplicateHandle(process, handle.cast(), process, &mut new, 0, false as _, DUPLICATE_SAME_ACCESS) };
        assert!(success, "DuplicateHandle failed with {:?}", Error::get_last());
        // N.B. handle != new - this isn't refcounting per se

        Self(new.cast())
    }

    pub fn as_handle(&self) -> HDESK { self.0 }
}

impl AsRef<HDESK>   for OwnedHandle { fn as_ref(&self) -> &HDESK { &self.0 } }
impl AsRef<handle::Owned> for OwnedHandle { fn as_ref(&self) -> &handle::Owned { unsafe { std::mem::transmute(self) } } }
impl Clone          for OwnedHandle { fn clone(&self) -> Self { unsafe { Self::clone_from_raw(self.0) } } }
impl Debug          for OwnedHandle { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "desktop::OwnedHandle(0x{:08x})", self.0 as usize) } }
impl Drop           for OwnedHandle { fn drop(&mut self) { assert!(self.0.is_null() || (0 != unsafe { CloseDesktop(self.0) }), "CloseDesktop({:?}) failed with GetLastError()={:?}", self.0, Error::get_last()); } }

impl From<&OwnedHandle>  for HDESK { fn from(desktop: &OwnedHandle) -> Self { desktop.0 } }
impl From<OwnedHandle> for handle::Owned { fn from(desktop: OwnedHandle) -> Self { unsafe { std::mem::transmute(desktop) } } }
