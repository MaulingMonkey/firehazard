use crate::*;

use winapi::shared::winerror::*;
use winapi::um::processthreadsapi::*;
use winapi::um::synchapi::WaitForSingleObject;
use winapi::um::winbase::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentthread)\] GetCurrentThread
pub fn get_current_thread() -> thread::PsuedoHandle { unsafe { thread::PsuedoHandle::from_raw_unchecked(GetCurrentThread()) } }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentthreadid)\] GetCurrentThreadId
pub fn get_current_thread_id() -> thread::Id { unsafe { GetCurrentThreadId() } }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-resumethread)\] ResumeThread
pub fn resume_thread(thread: &thread::OwnedHandle) -> Result<u32, Error> { let r = unsafe { ResumeThread(thread.as_handle()) }; if r as i32 != -1 { Ok(r) } else { Err(Error::get_last()) } }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-suspendthread)\] SuspendThread
pub fn suspend_thread(thread: &thread::OwnedHandle) -> Result<u32, Error> { let r = unsafe { SuspendThread(thread.as_handle()) }; if r as i32 != -1 { Ok(r) } else { Err(Error::get_last()) } }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getexitcodethread)\] GetExitCodeThread
///
/// ### Returns
/// *   `Ok(STILL_ACTIVE)` / `Ok(STATUS_PENDING)`   if `thread` is still running
/// *   `Ok(0)`                                     if `thread` exited "successfully"
/// *   `Ok(exit_code)`                             if `thread` exited otherwise
/// *   `Err(...)`                                  if `thread` lacks appropriate querying permissions?
/// *   `Err(...)`                                  if `thread` is an invalid handle?
pub fn get_exit_code_thread(thread: impl AsRef<thread::Handle>) -> Result<u32, Error> {
    let mut exit_code = 0;
    Error::get_last_if(0 == unsafe { GetExitCodeThread(thread.as_ref().as_handle(), &mut exit_code) })?;
    Ok(exit_code)
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-waitforsingleobject)\] WaitForSingleObject(thread, 0) == WAIT_TIMEOUT
pub fn is_thread_running(thread: impl AsRef<thread::Handle>) -> bool {
    WAIT_TIMEOUT == unsafe { WaitForSingleObject(thread.as_ref().as_handle(), 0) }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-waitforsingleobject)\] WaitForSingleObject(thread, INFINITE) +<br>
/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getexitcodethread)\] GetExitCodeThread
pub fn wait_for_thread(thread: impl AsRef<thread::Handle>) -> Result<u32, Error> {
    match unsafe { WaitForSingleObject(thread.as_ref().as_handle(), INFINITE) } {
        WAIT_OBJECT_0       => {},
        WAIT_ABANDONED_0    => return Err(Error(ERROR_ABANDONED_WAIT_0)),   // shouldn't happen as `thread` isn't a mutex, right?
        WAIT_TIMEOUT        => return Err(Error(ERROR_ABANDONED_WAIT_63)),  // shouldn't happen - hopefully the `63` hints that something is funky?
        WAIT_FAILED         => return Err(Error::get_last()),
        _                   => return Err(Error(ERROR_ABANDONED_WAIT_63)),  // shouldn't happen - hopefully the `63` hints that something is funky?
    }
    get_exit_code_thread(thread)
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-exitthread)\] ExitThread
pub fn exit_thread(exit_code: u32) { unsafe { ExitThread(exit_code) } }

#[cfg(std)] #[test] fn test_wait_exit() {
    use winapi::um::minwinbase::STILL_ACTIVE;
    use std::thread::*;
    let child = spawn(|| { sleep(core::time::Duration::from_millis(500)); exit_thread(3); });
    let child = thread::OwnedHandle::from(child);

    assert!(is_thread_running(&child));
    assert_eq!(STILL_ACTIVE, get_exit_code_thread(&child).unwrap());

    assert_eq!(3, wait_for_thread(&child).unwrap());

    assert!(!is_thread_running(&child));
    assert_eq!(3, get_exit_code_thread(&child).unwrap());
}
