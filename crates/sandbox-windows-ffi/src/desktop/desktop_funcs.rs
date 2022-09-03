use crate::*;
use abistr::*;
use winapi::ctypes::c_char;
use winapi::shared::minwindef::{FALSE, LPARAM, BOOL, TRUE};
use winapi::shared::ntdef::HANDLE;
use winapi::shared::windef::HDESK;
use winapi::shared::winerror::*;
use winapi::um::errhandlingapi::SetLastError;
use winapi::um::handleapi::DuplicateHandle;
use winapi::um::winnt::DUPLICATE_SAME_ACCESS;
use winapi::um::winuser::*;
use core::ptr::null;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createdesktopa)\]
/// CreateDesktopA
///
/// ### Example
/// ```
/// # use firehazard::*;
/// # use firehazard::access::*;
/// # use abistr::cstr;
/// let desktop = create_desktop_a(cstr!("PlaygroundDesktop"), (), None, None, GENERIC_ALL, None).unwrap();
/// # let desktop = create_desktop_a(cstr!("PlaygroundDesktop"), (), None, None, GENERIC_ALL, None).unwrap();
/// ```
pub fn create_desktop_a(
    desktop:        impl TryIntoAsCStr,
    device:         impl TryIntoAsOptCStr,
    devmode:        Option<core::convert::Infallible>,
    flags:          impl Into<desktop::Flags>,
    desired_access: impl Into<desktop::AccessRights>,
    sa:             Option<&security::Attributes>,
) -> Result<desktop::OwnedHandle, Error> {
    let handle = unsafe { CreateDesktopA(
        desktop.try_into().map_err(|_| Error(E_STRING_NOT_NULL_TERMINATED as _))?.as_cstr(),
        device.try_into().map_err(|_| Error(E_STRING_NOT_NULL_TERMINATED as _))?.as_opt_cstr(),
        none2null(devmode),
        flags.into().into(),
        desired_access.into().into(),
        sa.map_or(null(), |sa| sa) as *mut _
    )};
    Error::get_last_if(handle.is_null())?;
    unsafe { desktop::OwnedHandle::from_raw(handle) }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createdesktopw)\]
/// CreateDesktopW
///
/// ### Example
/// ```
/// # use firehazard::*;
/// # use firehazard::access::*;
/// # use abistr::cstr16;
/// let desktop = create_desktop_w(cstr16!("PlaygroundDesktop"), (), None, None, GENERIC_ALL, None).unwrap();
/// # let desktop = create_desktop_w(cstr16!("PlaygroundDesktop"), (), None, None, GENERIC_ALL, None).unwrap();
/// ```
pub fn create_desktop_w(
    desktop:        impl TryIntoAsCStr<u16>,
    device:         impl TryIntoAsOptCStr<u16>,
    devmode:        Option<core::convert::Infallible>,
    flags:          impl Into<desktop::Flags>,
    desired_access: impl Into<desktop::AccessRights>,
    sa:             Option<&security::Attributes>,
) -> Result<desktop::OwnedHandle, Error> {
    let handle = unsafe { CreateDesktopW(
        desktop.try_into().map_err(|_| Error(E_STRING_NOT_NULL_TERMINATED as _))?.as_cstr(),
        device.try_into().map_err(|_| Error(E_STRING_NOT_NULL_TERMINATED as _))?.as_opt_cstr(),
        none2null(devmode),
        flags.into().into(),
        desired_access.into().into(),
        sa.map_or(null(), |sa| sa) as *mut _
    )};
    Error::get_last_if(handle.is_null())?;
    unsafe { desktop::OwnedHandle::from_raw(handle) }
}

// https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createdesktopexa
// CreateDesktopExA: adds heap size + reserved pvoid
// https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createdesktopexw
// CreateDesktopExW: adds heap size + reserved pvoid

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumdesktopsa)\]
/// EnumDesktopsA
///
/// ### Example
/// ```
/// # use firehazard::*;
/// let winsta = open_process_window_station().unwrap();
/// enum_desktops_a(&winsta, |desktop| {
///     println!("{desktop:?}");
///     Ok(())
/// }).unwrap();
/// ```
///
/// ### Output
/// ```text
/// "Default"
/// ```
///
/// ### Errata
/// The docs for [`EnumDesktopsA`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumdesktopsa) state:
/// >   If this parameter (`winsta`) is NULL, the current window station is used.
///
/// However, in my testing (Windows 10.0.19043.1889), this appears to be a lie: if the parameter is NULL, this enumerates
/// *window stations* instead of enumerating *desktops of said window station*.  As such, I've made `winsta` a non-optional
/// type in this function.
pub fn enum_desktops_a<F: FnMut(CStrPtr) -> Result<(), Error>>(
    winsta:         &winsta::OwnedHandle,
    mut enum_func:  F,
) -> Result<(), Error> {
    let enum_func : *mut F = &mut enum_func;
    Error::get_last_if(FALSE == unsafe { EnumDesktopsA(winsta.as_handle(), Some(fwd_enum_desktops_a::<F>), enum_func as LPARAM) })
}

