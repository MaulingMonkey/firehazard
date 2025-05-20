#[doc(no_inline)] pub use call_w as call;



#[doc(alias = "CallNamedPipeA")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-callnamedpipea)\]
/// CallNamedPipeA
///
pub fn call_a<'out_buffer>(
    name:               impl string::InNarrow,
    in_buffer:          &[u8],
    out_buffer:         &'out_buffer mut [core::mem::MaybeUninit<u8>],
    timeout:            impl Into<firehazard::NMPWAIT>,
) -> Result<&'out_buffer mut [u8], firehazard::Error> {
    let in_buffer_len32     = u32::try_from(in_buffer .len()).map_err(|_| ERROR::INVALID_PARAMETER)?;
    let out_buffer_len32    = u32::try_from(out_buffer.len()).unwrap_or(!0);
    let timeout             = timeout.into().0;

    let mut read_bytes = 0;
    string::convert_to_cstrnn::<{limit::stack::PIPE_NAME}, _, _>(
        name, |name| firehazard::Error::get_last_if(0 == unsafe { winapi::um::winbase::CallNamedPipeA(
            name.as_cstr(),
            in_buffer.as_ptr() as *const _ as *mut _,
            in_buffer_len32,
            out_buffer.as_mut_ptr().cast(),
            out_buffer_len32,
            &mut read_bytes,
            timeout,
        )})
    )??;

    Ok(unsafe { slice::assume_init_mut(&mut out_buffer[..usize::from32(read_bytes)]) })
}



#[doc(alias = "CallNamedPipe")]
#[doc(alias = "CallNamedPipeW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-callnamedpipew)\]
/// CallNamedPipeW
///
pub fn call_w<'out_buffer>(
    name:               impl string::InWide,
    in_buffer:          &[u8],
    out_buffer:         &'out_buffer mut [core::mem::MaybeUninit<u8>],
    timeout:            impl Into<firehazard::NMPWAIT>,
) -> Result<&'out_buffer mut [u8], firehazard::Error> {
    let in_buffer_len32     = u32::try_from(in_buffer .len()).map_err(|_| ERROR::INVALID_PARAMETER)?;
    let out_buffer_len32    = u32::try_from(out_buffer.len()).unwrap_or(!0);
    let timeout             = timeout.into().0;

    let mut read_bytes = 0;
    string::convert_to_cstrnn::<{limit::stack::PIPE_NAME}, _, _>(
        name, |name| firehazard::Error::get_last_if(0 == unsafe { winapi::um::namedpipeapi::CallNamedPipeW(
            name.as_cstr(),
            in_buffer.as_ptr() as *const _ as *mut _,
            in_buffer_len32,
            out_buffer.as_mut_ptr().cast(),
            out_buffer_len32,
            &mut read_bytes,
            timeout,
        )})
    )??;

    Ok(unsafe { slice::assume_init_mut(&mut out_buffer[..usize::from32(read_bytes)]) })
}
