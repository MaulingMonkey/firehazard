#[doc(alias = "CallNamedPipe")]
#[doc(alias = "CallNamedPipeA")]
#[doc(alias = "CallNamedPipeW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-callnamedpipew)\]
/// CallNamedPipe
///
pub fn call<'out_buffer>(
    name:               impl TryIntoAsNarrowOrWideCStrNonNull,
    in_buffer:          &[u8],
    out_buffer:         &'out_buffer mut [core::mem::MaybeUninit<u8>],
    timeout:            impl Into<firehazard::NMPWAIT>,
) -> Result<&'out_buffer mut [u8], firehazard::Error> {
    let in_buffer_len32     = u32::try_from(in_buffer.len()).map_err(|_| ERROR_INVALID_PARAMETER)?;
    let in_buffer           = in_buffer.as_ptr() as *const _ as *mut _;
    let out_buffer_len32    = u32::try_from(out_buffer.len()).unwrap_or(!0);
    let out_buffer_ptr      = out_buffer.as_mut_ptr().cast();
    let timeout             = timeout.into().0;

    let mut read_bytes = 0;
    firehazard::Error::get_last_if(0 == match name.try_into()?.as_narrow_or_wide() {
        NarrowOrWideCStrNonNull::Narrow(name) => unsafe { winapi::um::winbase::CallNamedPipeA(
            name.as_cstr(),
            in_buffer,
            in_buffer_len32,
            out_buffer_ptr,
            out_buffer_len32,
            &mut read_bytes,
            timeout,
        )},
        NarrowOrWideCStrNonNull::Wide(name) => unsafe { winapi::um::namedpipeapi::CallNamedPipeW(
            name.as_cstr(),
            in_buffer,
            in_buffer_len32,
            out_buffer_ptr,
            out_buffer_len32,
            &mut read_bytes,
            timeout,
        )},
    })?;

    Ok(unsafe { crate::slice_assume_init_mut(&mut out_buffer[..usize::from32(read_bytes)]) })
}

#[deprecated ="use `pipe::named::call` instead"                     ] #[doc(hidden)] pub use call as call_a;
#[deprecated ="use `pipe::named::call` instead"                     ] #[doc(hidden)] pub use call as call_w;
