#[doc(alias = "CreatePipe")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createpipe)\]
/// CreatePipe
///
/// ### Alternatives
/// *   [`std::io::pipe`] &mdash; cross platform
///
/// ### Example
/// ```
/// # use firehazard::*;
/// let (read, write) = pipe::create(None, 0).unwrap();
/// ```
///
pub fn create(pipe_attributes: Option<&security::Attributes>, size: u32) -> firehazard::Result<(pipe::sync::OwnedReader, pipe::sync::OwnedWriter)> {
    let mut read = null_mut();
    let mut write = null_mut();
    firehazard::Error::get_last_if(0 == unsafe { winapi::um::namedpipeapi::CreatePipe(
        &mut read,
        &mut write,
        pipe_attributes.map_or(null_mut(), |a| a as *const _ as *mut _), size)
    })?;
    let read  = NonNull::new(read ).map(|nn| pipe::sync::OwnedReader(nn)).ok_or(ERROR_INVALID_HANDLE);
    let write = NonNull::new(write).map(|nn| pipe::sync::OwnedWriter(nn)).ok_or(ERROR_INVALID_HANDLE);
    Ok((read?, write?))
}
