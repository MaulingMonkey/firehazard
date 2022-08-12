use crate::thread;

use winapi::um::processthreadsapi::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentthread)\] GetCurrentThread
pub fn get_current_thread() -> thread::PsuedoHandle { unsafe { thread::PsuedoHandle::from_raw_unchecked(GetCurrentThread()) } }
