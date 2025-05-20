#[doc(alias = "WaitNamedPipe")] #[doc(no_inline)] pub use wait_w as wait;



#[doc(alias = "WaitNamedPipeA")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-waitnamedpipea)\]
/// WaitNamedPipeA
///
pub fn wait_a(
    name:           impl string::InNarrow,
    timeout:        impl Into<firehazard::NMPWAIT>,
) -> Result<(), firehazard::Error> {
    let timeout = timeout.into().0;

    string::convert_to_cstrnn::<{limit::stack::PIPE_NAME}, _, _>(name, |name| {
        firehazard::Error::get_last_if(0 == unsafe { winapi::um::winbase::WaitNamedPipeA(
            name.as_cstr(),
            timeout,
        )})
    })?
}



#[doc(alias = "WaitNamedPipeW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-waitnamedpipew)\]
/// WaitNamedPipeW
///
pub fn wait_w(
    name:           impl string::InWide,
    timeout:        impl Into<firehazard::NMPWAIT>,
) -> Result<(), firehazard::Error> {
    let timeout = timeout.into().0;

    string::convert_to_cstrnn::<{limit::stack::PIPE_NAME}, _, _>(name, |name| {
        firehazard::Error::get_last_if(0 == unsafe { winapi::um::namedpipeapi::WaitNamedPipeW(
            name.as_cstr(),
            timeout,
        )})
    })?
}
