#[doc(alias = "GetCurrentThreadId")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentthreadid)\]
/// GetCurrentThreadId
///
pub fn get_current_thread_id() -> firehazard::thread::Id { unsafe { winapi::um::processthreadsapi::GetCurrentThreadId() } }