unsafe extern "system" fn fwd_enum_desktops_a<F: FnMut(CStrPtr) -> Result<(), Error>>(desktop: *mut c_char, param: LPARAM) -> BOOL {
    let desktop = unsafe { CStrPtr::from_ptr_unbounded(desktop) };
    let f = unsafe { &mut *(param as *mut F) };
    match f(desktop) {
        Ok(()) => TRUE,
        Err(e) => {
            unsafe { SetLastError(e.0) };
            FALSE
        },
    }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumdesktopsw)\]
/// EnumDesktopsW
///
/// ### Example
/// ```
/// # use firehazard::*;
/// let winsta = open_process_window_station().unwrap();
/// enum_desktops_w(&winsta, |desktop| {
///     println!("{desktop:?}");
///     Ok(())
/// }).unwrap();
/// ```
///
/// ### Output
/// ```text
/// "Default"
/// ```
///
/// ### Errata
/// The docs for [`EnumDesktopsW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumdesktopsw) state:
/// >   If this parameter (`winsta`) is NULL, the current window station is used.
///
/// However, in my testing (Windows 10.0.19043.1889), this appears to be a lie: if the parameter is NULL, this enumerates
/// *window stations* instead of enumerating *desktops of said window station*.  As such, I've made `winsta` a non-optional
/// type in this function.
pub fn enum_desktops_w<F: FnMut(CStrPtr<u16>) -> Result<(), Error>>(
    winsta:         &winsta::OwnedHandle,
    mut enum_func:  F,
) -> Result<(), Error> {
    let enum_func : *mut F = &mut enum_func;
    Error::get_last_if(FALSE == unsafe { EnumDesktopsW(winsta.as_handle(), Some(fwd_enum_desktops_w::<F>), enum_func as LPARAM) })
}

unsafe extern "system" fn fwd_enum_desktops_w<F: FnMut(CStrPtr<u16>) -> Result<(), Error>>(desktop: *mut u16, param: LPARAM) -> BOOL {
    let desktop = unsafe { CStrPtr::<u16>::from_ptr_unbounded(desktop) };
    let f = unsafe { &mut *(param as *mut F) };
    match f(desktop) {
        Ok(()) => TRUE,
        Err(e) => {
            unsafe { SetLastError(e.0) };
            FALSE
        },
    }
}

// https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumdesktopwindows
// EnumDesktopWindows

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getthreaddesktop)\]
/// GetThreadDesktop + DuplicateHandle
///
/// ### Example
/// ```
/// # use firehazard::*;
/// let desktop = open_thread_desktop(get_current_thread_id()).unwrap();
/// ```
///
/// ### Errata
/// The docs for [`GetThreadDesktop`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getthreaddesktop) state:
/// >   You do not need to call the CloseDesktop function to close the returned handle.
///
/// A borrowed handle is super awkward here, so this function returns a *duplicated* handle that can be closed instead.
pub fn open_thread_desktop(thread_id: thread::Id) -> Result<desktop::OwnedHandle, Error> {
    let mut desktop : HANDLE = unsafe { GetThreadDesktop(thread_id) }.cast();
    Error::get_last_if(desktop.is_null())?;
    let process = get_current_process().as_handle();
    Error::get_last_if(FALSE == unsafe { DuplicateHandle(process, desktop, process, &mut desktop, 0, 0, DUPLICATE_SAME_ACCESS) })?;
    unsafe { desktop::OwnedHandle::from_raw(desktop.cast()) }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-opendesktopa)\]
/// OpenDesktopA
///
/// ### Example
/// ```
/// # use firehazard::*;
/// # use firehazard::access::*;
/// # use abistr::cstr;
/// let desktop = open_desktop_a(cstr!("Default"), None, false, GENERIC_ALL).unwrap();
/// ```
pub fn open_desktop_a(
    desktop:        impl TryIntoAsCStr,
    flags:          impl Into<desktop::Flags>,
    inherit:        bool,
    desired_access: impl Into<desktop::AccessRights>,
) -> Result<desktop::OwnedHandle, Error> {
    let handle = unsafe { OpenDesktopA(
        desktop.try_into().map_err(|_| Error(E_STRING_NOT_NULL_TERMINATED as _))?.as_cstr(),
        flags.into().into(),
        inherit as _,
        desired_access.into().into()
    )};
    Error::get_last_if(handle.is_null())?;
    unsafe { desktop::OwnedHandle::from_raw(handle) }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-opendesktopw)\]
