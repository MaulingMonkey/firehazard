#[doc(alias = "TransactNamedPipe")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-transactnamedpipe)\]
/// TransactNamedPipe(..., nullptr)
///
pub fn transact<'buffer>(
    handle:         &impl firehazard::AsLocalHandle, // XXX
    in_buffer:      &[u8],
    out_buffer:     &'buffer mut [core::mem::MaybeUninit<u8>],
    overlapped:     Option<core::convert::Infallible>,
) -> Result<&'buffer mut [u8], firehazard::Error> {
    use crate::From32;
    use winapi::shared::winerror::ERROR_BAD_LENGTH;

    let in_buffer_len32     = u32::try_from(in_buffer.len()).map_err(|_| ERROR_BAD_LENGTH)?;
    let out_buffer_len32    = u32::try_from(out_buffer.len()).unwrap_or(!0);
    let out_buffer = &mut out_buffer[..usize::from32(out_buffer_len32)];
    let mut bytes_read = 0;

    firehazard::Error::get_last_if(0 == unsafe { winapi::um::namedpipeapi::TransactNamedPipe(
        handle              .as_handle(),
        in_buffer           .as_ptr() as *const _ as *mut _, // SAFETY: surely TransactNamedPipe doesn't mutate in_buffer...?
        in_buffer_len32,
        out_buffer          .as_mut_ptr().cast(),
        out_buffer_len32,
        &mut bytes_read,
        crate::none2null(overlapped),
    )})?;

    Ok(unsafe { crate::slice_assume_init_mut(&mut out_buffer[..usize::from32(bytes_read)]) })
}



#[doc(alias = "TransactNamedPipe")]
#[cfg(doc)]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-transactnamedpipe)\]
/// <strike>TransactNamedPipe(..., &overlapped)</strike>
///
pub unsafe fn transact_overlapped<'buffer>(
    handle:         &impl firehazard::AsLocalHandle, // XXX
    in_buffer:      &[u8],
    out_buffer:     &'buffer mut [core::mem::MaybeUninit<u8>],
    overlapped:     &'_ mut winapi::um::minwinbase::OVERLAPPED,
) -> Result<&'buffer mut [u8], firehazard::Error> {
    use winapi::shared::winerror::*;
    let _ = (handle, in_buffer, out_buffer, overlapped);
    unimplemented!();
    Err(ERROR_CALL_NOT_IMPLEMENTED)
}
