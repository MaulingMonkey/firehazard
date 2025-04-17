#[doc(alias = "ExitThread")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-exitthread)\]
/// ExitThread
///
pub fn exit_thread(exit_code: u32) { unsafe { winapi::um::processthreadsapi::ExitThread(exit_code) } }
