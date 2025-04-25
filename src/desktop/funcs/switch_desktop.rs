#[doc(alias = "SwitchDesktop")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-switchdesktop)\]
/// SwitchDesktop
///
/// Make the specified desktop the visible desktop.
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
///
pub fn switch_desktop(desktop: &desktop::OwnedHandle) -> firehazard::Result<()> {
    firehazard::Error::get_last_if(FALSE == unsafe { winapi::um::winuser::SwitchDesktop(
        desktop.as_handle()
    )})
}
