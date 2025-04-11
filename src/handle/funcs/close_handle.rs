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
pub fn close_handle(object: impl Into<firehazard::handle::Owned>) -> Result<(), firehazard::Error> {
    use firehazard::*;

    let object = object.into();
    let h = core::mem::ManuallyDrop::new(object).as_handle();
    Error::get_last_if(0 == unsafe { winapi::um::handleapi::CloseHandle(h) })
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle)\]
/// CloseHandle or panic
///
#[track_caller] pub(crate) unsafe fn drop_close_handle_nn<T>(this: &mut impl firehazard::AsLocalHandleNN<T>) {
    let handle = this.as_handle_nn().as_ptr().cast();
    if 0 != unsafe { winapi::um::handleapi::CloseHandle(handle) } { return }
    panic!("CloseHandle(0x{:X}) failed with GetLastError()={:?}", handle as usize, firehazard::Error::get_last());
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-duplicatehandle)\]
/// <strike>DuplicateHandle(process, handle, 0, 0, 0, DUPLICATE_CLOSE_SOURCE)</strike>
///
#[cfg(doc)]
pub unsafe fn close_remote_handle(process: &firehazard::process::Handle, handle: firehazard::HANDLE) -> Result<(), firehazard::Error> {
    use firehazard::*;

    Error::get_last_if(0 == unsafe { winapi::um::handleapi::DuplicateHandle(
        process.as_handle(),
        handle,
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        0,
        false as _,
        winapi::um::winnt::DUPLICATE_CLOSE_SOURCE
    )})
}
