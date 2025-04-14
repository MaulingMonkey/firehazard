#[doc(alias = "ConnectNamedPipe")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-connectnamedpipe)\]
/// ConnectNamedPipe(handle, nullptr)
///
pub fn connect_named_pipe(
    handle:         &impl firehazard::AsLocalHandle, // XXX
    overlapped:     Option<core::convert::Infallible>,
) -> Result<(), firehazard::Error> {
    firehazard::Error::get_last_if(0 == unsafe { winapi::um::namedpipeapi::ConnectNamedPipe(
        handle.as_handle(),
        crate::none2null(overlapped),
    )})
}



#[doc(alias = "ConnectNamedPipe")]
#[cfg(doc)]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-connectnamedpipe)\]
/// ~~ConnectNamedPipe(handle, &overlapped)~~
///
pub unsafe fn connect_named_pipe_overlapped(
    handle:         &impl firehazard::AsLocalHandle, // XXX
    overlapped:     &mut winapi::um::minwinbase::OVERLAPPED,
) -> Result<(), firehazard::Error> {
    firehazard::Error::get_last_if(0 == unsafe { winapi::um::namedpipeapi::ConnectNamedPipe(
        handle.as_handle(),
        overlapped,
    )})
}
