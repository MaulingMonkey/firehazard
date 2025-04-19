#[doc(alias = "WaitForSingleObject")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-waitforsingleobject)\]
/// WaitForSingleObject(thread, 0) == WAIT_TIMEOUT
///
/// | Thread State      | Returns   |
/// | ------------------| ----------|
/// | Running           | true      |
/// | Blocked           | true      |
/// | Suspended         | true      |
/// | Exited            | false     |
/// | Killed            | false     |
///
/// Since [`thread::Handle`](crate::thread::Handle) should be a valid handle,
/// it's "impossible" to pass a dangling/invalid handle.  If you do anyways,
/// the return value is indeterminite, `STATUS_INVALID_HANDLE` may be thrown if
/// [strict handle checks](crate::process::mitigation::StrictHandleCheckPolicy)
/// are enabled.
///
pub fn is_thread_alive<'a>(thread: impl AsRef<firehazard::thread::Handle<'a>>) -> bool {
    use firehazard::AsLocalHandle;
    winapi::shared::winerror::WAIT_TIMEOUT == unsafe { winapi::um::synchapi::WaitForSingleObject(
        thread.as_ref().as_handle(),
        0, // wait duration
    )}
}

#[doc(hidden)] #[deprecated = "renamed to is_thread_alive: will return `true` for suspended threads, which isn't \"running\" per se"] pub use is_thread_alive as is_thread_running;
