#[doc(alias = "GetExitCodeThread")]
#[doc(alias = "WaitForSingleObject")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-waitforsingleobject)\] WaitForSingleObject(thread, INFINITE) +<br>
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getexitcodethread)\] GetExitCodeThread
///
pub fn wait_for_thread<'a>(thread: impl Into<thread::Handle<'a>>) -> firehazard::Result<u32> {
    use winapi::shared::winerror::*;
    use winapi::um::winbase::*;

    let thread = thread.into();
    match unsafe { winapi::um::synchapi::WaitForSingleObject(thread.as_handle(), INFINITE) } {
        WAIT_OBJECT_0       => {},
        WAIT_ABANDONED_0    => return Err(firehazard::Error(ERROR_ABANDONED_WAIT_0)),   // shouldn't happen as `thread` isn't a mutex, right?
        WAIT_TIMEOUT        => return Err(firehazard::Error(ERROR_ABANDONED_WAIT_63)),  // shouldn't happen - hopefully the `63` hints that something is funky?
        WAIT_FAILED         => return Err(firehazard::Error::get_last()),
        _                   => return Err(firehazard::Error(ERROR_ABANDONED_WAIT_63)),  // shouldn't happen - hopefully the `63` hints that something is funky?
    }
    get_exit_code_thread(thread)
}
