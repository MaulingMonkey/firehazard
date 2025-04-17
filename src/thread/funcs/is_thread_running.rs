#[doc(alias = "WaitForSingleObject")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-waitforsingleobject)\]
/// WaitForSingleObject(thread, 0) == WAIT_TIMEOUT
///
pub fn is_thread_running<'a>(thread: impl AsRef<firehazard::thread::Handle<'a>>) -> bool {
    use firehazard::AsLocalHandle;
    winapi::shared::winerror::WAIT_TIMEOUT == unsafe { winapi::um::synchapi::WaitForSingleObject(
        thread.as_ref().as_handle(),
        0, // wait duration
    )}
}
