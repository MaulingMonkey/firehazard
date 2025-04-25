#[doc(alias = "EnumDesktopsA")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumdesktopsa)\]
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
/// The docs for [`EnumDesktopsA`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumdesktopsa) state:
/// >   If this parameter (`winsta`) is NULL, the current window station is used.
///
/// However, in my testing (Windows 10.0.19043.1889), this appears to be a lie: if the parameter is NULL, this enumerates
/// *window stations* instead of enumerating *desktops of said window station*.  As such, I've made `winsta` a non-optional
/// type in this function.
///
pub fn enum_desktops_a<
    F: FnMut(CStrPtr) -> firehazard::Result<()>
>(
    winsta:         &winsta::OwnedHandle,
    mut enum_func:  F,
) -> firehazard::Result<()> {
    let enum_func : *mut F = &mut enum_func;
    firehazard::Error::get_last_if(FALSE == unsafe { winapi::um::winuser::EnumDesktopsA(
        winsta.as_handle(),
        Some(fwd_enum_desktops_a::<F>),
        enum_func as _,
    )})
}

unsafe extern "system" fn fwd_enum_desktops_a<
    F: FnMut(CStrPtr) -> firehazard::Result<()>
>(
    desktop:    *mut core::ffi::c_char,
    param:      winapi::shared::minwindef::LPARAM,
) -> BOOL {
    let desktop = unsafe { CStrPtr::from_ptr_unbounded(desktop) };
    let f = unsafe { &mut *(param as *mut F) };
    match f(desktop) {
        Ok(()) => TRUE,
        Err(e) => {
            unsafe { winapi::um::errhandlingapi::SetLastError(e.0) };
            FALSE
        },
    }
}



#[doc(alias = "EnumDesktops")]
#[doc(alias = "EnumDesktopsW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumdesktopsw)\]
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
/// The docs for [`EnumDesktopsW`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumdesktopsw) state:
/// >   If this parameter (`winsta`) is NULL, the current window station is used.
///
/// However, in my testing (Windows 10.0.19043.1889), this appears to be a lie: if the parameter is NULL, this enumerates
/// *window stations* instead of enumerating *desktops of said window station*.  As such, I've made `winsta` a non-optional
/// type in this function.
///
pub fn enum_desktops_w<F: FnMut(CStrPtr<u16>) -> firehazard::Result<()>>(
    winsta:         &winsta::OwnedHandle,
    mut enum_func:  F,
) -> firehazard::Result<()> {
    let enum_func : *mut F = &mut enum_func;
    firehazard::Error::get_last_if(FALSE == unsafe { winapi::um::winuser::EnumDesktopsW(
        winsta.as_handle(),
        Some(fwd_enum_desktops_w::<F>),
        enum_func as _,
    )})
}

unsafe extern "system" fn fwd_enum_desktops_w<
    F: FnMut(CStrPtr<u16>) -> firehazard::Result<()>
>(
    desktop:    *mut u16,
    param:      winapi::shared::minwindef::LPARAM,
) -> BOOL {
    let desktop = unsafe { CStrPtr::<u16>::from_ptr_unbounded(desktop) };
    let f = unsafe { &mut *(param as *mut F) };
    match f(desktop) {
        Ok(()) => TRUE,
        Err(e) => {
            unsafe { winapi::um::errhandlingapi::SetLastError(e.0) };
            FALSE
        },
    }
}



// https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumdesktopwindows
// TODO: EnumDesktopWindows
