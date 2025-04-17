#[doc(alias = "ExitThread")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-exitthread)\]
/// ExitThread
///
pub fn exit_thread(exit_code: u32) -> ! {
    unsafe { winapi::um::processthreadsapi::ExitThread(exit_code) };
    if cfg!(debug_assertions) {
        panic!("undefined behavior: ExitThread returned, but the caller's thread should've exited!");
    } else {
        unsafe { core::hint::unreachable_unchecked() }
    }
}
