//! Throw away `SeShutdownPrivilege`

use firehazard::*;
use abistr::cstr;
use winapi::shared::winerror::ERROR_ACCESS_DENIED;
use std::io;
use std::process::{Command, Stdio};

fn main() {
    // Not yet locked down
    let r = shutdown("/s /t 3600");
    let _ = shutdown("/a");
    assert!(matches!(r, Ok(Some(0))), "initial `shutdown /a` failed: {r:?}");

    // Lock down privileges
    let se_shutdown = lookup_privilege_value_a((), cstr!("SeShutdownPrivilege")).unwrap();
    let access
        = token::QUERY              // Required to enumerate the `token.privileges()`, as used internally by `privileges_disable_if`
        | token::ADJUST_PRIVILEGES  // Required to modify the privileges
        ;
    let token = open_process_token(get_current_process(), access).unwrap();
    token.privileges_remove_if(|p| p == se_shutdown).unwrap();
    // FOOTGUN: don't just use privileges_disable_if - `SeShutdownPRivilege` is already disabled, and disabled privileges can be re-enabled, as `shutdown` will!

    // Locked down
    let r = shutdown("/s /t 3600");
    let _ = shutdown("/a");
    assert!(matches!(r, Ok(Some(ERROR_ACCESS_DENIED))), "initial `shutdown /a` succeeded despite trying to throw away `SeShutdownPrivilege`: {r:?}");
}

fn shutdown(args: &str) -> io::Result<Option<u32>> {
    Command::new("shutdown").args(args.split(' ')).stderr(Stdio::null()).status().map(|s| s.code().map(|c| c as _))
}
