#[doc(alias = "GetThreadId")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadid)\]
/// GetThreadId
///
/// Get a thread's ID.
/// May fail if `thread` lacks the [`THREAD_QUERY_LIMITED_INFORMATION` access right](https://learn.microsoft.com/en-us/windows/win32/procthread/thread-security-and-access-rights).
/// For a more durable reference to a thread, consider using a handle instead.
///
/// ### Alternatives
///
/// *   <code>std::thread::[Thread](std::thread::Thread)::[id](std::thread::Thread::id)()</code> &mdash; cross platform, 64+ bit, but unrelated to system thread IDs
///
/// ### Example
///
/// ```
/// # use firehazard::*;
/// let tid = get_thread_id(get_current_thread()).unwrap();
/// assert_eq!(tid, get_current_thread_id());
///
/// // while Win32 thread IDs and std ThreadId s are unrelated, this *could* occasionally fail:
/// // assert_ne!(u64::from(tid), std::thread::current().id().as_u64().get());
/// ```
///
pub fn get_thread_id<'a>(thread: impl Into<firehazard::thread::PseudoHandle<'a>>) -> Result<firehazard::thread::Id, firehazard::Error> {
    use crate::AsLocalHandle;
    let id = unsafe { winapi::um::processthreadsapi::GetThreadId(thread.into().as_handle()) };
    firehazard::Error::get_last_if(id == 0)?;
    Ok(id)
}

#[doc(alias = "GetThreadId")]
/// GetThreadId(handle) != 0
///
/// If `handle` is a thread handle, but lacks the [`THREAD_QUERY_LIMITED_INFORMATION` access right](https://learn.microsoft.com/en-us/windows/win32/procthread/thread-security-and-access-rights), this may mistakenly return false.
/// On the other hand, can such a limited handle really be considered a proper thread handle?
/// The *one* thread-like thing you *might* do on a thread that's *that* locked down is wait on a `SYNCHRONIZE`-rights handle.
///
/// | `handle`                                  | Result                                        | Exception <br> [(Strict Handle Checks)](crate::process::mitigation::StrictHandleCheckPolicy)  |
/// | ------------------------------------------|:---------------------------------------------:|:---------------------------------------------------------------------------------------------:|
/// | null                                      | <span style="opacity: 50%">false</span>       | <span style="opacity: 50%">None</span>                                                        |
/// | INVALID_HANDLE_VALUE                      | <span style="opacity: 50%">false</span>       | <span style="opacity: 50%">None</span>                                                        |
/// | closed/dangling                           | <span style="opacity: 50%">false</span>       | STATUS_INVALID_HANDLE                                                                         |
/// | never valid                               | <span style="opacity: 50%">false</span>       | STATUS_INVALID_HANDLE                                                                         |
/// | **Valid Handles**                         |                                               |                                                                                               |
/// | access token                              | <span style="opacity: 50%">false</span>       | <span style="opacity: 50%">None</span>                                                        |
/// | desktop                                   | <span style="opacity: 50%">false</span>       | <span style="opacity: 50%">None</span>                                                        |
/// | file                                      | <span style="opacity: 50%">false</span>       | <span style="opacity: 50%">None</span>                                                        |
/// | job                                       | <span style="opacity: 50%">false</span>       | <span style="opacity: 50%">None</span>                                                        |
/// | process (inc. [get_current_process]\())   | <span style="opacity: 50%">false</span>       | <span style="opacity: 50%">None</span>                                                        |
/// | thread (inc. [get_current_thread]\())     | **true**                                      | <span style="opacity: 50%">None</span>                                                        |
/// | winsta                                    | <span style="opacity: 50%">false</span>       | <span style="opacity: 50%">None</span>                                                        |
///
pub(crate) fn is_thread_handle(handle: &impl firehazard::AsLocalHandle) -> bool {
    0 != unsafe { winapi::um::processthreadsapi::GetThreadId(handle.as_handle()) }
}



tests! {
    use winapi::shared::winerror::{ERROR_ACCESS_DENIED, ERROR_INVALID_HANDLE};

    #[test] #[strict_handle_check_exception = 0] fn is_thread_handle_null()                             { assert!(!is_thread_handle(&crate::handle::invalid::null()                                         )) }
    #[test] #[strict_handle_check_exception = 0] fn is_thread_handle_invalid_handle_value()             { assert!(!is_thread_handle(&crate::handle::invalid::invalid_value()                                )) }
    #[test] #[isolate] #[strict_handle_check_exception = 0xC0000008] fn is_thread_handle_never_valid()  { assert!(!is_thread_handle(&crate::handle::invalid::never_valid()                                  )) }
    #[test] #[isolate] #[strict_handle_check_exception = 0xC0000008] fn is_thread_handle_dangling()     { assert!(!is_thread_handle(&crate::handle::invalid::dangling()                                     )) }
    #[test] #[strict_handle_check_exception = 0] fn is_thread_handle_access_token_pseudo()              { assert!(!is_thread_handle(&get_current_process_token()                                            )) }
    #[test] #[strict_handle_check_exception = 0] fn is_thread_handle_access_token_process()             { assert!(!is_thread_handle(&open_process_token(get_current_process(), token::ALL_ACCESS).unwrap()  )) }
    #[test] #[strict_handle_check_exception = 0] fn is_thread_handle_access_token_impersonation() {
        let token = open_process_token(get_current_process(), token::ALL_ACCESS).unwrap();
        let token = duplicate_token_ex(&token, token::ALL_ACCESS, None, security::Delegation, token::Impersonation).unwrap();
        assert!(!is_thread_handle(&token));
    }

    #[test] #[strict_handle_check_exception = 0]
    fn is_thread_handle_desktop() {
        let desktop = open_input_desktop(None, false, access::GENERIC_ALL).unwrap();
        assert!(!is_thread_handle(&handle::Borrowed::from(&desktop)));
    }

    #[test] #[strict_handle_check_exception = 0]
    fn is_thread_handle_job() {
        let job = create_job_object_w(None, ()).unwrap();
        assert!(!is_thread_handle(&job));
    }

    #[test] #[strict_handle_check_exception = 0]
    fn is_thread_handle_process_pseudo() {
        assert!(!is_thread_handle(&get_current_process()));
    }

    #[test] #[strict_handle_check_exception = 0]
    fn is_thread_handle_process_real() {
        let p = std::process::Command::new(std::env::current_exe().unwrap())
            .arg("--help")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn().unwrap();
        assert!(!is_thread_handle(&p));
    }

    #[test] #[strict_handle_check_exception = 0]
    fn is_thread_handle_thread_pseudo() {
        assert!(is_thread_handle(&get_current_thread()));
    }

    #[test] #[strict_handle_check_exception = 0]
    fn is_thread_handle_thread_real() {
        let thread = std::thread::spawn(||{});
        assert!(is_thread_handle(&thread));
    }

    #[test] #[strict_handle_check_exception = 0]
    fn is_thread_handle_winsta() {
        let winsta0 = open_window_station_w(cstr16!("WinSta0"), false, winsta::ALL_ACCESS).unwrap();
        assert!(!is_thread_handle(&handle::Borrowed::from(&winsta0)));
    }
}
