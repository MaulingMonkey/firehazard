#[doc(alias = "CloseDesktop")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-closedesktop)\]
/// CloseDesktop
///
/// Explicitly close a desktop handle.
/// This is generally not necessary - owned handle types will automatically close themselves when dropped.
/// While there is a semi-regular error, `ERROR_BUSY`, that will be returned if you try to close a handle that's bound
/// with `SetThreadDesktop`, [`with_thread_desktop`] prohibits this at compile time by borrowing said handle.
/// So, this error should only really happen if you're bypassing this crate and using C++ - or FFI - to `SetThreadDesktop`.
/// It is *not* an error for *another* handle to the same desktop to be active on the current thread.
///
/// Note the awkward error type: ([desktop::OwnedHandle], [Error])
///
/// ### Examples
/// ```
/// # use firehazard::*;
/// # use firehazard::access::*;
/// # use abistr::cstr;
/// # use winapi::shared::winerror::*;
/// let desktop1name = cstr!("close_desktop_1");
/// let desktop1 = create_desktop_a(desktop1name, (), None, None, GENERIC_ALL, None).unwrap();
/// close_desktop(desktop1).unwrap(); // ≈ drop(desktop1), but explicit error checking
///
/// let desktop2name= cstr!("close_desktop_2");
/// let desktop2a   = create_desktop_a(desktop2name, (), None, None, GENERIC_ALL, None).unwrap();
/// let desktop2bee = open_desktop_a(  desktop2name, None, false, GENERIC_ALL).unwrap();
/// with_thread_desktop(&desktop2a, || {
///     close_desktop(desktop2bee).unwrap(); // closeable
///
///     // compile error - borrowed by with_thread_desktop:
///     // close_desktop(desktop2a).unwrap_err();
///
///     // cursed as heck - 2nd owner of same handle, not panic safe, evil demo purpouses only:
///     let desktop2a = unsafe { desktop::OwnedHandle::from_raw_nn(desktop2a.as_handle_nn()) };
///     let (desktop2a, error) = close_desktop(desktop2a).unwrap_err();
///     std::mem::forget(desktop2a); // uncurse: eliminate 2nd owner of same handle
///     assert_eq!(ERROR_BUSY, error); // handle in use by current thread
/// }).unwrap();
/// ```
/// ```compile_fail
/// # use firehazard::*;
/// # use firehazard::access::*;
/// # use abistr::cstr;
/// # use winapi::shared::winerror::*;
/// // No, you can't use `close_handle`:
/// let desktop_name = cstr!("close_desktop_3");
/// let desktop = create_desktop_a(desktop_name, (), None, None, GENERIC_ALL, None).unwrap();
/// let dupe = unsafe { desktop::OwnedHandle::from_raw_nn(desktop.as_handle_nn()) };
/// assert_eq!(ERROR_INVALID_HANDLE, close_handle(dupe).unwrap_err()); // now a compile error
/// let _ : () = close_desktop(desktop).unwrap();
/// ```
///
pub fn close_desktop(desktop: impl Into<desktop::OwnedHandle>) -> Result<(), (desktop::OwnedHandle, firehazard::Error)> {
    let desktop = desktop.into();
    if FALSE != unsafe { winapi::um::winuser::CloseDesktop(desktop.as_handle()) } {
        core::mem::forget(desktop);
        Ok(())
    } else {
        Err((desktop, firehazard::Error::get_last()))
    }
}
