use sandbox_windows_ffi::*;
use abistr::*;

fn main() {
    let original = open_thread_desktop(get_current_thread_id()).unwrap();
    let desktop = create_desktop_a(cstr!("examples_ui_switch_desktop"), (), None, None, access::GENERIC_ALL, None).unwrap();

    // Sanity check we have permission to return to the original desktop before switching away from it
    switch_desktop(&original).expect("unable to switch_desktop to original desktop, that's a bit sketchy");

    // Switch to our new desktop (an empty black screen without explorer.exe rendering a background) for 3 seconds
    switch_desktop(&desktop).unwrap();
    sleep_ms(3000);
    switch_desktop(&original).unwrap();
}
