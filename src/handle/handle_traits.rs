use crate::*;
use winapi::ctypes::c_void;
use winapi::shared::ntdef::HANDLE;
use core::ptr::NonNull;



/// A trait for objects that can be constructed from `HANDLE`s owned by the local process.
///
/// ### Safety
/// #### Kernel Object Type
/// `handle` should be a handle of the correct [kernel object type](https://docs.microsoft.com/en-us/windows/win32/sysinfo/kernel-objects).
/// That is, creating a _process_ handle from a _thread_ handle or a _desktop_ handle is possibly undefined behavior.
///
/// #### Ownership
/// If `Self` is an *owned* handle, `from_raw` / `from_raw_nn` take ownership of `handle`.
/// No other code should close or attempt to claim ownership over said handle, and `Self` will typically
/// call [`CloseHandle`] (or `CloseDesktop` or `FreeLibrary` or ...) when `Drop`ed.
///
/// If `Self<'a>` is a *borrowed* or *psuedo* handle, `handle` must remain valid for the lifetime of `'a`.
/// This is likely longer than the lifetime of `Self` if `Self` is `Clone` or `Copy` - e.g. if `Self` is `Handle<'static>`, `handle` should remain permanently opened.
///
/// #### Soundness
/// One could argue that these functions, *technically,* are sound - and shouldn't be `unsafe`.
/// Windows functions passed invalid handles will generally fail with `ERROR_INVALID_HANDLE`, or trigger process termination thanks to [`PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY`].
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
pub trait FromLocalHandle<H=c_void> : Sized {
    /// ### Safety
    /// Assuming `handle` isn't null:
    /// *   `handle` should have the correct [type](https://docs.microsoft.com/en-us/windows/win32/sysinfo/kernel-objects) (passing an HDESK where HWINSTA was expected may be undefined behavior)
    /// *   `handle` will be borrowed for `'a` by `Self<'a>` or have ownership transfered to `Self` (destroying the handle out from underneath either may be undefined behavior)
    unsafe fn from_raw(handle: *mut H) -> Result<Self, Error> {
        let handle = core::ptr::NonNull::new(handle).ok_or(Error(winapi::shared::winerror::ERROR_INVALID_HANDLE))?;
        Ok(unsafe { Self::from_raw_nn(handle) })
    }

    /// ### Safety
    /// *   `handle` should have the correct [type](https://docs.microsoft.com/en-us/windows/win32/sysinfo/kernel-objects) (passing an HDESK where HWINSTA was expected may be undefined behavior)
    /// *   `handle` will be borrowed for `'a` by `Self<'a>` or have ownership transfered to `Self` (destroying the handle out from underneath either may be undefined behavior)
    unsafe fn from_raw_nn(handle: NonNull<H>) -> Self;

    /// ### Safety
    /// Assuming `*handle` isn't null:
    /// *   `*handle` should have the correct [type](https://docs.microsoft.com/en-us/windows/win32/sysinfo/kernel-objects) (passing an HDESK where HWINSTA was expected may be undefined behavior)
    /// *   `*handle` will be borrowed for `'a` by `&'a Self<'a>` (destroying the handle out from underneath either may be undefined behavior)
    unsafe fn borrow_from_raw(handle: &*mut H) -> Result<&Self, Error> {
        if handle.is_null() { return Err(Error(winapi::shared::winerror::ERROR_INVALID_HANDLE)) }
        Ok(unsafe { Self::borrow_from_raw_nn(core::mem::transmute(handle)) })
    }

    /// ### Safety
    /// *   `*handle` should have the correct [type](https://docs.microsoft.com/en-us/windows/win32/sysinfo/kernel-objects) (passing an HDESK where HWINSTA was expected may be undefined behavior)
    /// *   `*handle` will be borrowed for `'a` by `&'a Self<'a>` (destroying the handle out from underneath either may be undefined behavior)
    unsafe fn borrow_from_raw_nn(handle: &NonNull<H>) -> &Self;
}

/// Some kind of wrapper around a HANDLE owned by the current/local process.
pub trait AsLocalHandle<H=HANDLE> : Sized {
    /// [`winapi`]-friendly HANDLE
    fn as_handle(&self) -> H;
    fn into_handle(self) -> H { let h = self.as_handle(); core::mem::forget(self); h }
}

/// Some kind of wrapper around a non-null HANDLE owned by the current/local process.
pub trait AsLocalHandleNN<H=c_void> : AsLocalHandle<*mut H> {
    /// HANDLE, but [NonNull].
    fn as_handle_nn(&self) -> NonNull<H>;
    fn into_handle_nn(self) -> NonNull<H> { let h = self.as_handle_nn(); core::mem::forget(self); h }
}

impl<H, T: AsLocalHandleNN<H>> AsLocalHandle<*mut H> for T { fn as_handle(&self) -> *mut H { self.as_handle_nn().as_ptr() } }

// trait DuplicateFromLocal ?

// pub unsafe trait DuplicateableHandle : AsLocalHandle {
//     type Owned : FromLocalHandle;
// }
