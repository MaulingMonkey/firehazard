use crate::error::LastError;
use crate::process;

use winapi::um::processthreadsapi::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessa)\] CreateProcessA
fn _create_process_a() -> Result<process::OwnedHandle, LastError> { unimplemented!() }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessw)\] CreateProcessW
fn _create_process_w() -> Result<process::OwnedHandle, LastError> { unimplemented!() }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessasusera)\] CreateProcessAsUserA
fn _create_process_user_a() -> Result<process::OwnedHandle, LastError> { unimplemented!() }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessasuserw)\] CreateProcessAsUserW
fn _create_process_user_w() -> Result<process::OwnedHandle, LastError> { unimplemented!() }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-createprocesswithlogonw)\] CreateProcessWithLogonW
fn _create_process_with_logon_w() -> Result<process::OwnedHandle, LastError> { unimplemented!() }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-createprocesswithtokenw)\] CreateProcessWithTokenW
fn _create_process_with_token_w() -> Result<process::OwnedHandle, LastError> { unimplemented!() }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocess)\] GetCurrentProcess
pub fn get_current_process() -> process::PsuedoHandle { unsafe { process::PsuedoHandle::from_raw_unchecked(GetCurrentProcess()) } }

// get/set process afinity masks, etc.
