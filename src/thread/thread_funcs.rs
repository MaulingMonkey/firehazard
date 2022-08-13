use crate::error::LastError;
use crate::thread;

use winapi::um::processthreadsapi::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentthread)\] GetCurrentThread
pub fn get_current_thread() -> thread::PsuedoHandle { unsafe { thread::PsuedoHandle::from_raw_unchecked(GetCurrentThread()) } }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-resumethread)\] ResumeThread
pub fn resume_thread(thread: &thread::OwnedHandle) -> Result<u32, LastError> { let r = unsafe { ResumeThread(thread.as_handle()) }; if r as i32 != -1 { Ok(r) } else { Err(LastError::get()) } }

#[allow(dead_code)]
/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-suspendthread)\] SuspendThread
pub(crate) fn suspend_thread(thread: &thread::OwnedHandle) -> Result<u32, LastError> { let r = unsafe { SuspendThread(thread.as_handle()) }; if r as i32 != -1 { Ok(r) } else { Err(LastError::get()) } }