/// OpenDesktopW
///
/// ### Example
/// ```
/// # use firehazard::*;
/// # use firehazard::access::*;
/// # use abistr::cstr16;
/// let desktop = open_desktop_w(cstr16!("Default"), None, false, GENERIC_ALL).unwrap();
/// ```
pub fn open_desktop_w(
    desktop:        impl TryIntoAsCStr<u16>,
    flags:          impl Into<desktop::Flags>,
    inherit:        bool,
    desired_access: impl Into<desktop::AccessRights>,
) -> Result<desktop::OwnedHandle, Error> {
    let handle = unsafe { OpenDesktopW(
        desktop.try_into().map_err(|_| Error(E_STRING_NOT_NULL_TERMINATED as _))?.as_cstr(),
        flags.into().into(),
        inherit as _,
        desired_access.into().into()
    )};
    Error::get_last_if(handle.is_null())?;
    unsafe { desktop::OwnedHandle::from_raw(handle) }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-openinputdesktop)\]
/// OpenInputDesktop
///
/// ### Example
/// ```
/// # use firehazard::*;
/// # use firehazard::access::*;
/// let desktop = open_input_desktop(None, false, GENERIC_ALL).unwrap();
/// ```
pub fn open_input_desktop(
    flags:          impl Into<desktop::Flags>,
    inherit:        bool,
    desired_access: impl Into<desktop::AccessRights>,
) -> Result<desktop::OwnedHandle, Error> {
    let handle = unsafe { OpenInputDesktop(flags.into().into(), inherit as _, desired_access.into().into()) };
    Error::get_last_if(handle.is_null())?;
    unsafe { desktop::OwnedHandle::from_raw(handle) }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-switchdesktop)\]
/// SwitchDesktop
///
/// ### Example
/// ```no_run
/// # use firehazard::*;
/// # use abistr::*;
/// let original = open_thread_desktop(get_current_thread_id()).unwrap();
/// let desktop = create_desktop_a(cstr!("examples_ui_switch_desktop"), (), None, None, access::GENERIC_ALL, None).unwrap();
///
/// // Sanity check we have permission to return to the original desktop before switching away from it
/// switch_desktop(&original).expect("unable to switch_desktop to original desktop, that's a bit sketchy");
///
/// // Switch to our new desktop (an empty black screen without explorer.exe rendering a background) for 3 seconds
/// switch_desktop(&desktop).unwrap();
/// sleep_ms(3000);
/// switch_desktop(&original).unwrap();
/// ```
pub fn switch_desktop(desktop: &desktop::OwnedHandle) -> Result<(), Error> {
    Error::get_last_if(FALSE == unsafe { SwitchDesktop(desktop.as_handle()) })
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setthreaddesktop)\]
/// SetThreadDesktop x2 + GetThreadDesktop
///
/// ### ⚠️ Warning ⚠️
/// New child processes appear to inherit the process's initial desktop, not the thread's current desktop.
/// To spawn a child process on a new desktop, instead specify [process::StartupInfoW::desktop].
///
/// ### Example
/// ```
/// # use firehazard::*;
/// # use firehazard::access::*;
/// # use abistr::cstr;
/// # use winapi::um::winuser::*;
/// let temp1 = create_desktop_a(cstr!("wtd.temp1"), (), None, None, GENERIC_ALL, None).unwrap();
/// let temp2 = create_desktop_a(cstr!("wtd.temp2"), (), None, None, GENERIC_ALL, None).unwrap();
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
pub fn with_thread_desktop<R>(desktop: &desktop::OwnedHandle, f: impl FnOnce()->R) -> Result<R, Error> {
    let thread = get_current_thread_id();
    let original = unsafe { GetThreadDesktop(thread) };
    let desktop = desktop.as_handle();
    Error::get_last_if(original.is_null())?;
    Error::get_last_if(FALSE == unsafe { SetThreadDesktop(desktop) })?;

    struct RestoreDesktopOnDrop(HDESK);
    impl Drop for RestoreDesktopOnDrop { fn drop(&mut self) { assert_eq!(FALSE, unsafe { SetThreadDesktop(self.0) }) } }
    let restore_desktop = RestoreDesktopOnDrop(original);

    let r = f();

    debug_assert_eq!(desktop, unsafe { GetThreadDesktop(thread) });
    core::mem::forget(restore_desktop); // manually restore for error codes:
    Error::get_last_if(FALSE == unsafe { SetThreadDesktop(original) })?;
    Ok(r)
}
