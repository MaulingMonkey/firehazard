/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getnamedpipeclientprocessid)\]
/// GetNamedPipeClientProcessId
///
/// ### Example
/// ```
/// # use firehazard::*;
/// let (read, write) = create_pipe(None, 0).unwrap();
///
/// // While you'll likely pass a pipe end to a child process, both ends start out owned by the current process.
/// let pid = get_current_process_id();
/// assert_eq!(pid, get_named_pipe_client_process_id(&read ).unwrap());
/// assert_eq!(pid, get_named_pipe_server_process_id(&read ).unwrap());
/// assert_eq!(pid, get_named_pipe_client_process_id(&write).unwrap());
/// assert_eq!(pid, get_named_pipe_server_process_id(&write).unwrap());
/// ```
///
pub fn get_named_pipe_client_process_id(
    handle: &impl firehazard::AsLocalHandle, // XXX
) -> Result<firehazard::process::Id, firehazard::Error> {
    let mut pid = 0;
    firehazard::Error::get_last_if(0 == unsafe { winapi::um::winbase::GetNamedPipeClientProcessId(
        handle.as_handle(),
        &mut pid,
    )})?;
    Ok(pid)
}
