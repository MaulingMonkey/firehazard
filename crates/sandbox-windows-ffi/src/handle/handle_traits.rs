use crate::*;
use winapi::shared::ntdef::HANDLE;



pub trait FromLocalHandle<H=HANDLE> : Sized {
    /// ### Safety
    /// #### Kernel Object Type
    /// `handle` should be a handle of the correct [kernel object type](https://docs.microsoft.com/en-us/windows/win32/sysinfo/kernel-objects).
    /// That is, creating a _process_ handle from a _thread_ handle or a _desktop_ handle is possibly undefined behavior.
    ///
    /// #### Ownership
    /// If `Self` is an *owned* handle, `from_raw` takes ownership of `handle`.
    /// No other code should close or attempt to claim ownership over said handle, and `Self` will likely
    /// call [`CloseHandle`] (or `CloseDesktop` or `FreeLibrary` or ...) when `Drop`ed.
    ///
    /// If `Self<'a>` is a *borrowed* or *psuedo* handle, `handle` must remain valid for the lifetime of `'a`.
    /// This is likely longer than the lifetime of `Self` if `Self` is `Clone` or `Copy` - e.g. if `Self` is `Handle<'static>`, `handle` should remain permanently opened.
    ///
    /// #### Soundness
    /// One could argue that this function, *technically,* is sound - and shouldn't be `unsafe`.
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
    unsafe fn from_raw(handle: H) -> Result<Self, Error>;
}

/// Some kind of wrapper around a HANDLE owned by the current/local process.
pub trait AsLocalHandle<H=HANDLE> {
    /// [`winapi`]-friendly HANDLE
    fn as_handle(&self) -> H;
}

/// Some kind of wrapper around a non-null HANDLE owned by the current/local process.
pub trait AsLocalHandleNN : AsLocalHandle {
    /// HANDLE, but [core::ptr::NonNull].
    fn as_handle_nn(&self) -> HANDLENN;
}

impl<T: AsLocalHandleNN> AsLocalHandle for T { fn as_handle(&self) -> HANDLE { self.as_handle_nn().as_ptr() } }

// trait DuplicateFromLocal ?

// pub unsafe trait DuplicateableHandle : AsLocalHandle {
//     type Owned : FromLocalHandle;
// }
