//! Throw away `SeShutdownPrivilege`

use sandbox_windows_ffi::*;

use abistr::cstr;

use winapi::shared::winerror::*;

fn main() {
    // Allowed by initial permissive token
    let permissive = open_process_token(get_current_process(), token::ALL_ACCESS).unwrap();
    let r = shutdown_as_user("/s /t 3600", &permissive);
    let _ = shutdown_as_user("/a", &permissive);
    assert!(matches!(r, Ok(0)), "initial `shutdown /s /t 3600` failed: {r:?}");

    // Denied by DISABLE_MAX_PRIVILEGE
    let restrictive = create_restricted_token(&permissive, token::DISABLE_MAX_PRIVILEGE, None, None, None).unwrap();
    let r = shutdown_as_user("/s /t 3600", &restrictive);
    let _ = shutdown_as_user("/a", &restrictive);
    assert!(matches!(r, Ok(ERROR_ACCESS_DENIED)), "`shutdown /s /t 3600` succeeded despite trying to throw away `SeShutdownPrivilege`: {r:?}");

    // Denied by explicitly removing SeShutdownPrivilege
    let se_shutdown = lookup_privilege_value_a(cstr!("SeShutdownPrivilege")).unwrap();
    let restrictive = create_restricted_token(&permissive, None, None, Some(&[privilege::LuidAndAttributes::new(se_shutdown, 0)]), None).unwrap();
    let r = shutdown_as_user("/s /t 3600", &restrictive);
    let _ = shutdown_as_user("/a", &restrictive);
    assert!(matches!(r, Ok(ERROR_ACCESS_DENIED)), "`shutdown /s /t 3600` succeeded despite trying to throw away `SeShutdownPrivilege`: {r:?}");
}

fn shutdown_as_user(args: &str, token: &token::OwnedHandle) -> Result<u32, Error> {
    let mut args = format!("shutdown {args}\0").encode_utf16().collect::<Vec<_>>();
    let si = process::StartupInfoW::default();
    let cmd = create_process_as_user_w(token, (), Some(&mut args[..]), None, None, false, process::DETACHED_PROCESS, process::environment::Inherit, (), &si)?;
    wait_for_process(&cmd.process)
}
