use crate::prelude::*;



/// A trait for objects that can be constructed from `HANDLE`s owned by the local process.
///
/// ### Safety
/// #### Kernel Object Type
/// `handle` should be a handle of the correct [kernel object type](https://learn.microsoft.com/en-us/windows/win32/sysinfo/kernel-objects).
/// That is, creating a _process_ handle from a _thread_ handle or a _desktop_ handle is possibly undefined behavior.
///
/// #### Ownership
/// If `Self` is an *owned* handle, `from_raw` / `from_raw_nn` take ownership of `handle`.
/// No other code should close or attempt to claim ownership over said handle, and `Self` will typically
/// call [`CloseHandle`] (or `CloseDesktop` or `FreeLibrary` or ...) when `Drop`ed.
///
/// If `Self<'a>` is a *borrowed* or *pseudo* handle, `handle` must remain valid for the lifetime of `'a`.
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
/// [`CloseHandle`]:                                    https://learn.microsoft.com/en-us/wsindows/win32/api/handleapi/nf-handleapi-closehandle
/// [`PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY`]:  https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-process_mitigation_strict_handle_check_policy
pub trait FromLocalHandle<H=c_void> : Sized {

    /// ### Safety
    /// Assuming `handle` isn't null:
    /// *   `handle` should have the correct [type](https://learn.microsoft.com/en-us/windows/win32/sysinfo/kernel-objects) (passing an HDESK where HWINSTA was expected may be undefined behavior)
    /// *   `handle` will be borrowed for `'a` by `Self<'a>` or have ownership transfered to `Self` (destroying the handle out from underneath either may be undefined behavior)
    unsafe fn from_raw(handle: *mut H) -> firehazard::Result<Self> {
        let handle = NonNull::new(handle).ok_or(ERROR_INVALID_HANDLE)?;
        Ok(unsafe { Self::from_raw_nn(handle) })
    }

    /// ### Safety
    /// *   `handle` should have the correct [type](https://learn.microsoft.com/en-us/windows/win32/sysinfo/kernel-objects) (passing an HDESK where HWINSTA was expected may be undefined behavior)
    /// *   `handle` will be borrowed for `'a` by `Self<'a>` or have ownership transfered to `Self` (destroying the handle out from underneath either may be undefined behavior)
    unsafe fn from_raw_nn(handle: NonNull<H>) -> Self;

    /// ### Safety
    /// Assuming `*handle` isn't null:
    /// *   `*handle` should have the correct [type](https://learn.microsoft.com/en-us/windows/win32/sysinfo/kernel-objects) (passing an HDESK where HWINSTA was expected may be undefined behavior)
    /// *   `*handle` will be borrowed for `'a` by `&'a Self<'a>` (destroying the handle out from underneath either may be undefined behavior)
    unsafe fn borrow_from_raw(handle: &*mut H) -> firehazard::Result<&Self> {
        NonNull::new(*handle).ok_or(ERROR_INVALID_HANDLE)?;
        Ok(unsafe { Self::borrow_from_raw_nn(core::mem::transmute(handle)) })
    }

    /// ### Safety
    /// *   `*handle` should have the correct [type](https://learn.microsoft.com/en-us/windows/win32/sysinfo/kernel-objects) (passing an HDESK where HWINSTA was expected may be undefined behavior)
    /// *   `*handle` will be borrowed for `'a` by `&'a Self<'a>` (destroying the handle out from underneath either may be undefined behavior)
    unsafe fn borrow_from_raw_nn(handle: &NonNull<H>) -> &Self;
}



/// Some kind of wrapper around a HANDLE owned by the current/local process.
///
pub trait AsLocalHandle<H=c_void> : Sized {
    /// [`winapi`]-friendly HANDLE
    fn as_handle(&self) -> *mut H;
    fn into_handle(self) -> *mut H { let h = self.as_handle(); core::mem::forget(self); h }
}



