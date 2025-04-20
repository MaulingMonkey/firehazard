#[doc(alias = "GetExitCodeThread")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getexitcodethread)\]
/// GetExitCodeThread
///
/// ### Returns
/// *   `Ok(STILL_ACTIVE)` / `Ok(STATUS_PENDING)`   if `thread` is still running
/// *   `Ok(0)`                                     if `thread` exited "successfully"
/// *   `Ok(exit_code)`                             if `thread` exited otherwise
/// *   `Err(...)`                                  if `thread` lacks appropriate querying permissions?
/// *   `Err(...)`                                  if `thread` is an invalid handle?
///
pub fn get_exit_code_thread<'a>(thread: impl Into<firehazard::thread::Handle<'a>>) -> Result<u32, firehazard::Error> {
    use firehazard::AsLocalHandle;
    let mut exit_code = 0;
    firehazard::Error::get_last_if(0 == unsafe { winapi::um::processthreadsapi::GetExitCodeThread(
        thread.into().as_handle(),
        &mut exit_code,
    )})?;
    Ok(exit_code)
}
