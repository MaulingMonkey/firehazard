use crate::*;

use winapi::shared::winerror::*;
use winapi::um::processthreadsapi::*;
use winapi::um::synchapi::WaitForSingleObject;
use winapi::um::winbase::*;



#[doc(alias = "GetCurrentThread")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentthread)\]
/// GetCurrentThread
///
pub fn get_current_thread() -> thread::PsuedoHandle<'static> { unsafe { thread::PsuedoHandle::from_raw(GetCurrentThread()).unwrap() } }



#[doc(alias = "GetCurrentThreadId")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentthreadid)\]
/// GetCurrentThreadId
///
pub fn get_current_thread_id() -> thread::Id { unsafe { GetCurrentThreadId() } }



#[doc(alias = "ResumeThread")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-resumethread)\]
/// ResumeThread
///
/// Resumes execution of a previously suspended thread.
///
/// ### Safety
/// As this operates on simple refcounts, and not reciepts or other proof of ownership over the suspension request,
/// this function can allow you to resume a thread out from underneath a debugger, garbage collector,
/// or other piece of code that expected a thread to remain suspended - and the data said thread accessed, unmodified.
///
/// This is practically **begging** for undefined behavior.
///
/// ### Returns
/// *   `Ok(previous_suspend_count)`    &mdash; if the call succeeded and the suspend count decreased
/// *   `Err(ERROR_BAD_ACCESS)`?        &mdash; if `thread`'s handle lacks the `THREAD_SUSPEND_RESUME` access right
/// *   `Err(ERROR_INVALID_HANDLE)`?    &mdash; if `thread`'s handle is invalid (or not a thread)
///
pub unsafe fn resume_thread<'a>(thread: impl AsRef<thread::Handle<'a>>) -> Result<u32, Error> {
    let r = unsafe { ResumeThread(thread.as_ref().as_handle()) };
    Error::get_last_if(r as i32 == -1)?;
    Ok(r)
}



#[doc(alias = "SuspendThread")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-suspendthread)\]
/// SuspendThread
///
/// Suspend execution of a thread.
///
/// **You are strongly discouraged from suspending threads in your own process.**
/// Doing so anyways is practically guaranteed to randomly deadlock your process.
/// You will suspend threads in random states - including states where they hold random system or allocator locks.
/// Instead, use a proper mutex or semaphore, allowing threads to become "suspended" in known states where they don't hold critical locks.
///
/// ### Safety
/// I assume bad things are likely to happen if the refcount overflows (e.g. acting as a [`resume_thread`], which is `unsafe`.)
/// It might be worth testing to verify if that's true or not... were it not for the big fat warning above about suspending your own process's threads - that you should read anyways.
/// (Randomly deadlocking your process is technically sound, though, so it stays out of the safety section.)
///
/// ### Returns
/// *   `Ok(previous_suspend_count)`    &mdash; if the call succeeded and the suspend count decreased
/// *   `Err(ERROR_BAD_ACCESS)`?        &mdash; if `thread`'s handle lacks the `THREAD_SUSPEND_RESUME` access right
/// *   `Err(ERROR_INVALID_HANDLE)`?    &mdash; if `thread`'s handle is invalid (or not a thread)
///
pub unsafe fn suspend_thread<'a>(thread: impl AsRef<thread::Handle<'a>>) -> Result<u32, Error> {
    let r = unsafe { SuspendThread(thread.as_ref().as_handle()) };
    Error::get_last_if(r as i32 == -1)?;
    Ok(r)
}



#[doc(alias = "GetExitCodeThread")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getexitcodethread)\]
/// GetExitCodeThread
///
/// ### Returns
/// *   `Ok(STILL_ACTIVE)` / `Ok(STATUS_PENDING)`   if `thread` is still running
/// *   `Ok(0)`                                     if `thread` exited "successfully"
/// *   `Ok(exit_code)`                             if `thread` exited otherwise
/// *   `Err(...)`                                  if `thread` lacks appropriate querying permissions?
/// *   `Err(...)`                                  if `thread` is an invalid handle?
///
pub fn get_exit_code_thread<'a>(thread: impl AsRef<thread::Handle<'a>>) -> Result<u32, Error> {
    let mut exit_code = 0;
    Error::get_last_if(0 == unsafe { GetExitCodeThread(thread.as_ref().as_handle(), &mut exit_code) })?;
    Ok(exit_code)
}



#[doc(alias = "WaitForSingleObject")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-waitforsingleobject)\]
/// WaitForSingleObject(thread, 0) == WAIT_TIMEOUT
///
pub fn is_thread_running<'a>(thread: impl AsRef<thread::Handle<'a>>) -> bool {
    WAIT_TIMEOUT == unsafe { WaitForSingleObject(thread.as_ref().as_handle(), 0) }
}



#[doc(alias = "GetExitCodeThread")]
#[doc(alias = "WaitForSingleObject")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-waitforsingleobject)\] WaitForSingleObject(thread, INFINITE) +<br>
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getexitcodethread)\] GetExitCodeThread
///
pub fn wait_for_thread<'a>(thread: impl AsRef<thread::Handle<'a>>) -> Result<u32, Error> {
    match unsafe { WaitForSingleObject(thread.as_ref().as_handle(), INFINITE) } {
        WAIT_OBJECT_0       => {},
        WAIT_ABANDONED_0    => return Err(Error(ERROR_ABANDONED_WAIT_0)),   // shouldn't happen as `thread` isn't a mutex, right?
        WAIT_TIMEOUT        => return Err(Error(ERROR_ABANDONED_WAIT_63)),  // shouldn't happen - hopefully the `63` hints that something is funky?
        WAIT_FAILED         => return Err(Error::get_last()),
        _                   => return Err(Error(ERROR_ABANDONED_WAIT_63)),  // shouldn't happen - hopefully the `63` hints that something is funky?
    }
    get_exit_code_thread(thread)
}



#[doc(alias = "ExitThread")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-exitthread)\]
/// ExitThread
///
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
