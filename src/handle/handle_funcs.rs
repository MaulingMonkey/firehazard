#![cfg_attr(not(doc), allow(unused_imports))]

use crate::*;

use winapi::ctypes::c_void;
use winapi::shared::minwindef::FALSE;
pub(crate) use winapi::shared::ntdef::HANDLE;
use winapi::shared::winerror::*;
use winapi::um::handleapi::*;
use winapi::um::winnt::{DUPLICATE_CLOSE_SOURCE, DUPLICATE_SAME_ACCESS};

use core::fmt::{self, Formatter};
use core::ptr::{null_mut, NonNull};



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/sysinfo/kernel-objects)\]
/// A raw **N**on **N**ull Handle
pub type HANDLENN = NonNull<c_void>;

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle)\]
/// CloseHandle
///
/// Explicitly close a file handle.  This is generally not necessary - owned handle types will automatically close
/// themselves when dropped - but for those niche use cases where you want to inspect the error code by closing said
/// handle yourself - perhaps as part of a 1:1, no behavior changes port of some existing code? - this function is provided.
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
/// // strict handle checks, if enabled, will kill the process before reaching this code:
/// assert_eq!(ERROR_INVALID_HANDLE, e);
/// # }
/// ```
pub fn close_handle(object: impl Into<handle::Owned>) -> Result<(), Error> {
    let object = object.into();
    let h = object.as_handle();
    core::mem::forget(object);
    Error::get_last_if(FALSE == unsafe { CloseHandle(h) })
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle)\]
/// CloseHandle or panic
#[track_caller] pub(crate) unsafe fn drop_close_handle_nn<T>(this: &mut impl AsLocalHandleNN<T>) {
    let handle = this.as_handle_nn().as_ptr().cast();
    assert!(0 != unsafe { CloseHandle(handle) }, "CloseHandle(0x{:X}) failed with GetLastError()={:?}", handle as usize, Error::get_last());
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-duplicatehandle)\]
/// <strike>DuplicateHandle(process, handle, 0, 0, 0, DUPLICATE_CLOSE_SOURCE)</strike>
#[cfg(doc)]
pub unsafe fn close_remote_handle(process: &process::Handle, handle: HANDLE) -> Result<(), Error> {
    Error::get_last_if(FALSE == unsafe { DuplicateHandle(process.as_handle(), handle, null_mut(), null_mut(), 0, false as _, DUPLICATE_CLOSE_SOURCE)})
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-compareobjecthandles)\]
/// <strike>CompareObjectHandles</strike>
#[cfg(doc)] /// NYI (windows SDK too early to link this API?)
pub fn compare_object_handles(first: &handle::Owned, second: &handle::Owned) -> bool {
    // #[link(name = "kernelbase")] extern {} // unable to link against kernelbase?
    FALSE != unsafe { CompareObjectHandles(first.as_handle(), second.as_handle()) }
}

pub(crate) fn debug<T>(fmt: &mut Formatter, module: &str, name: &str, handle: NonNull<T>) -> fmt::Result {
    write!(fmt, "{module}::{name}(")?;
    match handle.as_ptr() as isize {
        // N.B. these are semi-ambiguous: C:\Program Files (x86)\Windows Kits\10\Include\10.0.22621.0\um\winnt.h:
        // #define MEMORY_CURRENT_PARTITION_HANDLE         ((HANDLE) (LONG_PTR) -1)
        // #define MEMORY_SYSTEM_PARTITION_HANDLE          ((HANDLE) (LONG_PTR) -2)
        // #define MEMORY_EXISTING_VAD_PARTITION_HANDLE    ((HANDLE) (LONG_PTR) -3)
        // presumably these handles would be blocked from being convertable to generic handle::Psuedo handles.
        -1  => write!(fmt, "-1 aka GetCurrentProcess()"),
        -2  => write!(fmt, "-2 aka GetCurrentThread()"),
        -3  => write!(fmt, "-3 aka GetCurrentSession()"),               // https://stackoverflow.com/a/45632388/953531
        -4  => write!(fmt, "-4 aka GetCurrentProcessToken()"),
        -5  => write!(fmt, "-5 aka GetCurrentThreadToken()"),
        -6  => write!(fmt, "-6 aka GetCurrentThreadEffectiveToken()"),
        o   => write!(fmt, "0x{:08x}", o as usize),
    }?;
    write!(fmt, ")")
}

