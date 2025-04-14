#[doc(alias = "GetNamedPipeServerSessionId")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getnamedpipeserversessionid)\]
/// GetNamedPipeServerSessionId
///
/// ### Example
/// ```
/// # use firehazard::*;
/// let (read, write) = pipe::create(None, 0).unwrap();
///
/// dbg!(pipe::named::get_client_session_id(&read ).unwrap());
/// dbg!(pipe::named::get_server_session_id(&read ).unwrap());
/// dbg!(pipe::named::get_client_session_id(&write).unwrap());
/// dbg!(pipe::named::get_server_session_id(&write).unwrap());
///
pub fn get_server_session_id(
    handle: &impl firehazard::AsLocalHandle, // XXX
) -> Result<winapi::shared::minwindef::ULONG, firehazard::Error> {
    let mut pid = 0;
    firehazard::Error::get_last_if(0 == unsafe { winapi::um::winbase::GetNamedPipeServerSessionId(
        handle.as_handle(),
        &mut pid,
    )})?;
    Ok(pid)
}
