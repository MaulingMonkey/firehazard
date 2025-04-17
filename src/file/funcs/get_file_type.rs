#[doc(alias = "GetFileType")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfiletype)\]
/// GetFileType
///
/// ### Examples
///
/// ```
/// # use firehazard::*;
/// # #[cfg(nope)]
/// let remote  = std::fs::File::open(r"\\computer\share\file\on\the\network.txt").unwrap();
/// let file    = std::fs::File::open("Readme.md").unwrap();
/// let console = std::fs::File::open("CON").unwrap();
/// let (pipe_r, pipe_w) = pipe::create(None, 0).unwrap();
///
/// //sert_eq!(Ok(file::TYPE_UNKNOWN), get_file_type(&???)); // TODO: find an example?
/// # #[cfg(nope)]
/// assert_eq!(Ok(file::TYPE_DISK), get_file_type(&remote));
/// assert_eq!(Ok(file::TYPE_DISK), get_file_type(&file));
/// assert_eq!(Ok(file::TYPE_CHAR), get_file_type(&console));
/// assert_eq!(Ok(file::TYPE_PIPE), get_file_type(&pipe_r));
/// assert_eq!(Ok(file::TYPE_PIPE), get_file_type(&pipe_w));
/// ```
///
/// ### Errors
///
/// | `handle`                  | Error <br> (via GetLastError)                 | Exception <br> [(Strict Handle Checks)](crate::process::mitigation::StrictHandleCheckPolicy)  |
/// | ------------------------- |:---------------------------------------------:|:---------------------------------------------------------------------------------------------:|
/// | null                      | ERROR_INVALID_HANDLE                          | <span style="opacity: 50%">None</span>                                                        |
/// | INVALID_HANDLE_VALUE      | ERROR_INVALID_HANDLE                          | <span style="opacity: 50%">None</span>                                                        |
/// | closed/dangling           | ERROR_INVALID_HANDLE                          | STATUS_INVALID_HANDLE                                                                         |
/// | never valid               | ERROR_INVALID_HANDLE                          | STATUS_INVALID_HANDLE                                                                         |
/// | **Valid non-files:**      |                                               |                                                                                               |
/// | access token              | ERROR_INVALID_HANDLE                          | <span style="opacity: 50%">None</span>                                                        |
/// | desktop                   | ERROR_INVALID_HANDLE                          | <span style="opacity: 50%">None</span>                                                        |
/// | job                       | ERROR_INVALID_HANDLE                          | <span style="opacity: 50%">None</span>                                                        |
/// | process                   | ERROR_INVALID_HANDLE                          | <span style="opacity: 50%">None</span>                                                        |
/// | thread                    | ERROR_INVALID_HANDLE                          | <span style="opacity: 50%">None</span>                                                        |
/// | winsta                    | ERROR_INVALID_HANDLE                          | <span style="opacity: 50%">None</span>                                                        |
///
pub fn get_file_type(handle: &impl firehazard::AsLocalHandle) -> Result<u32, firehazard::Error> {
    unsafe { winapi::um::errhandlingapi::SetLastError(0) }; // docs imply this isn't necessary, but I'm not sure I trust the docs, and I lack a test case for FILE_TYPE_UNKNOWN
    let ty = unsafe { winapi::um::fileapi::GetFileType(handle.as_handle()) };
    match firehazard::Error::get_last_if(ty == 0) {
        Ok(())                  => Ok(ty),
        Err(err) if err == 0    => Ok(ty), // NO_ERROR, ty == FILE_TYPE_UNKNOWN legitimately
        Err(err)                => Err(err),
    }
}



