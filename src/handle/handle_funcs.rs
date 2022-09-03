#![cfg_attr(not(doc), allow(unused_imports))]

use crate::*;

use winapi::ctypes::c_void;
use winapi::shared::minwindef::FALSE;
use winapi::shared::ntdef::HANDLE;
use winapi::um::handleapi::*;
use winapi::um::winnt::DUPLICATE_CLOSE_SOURCE;

use core::ptr::{null_mut, NonNull};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/sysinfo/kernel-objects)\]
/// A raw **N**on **N**ull Handle
pub type HANDLENN = NonNull<c_void>;

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle)\]
/// CloseHandle
///
/// ### Example
/// ```
/// # #[cfg(std)] {
/// # use firehazard::*;
/// # use winapi::shared::winerror::*;
/// let thread : thread::OwnedHandle = std::thread::spawn(||{}).into();
/// let dangling = unsafe { thread::OwnedHandle::from_raw(thread.as_handle()).unwrap() };
/// let _ : ()    = close_handle( thread ).unwrap();
/// let e : Error = close_handle(dangling).unwrap_err();
/// assert_eq!(ERROR_INVALID_HANDLE, e);
/// # }
/// ```
pub fn close_handle(object: impl Into<handle::Owned>) -> Result<(), Error> {
    let object = object.into();
    let h = object.as_handle();
    core::mem::forget(object);
    Error::get_last_if(FALSE == unsafe { CloseHandle(h) })
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle)\]
/// CloseHandle or panic
#[track_caller] pub(crate) unsafe fn drop_close_handle_nn<T>(this: &mut impl AsLocalHandleNN<T>) {
    let handle = this.as_handle_nn().as_ptr().cast();
    assert!(0 != unsafe { CloseHandle(handle) }, "CloseHandle(0x{:X}) failed with GetLastError()={:?}", handle as usize, Error::get_last());
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-duplicatehandle)\]
/// <strike>DuplicateHandle(process, handle, 0, 0, 0, 0, DUPLICATE_CLOSE_SOURCE)</strike>
#[cfg(doc)]
pub unsafe fn close_remote_handle(process: &process::Handle, handle: HANDLE) -> Result<(), Error> {
    Error::get_last_if(FALSE == unsafe { DuplicateHandle(process.as_handle(), handle, null_mut(), null_mut(), 0, false as _, DUPLICATE_CLOSE_SOURCE)})
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-compareobjecthandles)\]
/// <strike>CompareObjectHandles</strike>
#[cfg(doc)] /// NYI (windows SDK too early to link this win10+ API?)
pub fn compare_object_handles(first: &handle::Owned, second: &handle::Owned) -> bool {
    // #[link(name = "kernelbase")] extern {} // unable to link against kernelbase?
    FALSE != unsafe { CompareObjectHandles(first.as_handle(), second.as_handle()) }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-duplicatehandle)\]
/// <strike>DuplicateHandle</strike>
#[cfg(doc)] /// Too parameterized (may or may not close / be locally owned)
pub fn duplicate_handle<'t>(
    source_process: &process::Handle,
    source:         &handle::Owned,
    target_process: impl Into<Option<&'t process::Handle<'t>>>,
    desired_access: impl Into<access::Mask>,
    inherit_handle: bool,
    options:        u32,                                    // TODO: type
) -> Result<handle::Owned, Error> {
    let mut target = null_mut();
    Error::get_last_if(FALSE == unsafe { DuplicateHandle(
        source_process.as_handle(),
        source.as_handle(),
        target_process.into().map_or(null_mut(), |h| h.as_handle()),
        &mut target,
        desired_access.into().into(),
        inherit_handle as _,
        options,
    )})?;

    unsafe { handle::Owned::from_raw(target) }.ok_or(Error(ERROR_INVALID_HANDLE))
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-gethandleinformation)\]
/// GetHandleInformation
///
/// ### Example
/// ```
/// # #[cfg(std)] {
/// # use firehazard::*;
/// # use winapi::shared::winerror::*;
/// let thread : thread::OwnedHandle = std::thread::spawn(||{}).into();
/// let info = get_handle_information(&thread).unwrap();
/// assert_eq!(info, handle::Flags::default());
/// close_handle(thread).unwrap();
/// # }
/// ```
pub fn get_handle_information<'a>(object: impl AsRef<handle::Borrowed<'a>>) -> Result<handle::Flags, Error> {
    let mut flags = 0;
    Error::get_last_if(FALSE == unsafe { GetHandleInformation(object.as_ref().as_handle(), &mut flags) })?;
    Ok(unsafe { handle::Flags::from_unchecked(flags) })
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-sethandleinformation)\]
/// SetHandleInformation
///
/// ### Example
/// ```
/// # #[cfg(std)] {
/// # use firehazard::*;
/// # use winapi::shared::winerror::*;
/// let thread : thread::OwnedHandle = std::thread::spawn(||{}).into();
/// let info = get_handle_information(&thread).unwrap();
/// set_handle_information(&thread, !0, info).unwrap();
/// close_handle(thread).unwrap();
/// # }
/// ```
pub fn set_handle_information<'a>(object: impl AsRef<handle::Borrowed<'a>>, mask: impl Into<handle::FlagsMask>, flags: impl Into<handle::Flags>) -> Result<(), Error> {
    Error::get_last_if(FALSE == unsafe { SetHandleInformation(object.as_ref().as_handle(), mask.into().into(), flags.into().into()) })
}