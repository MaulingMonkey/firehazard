//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocesstoken)\]
//! OpenProcessToken

use crate::error::get_last_error;
use crate::token::Handle;
use winapi::um::processthreadsapi::{GetCurrentProcess, OpenProcessToken};
use winapi::um::winnt::TOKEN_ALL_ACCESS;
use std::ptr::null_mut;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocesstoken)\] OpenProcessToken(GetCurrentProcess(), TOKEN_ALL_ACCESS, ...)
///
/// ### Example
/// ```
/// # use win32_security_playground::*;
/// let token : token::Handle = open_process_token::current_process();
/// ```
pub fn current_process() -> Handle {
    let process = unsafe { GetCurrentProcess() };
    assert!(!process.is_null(), "GetCurrentProcess");

    let mut h = null_mut();
    let success = 0 != unsafe { OpenProcessToken(process, TOKEN_ALL_ACCESS, &mut h) };
    assert!(success, "OpenProcessToken GetLastError()={}", get_last_error());
    assert!(!h.is_null(), "OpenProcessToken");

    unsafe { Handle::from_raw(h) }
}
