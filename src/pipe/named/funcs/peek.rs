#[doc(alias = "PeekNamedPipe")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-peeknamedpipe)\]
/// PeekNamedPipe
///
pub fn peek<'buffer, 'a, 'b>(
    handle:                     &impl AsLocalHandle, // XXX
    buffer:                     impl Into<Option<&'buffer mut [core::mem::MaybeUninit<u8>]>>,
    total_bytes_avail:          impl Into<Option<&'a mut u32>>,
    bytes_left_this_message:    impl Into<Option<&'b mut u32>>,
) -> firehazard::Result<Option<&'buffer mut [u8]>> {
    let mut buffer = buffer.into();
    let buffer_len32 = u32::try_from(buffer.as_ref().map_or(0, |b| (*b).as_ref().len())).unwrap_or(!0);
    let mut bytes_read = 0;

    firehazard::Error::get_last_if(0 == unsafe { winapi::um::namedpipeapi::PeekNamedPipe(
        handle                  .as_handle(),
        buffer                  .as_mut().map_or(null_mut(), |b| b.as_mut_ptr().cast()),
        buffer_len32,
        &mut bytes_read,
        total_bytes_avail       .into().map_or(null_mut(), |v| v),
        bytes_left_this_message .into().map_or(null_mut(), |v| v),
    )})?;

    Ok(buffer.map(|b| unsafe { slice::assume_init_mut(&mut (*b)[..usize::from32(bytes_read)]) }))
}
