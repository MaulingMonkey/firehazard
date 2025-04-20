#[doc(alias = "GetCurrentThreadId")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentthreadid)\]
/// GetCurrentThreadId
///
/// Get the current thread's ID.
/// May fail if `thread` lacks the [`THREAD_QUERY_LIMITED_INFORMATION` access right](https://learn.microsoft.com/en-us/windows/win32/procthread/thread-security-and-access-rights).
/// For a more durable reference to a thread, consider using a handle instead.
///
/// ### Alternatives
///
/// *   <code>std::thread::[current](std::thread::current)().[id](std::thread::Thread::id)()</code> &mdash; cross platform, 64+ bit, but unrelated to system thread IDs
///
/// ### Example
///
/// ```
/// # use firehazard::*;
/// let tid = get_current_thread_id();
/// assert_eq!(tid, get_thread_id(get_current_thread()).unwrap());
///
/// // while Win32 thread IDs and std ThreadId s are unrelated, this *could* occasionally fail:
/// // assert_ne!(u64::from(tid), std::thread::current().id().as_u64().get());
/// ```
///
pub fn get_current_thread_id() -> firehazard::thread::Id { unsafe { winapi::um::processthreadsapi::GetCurrentThreadId() } }
