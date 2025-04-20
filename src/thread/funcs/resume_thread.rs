#[doc(alias = "ResumeThread")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-resumethread)\]
/// ResumeThread
///
/// Resumes execution of a previously suspended thread.
///
/// ### Safety
/// As this operates on simple refcounts, and not reciepts or other proof of ownership over the suspension request,
/// this function can allow you to resume a thread out from underneath a debugger, garbage collector,
/// or other piece of code that expected a thread to remain suspended - and the data said thread accessed, unmodified.
///
/// This is practically **begging** for undefined behavior.
///
/// ### Returns
/// *   `Ok(previous_suspend_count)`    &mdash; if the call succeeded and the suspend count decreased
/// *   `Err(ERROR_BAD_ACCESS)`?        &mdash; if `thread`'s handle lacks the `THREAD_SUSPEND_RESUME` access right
/// *   `Err(ERROR_INVALID_HANDLE)`?    &mdash; if `thread`'s handle is invalid (or not a thread)
///
pub unsafe fn resume_thread<'a>(thread: impl Into<firehazard::thread::Handle<'a>>) -> Result<u32, firehazard::Error> {
    use firehazard::AsLocalHandle;
    let r = unsafe { winapi::um::processthreadsapi::ResumeThread(thread.into().as_handle()) };
    firehazard::Error::get_last_if(r as i32 == -1)?;
    Ok(r)
}
