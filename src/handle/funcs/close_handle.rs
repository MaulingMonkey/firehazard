#[doc(alias = "CloseHandle")]
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
///
/// ### Errors
///
/// | `handle`                  | Error <br> (via GetLastError)                 | Exception <br> [(Strict Handle Checks)](crate::process::mitigation::StrictHandleCheckPolicy)  |
/// | ------------------------- |:---------------------------------------------:|:---------------------------------------------------------------------------------------------:|
/// | null                      | ERROR_INVALID_HANDLE                          | <span style="opacity: 50%">None</span>                                                        |
/// | INVALID_HANDLE_VALUE      | <span style="opacity: 50%">Success!</span>    | <span style="opacity: 50%">None</span>                                                        |
/// | closed/dangling           | ERROR_INVALID_HANDLE                          | <span style="opacity: 50%">None</span>                                                        |
/// | never valid               | ERROR_INVALID_HANDLE                          | <span style="opacity: 50%">None</span>                                                        |
///
pub fn close_handle(object: impl Into<handle::Owned>) -> firehazard::Result<()> {
    let object = object.into();
    let h = core::mem::ManuallyDrop::new(object).as_handle();
    firehazard::Error::get_last_if(0 == unsafe { winapi::um::handleapi::CloseHandle(h) })
}



#[doc(alias = "CloseHandle")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle)\]
/// CloseHandle or panic
///
#[track_caller] pub(crate) unsafe fn drop_close_handle_nn<T>(this: &mut impl AsLocalHandleNN<T>) {
    let handle = this.as_handle_nn().as_ptr().cast();
    if 0 != unsafe { winapi::um::handleapi::CloseHandle(handle) } { return }
    panic!("CloseHandle(0x{:X}) failed with GetLastError()={:?}", handle as usize, firehazard::Error::get_last());
}



#[doc(alias = "CloseHandle")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-duplicatehandle)\]
/// <strike>DuplicateHandle(process, handle, 0, 0, 0, DUPLICATE_CLOSE_SOURCE)</strike>
///
#[cfg(doc)]
pub unsafe fn close_remote_handle(process: &process::Handle, handle: HANDLE) -> firehazard::Result<()> {
    firehazard::Error::get_last_if(0 == unsafe { winapi::um::handleapi::DuplicateHandle(
        process.as_handle(),
        handle,
        null_mut(),
        null_mut(),
        0,
        false as _,
        winapi::um::winnt::DUPLICATE_CLOSE_SOURCE
    )})
}



tests! {
    use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
    use winapi::um::errhandlingapi::{GetLastError, SetLastError};

    #[test] #[strict_handle_check_exception = 0] // no exception
    fn close_handle_null() {
        assert!(0 == unsafe { CloseHandle(null_mut()) });
        assert_eq!(ERROR_INVALID_HANDLE, unsafe { GetLastError() });
    }

    #[test] #[strict_handle_check_exception = 0] // no exception
    fn close_handle_invalid() {
        unsafe { SetLastError(0) }; // clear error - the next call "succeeds"!
        assert!(0 != unsafe { CloseHandle(INVALID_HANDLE_VALUE) });
        assert_eq!(0, unsafe { GetLastError() });
    }

    #[test] #[isolate] #[strict_handle_check_exception = 0] // no exception
    fn close_handle_dangling() {
        use std::os::windows::io::AsRawHandle;
        let file = std::fs::File::open("Readme.md").unwrap();
        let handle = file.as_raw_handle();
        drop(file);
        assert!(0 == unsafe { CloseHandle(handle.cast()) });
        assert_eq!(ERROR_INVALID_HANDLE, unsafe { GetLastError() });
    }

    #[test] #[isolate] #[strict_handle_check_exception = 0] // no exception
    fn close_handle_never_valid() {
        assert!(0 == unsafe { CloseHandle(0x12345678_usize as *mut _) });
        assert_eq!(ERROR_INVALID_HANDLE, unsafe { GetLastError() });
    }
}