#[test] fn verify_debug_values() {
    assert_eq!(-1, get_current_process()                .as_handle() as isize);
    assert_eq!(-2, get_current_thread()                 .as_handle() as isize);
    assert_eq!(-4, get_current_process_token()          .as_handle() as isize);
    assert_eq!(-5, get_current_thread_token()           .as_handle() as isize);
    assert_eq!(-6, get_current_thread_effective_token() .as_handle() as isize);
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-duplicatehandle)\]
/// DuplicateHandle(-1, source, -1, access, inherit, 0)
///
/// ### Examples
/// ```
/// # use firehazard::*;
/// # use winapi::shared::winerror::*;
/// // You can duplicate process/thread psuedo-handles to get real handles:
/// let process = duplicate_handle_local(get_current_process(), access::GENERIC_ALL, false).unwrap();
/// let thread  = duplicate_handle_local(get_current_thread(),  access::GENERIC_ALL, false).unwrap();
///
/// # #[cfg(nope)] { // current disabled: token::Psuedo -> handle::Psuedo conversion
/// // You *cannot* duplicate token psuedo-handles to get real handles:
/// let t = duplicate_handle_local(get_current_process_token(), access::GENERIC_ALL, false);
/// assert_eq!(ERROR_INVALID_HANDLE, t.unwrap_err());
/// let t = duplicate_handle_local(get_current_thread_token(), access::GENERIC_ALL, false);
/// assert_eq!(ERROR_INVALID_HANDLE, t.unwrap_err());
/// let t = duplicate_handle_local(get_current_thread_effective_token(), access::GENERIC_ALL, false);
/// assert_eq!(ERROR_INVALID_HANDLE, t.unwrap_err());
/// # }
/// ```
pub fn duplicate_handle_local<'a>(source: impl AsRef<handle::Psuedo<'a>>, access: impl Into<access::Mask>, inherit_handle: bool) -> Result<handle::Owned, Error> { // TODO: better handle type inference
    let process = get_current_process();
    let source = source.as_ref();

    let mut target = null_mut();
    Error::get_last_if(FALSE == unsafe { DuplicateHandle(
        process.as_handle(),
        source.as_handle(),
        process.as_handle(),
        &mut target,
        access.into().into(),
        inherit_handle as _,
        0
    )})?;

    unsafe { handle::Owned::from_raw(target) }
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-duplicatehandle)\]
/// DuplicateHandle(-1, source, -1, 0, inherit, DUPLICATE_SAME_ACCESS)
///
/// ### Examples
/// ```
/// # use firehazard::*;
/// # use winapi::shared::winerror::*;
/// // You can duplicate process/thread psuedo-handles to get real handles:
/// let process = duplicate_handle_local_same_access(get_current_process(), false).unwrap();
/// let thread  = duplicate_handle_local_same_access(get_current_thread(),  false).unwrap();
///
/// # #[cfg(nope)] { // current disabled: token::Psuedo -> handle::Psuedo conversion
/// // You *cannot* duplicate token psuedo-handles to get real handles:
/// let t = duplicate_handle_local_same_access(get_current_process_token(), false);
/// assert_eq!(ERROR_INVALID_HANDLE, t.unwrap_err());
/// let t = duplicate_handle_local_same_access(get_current_thread_token(), false);
/// assert_eq!(ERROR_INVALID_HANDLE, t.unwrap_err());
/// let t = duplicate_handle_local_same_access(get_current_thread_effective_token(), false);
/// assert_eq!(ERROR_INVALID_HANDLE, t.unwrap_err());
/// # }
/// ```
pub fn duplicate_handle_local_same_access<'a>(source: impl AsRef<handle::Psuedo<'a>>, inherit_handle: bool) -> Result<handle::Owned, Error> { // TODO: better handle type inference
    let process = get_current_process();
    let source = source.as_ref();

    let mut target = null_mut();
    Error::get_last_if(FALSE == unsafe { DuplicateHandle(
        process.as_handle(),
        source.as_handle(),
        process.as_handle(),
        &mut target,
        0,
        inherit_handle as _,
        DUPLICATE_SAME_ACCESS
    )})?;

    unsafe { handle::Owned::from_raw(target) }
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-gethandleinformation)\]
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

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-sethandleinformation)\]
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
