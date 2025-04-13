#[cfg(std)] // required for OsStr / encode_wide
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-callnamedpipew)\]
/// CallNamedPipeW
///
pub fn call_named_pipe<'out_buffer>(
    name:               impl AsRef<std::ffi::OsStr>,
    in_buffer:          &[u8],
    out_buffer:         &'out_buffer mut [core::mem::MaybeUninit<u8>],
    timeout:            impl Into<firehazard::NMPWAIT>,
) -> Result<&'out_buffer mut [u8], firehazard::Error> {
    use winapi::shared::winerror::ERROR_ILLEGAL_CHARACTER;
    use std::os::windows::ffi::OsStrExt;

    let name = name.as_ref();
    let name = name.encode_wide().chain(Some(0)).collect::<std::vec::Vec<_>>();
    let name = abistr::CStrNonNull::from_units_with_nul(&name).map_err(|_| ERROR_ILLEGAL_CHARACTER)?;
    call_named_pipe_w(name, in_buffer, out_buffer, timeout)
}



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-callnamedpipea)\]
/// CallNamedPipeA
///
pub fn call_named_pipe_a<'out_buffer>(
    name:               impl abistr::TryIntoAsCStr,
    in_buffer:          &[u8],
    out_buffer:         &'out_buffer mut [core::mem::MaybeUninit<u8>],
    timeout:            impl Into<firehazard::NMPWAIT>,
) -> Result<&'out_buffer mut [u8], firehazard::Error> {
    use crate::From32;
    use abistr::AsCStr;
    use winapi::shared::winerror::ERROR_INVALID_PARAMETER;

    let in_buffer_len32 = u32::try_from(in_buffer.len()).map_err(|_| ERROR_INVALID_PARAMETER)?;
    let out_buffer_len32 = u32::try_from(out_buffer.len()).unwrap_or(!0);

    let mut read_bytes = 0;
    firehazard::Error::get_last_if(0 == unsafe { winapi::um::winbase::CallNamedPipeA(
        name.try_into()?.as_cstr(),
        in_buffer.as_ptr() as *const _ as *mut _,
        in_buffer_len32,
        out_buffer.as_mut_ptr().cast(),
        out_buffer_len32,
        &mut read_bytes,
        timeout.into().0,
    )})?;

    Ok(unsafe { crate::slice_assume_init_mut(&mut out_buffer[..usize::from32(read_bytes)]) })
}



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-callnamedpipew)\]
/// CallNamedPipeW
///
pub fn call_named_pipe_w<'out_buffer>(
    name:               impl abistr::TryIntoAsCStr<u16>,
    in_buffer:          &[u8],
    out_buffer:         &'out_buffer mut [core::mem::MaybeUninit<u8>],
    timeout:            impl Into<firehazard::NMPWAIT>,
) -> Result<&'out_buffer mut [u8], firehazard::Error> {
    use crate::From32;
    use abistr::AsCStr;
    use winapi::shared::winerror::ERROR_INVALID_PARAMETER;

    let in_buffer_len32 = u32::try_from(in_buffer.len()).map_err(|_| ERROR_INVALID_PARAMETER)?;
    let out_buffer_len32 = u32::try_from(out_buffer.len()).unwrap_or(!0);

    let mut read_bytes = 0;
    firehazard::Error::get_last_if(0 == unsafe { winapi::um::namedpipeapi::CallNamedPipeW(
        name.try_into()?.as_cstr(),
        in_buffer.as_ptr() as *const _ as *mut _,
        in_buffer_len32,
        out_buffer.as_mut_ptr().cast(),
        out_buffer_len32,
        &mut read_bytes,
        timeout.into().0,
    )})?;

    Ok(unsafe { crate::slice_assume_init_mut(&mut out_buffer[..usize::from32(read_bytes)]) })
}
