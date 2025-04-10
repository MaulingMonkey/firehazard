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
pub(crate) unsafe fn write_file(handle: &impl firehazard::AsLocalHandle, buffer: &[u8], overlapped: Option<core::convert::Infallible>) -> Result<u32, firehazard::Error> {
    let mut written = 0;
    firehazard::Error::get_last_if(0 == unsafe { winapi::um::fileapi::WriteFile(
        handle.as_handle().cast(),
        buffer.as_ptr().cast(),
        buffer.len().try_into().unwrap_or(!0u32),
        &mut written,
        crate::none2null(overlapped),
    )})?;
    Ok(written)
}

// TODO: examples

#[test] fn write_file_null() {
    let r = unsafe { write_file(&crate::handle::invalid::null(), &[0u8; 1024], None) };
    assert_eq!(r, Err(firehazard::Error(winapi::shared::winerror::ERROR_INVALID_HANDLE)));
}

#[test] fn write_file_invalid_handle_value() {
    let r = unsafe { write_file(&crate::handle::invalid::invalid_value(), &[0u8; 1024], None) };
    assert_eq!(r, Err(firehazard::Error(winapi::shared::winerror::ERROR_INVALID_HANDLE)));
}

#[test] fn write_file_bad_handle_value() {
    let r = unsafe { write_file(&crate::handle::invalid::never_valid(), &[0u8; 1024], None) };
    assert_eq!(r, Err(firehazard::Error(winapi::shared::winerror::ERROR_INVALID_HANDLE)));
}

#[cfg(std)] #[test] fn write_file_dangling() {
    let r = unsafe { write_file(&crate::handle::invalid::dangling(), &[0u8; 1024], None) };
    assert_eq!(r, Err(firehazard::Error(winapi::shared::winerror::ERROR_INVALID_HANDLE)));
}

#[cfg(std)] #[test] fn write_file_not_writeable() {
    let unwriteable = std::fs::File::open("Readme.md").unwrap();
    let r = unsafe { write_file(&unwriteable, &[0u8; 1024], None) };
    drop(unwriteable);
    assert_eq!(r, Err(firehazard::Error(winapi::shared::winerror::ERROR_ACCESS_DENIED)));
}

#[ignore] // XXX: This test takes a good minute
#[cfg(target_pointer_width = "64")] #[cfg(std)] #[test] fn read_write_file_5_gib() {
    use crate::*;
    use std::os::windows::fs::OpenOptionsExt;

    let file = std::fs::OpenOptions::new()
        .create(true).read(true).write(true).truncate(true)
        .custom_flags(winapi::um::winbase::FILE_FLAG_DELETE_ON_CLOSE)
        .open("target/read_write_file_5_gib.bin").unwrap();

    let mut huge = std::vec::Vec::new();
    huge.resize(5_usize << 30, 0u8); // 5 GiB

    assert_eq!(Ok(!0), unsafe { write_file(&file, &huge, None) });
    assert_eq!(Ok(0),  unsafe { set_file_pointer_ex(&file, io::SeekFrom::Start(0)) });
    assert_eq!(Ok(!0), unsafe { read_file(&file, &mut huge, None) });
}

// TODO: WriteFileEx
// TODO: WriteFileGather
