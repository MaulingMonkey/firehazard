//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocesstoken)\]
//! OpenProcessToken

use crate::*;
use crate::error::LastError;
use winapi::shared::winerror::ERROR_INVALID_HANDLE;
use winapi::um::processthreadsapi::{GetCurrentProcess, OpenProcessToken};
use std::ptr::null_mut;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocesstoken)\] OpenProcessToken(GetCurrentProcess(), TOKEN_ALL_ACCESS, ...)
///
/// ### Example
/// ```
/// # use win32_security_playground::*;
/// let token : token::Handle = open_process_token::current_process(token::ALL_ACCESS).unwrap();
/// ```
///
/// ### Errors
/// *   `ERROR_ACCESS_DENIED`   if the current process/thread token has an untrusted integrity level
pub fn current_process(rights: impl Into<token::AccessRights>) -> Result<token::Handle, LastError> {
    let process = unsafe { GetCurrentProcess() }; // -1 psuedo-handle / never fails

    let mut h = null_mut();
    let success = 0 != unsafe { OpenProcessToken(process, rights.into().into(), &mut h) };
    let h = unsafe { token::Handle::from_raw(h) };

    if      !success                { Err(LastError::get()) }
    else if h.as_handle().is_null() { Err(LastError(ERROR_INVALID_HANDLE)) }
    else                            { Ok(h) }
}
