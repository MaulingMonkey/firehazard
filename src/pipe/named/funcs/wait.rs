#[doc(alias = "WaitNamedPipe")]
#[doc(alias = "WaitNamedPipeW")]
#[cfg(std)]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-waitnamedpipew)\]
/// WaitNamedPipeW
///
pub fn wait(
    name:           impl AsRef<std::ffi::OsStr>,
    timeout:        impl Into<firehazard::NMPWAIT>,
) -> Result<(), firehazard::Error> {
    wait_w(
        crate::util::osstr_to_wide0(name.as_ref(), &mut std::vec::Vec::new())?,
        timeout,
    )
}



#[doc(alias = "WaitNamedPipeA")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-waitnamedpipea)\]
/// WaitNamedPipeA
///
pub fn wait_a(
    name:           impl abistr::TryIntoAsCStr,
    timeout:        impl Into<firehazard::NMPWAIT>,
) -> Result<(), firehazard::Error> {
    use abistr::AsCStr;

    firehazard::Error::get_last_if(0 == unsafe { winapi::um::winbase::WaitNamedPipeA(
        name        .try_into()?.as_cstr(),
        timeout     .into().0,
    )})
}



#[doc(alias = "WaitNamedPipe")]
#[doc(alias = "WaitNamedPipeW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-waitnamedpipew)\]
/// WaitNamedPipeW
///
pub fn wait_w(
    name:           impl abistr::TryIntoAsCStr<u16>,
    timeout:        impl Into<firehazard::NMPWAIT>,
) -> Result<(), firehazard::Error> {
    use abistr::AsCStr;

    firehazard::Error::get_last_if(0 == unsafe { winapi::um::namedpipeapi::WaitNamedPipeW(
        name        .try_into()?.as_cstr(),
        timeout     .into().0,
    )})
}
