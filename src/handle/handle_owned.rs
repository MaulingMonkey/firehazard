use crate::*;
use crate::handle::Handle;

use winapi::um::handleapi::{CloseHandle, DuplicateHandle};
use winapi::um::winnt::*;

use core::fmt::{self, Debug, Formatter};
use core::ops::Deref;
use core::ptr::null_mut;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/sysinfo/kernel-objects)\]
/// `HANDLE` to a kernel object
/// (owned: will [`CloseHandle`] on [`Drop`])
///
/// [`CloseHandle`]:    https://docs.microsoft.com/en-us/wsindows/win32/api/handleapi/nf-handleapi-closehandle
#[repr(transparent)] pub struct Owned(HANDLE);

impl Owned {
    pub const NULL : Self = Self(null_mut());

    /// ### Safety
    /// `handle` must be a valid [`CloseHandle`]able kernel object handle.
    ///
    /// This takes over ownership of `handle` and will [`CloseHandle`] it on [`Drop`].
    /// The caller must ensure no other code attempts to claim ownership over the same handle.
    /// You could argue that this function, *technically,* is sound - and shouldn't be `unsafe`.
    /// Functions passed invalid handles will generally fail with `ERROR_INVALID_HANDLE`, or trigger process termination thanks to [`PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY`].
    /// However, given the high likelyhood of undefined behavior from yanking handle ownership out from underneath:
    /// *   Wrappers
    /// *   Earlier versions of Windows
    /// *   ReactOS
    /// *   Wine
    /// *   Other third party reimplementations of the Win32 API
    ///
    /// I've chosen to make this function `unsafe` despite such arguable soundness.
    ///
    /// [`CloseHandle`]:                                    https://docs.microsoft.com/en-us/wsindows/win32/api/handleapi/nf-handleapi-closehandle
    /// [`PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY`]:  https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-process_mitigation_strict_handle_check_policy
    pub unsafe fn from_raw_unchecked(handle: HANDLE) -> Self { Self(handle) }

    /// ### Safety
    /// Same as [`Self::from_raw_unchecked`].
    pub unsafe fn from_raw(handle: HANDLE) -> Option<Self> { if handle.is_null() { None } else { Some(Self(handle)) } }

    /// ### Safety
    /// Similar to [`Self::from_raw_unchecked`] - however, ownedship is merely borrowed.
    /// The caller must ensure no other code attempts to release ownership over the same handle for the duration of the borrow.
    pub unsafe fn borrow_from_raw_unchecked(handle: &HANDLE) -> &Self { unsafe { core::mem::transmute(handle) } }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-duplicatehandle)\]
    /// `DuplicateHandle`
    ///
    /// Shallow clones the kernel object handle (clone references the same underlying kernel object.)
    ///
    /// ### Safety
    /// Same as [`Self::from_raw_unchecked`].
    pub unsafe fn clone_from_raw(handle: HANDLE) -> Self {
        let process = get_current_process().as_handle();
        let mut new = null_mut();
        let success = 0 != unsafe { DuplicateHandle(process, handle, process, &mut new, 0, false as _, DUPLICATE_SAME_ACCESS) };
        assert!(success, "DuplicateHandle failed with {:?}", Error::get_last());
        // N.B. handle != new - this isn't refcounting per se

        Self(new)
    }
}

impl AsRef<Handle>  for Owned { fn as_ref(&self) -> &Handle { unsafe { core::mem::transmute(self) } } }
impl AsRef<HANDLE>  for Owned { fn as_ref(&self) -> &HANDLE { &self.0 } }
impl Clone          for Owned { fn clone(&self) -> Self { unsafe { Self::clone_from_raw(self.0) } } }
impl Debug          for Owned { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "handle::Owned(0x{:08x})", self.0 as usize) } }
impl Deref          for Owned { type Target = Handle; fn deref(&self) -> &Self::Target { self.as_ref() } }
impl Drop           for Owned { fn drop(&mut self) { assert!(self.0.is_null() || (0 != unsafe { CloseHandle(self.0) }), "CloseHandle({:?}) failed with GetLastError()={:?}", self.0, Error::get_last()); } }

impl From<&Owned>  for HANDLE { fn from(handle: &Owned) -> Self { handle.0 } }