/// Some kind of wrapper around a non-null HANDLE owned by the current/local process.
///
pub trait AsLocalHandleNN<H=c_void> : AsLocalHandle<H> {
    /// HANDLE, but [NonNull].
    fn as_handle_nn(&self) -> NonNull<H>;
    fn into_handle_nn(self) -> NonNull<H> { let h = self.as_handle_nn(); core::mem::forget(self); h }
}

impl<H, T: AsLocalHandleNN<H>> AsLocalHandle<H> for T { fn as_handle(&self) -> *mut H { self.as_handle_nn().as_ptr() } }



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-duplicatehandle)\]
/// DuplicateHandle(-1, source, -1, 0, FALSE, DUPLICATE_SAME_ACCESS)
///
/// ### Safety
///
/// *   [`duplicate_handle_local_same_access`] and friends rely on `Self` and `Self::Owned` being compatible handles.
///
pub unsafe trait TryCloneToOwned {
    type Owned;

    #[doc(alias = "DuplicateHandle")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-duplicatehandle)\]
    /// DuplicateHandle(-1, source, -1, 0, FALSE, DUPLICATE_SAME_ACCESS)
    ///
    fn try_clone_to_owned(&self) -> firehazard::Result<Self::Owned>;
}

unsafe impl<T: TryCloneToOwned> TryCloneToOwned for &T {
    type Owned = T::Owned;
    fn try_clone_to_owned(&self) -> firehazard::Result<Self::Owned> { (*self).try_clone_to_owned() }
}

/// [`TryCloneToOwned`], but should only fail due to access permissions, quotas, or limits.
///
/// This is a marker trait indicating that [`TryCloneToOwned`] "should" be "infallible".
/// In practice, *all* handles can fail to clone due to these reasons:
///
/// *   Handle access permissions
/// *   Handle quotas/limits?
/// *   Running out of memory?
///
/// However, the *main* reason this trait is distinct from [`TryCloneToOwned`] is to exclude [`token::PseudoHandle`],
/// as none of the following can ever have their handles duplicated:
///
/// *   [get_current_process_token]\()
/// *   [get_current_thread_token]\()
/// *   [get_current_thread_effective_token]\()
///
/// Generally, you instead want e.g. [`duplicate_token_ex`] &mdash; which deep copies the underlying token,
/// instead of shallow copying a mere reference to the object via a new handle.)
///
pub trait CloneToOwned : TryCloneToOwned {
    #[doc(alias = "DuplicateHandle")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-duplicatehandle)\]
    /// DuplicateHandle(-1, source, -1, 0, FALSE, DUPLICATE_SAME_ACCESS)
    ///
    /// Panics on failure (bad access permissions, quotas/limits, OOM.)
    /// To not panic, you could instead use [`try_clone_to_owned`](TryCloneToOwned::try_clone_to_owned).
    ///
    /// On the other hand, if you're failing due to quotas/limits/OOM,
    /// other threads in your program have likely failed to duplicate handles in unexpected ways,
    /// possibly causing undefined behavior if they were written in C++.
    /// The better solution, IMO, to back off and limit your program's handle count,
    /// such that you leave a healthy headroom and don't get too close to the limit in the first place.
    ///
    fn clone_to_owned(&self) -> Self::Owned {
        self.try_clone_to_owned().expect("clone_to_owned: failed to clone handle (bad access permissions? quotas/limits? out of memory?)")
    }
}

#[doc(alias = "DuplicateHandle")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-duplicatehandle)\]
/// DuplicateHandle(-1, source, -1, 0, FALSE, DUPLICATE_SAME_ACCESS)
///
pub trait TryClone : TryCloneToOwned<Owned = Self> + Sized {
    #[doc(alias = "DuplicateHandle")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-duplicatehandle)\]
    /// DuplicateHandle(-1, source, -1, 0, FALSE, DUPLICATE_SAME_ACCESS)
    ///
    fn try_clone(&self) -> firehazard::Result<Self> { self.try_clone_to_owned() }
}

impl<T : TryCloneToOwned<Owned = T>> TryClone for T {}
