#[doc(alias = "SuspendThread")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-suspendthread)\]
/// SuspendThread
///
/// Suspend execution of a thread.
///
/// **You are strongly discouraged from suspending threads in your own process.**
/// Doing so anyways is practically guaranteed to randomly deadlock your process.
/// You will suspend threads in random states - including states where they hold random system or allocator locks.
/// Instead, use a proper mutex or semaphore, allowing threads to become "suspended" in known states where they don't hold critical locks.
///
/// ### Safety
/// I assume bad things are likely to happen if the refcount overflows (e.g. acting as a [`resume_thread`], which is `unsafe`.)
/// It might be worth testing to verify if that's true or not... were it not for the big fat warning above about suspending your own process's threads - that you should read anyways.
/// (Randomly deadlocking your process is technically sound, though, so it stays out of the safety section.)
///
/// ### Returns
/// *   `Ok(previous_suspend_count)`    &mdash; if the call succeeded and the suspend count decreased
/// *   `Err(ERROR_BAD_ACCESS)`?        &mdash; if `thread`'s handle lacks the `THREAD_SUSPEND_RESUME` access right
/// *   `Err(ERROR_INVALID_HANDLE)`?    &mdash; if `thread`'s handle is invalid (or not a thread)
///
pub unsafe fn suspend_thread<'a>(thread: impl AsRef<firehazard::thread::Handle<'a>>) -> Result<u32, firehazard::Error> {
    use firehazard::AsLocalHandle;
    let r = unsafe { winapi::um::processthreadsapi::SuspendThread(thread.as_ref().as_handle()) };
    firehazard::Error::get_last_if(r as i32 == -1)?;
    Ok(r)
}
