#[doc(alias = "GetCurrentThread")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentthread)\]
/// GetCurrentThread
///
/// Get a psuedo-handle (currently `-2`) to the current thread.
/// To get a real, [`Send`]able handle, use [`try_clone_to_owned`](firehazard::thread::PseudoHandle::try_clone_to_owned).
///
/// ### Alternatives
///
/// *   <code>std::thread::[current](std::thread::current)()</code> &mdash; cross platform, requires [`std`].
///
/// ### Examples
///
/// ```
/// # use firehazard::*;
/// let a : thread::PseudoHandle = get_current_thread();
/// let b : thread::OwnedHandle  = a.try_clone_to_owned().unwrap();
/// # #[cfg(std)]
/// let c : std::thread::Thread  = std::thread::current();
///
/// assert_eq!(-2, a.as_handle() as isize);
/// assert_ne!(-2, b.as_handle() as isize);
/// //sert_eq!(b.as_handle(), c.as_raw_handle()); // XXX: Thread doesn't impl AsRawHandle
/// ```
///
pub fn get_current_thread() -> firehazard::thread::PseudoHandle<'static> {
    use firehazard::FromLocalHandle;
    unsafe { firehazard::thread::PseudoHandle::from_raw(winapi::um::processthreadsapi::GetCurrentThread()).unwrap() }
}
