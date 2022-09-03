//! ### Examples
//! ```cmd
//! :: Display alt desktop for 3 seconds
//! cargo run --example ui_switch_desktop
//!
//! :: Display notepad.exe on alt desktop until it's closed
//! cargo run --example ui_switch_desktop -- C:\Windows\System32\notepad.exe
//! ```

use firehazard::*;
use firehazard::process::{environment, StartupInfoW};
use abistr::*;

fn main() {
    let mut args = std::env::args_os();
    let _this_exe = args.next();
    let target_exe = args.next().map(std::path::PathBuf::from);

    let original = open_thread_desktop(get_current_thread_id()).unwrap();
    let desktop_name = cstr16!("examples_ui_switch_desktop");
    let desktop = create_desktop_w(desktop_name, (), None, None, access::GENERIC_ALL, None).unwrap();

    // Sanity check we have permission to return to the original desktop before switching away from it
    switch_desktop(&original).expect("unable to switch_desktop to original desktop, that's a bit sketchy");

    let child = target_exe.map(|t| {
        let args : &[&'static str] = &[];
        let mut cmd = argv_to_command_line_0(t, args);
        create_process_w((), Some(&mut cmd[..]), None, None, false, None, environment::Inherit, (), &StartupInfoW {
            desktop:    Some(desktop_name),
            ..Default::default()
        }).unwrap()
    });

    // Switch to our new desktop (an empty black screen without explorer.exe rendering a background) for 3 seconds
    switch_desktop(&desktop).unwrap();

    let process_result = if let Some(child) = child {
        wait_for_process(&child.process)
    } else {
        sleep_ms(3000);
        Ok(0)
    };

    switch_desktop(&original).unwrap();
    process_result.unwrap();
}
