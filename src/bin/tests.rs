use abistr::cstr;

use win32_security_playground::*;
use win32_security_playground::error::LastError;

use winapi::shared::winerror::*;
use winapi::um::securitybaseapi::SetTokenInformation;
use winapi::um::winnt::{SE_GROUP_LOGON_ID, TokenIntegrityLevel};

use std::mem::size_of_val;
use std::process::{Command, Stdio};



fn main() {
    use win32_security_playground::token::*;
    let t = open_process_token::current_process();
    let t2 = t.clone();
    assert!(t.as_handle() != t2.as_handle());
    t2.privileges_disable_if(|_| true).unwrap();
    t2.privileges_remove_if(|_| true).unwrap();

    macro_rules! dbg { ($expr:expr) => { println!("{}:{} {} = {:?}", file!(), line!(), stringify!($expr), $expr) }; }
    macro_rules! dbgl { ($expr:expr) => {{
        println!("{}:{} {} = [", file!(), line!(), stringify!($expr));
        for e in $expr {
            println!("    {:?},", e);
        }
        println!("]");
    }}}

    // https://docs.rs/winapi/latest/src/winapi/shared/winerror.rs.html
    dbg!(ERROR_BAD_LENGTH);             // 24
    dbg!(ERROR_INVALID_PARAMETER);      // 87
    dbg!(ERROR_INSUFFICIENT_BUFFER);    // 122
    dbg!(ERROR_NO_TOKEN);               // 1008
    dbg!(ERROR_INCORRECT_SIZE);         // 1462



    let privileges = t.get_token_privileges().unwrap();

    let groups      = t.get_token_groups().unwrap();
    let groups      = groups.groups();
    let to_disable  = groups.iter().copied().filter(|g| g.attributes & SE_GROUP_LOGON_ID == 0).collect::<Vec<_>>();
    let to_restrict = vec![sid::AndAttributes::new(sid!(S-1-0-0), 0)]; // restrict null SID

    let restricted = unsafe { create_restricted_token(&t, 0, Some(&to_disable), Some(privileges.privileges()), Some(&to_restrict)) }.unwrap();

    let integrity = sid::AndAttributes::new(sid!(S-1-16-0), 0); // untrusted integrity level
    assert!(0 != unsafe { SetTokenInformation(restricted.as_handle(), TokenIntegrityLevel, (&integrity) as *const _ as *mut _, size_of_val(&integrity) as _) }, "SetTokenInformation GetLastError()={:?}", LastError::get());

    let restricted_groups_and_privileges = restricted.get_token_groups_and_privileges().unwrap();
    dbg!(restricted.get_token_has_restrictions());
    dbgl!(restricted_groups_and_privileges.sids());
    dbgl!(restricted_groups_and_privileges.restricted_sids());
    dbgl!(restricted_groups_and_privileges.privileges());



    // https://docs.microsoft.com/en-us/windows/win32/api/winnt/ne-winnt-token_information_class
    dbg!(t.get_token_user());
    dbgl!(t.get_token_groups().unwrap().groups());
    dbgl!(t.get_token_privileges().unwrap().privileges());
    dbgl!(t2.get_token_privileges().unwrap().privileges());
    dbg!(t.get_token_owner());
    dbg!(t.get_token_primary_group());
    dbg!(t.get_token_default_dacl().map(|d| d.DefaultDacl));
    dbg!(t.get_token_source().map(|s| unsafe { std::mem::transmute::<[i8; 8], abistr::CStrBuf::<u8, 8>>(s.SourceName) }));
    dbg!(t.get_token_type());
    dbg!(t.get_token_impersonation_level());
    dbg!(t.get_token_statistics().map(|s| s.GroupCount)); // several more subfields
    dbg!(t.get_token_session_id());
    dbgl!(t.get_token_groups_and_privileges().unwrap().sids());
    dbgl!(t.get_token_groups_and_privileges().unwrap().restricted_sids());
    dbgl!(t.get_token_groups_and_privileges().unwrap().privileges());
    dbg!(t.get_token_groups_and_privileges().unwrap().authentication_id());
    dbg!(t.get_token_sandbox_inert());
    dbg!(t.get_token_origin().map(|o| Luid::from(o.OriginatingLogonSession)));
    dbg!(t.get_token_elevation_type());
    dbg!(t.get_token_linked_token().map(|t| t.LinkedToken));
    dbg!(t.get_token_elevation().map(|te| te.TokenIsElevated != 0));
    dbg!(t.get_token_is_elevated());
    dbg!(t.get_token_has_restrictions());
    dbg!(t.get_token_access_information().map(|i| i.AppContainerNumber)); // several more subfields
    dbg!(t.get_token_virtualization_allowed());
    dbg!(t.get_token_virtualization_enabled());
    dbg!(t.get_token_integrity_level());
    dbg!(t.get_token_ui_access());
    dbg!(t.get_token_mandatory_policy().map(|p| p.Policy));
    dbg!(t.get_token_login_sid());
    dbg!(t.get_token_is_app_container());
    dbg!(t.get_token_capabilities());
    dbg!(t.get_token_app_container_sid());
    dbg!(t.get_token_app_container_number());
    dbg!(t.get_token_user_claim_attributes().map(|a| a.AttributeCount));
    dbg!(t.get_token_device_claim_attributes().map(|a| a.AttributeCount));
    dbg!(t.get_token_device_groups());
    dbg!(t.get_token_restricted_device_groups());

    //assert_eq!((0, 0), attempt_shutdown()); // spammy UI dialogs in tests
    discard_privileges();
    assert_eq!((ERROR_ACCESS_DENIED, ERROR_ACCESS_DENIED), attempt_shutdown());
}

fn attempt_shutdown() -> (u32, u32) {
    let start = Command::new("shutdown").args("/s /t 3600".split(' ')).stderr(Stdio::null()).status().unwrap().code().unwrap_or(-1) as _;
    let abort = Command::new("shutdown").arg("/a").stderr(Stdio::null()).status().unwrap().code().unwrap_or(-1) as _;
    (start, abort)
}

fn discard_privileges() {
    let se_shutdown = privilege::Luid::lookup_privilege_value_a(cstr!("SeShutdownPrivilege")).unwrap();
    open_process_token::current_process().privileges_remove_if(|p| p == se_shutdown).unwrap();
}
