//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openthreadtoken)\]
//! OpenThreadToken

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openthreadtoken)\] OpenThreadToken(GetCurrentThread(), TOKEN_ALL_ACCESS, ...)
///
/// ### Example
/// ```
/// # use win32_security_playground::*;
/// assert!(open_thread_token::current_thread(false).is_none());
/// // TODO: set/verify token and unwrap some
/// ```
///
/// ### Returns
/// * `None` if the current thread had no impersonation token set (e.g. OpenThreadToken failed with GetLastError() == ERROR_NO_TOKEN)
pub fn current_thread(as_self: bool) -> Option<crate::token::Handle> {
    use crate::error::get_last_error;
    use crate::token::Handle;
    use winapi::shared::winerror::ERROR_NO_TOKEN;
    use winapi::um::processthreadsapi::{GetCurrentThread, OpenThreadToken};
    use winapi::um::winnt::TOKEN_ALL_ACCESS;
    use std::ptr::null_mut;

    let thread = unsafe { GetCurrentThread() };
    assert!(!thread.is_null(), "GetCurrentThread");

    let mut h = null_mut();
    let success = 0 != unsafe { OpenThreadToken(thread, TOKEN_ALL_ACCESS, as_self as _, &mut h) };
    if !success {
        match get_last_error() {
            ERROR_NO_TOKEN  => return None,
            gle             => panic!("OpenThreadToken GetLastError()={gle}"),
        }
    } else {
        assert!(!h.is_null(), "OpenThreadToken");
        Some(unsafe { Handle::from_raw(h) })
    }
}
