#[doc(alias = "ImpersonateNamedPipeClient")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-impersonatenamedpipeclient)\]
/// ImpersonateNamedPipeClient
///
pub fn impersonate_client(
    handle: &impl firehazard::AsLocalHandle, // XXX
) -> Result<(), firehazard::Error> {
    firehazard::Error::get_last_if(0 == unsafe { winapi::um::namedpipeapi::ImpersonateNamedPipeClient(
        handle.as_handle(),
    )})
}

// TODO: Read up on this
//  *   <https://jsecurity101.medium.com/exploring-impersonation-through-the-named-pipe-filesystem-driver-15f324dfbaf2>
//  *   <https://learn.microsoft.com/en-us/windows/win32/ipc/impersonating-a-named-pipe-client>
