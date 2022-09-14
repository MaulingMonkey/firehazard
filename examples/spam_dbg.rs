use firehazard::*;

macro_rules! dbg { ($expr:expr) => { println!("{}\\{}:{} {} = {:?}", env!("CARGO_MANIFEST_DIR"), file!(), line!(), stringify!($expr), $expr) }; }
macro_rules! dbgl { ($expr:expr) => {{
    println!("{}\\{}:{} {} = [", env!("CARGO_MANIFEST_DIR"), file!(), line!(), stringify!($expr));
    for e in $expr {
        println!("    {:?},", e);
    }
    println!("]");
}}}

fn main() {
    let t = open_process_token(get_current_process(), token::ALL_ACCESS).unwrap();
    let r = create_restricted_token(&t, None, None, None, Some(&[sid::AndAttributes::new(sid::NULL, None)])).unwrap();

    // https://docs.microsoft.com/en-us/windows/win32/api/winnt/ne-winnt-token_information_class
    dbg!(t.user());
    dbgl!(t.groups().unwrap().groups());
    dbgl!(t.privileges().unwrap().privileges());
    dbg!(t.owner());
    dbg!(t.primary_group());
    dbgl!(t.default_dacl().unwrap().default_dacl().aces());
    dbg!(t.source());
    dbg!(t.ty());
    dbg!(t.impersonation_level());
    dbg!(t.statistics().map(|s| s.GroupCount)); // several more subfields
    dbgl!(t.restricted_sids().unwrap().groups());
    dbgl!(r.restricted_sids().unwrap().groups());
    dbg!(t.session_id());
    dbgl!(t.groups_and_privileges().unwrap().sids());
    dbgl!(t.groups_and_privileges().unwrap().restricted_sids());
    dbgl!(t.groups_and_privileges().unwrap().privileges());
    dbg!(t.groups_and_privileges().unwrap().authentication_id());
    dbg!(t.sandbox_inert());
    dbg!(t.origin().map(|o| Luid::from(o.OriginatingLogonSession)));
    dbg!(t.elevation_type());
    dbg!(t.linked_token().map(|t| t.LinkedToken));
    dbg!(t.elevation());
    dbg!(t.is_elevated());
    dbg!(t.has_restrictions());
    dbg!(r.has_restrictions());
    dbg!(t.access_information().map(|i| i.AppContainerNumber)); // several more subfields
    dbg!(t.virtualization_allowed());
    dbg!(t.virtualization_enabled());
    dbg!(t.integrity_level());
    dbg!(t.ui_access());
    dbg!(t.mandatory_policy());
    dbg!(t.logon_sid());
    dbg!(t.is_app_container());
    dbg!(t.capabilities());
    dbg!(t.app_container_sid());
    dbg!(t.app_container_number());
    dbg!(t.user_claim_attributes().map(|a| a.AttributeCount));
    dbg!(t.device_claim_attributes().map(|a| a.AttributeCount));
    dbg!(t.device_groups());
    dbg!(t.restricted_device_groups());
    dbg!(r.restricted_device_groups());
}
