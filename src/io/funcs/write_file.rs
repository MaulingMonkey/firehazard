/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-writefile)\]
/// WriteFile(..., nullptr)
///
/// ### Errors
///
/// | `handle`                  | Error |
/// | --------------------------| ------|
/// | null                      | [Err]\(≈ ERROR_INVALID_HANDLE\)
/// | INVALID_HANDLE_VALUE      | [Err]\(≈ ERROR_INVALID_HANDLE\)
/// | closed/dangling           | [Err]\(≈ ERROR_INVALID_HANDLE\) or, if [strict handle checks](crate::process::mitigation::StrictHandleCheckPolicy) are enabled, an exception.
/// | never valid               | [Err]\(≈ ERROR_INVALID_HANDLE\) or, if [strict handle checks](crate::process::mitigation::StrictHandleCheckPolicy) are enabled, an exception.
/// | unwriteable               | [Err]\(≈ ERROR_ACCESS_DENIED\)
///
pub(crate) unsafe fn write_file(handle: winapi::um::winnt::HANDLE, buffer: &[u8], overlapped: Option<core::convert::Infallible>) -> crate::io::Result<usize> {
    use crate::From32;
    let mut written = 0;
    firehazard::Error::get_last_if(0 == unsafe { winapi::um::fileapi::WriteFile(
        handle,
        buffer.as_ptr().cast(),
        buffer.len().try_into().unwrap_or(!0u32),
        &mut written,
        crate::none2null(overlapped),
    )})?;
    Ok(usize::from32(written))
}

// TODO: examples
// TODO: stronger type for handle

#[test] fn write_file_null() {
    use core::ptr::null_mut;
    let r = unsafe { write_file(null_mut(), &[0u8; 1024], None) };
    assert_eq!(r.map_err(|e| e.raw_os_error()), Err(Some(winapi::shared::winerror::ERROR_INVALID_HANDLE as _)));
}

#[test] fn write_file_invalid_handle_value() {
    let r = unsafe { write_file(winapi::um::handleapi::INVALID_HANDLE_VALUE, &[0u8; 1024], None) };
    assert_eq!(r.map_err(|e| e.raw_os_error()), Err(Some(winapi::shared::winerror::ERROR_INVALID_HANDLE as _)));
}

#[test] fn write_file_bad_handle_value() {
    let r = unsafe { write_file(0x12345678_usize as *mut _, &[0u8; 1024], None) };
    assert_eq!(r.map_err(|e| e.raw_os_error()), Err(Some(winapi::shared::winerror::ERROR_INVALID_HANDLE as _)));
}

#[cfg(std)] #[test] fn write_file_dangling() {
    use std::os::windows::io::AsRawHandle;
    let file = std::fs::File::open("Readme.md").unwrap();
    let dangling = file.as_raw_handle().cast();
    drop(file);
    let r = unsafe { write_file(dangling, &[0u8; 1024], None) };
    assert_eq!(r.map_err(|e| e.raw_os_error()), Err(Some(winapi::shared::winerror::ERROR_INVALID_HANDLE as _)));
}

#[cfg(std)] #[test] fn write_file_not_writeable() {
    use std::os::windows::io::AsRawHandle;
    let file = std::fs::File::open("Readme.md").unwrap();
    let unwriteable = file.as_raw_handle().cast();
    let r = unsafe { write_file(unwriteable, &[0u8; 1024], None) };
    drop(file);
    assert_eq!(r.map_err(|e| e.raw_os_error()), Err(Some(winapi::shared::winerror::ERROR_ACCESS_DENIED as _)));
}

// TODO: WriteFileEx
// TODO: WriteFileGather
