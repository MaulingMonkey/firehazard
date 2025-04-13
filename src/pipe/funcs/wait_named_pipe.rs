#[cfg(std)]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-waitnamedpipew)\]
/// WaitNamedPipeW
///
pub fn wait_named_pipe(
    name:           impl AsRef<std::ffi::OsStr>,
    timeout:        impl Into<firehazard::NMPWAIT>,
) -> Result<(), firehazard::Error> {
    use winapi::shared::winerror::ERROR_ILLEGAL_CHARACTER;
    use std::os::windows::ffi::OsStrExt;

    let name = name.as_ref();
    let name = name.encode_wide().chain(Some(0)).collect::<std::vec::Vec<_>>();
    let name = abistr::CStrNonNull::from_units_with_nul(&name).map_err(|_| ERROR_ILLEGAL_CHARACTER)?;
    wait_named_pipe_w(name, timeout)
}



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-waitnamedpipea)\]
/// WaitNamedPipeA
///
pub fn wait_named_pipe_a(
    name:           impl abistr::TryIntoAsCStr,
    timeout:        impl Into<firehazard::NMPWAIT>,
) -> Result<(), firehazard::Error> {
    use abistr::AsCStr;

    firehazard::Error::get_last_if(0 == unsafe { winapi::um::winbase::WaitNamedPipeA(
        name        .try_into()?.as_cstr(),
        timeout     .into().0,
    )})
}



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-waitnamedpipew)\]
/// WaitNamedPipeW
///
pub fn wait_named_pipe_w(
    name:           impl abistr::TryIntoAsCStr<u16>,
    timeout:        impl Into<firehazard::NMPWAIT>,
) -> Result<(), firehazard::Error> {
    use abistr::AsCStr;

    firehazard::Error::get_last_if(0 == unsafe { winapi::um::namedpipeapi::WaitNamedPipeW(
        name        .try_into()?.as_cstr(),
        timeout     .into().0,
    )})
}
