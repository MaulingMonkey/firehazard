#[doc(alias = "FlushFileBuffers")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-flushfilebuffers)\]
/// FlushFileBuffers
///
///
///
/// ### Alternatives
/// *   <code>std::[fs](std::fs)::[File](std::fs::File)::{[sync_all](std::fs::File::sync_all), [sync_data](std::fs::File::sync_data)}</code>
///     &mdash; cross platform, requires [`std`], implemented in terms of `FlushFileBuffers` (at least at one point.)
///
///
///
/// ### Examples
///
/// ```
/// # use firehazard::*;
/// # use winapi::shared::winerror::*;
/// # use std::os::windows::fs::OpenOptionsExt;
/// #
/// let writable = std::fs::OpenOptions::new()
///     .create(true).write(true).truncate(true)
///     .custom_flags(winapi::um::winbase::FILE_FLAG_DELETE_ON_CLOSE)
///     .open("target/flush_file_buffers_example.bin").unwrap();
/// flush_file_buffers(&writable).unwrap();
///
/// // Fails on read-only handles:
/// let readme = std::fs::File::open("Readme.md").unwrap();
/// assert_eq!(ERROR_ACCESS_DENIED, flush_file_buffers(&readme).unwrap_err());
/// ```
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
/// | **Valid files:**          |                                               |                                                                                               |
/// | read-only file            | ERROR_ACCESS_DENIED                           | <span style="opacity: 50%">None</span>                                                        |
/// | writeable files           | <span style="opacity: 50%">None</span>        | <span style="opacity: 50%">None</span>                                                        |
/// | **Valid non-files:**      |                                               |                                                                                               |
/// | access token              | ERROR_INVALID_HANDLE                          | <span style="opacity: 50%">None</span>                                                        |
/// | desktop                   | ERROR_INVALID_HANDLE                          | <span style="opacity: 50%">None</span>                                                        |
/// | job                       | ERROR_INVALID_HANDLE                          | <span style="opacity: 50%">None</span>                                                        |
/// | process                   | ERROR_INVALID_HANDLE                          | <span style="opacity: 50%">None</span>                                                        |
/// | thread                    | ERROR_INVALID_HANDLE                          | <span style="opacity: 50%">None</span>                                                        |
/// | winsta                    | ERROR_INVALID_HANDLE                          | <span style="opacity: 50%">None</span>                                                        |
///
pub fn flush_file_buffers<'a>(file: impl Into<handle::Pseudo<'a>>) -> firehazard::Result<()> {
    firehazard::Error::get_last_if(0 == unsafe { winapi::um::fileapi::FlushFileBuffers(
        file.into().as_handle().cast(),
    )})
}



tests! {
    use abistr::*;
    use firehazard::*;
    use winapi::shared::winerror::{ERROR_ACCESS_DENIED, ERROR_INVALID_HANDLE};

    #[test] #[strict_handle_check_exception = 0] // no exception
    fn flush_file_buffers_null() {
        let r = firehazard::Error::get_last_if(0 == unsafe { winapi::um::fileapi::FlushFileBuffers(core::ptr::null_mut()) });
        assert_eq!(r, Err(Error(ERROR_INVALID_HANDLE)));
    }

    #[test] #[strict_handle_check_exception = 0] // no exception
    fn flush_file_buffers_invalid_handle_value() {
        let r = flush_file_buffers(crate::handle::invalid::invalid_value());
        assert_eq!(r, Err(Error(ERROR_INVALID_HANDLE)));
    }

    #[test] #[isolate] #[strict_handle_check_exception = 0xC0000008] // STATUS_INVALID_HANDLE
    fn flush_file_buffers_never_valid() {
        let r = flush_file_buffers(crate::handle::invalid::never_valid());
        assert_eq!(r, Err(Error(ERROR_INVALID_HANDLE)));
    }

    #[test] #[isolate] #[strict_handle_check_exception = 0xC0000008] // STATUS_INVALID_HANDLE
    fn flush_file_buffers_dangling() {
        let r = flush_file_buffers(crate::handle::invalid::dangling());
        assert_eq!(r, Err(Error(ERROR_INVALID_HANDLE)));
    }

    #[test] #[strict_handle_check_exception = 0]
    fn flush_file_buffers_access_token_pseudo() {
        let r = firehazard::Error::get_last_if(0 == unsafe { winapi::um::fileapi::FlushFileBuffers(get_current_process_token().into_handle().cast()) });
        assert_eq!(r, Err(Error(ERROR_INVALID_HANDLE)));
    }

    #[test] #[strict_handle_check_exception = 0]
    fn flush_file_buffers_access_token_process() {
        let token = open_process_token(get_current_process(), token::ALL_ACCESS).unwrap();
        let r = flush_file_buffers(&token);
        assert_eq!(r, Err(Error(ERROR_INVALID_HANDLE)));
    }

    #[test] #[strict_handle_check_exception = 0]
    fn flush_file_buffers_access_token_impersonation() {
        let token = open_process_token(get_current_process(), token::ALL_ACCESS).unwrap();
        let token = duplicate_token_ex(&token, token::ALL_ACCESS, None, security::Delegation, token::Impersonation).unwrap();
        let r = flush_file_buffers(&token);
        assert_eq!(r, Err(Error(ERROR_INVALID_HANDLE)));
    }

    #[test] #[strict_handle_check_exception = 0]
    fn flush_file_buffers_desktop() {
        let desktop = open_input_desktop(None, false, access::GENERIC_ALL).unwrap();
        let r = flush_file_buffers(&desktop);
        assert_eq!(r, Err(Error(ERROR_INVALID_HANDLE)));
    }

    #[test] #[strict_handle_check_exception = 0]
    fn flush_file_buffers_job() {
        let job = create_job_object_w(None, ()).unwrap();
        let r = flush_file_buffers(&job);
        assert_eq!(r, Err(Error(ERROR_INVALID_HANDLE)));
    }

    #[test] #[strict_handle_check_exception = 0]
    fn flush_file_buffers_process_pseudo() {
        let r = flush_file_buffers(get_current_process());
        assert_eq!(r, Err(Error(ERROR_INVALID_HANDLE)));
    }

    #[test] #[strict_handle_check_exception = 0]
    fn flush_file_buffers_process_real() {
        let p = std::process::Command::new(std::env::current_exe().unwrap())
            .arg("--help")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn().unwrap();
        let r = flush_file_buffers(&process::OwnedHandle::from(p));
        assert_eq!(r, Err(Error(ERROR_INVALID_HANDLE)));
    }

    #[test] #[strict_handle_check_exception = 0]
    fn flush_file_buffers_thread_pseudo() {
        let r = flush_file_buffers(get_current_thread());
        assert_eq!(r, Err(Error(ERROR_INVALID_HANDLE)));
    }

    #[test] #[strict_handle_check_exception = 0]
    fn flush_file_buffers_thread_real() {
        let thread = std::thread::spawn(||{});
        let r = flush_file_buffers(&thread::OwnedHandle::from(thread));
        assert_eq!(r, Err(Error(ERROR_INVALID_HANDLE)));
    }

    #[test] #[strict_handle_check_exception = 0]
    fn flush_file_buffers_winsta() {
        let winsta0 = open_window_station_w(cstr16!("WinSta0"), false, winsta::ALL_ACCESS).unwrap();
        let r = flush_file_buffers(&winsta0);
        assert_eq!(r, Err(Error(ERROR_INVALID_HANDLE)));
    }
}
