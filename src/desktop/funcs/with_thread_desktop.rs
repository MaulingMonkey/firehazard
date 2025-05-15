#[doc(alias = "SetThreadDesktop")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setthreaddesktop)\]
/// SetThreadDesktop x2 + GetThreadDesktop
///
/// Temporarilly set the thread's desktop.
///
/// ### ⚠️ Warning ⚠️
/// New child processes appear to inherit the **process**'s initial desktop, not the thread's current desktop.
/// To spawn a child process on a new desktop, instead specify [process::StartupInfoW::desktop].
///
/// ### Example
/// ```
/// # use firehazard::*;
/// # use firehazard::access::*;
/// # use winapi::um::winuser::*;
/// let temp1 = create_desktop_a(c"wtd.temp1", (), None, None, GENERIC_ALL, None).unwrap();
/// let temp2 = create_desktop_a(c"wtd.temp2", (), None, None, GENERIC_ALL, None).unwrap();
/// let orig  = open_thread_desktop(get_current_thread_id()).unwrap();
/// with_thread_desktop(&temp1, || {
///     with_thread_desktop(&temp2, || {
///         with_thread_desktop(&temp1, || {
///             with_thread_desktop(&orig, || {
///                 // ...
///             }).unwrap();
///         }).unwrap();
///     }).unwrap();
/// }).unwrap();
/// ```
///
/// ### Errata
/// Thread ownership of `HDESK`s is a little wonky:
/// *   `CloseDesktop(desk)` will fail with `GetLastError() == ERROR_BUSY` if any threads are set to use `desk` as their desktops.
///     This is conceptually similar to the thread having a `std::cell::Ref<'static, Handle>` and panicing with a borrowing error.
///
/// *   `SetThreadDesktop(null)` is an error / noop and will not unlock the previously set desktop.
///
/// *   `GetThreadDesktop(thread_id)` returns a real handle while noting:
///     > You do not need to call the `CloseDesktop` function to close the returned handle.
///
///     To be clear - this is presumably because whatever code created said desktop is assumed to exclusively own, it and be in charge of closing it if needed.
///     Your code can just... kinda reborrow it without locking it, restore to it, etc.
///     You can extend the lifetime via `DuplicateHandle` and then restore the desktop via said duplicate, but then you cannot drop/close said duplicate handle.
///
/// *   By strictly enforcing LIFO stacking order / borrowing for the thread's desktops, [`with_thread_desktop`] avoids the
///     awkward ownership issues of `'static` lifetimes that would be involved with directly exposing SetThreadDesktop.
///
pub fn with_thread_desktop<R>(desktop: &desktop::OwnedHandle, f: impl FnOnce()->R) -> firehazard::Result<R> {
    let thread = get_current_thread_id();
    let original = unsafe { winapi::um::winuser::GetThreadDesktop(thread) };
    let desktop = desktop.as_handle();
    firehazard::Error::get_last_if(original.is_null())?;
    firehazard::Error::get_last_if(FALSE == unsafe { winapi::um::winuser::SetThreadDesktop(desktop) })?;

    struct RestoreDesktopOnDrop(winapi::shared::windef::HDESK);
    impl Drop for RestoreDesktopOnDrop { fn drop(&mut self) { assert_eq!(FALSE, unsafe { winapi::um::winuser::SetThreadDesktop(self.0) }) } }
    let restore_desktop = RestoreDesktopOnDrop(original);

    let r = f();

    debug_assert_eq!(desktop, unsafe { winapi::um::winuser::GetThreadDesktop(thread) });
    core::mem::forget(restore_desktop); // manually restore for error codes:
    firehazard::Error::get_last_if(FALSE == unsafe { winapi::um::winuser::SetThreadDesktop(original) })?;
    Ok(r)
}
