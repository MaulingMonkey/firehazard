#[doc(alias = "ReadFile")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-readfile)\]
/// ReadFile(..., nullptr)
///
/// ### Errors
///
/// | `handle`                  | Error <br> (via GetLastError)                 | Exception <br> [(Strict Handle Checks)](crate::process::mitigation::StrictHandleCheckPolicy)  |
/// | ------------------------- |:---------------------------------------------:|:---------------------------------------------------------------------------------------------:|
/// | null                      | ERROR_INVALID_HANDLE                          | <span style="opacity: 50%">None</span>                                                        |
/// | INVALID_HANDLE_VALUE      | ERROR_INVALID_HANDLE                          | <span style="opacity: 50%">None</span>                                                        |
/// | closed/dangling           | ERROR_INVALID_HANDLE                          | STATUS_INVALID_HANDLE                                                                         |
/// | never valid               | ERROR_INVALID_HANDLE                          | STATUS_INVALID_HANDLE                                                                         |
/// | unreadable                | ERROR_ACCESS_DENIED                           | <span style="opacity: 50%">None</span>                                                        |
///
pub(crate) unsafe fn read_file(handle: &impl firehazard::AsLocalHandle, buffer: &mut [u8], overlapped: Option<core::convert::Infallible>) -> Result<u32, firehazard::Error> {
    let mut read = 0;
    firehazard::Error::get_last_if(0 == unsafe { winapi::um::fileapi::ReadFile(
        handle.as_handle().cast(),
        buffer.as_mut_ptr().cast(),
        buffer.len().try_into().unwrap_or(!0u32),
        &mut read,
        crate::none2null(overlapped),
    )})?;
    Ok(read)
}

// TODO: ### Examples
// TODO: ReadFileEx
// TODO: ReadFileScatter



tests! {
    use firehazard::*;
    use winapi::shared::winerror::{ERROR_ACCESS_DENIED, ERROR_INVALID_HANDLE};

    #[test] #[strict_handle_check_exception = 0] // no exception
    fn read_file_null() {
        let r = unsafe { read_file(&crate::handle::invalid::null(), &mut [0u8; 1024], None) };
        assert_eq!(r, Err(Error(ERROR_INVALID_HANDLE)));
    }

    #[test] #[strict_handle_check_exception = 0] // no exception
    fn read_file_invalid_handle_value() {
        let r = unsafe { read_file(&crate::handle::invalid::invalid_value(), &mut [0u8; 1024], None) };
        assert_eq!(r, Err(Error(ERROR_INVALID_HANDLE)));
    }

    #[test] #[isolate] #[strict_handle_check_exception = 0xC0000008] // STATUS_INVALID_HANDLE
    fn read_file_never_valid() {
        let r = unsafe { read_file(&crate::handle::invalid::never_valid(), &mut [0u8; 1024], None) };
        assert_eq!(r, Err(Error(ERROR_INVALID_HANDLE)));
    }

    #[test] #[isolate] #[strict_handle_check_exception = 0xC0000008] // STATUS_INVALID_HANDLE
    fn read_file_dangling() {
        let r = unsafe { read_file(&crate::handle::invalid::dangling(), &mut [0u8; 1024], None) };
        assert_eq!(r, Err(Error(ERROR_INVALID_HANDLE)));
    }

    #[test] #[strict_handle_check_exception = 0] // no exception
    fn read_file_not_readable() {
        use std::os::windows::fs::OpenOptionsExt;
        let unreadable = std::fs::OpenOptions::new()
            .write(true).create(true)
            .custom_flags(winapi::um::winbase::FILE_FLAG_DELETE_ON_CLOSE)
            .open("target/read_file_not_readable.bin").unwrap();
        let r = unsafe { read_file(&unreadable, &mut [0u8; 1024], None) };
        drop(unreadable);
        assert_eq!(r, Err(Error(ERROR_ACCESS_DENIED)));
    }
}
