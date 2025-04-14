#[doc(alias = "GetNamedPipeClientComputerName")]
#[doc(alias = "GetNamedPipeClientComputerNameW")]
#[cfg(std)]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-getnamedpipeclientcomputernamew)\]
/// GetNamedPipeClientComputerNameW
///
/// ### Example
/// ```
/// # use firehazard::*;
/// # use abistr::*;
/// # use winapi::shared::winerror::*;
/// #
/// // This function fails on local pipes
/// let local = create_named_pipe_w(
///     cstr16!(r"\\.\pipe\local\example-for-client-computer-name-check"),
///     pipe::ACCESS_DUPLEX, 0, pipe::UNLIMITED_INSTANCES,
///     0, 0, None, None,
/// ).unwrap();
/// assert_eq!(ERROR_PIPE_LOCAL, get_named_pipe_client_computer_name(&local).unwrap_err());
///
/// // ...including local *anonymous* pipes
/// let (read, write) = create_pipe(None, 0).unwrap();
/// assert_eq!(ERROR_PIPE_LOCAL, get_named_pipe_client_computer_name(&read ).unwrap_err());
/// assert_eq!(ERROR_PIPE_LOCAL, get_named_pipe_client_computer_name(&write).unwrap_err());
///
/// // ...including local *server* pipes available for remote connection
/// let global = create_named_pipe_w(
///     cstr16!(r"\\.\pipe\example-for-client-computer-name-check"),
///     pipe::ACCESS_DUPLEX, pipe::ACCEPT_REMOTE_CLIENTS, pipe::UNLIMITED_INSTANCES,
///     0, 0, None, None,
/// ).unwrap();
/// assert_eq!(ERROR_PIPE_LOCAL, get_named_pipe_client_computer_name(&global).unwrap_err());
/// ```
pub fn get_named_pipe_client_computer_name(
    handle: &impl firehazard::AsLocalHandle,
) -> Result<std::ffi::OsString, firehazard::Error> {
    use std::os::windows::ffi::OsStringExt;

    let mut buf = [core::mem::MaybeUninit::uninit(); winapi::shared::minwindef::MAX_PATH];
    let buf = get_named_pipe_client_computer_name_w(handle, &mut buf[..])?;
    Ok(std::ffi::OsString::from_wide(buf))
}



#[doc(alias = "GetNamedPipeClientComputerNameA")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getnamedpipeclientcomputernamea)\]
/// GetNamedPipeClientComputerNameA
///
pub fn get_named_pipe_client_computer_name_a<'name>(
    handle:                 &impl firehazard::AsLocalHandle, // XXX
    client_computer_name:   &'name mut [core::mem::MaybeUninit<u8>],
) -> Result<&'name [u8], firehazard::Error> { // XXX: CStr?
    use crate::From32;

    let client_computer_name_len32 = u32::try_from(client_computer_name.len()).unwrap_or(!0);
    let client_computer_name_len = usize::from32(client_computer_name_len32);
    let client_computer_name = &mut client_computer_name[..client_computer_name_len];
    firehazard::Error::get_last_if(0 == unsafe { winapi::um::winbase::GetNamedPipeClientComputerNameA(
        handle                      .as_handle(),
        client_computer_name        .as_mut_ptr().cast(),
        client_computer_name_len32,
    )})?;
    let nul = client_computer_name.iter().copied().position(|b| 0 == unsafe { b.assume_init() });
    let client_computer_name = &mut client_computer_name[..nul.unwrap_or(client_computer_name_len)];
    Ok(unsafe { crate::slice_assume_init_mut(client_computer_name) })
}



#[doc(alias = "GetNamedPipeClientComputerName")]
#[doc(alias = "GetNamedPipeClientComputerNameW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-getnamedpipeclientcomputernamew)\]
/// GetNamedPipeClientComputerNameW
///
pub fn get_named_pipe_client_computer_name_w<'name>(
    handle:                 &impl firehazard::AsLocalHandle, // XXX
    client_computer_name:   &'name mut [core::mem::MaybeUninit<u16>],
) -> Result<&'name mut [u16], firehazard::Error> { // XXX: OsStr?
    use crate::From32;

    let client_computer_name_len32 = u32::try_from(client_computer_name.len()).unwrap_or(!0);
    let client_computer_name_len = usize::from32(client_computer_name_len32);
    let client_computer_name = &mut client_computer_name[..client_computer_name_len];
    firehazard::Error::get_last_if(0 == unsafe { winapi::um::namedpipeapi::GetNamedPipeClientComputerNameW(
        handle                      .as_handle(),
        client_computer_name        .as_mut_ptr().cast(),
        client_computer_name_len32,
    )})?;
    let nul = client_computer_name.iter().copied().position(|b| 0 == unsafe { b.assume_init() });
    let client_computer_name = &mut client_computer_name[..nul.unwrap_or(client_computer_name_len)];
    Ok(unsafe { crate::slice_assume_init_mut(client_computer_name) })
}