tests! {
    use abistr::*;
    use firehazard::*;
    use winapi::shared::winerror::{ERROR_ACCESS_DENIED, ERROR_INVALID_HANDLE};

    #[test] #[strict_handle_check_exception = 0] // no exception
    fn get_file_type_null() {
        let r = get_file_type(&crate::handle::invalid::null());
        assert_eq!(r, Err(Error(ERROR_INVALID_HANDLE)));
    }

    #[test] #[strict_handle_check_exception = 0] // no exception
    fn get_file_type_invalid_handle_value() {
        let r = get_file_type(&crate::handle::invalid::invalid_value());
        assert_eq!(r, Err(Error(ERROR_INVALID_HANDLE)));
    }

    #[test] #[isolate] #[strict_handle_check_exception = 0xC0000008] // STATUS_INVALID_HANDLE
    fn get_file_type_never_valid() {
        let r = get_file_type(&crate::handle::invalid::never_valid());
        assert_eq!(r, Err(Error(ERROR_INVALID_HANDLE)));
    }

    #[test] #[isolate] #[strict_handle_check_exception = 0xC0000008] // STATUS_INVALID_HANDLE
    fn get_file_type_dangling() {
        let r = get_file_type(&crate::handle::invalid::dangling());
        assert_eq!(r, Err(Error(ERROR_INVALID_HANDLE)));
    }

    #[test] #[strict_handle_check_exception = 0]
    fn get_file_type_access_token_psuedo() {
        let r = get_file_type(&get_current_process_token());
        assert_eq!(r, Err(Error(ERROR_INVALID_HANDLE)));
    }

    #[test] #[strict_handle_check_exception = 0]
    fn get_file_type_access_token_process() {
        let token = open_process_token(get_current_process(), token::ALL_ACCESS).unwrap();
        let r = get_file_type(&token);
        assert_eq!(r, Err(Error(ERROR_INVALID_HANDLE)));
    }

    #[test] #[strict_handle_check_exception = 0]
    fn get_file_type_access_token_impersonation() {
        let token = open_process_token(get_current_process(), token::ALL_ACCESS).unwrap();
        let token = duplicate_token_ex(&token, token::ALL_ACCESS, None, security::Delegation, token::Impersonation).unwrap();
        let r = get_file_type(&token);
        assert_eq!(r, Err(Error(ERROR_INVALID_HANDLE)));
    }

    #[test] #[strict_handle_check_exception = 0]
    fn get_file_type_desktop() {
        let desktop = open_input_desktop(None, false, access::GENERIC_ALL).unwrap();
        let r = get_file_type(&handle::Borrowed::from(&desktop));
        assert_eq!(r, Err(Error(ERROR_INVALID_HANDLE)));
    }

    #[test] #[strict_handle_check_exception = 0]
    fn get_file_type_job() {
        let job = create_job_object_w(None, ()).unwrap();
        let r = get_file_type(&job);
        assert_eq!(r, Err(Error(ERROR_INVALID_HANDLE)));
    }

    #[test] #[strict_handle_check_exception = 0]
    fn get_file_type_process_psuedo() {
        let r = get_file_type(&get_current_process());
        assert_eq!(r, Err(Error(ERROR_INVALID_HANDLE)));
    }

    #[test] #[strict_handle_check_exception = 0]
    fn get_file_type_process_real() {
        let p = std::process::Command::new(std::env::current_exe().unwrap())
            .arg("--help")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn().unwrap();
        let r = get_file_type(&p);
        assert_eq!(r, Err(Error(ERROR_INVALID_HANDLE)));
    }

    #[test] #[strict_handle_check_exception = 0]
    fn get_file_type_thread_psuedo() {
        let r = get_file_type(&get_current_thread());
        assert_eq!(r, Err(Error(ERROR_INVALID_HANDLE)));
    }

    #[test] #[strict_handle_check_exception = 0]
    fn get_file_type_thread_real() {
        let thread = std::thread::spawn(||{});
        let r = get_file_type(&thread);
        assert_eq!(r, Err(Error(ERROR_INVALID_HANDLE)));
    }

    #[test] #[strict_handle_check_exception = 0]
    fn get_file_type_winsta() {
        let winsta0 = open_window_station_w(cstr16!("WinSta0"), false, winsta::ALL_ACCESS).unwrap();
        let r = get_file_type(&handle::Borrowed::from(&winsta0));
        assert_eq!(r, Err(Error(ERROR_INVALID_HANDLE)));
    }
}
