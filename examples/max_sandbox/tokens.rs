use firehazard::*;

pub struct Tokens {
    pub permissive: token::OwnedHandle,
    pub restricted: token::OwnedHandle,
}

pub fn create(target: &crate::settings::Target) -> Tokens {
    let sandbox_process_token = open_process_token(get_current_process(), token::ALL_ACCESS).unwrap();
    let gap = sandbox_process_token.groups_and_privileges().unwrap();
    let all_group_sids = gap.sids().iter().map(|g| g.sid).collect::<Vec<_>>();
    let logon_session_sid = sandbox_process_token.logon_sid().unwrap();
    let logon_session_sid = logon_session_sid.groups()[0].sid;

    let permissive = create_restricted_token_filter(
        &sandbox_process_token,
        None,
        |saa| !target.spawn.enabled.iter().any(|e| *saa.sid == **e),
        |p| !target.spawn.privileges.contains(&p.luid),
        Some(&target.spawn.restricted.as_ref().unwrap_or(&all_group_sids).iter().copied().map(|r| sid::AndAttributes::new(r, 0)).collect::<Vec<_>>()[..]),
    ).unwrap();

    let restricted = create_restricted_token_filter(
        &sandbox_process_token,
        None,
        |saa| !target.lockdown.enabled.iter().any(|e| *saa.sid == **e),
        |p| !target.lockdown.privileges.contains(&p.luid),
        Some(&target.lockdown.restricted.as_ref().unwrap_or(&all_group_sids).iter().copied().map(|r| sid::AndAttributes::new(r, 0)).collect::<Vec<_>>()[..]),
    ).unwrap();

    permissive.set_integrity_level(sid::AndAttributes::new(target.spawn.integrity.sid(), 0)).unwrap();
    restricted.set_integrity_level(sid::AndAttributes::new(target.spawn.integrity.sid(), 0)).unwrap(); // lower child token to target.lockdown.integrity post-spawn
    let permissive = duplicate_token_ex(&permissive, token::ALL_ACCESS, None, security::Impersonation, token::Impersonation).unwrap(); // primary -> impersonation token

    if false { // the need for this is currently being eliminated via abuse of the debugger APIs
        // For the child process to lower itself to untrusted integrity, in needs `token::ADJUST_DEFAULT` access
        // under the thread's current access token (currently done before `revert_to_self()`, so `permissive`).
        // `permissive` is currently restricted to only "Everyone" and "LogonSession_x_yyyyyyy" - the latter seems narrower, so we grant access to it.
        let mut acl = acl::Builder::new(acl::REVISION);
        acl.add_acl(acl::REVISION, 0, restricted.default_dacl().unwrap().default_dacl()).unwrap(); // allow debuggers to attach, task managers to kill, etc.
        acl.add_access_allowed_ace(acl::REVISION, token::ADJUST_DEFAULT | token::QUERY, logon_session_sid).unwrap();
        acl.finish().unwrap();
        restricted.set_default_dacl(&mut acl).unwrap();
    }

    Tokens { permissive, restricted }
}
