#[doc(alias = "DisconnectNamedPipe")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-disconnectnamedpipe)\]
/// DisconnectNamedPipe
///
pub fn disconnect_named_pipe(
    handle: &impl firehazard::AsLocalHandle, // XXX
) -> Result<(), firehazard::Error> {
    firehazard::Error::get_last_if(0 == unsafe { winapi::um::namedpipeapi::DisconnectNamedPipe(
        handle.as_handle()
    )})
}
