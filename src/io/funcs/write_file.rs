#[doc(alias = "WriteFile")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-writefile)\]
/// WriteFile(..., nullptr)
///
///
///
/// ### Safety
/// Use this only on `handle`s that were *not* created using `FILE_FLAG_OVERLAPPED`.
///
/// Per [`rust-lang/rust#81357` File implementation on Windows has unsound methods](https://github.com/rust-lang/rust/issues/81357) under the header **read** (which applies to write too):
/// *   If `handle` was created with `FILE_FLAG_OVERLAPPED`
/// *   If multiple I/O requests are made to `handle`
///
/// This function can return a "successful" 0-byte write in response to a *different* I/O request completing,
/// when it is in fact still asyncronously reading from `buffer` (and accessing a dropped `WriteFile`-internal `OVERLAPPED`?)
///
///
///
/// ### Errors
///
/// | `handle`                  | Error <br> (via GetLastError)                 | Exception <br> [(Strict Handle Checks)](crate::process::mitigation::StrictHandleCheckPolicy)  |
/// | ------------------------- |:---------------------------------------------:|:---------------------------------------------------------------------------------------------:|
/// | null                      | ERROR_INVALID_HANDLE                          | <span style="opacity: 50%">None</span>                                                        |
/// | INVALID_HANDLE_VALUE      | ERROR_INVALID_HANDLE                          | <span style="opacity: 50%">None</span>                                                        |
/// | closed/dangling           | ERROR_INVALID_HANDLE                          | STATUS_INVALID_HANDLE                                                                         |
/// | never valid               | ERROR_INVALID_HANDLE                          | STATUS_INVALID_HANDLE                                                                         |
/// | unwritable                | ERROR_ACCESS_DENIED                           | <span style="opacity: 50%">None</span>                                                        |
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

// TODO: ### Examples
// TODO: WriteFileEx
// TODO: WriteFileGather



tests! {
    use firehazard::*;
    use winapi::shared::winerror::{ERROR_ACCESS_DENIED, ERROR_INVALID_HANDLE};

    #[test] #[strict_handle_check_exception = 0] // no exception
    fn write_file_basic() {
        use std::os::windows::fs::OpenOptionsExt;
        let file = std::fs::OpenOptions::new()
            .write(true).create(true)
            .custom_flags(winapi::um::winbase::FILE_FLAG_DELETE_ON_CLOSE)
            .open("target/write_file.bin").unwrap();
        assert_eq!(Ok(0), unsafe { write_file(&file, &[], None) });
        assert_eq!(Ok(4), unsafe { write_file(&file, &[0u8; 4], None) });
        assert_eq!(Ok(0), unsafe { write_file(&file, &[], None) });
    }

    #[test] #[strict_handle_check_exception = 0] // no exception
    fn write_file_null() {
        let r = unsafe { write_file(&crate::handle::invalid::null(), &[0u8; 1024], None) };
        assert_eq!(r, Err(Error(ERROR_INVALID_HANDLE)));
    }

    #[test] #[strict_handle_check_exception = 0] // no exception
    fn write_file_invalid_handle_value() {
        let r = unsafe { write_file(&crate::handle::invalid::invalid_value(), &[0u8; 1024], None) };
        assert_eq!(r, Err(Error(ERROR_INVALID_HANDLE)));
    }

    #[test] #[isolate] #[strict_handle_check_exception = 0xC0000008] // STATUS_INVALID_HANDLE
    fn write_file_never_valid() {
        let r = unsafe { write_file(&crate::handle::invalid::never_valid(), &[0u8; 1024], None) };
        assert_eq!(r, Err(Error(ERROR_INVALID_HANDLE)));
    }

    #[test] #[isolate] #[strict_handle_check_exception = 0xC0000008] // STATUS_INVALID_HANDLE
    fn write_file_dangling() {
        let r = unsafe { write_file(&crate::handle::invalid::dangling(), &[0u8; 1024], None) };
        assert_eq!(r, Err(Error(ERROR_INVALID_HANDLE)));
    }

    #[test] #[strict_handle_check_exception = 0] // no exception
    fn write_file_not_writable() {
        let unwritable = std::fs::File::open("Readme.md").unwrap();
        let r = unsafe { write_file(&unwritable, &[0u8; 1024], None) };
        drop(unwritable);
        assert_eq!(r, Err(Error(ERROR_ACCESS_DENIED)));
    }

    #[test] #[strict_handle_check_exception = 0] // no exception
    fn write_file_not_writable_0_bytes() {
        let unwritable = std::fs::File::open("Readme.md").unwrap();
        let r = unsafe { write_file(&unwritable, &[], None) };
        drop(unwritable);
        assert_eq!(r, Err(Error(ERROR_ACCESS_DENIED)));
    }
}

#[ignore] // XXX: This test takes a good minute
#[cfg(target_pointer_width = "64")] #[cfg(std)] #[test] fn read_write_file_5_gib() {
    use crate::prelude::*;
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
