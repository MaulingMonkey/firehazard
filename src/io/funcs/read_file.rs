/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-readfile)\]
/// ReadFile(..., nullptr)
///
/// ### Errors
///
/// | `handle`                  | Error |
/// | --------------------------| ------|
/// | null                      | [Err]\(≈ ERROR_INVALID_HANDLE\)
/// | INVALID_HANDLE_VALUE      | [Err]\(≈ ERROR_INVALID_HANDLE\)
/// | closed/dangling           | [Err]\(≈ ERROR_INVALID_HANDLE\) or, if [strict handle checks](crate::process::mitigation::StrictHandleCheckPolicy) are enabled, an exception.
/// | never valid               | [Err]\(≈ ERROR_INVALID_HANDLE\) or, if [strict handle checks](crate::process::mitigation::StrictHandleCheckPolicy) are enabled, an exception.
/// | unreadable                | [Err]\(≈ ERROR_ACCESS_DENIED\)
///
pub(crate) unsafe fn read_file(handle: &impl firehazard::AsLocalHandle, buffer: &mut [u8], overlapped: Option<core::convert::Infallible>) -> Result<usize, firehazard::Error> {
    use crate::From32;
    let mut read = 0;
    firehazard::Error::get_last_if(0 == unsafe { winapi::um::fileapi::ReadFile(
        handle.as_handle().cast(),
        buffer.as_mut_ptr().cast(),
        buffer.len().try_into().unwrap_or(!0u32),
        &mut read,
        crate::none2null(overlapped),
    )})?;
    Ok(usize::from32(read))
}

// TODO: examples

#[test] fn read_file_null() {
    let r = unsafe { read_file(&crate::handle::invalid::null(), &mut [0u8; 1024], None) };
    assert_eq!(r, Err(firehazard::Error(winapi::shared::winerror::ERROR_INVALID_HANDLE)));
}

#[test] fn read_file_invalid_handle_value() {
    let r = unsafe { read_file(&crate::handle::invalid::invalid_value(), &mut [0u8; 1024], None) };
    assert_eq!(r, Err(firehazard::Error(winapi::shared::winerror::ERROR_INVALID_HANDLE)));
}

#[test] fn read_file_bad_handle_value() {
    let r = unsafe { read_file(&crate::handle::invalid::never_valid(), &mut [0u8; 1024], None) };
    assert_eq!(r, Err(firehazard::Error(winapi::shared::winerror::ERROR_INVALID_HANDLE)));
}

#[cfg(std)] #[test] fn read_file_dangling() {
    let r = unsafe { read_file(&crate::handle::invalid::dangling(), &mut [0u8; 1024], None) };
    assert_eq!(r, Err(firehazard::Error(winapi::shared::winerror::ERROR_INVALID_HANDLE)));
}

#[cfg(std)] #[test] fn read_file_not_readable() {
    let unreadable = std::fs::OpenOptions::new().write(true).create(true).open("target/tmp/read_file_not_readable.bin").unwrap();
    let r = unsafe { read_file(&unreadable, &mut [0u8; 1024], None) };
    drop(unreadable);
    assert_eq!(r, Err(firehazard::Error(winapi::shared::winerror::ERROR_ACCESS_DENIED)));
}

// TODO: ReadFileEx
// TODO: ReadFileScatter
