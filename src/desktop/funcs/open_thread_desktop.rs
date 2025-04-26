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
    let desktop = unsafe { winapi::um::winuser::GetThreadDesktop(thread_id) }.cast();
    unsafe { desktop::OwnedHandle::borrow_from_raw(&desktop) }?.try_clone()
}
