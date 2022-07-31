use winapi::shared::ntdef::LUID;
use winapi::shared::winerror::*;



fn main() {
    use win32_security_playground::handle::*;
    let t = open_current_process_token();
    let t = t.clone();

    macro_rules! dbg { ($expr:expr) => { println!("{}:{} {} = {:?}", file!(), line!(), stringify!($expr), $expr) }; }
    macro_rules! dbgl { ($expr:expr) => {{
        println!("{}:{} {} = [", file!(), line!(), stringify!($expr));
        for e in $expr {
            println!("    {:?},", e);
        }
        println!("]");
    }}}

    // https://docs.rs/winapi/latest/src/winapi/shared/winerror.rs.html
    dbg!(ERROR_INVALID_PARAMETER);      // 87
    dbg!(ERROR_INSUFFICIENT_BUFFER);    // 122
    dbg!(ERROR_NO_TOKEN);               // 1008
    dbg!(ERROR_INCORRECT_SIZE);         // 1462

    // https://docs.microsoft.com/en-us/windows/win32/api/winnt/ne-winnt-token_information_class
    dbg!(t.get_token_user());
    dbgl!(t.get_token_groups().unwrap().groups());
    dbgl!(t.get_token_privileges().unwrap().privileges());
    dbg!(t.get_token_owner());
    dbg!(t.get_token_primary_group());
    dbg!(t.get_token_default_dacl().map(|d| d.DefaultDacl));
    dbg!(t.get_token_source().map(|s| unsafe { std::mem::transmute::<[i8; 8], abistr::CStrBuf::<u8, 8>>(s.SourceName) }));
    dbg!(t.get_token_type());
    dbg!(t.get_token_impersonation_level());
    dbg!(t.get_token_statistics().map(|s| s.GroupCount)); // several more subfields
    dbg!(t.get_token_session_id());
    dbg!(t.get_token_groups_and_privileges().map(|gap| gap.SidCount));
    dbg!(t.get_token_sandbox_inert());
    dbg!(t.get_token_origin().map(|o| unsafe { std::mem::transmute::<LUID, u64>(o.OriginatingLogonSession) }));
    dbg!(t.get_token_elevation_type());
    dbg!(t.get_token_linked_token().map(|t| t.LinkedToken));
    dbg!(t.get_token_elevation().map(|te| te.TokenIsElevated != 0));
    dbg!(t.get_token_is_elevated());
    dbg!(t.get_token_has_restrictions());
    dbg!(t.get_token_access_information().map(|i| i.AppContainerNumber)); // several more subfields
    dbg!(t.get_token_virtualization_allowed());
    dbg!(t.get_token_virtualization_enabled());
    dbg!(t.get_token_integrity_level().map(|i| i.Label.Attributes)); // Sid
    dbg!(t.get_token_ui_access());
    dbg!(t.get_token_mandatory_policy().map(|p| p.Policy));
    dbg!(t.get_token_login_sid());
    dbg!(t.get_token_is_app_container());
    dbg!(t.get_token_capabilities());
    dbg!(t.get_token_app_container_sid().map(|s| s.TokenAppContainer));
    dbg!(t.get_token_app_container_number());
    dbg!(t.get_token_user_claim_attributes().map(|a| a.AttributeCount));
    dbg!(t.get_token_device_claim_attributes().map(|a| a.AttributeCount));
    dbg!(t.get_token_device_groups());
    dbg!(t.get_token_restricted_device_groups());
}
