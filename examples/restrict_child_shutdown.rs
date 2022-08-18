//! Throw away `SeShutdownPrivilege`

use win32_security_playground::*;

use abistr::cstr;

use winapi::shared::winerror::*;
use winapi::um::processthreadsapi::STARTUPINFOW;
use winapi::um::winbase::DETACHED_PROCESS;
use winapi::um::winnt::DISABLE_MAX_PRIVILEGE;

fn main() {
    // Allowed by initial permissive token
    let permissive = open_process_token(get_current_process(), token::ALL_ACCESS).unwrap();
    let r = shutdown_as_user("/s /t 3600", &permissive);
    let _ = shutdown_as_user("/a", &permissive);
    assert!(matches!(r, Ok(0)), "initial `shutdown /s /t 3600` failed: {r:?}");

    // Denied by DISABLE_MAX_PRIVILEGE
    let restrictive = unsafe { create_restricted_token(&permissive, DISABLE_MAX_PRIVILEGE, None, None, None) }.unwrap();
    let r = shutdown_as_user("/s /t 3600", &restrictive);
    let _ = shutdown_as_user("/a", &restrictive);
    assert!(matches!(r, Ok(ERROR_ACCESS_DENIED)), "`shutdown /s /t 3600` succeeded despite trying to throw away `SeShutdownPrivilege`: {r:?}");

    // Denied by explicitly removing SeShutdownPrivilege
    let se_shutdown = privilege::Luid::lookup_privilege_value_a(cstr!("SeShutdownPrivilege")).unwrap();
    let restrictive = unsafe { create_restricted_token(&permissive, 0, None, Some(&[privilege::LuidAndAttributes::new(se_shutdown, 0)]), None) }.unwrap();
    let r = shutdown_as_user("/s /t 3600", &restrictive);
    let _ = shutdown_as_user("/a", &restrictive);
    assert!(matches!(r, Ok(ERROR_ACCESS_DENIED)), "`shutdown /s /t 3600` succeeded despite trying to throw away `SeShutdownPrivilege`: {r:?}");
}

fn shutdown_as_user(args: &str, token: &token::Handle) -> Result<u32, Error> {
    let mut args = format!("shutdown {args}\0").encode_utf16().collect::<Vec<_>>();
    let si = STARTUPINFOW { cb: std::mem::size_of::<STARTUPINFOW>() as _, .. unsafe { std::mem::zeroed() } };
    let cmd = unsafe { create_process_as_user_w(token, (), Some(&mut args[..]), None, None, false, DETACHED_PROCESS, None, (), &si)}?;
    wait_for_process(cmd.process)
}
