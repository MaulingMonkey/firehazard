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
pub(crate) unsafe fn read_file(handle: winapi::um::winnt::HANDLE, buffer: &mut [u8], overlapped: Option<core::convert::Infallible>) -> crate::io::Result<usize> {
    use crate::From32;
    let mut read = 0;
    firehazard::Error::get_last_if(0 == unsafe { winapi::um::fileapi::ReadFile(
        handle,
        buffer.as_mut_ptr().cast(),
        buffer.len().try_into().unwrap_or(!0u32),
        &mut read,
        crate::none2null(overlapped),
    )})?;
    Ok(usize::from32(read))
}

// TODO: examples
// TODO: stronger type for handle

#[test] fn read_file_null() {
    use core::ptr::null_mut;
    let r = unsafe { read_file(null_mut(), &mut [0u8; 1024], None) };
    assert_eq!(r.map_err(|e| e.raw_os_error()), Err(Some(winapi::shared::winerror::ERROR_INVALID_HANDLE as _)));
}

#[test] fn read_file_invalid_handle_value() {
    let r = unsafe { read_file(winapi::um::handleapi::INVALID_HANDLE_VALUE, &mut [0u8; 1024], None) };
    assert_eq!(r.map_err(|e| e.raw_os_error()), Err(Some(winapi::shared::winerror::ERROR_INVALID_HANDLE as _)));
}

#[test] fn read_file_bad_handle_value() {
    let r = unsafe { read_file(0x12345678_usize as *mut _, &mut [0u8; 1024], None) };
    assert_eq!(r.map_err(|e| e.raw_os_error()), Err(Some(winapi::shared::winerror::ERROR_INVALID_HANDLE as _)));
}

#[cfg(std)] #[test] fn read_file_dangling() {
    use std::os::windows::io::AsRawHandle;
    let file = std::fs::File::open("Readme.md").unwrap();
    let dangling = file.as_raw_handle().cast();
    drop(file);
    let r = unsafe { read_file(dangling, &mut [0u8; 1024], None) };
    assert_eq!(r.map_err(|e| e.raw_os_error()), Err(Some(winapi::shared::winerror::ERROR_INVALID_HANDLE as _)));
}

#[cfg(std)] #[test] fn read_file_not_readable() {
    use std::os::windows::io::AsRawHandle;
    let file = std::fs::OpenOptions::new().write(true).create(true).open("target/tmp/read_file_not_readable.bin").unwrap();
    let unreadable = file.as_raw_handle().cast();
    let r = unsafe { read_file(unreadable, &mut [0u8; 1024], None) };
    drop(file);
    assert_eq!(r.map_err(|e| e.raw_os_error()), Err(Some(winapi::shared::winerror::ERROR_ACCESS_DENIED as _)));
}

// TODO: ReadFileEx
// TODO: ReadFileScatter
