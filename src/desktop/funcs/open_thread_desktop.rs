#[doc(alias = "GetThreadDesktop")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getthreaddesktop)\]
/// GetThreadDesktop + DuplicateHandle
///
/// ### Example
/// ```
/// # use firehazard::*;
/// let desktop = open_thread_desktop(get_current_thread_id()).unwrap();
/// ```
///
/// ### Errata
/// The docs for [`GetThreadDesktop`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getthreaddesktop) state:
/// >   You do not need to call the CloseDesktop function to close the returned handle.
///
/// A borrowed handle is super awkward here, so this function returns a *duplicated* handle that can be closed instead.
///
pub fn open_thread_desktop(thread_id: thread::Id) -> firehazard::Result<desktop::OwnedHandle> {
    let mut desktop : HANDLE = unsafe { winapi::um::winuser::GetThreadDesktop(thread_id) }.cast();
    firehazard::Error::get_last_if(desktop.is_null())?;
    let process = get_current_process().as_handle();
    firehazard::Error::get_last_if(FALSE == unsafe { winapi::um::handleapi::DuplicateHandle(
        process, desktop, process, &mut desktop, 0, 0,
        winapi::um::winnt::DUPLICATE_SAME_ACCESS,
    )})?;
    unsafe { desktop::OwnedHandle::from_raw(desktop.cast()) }
}
