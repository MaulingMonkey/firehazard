use win32_security_playground::*;

use abistr::cstr;

use winapi::um::processthreadsapi::STARTUPINFOW;
use winapi::um::winbase::*;
use winapi::um::winnt::SecurityImpersonation;

use std::collections::HashSet;
use std::os::windows::prelude::*;
use std::path::PathBuf;

#[allow(dead_code)] // individual trust levels
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)] #[repr(u8)] enum Integrity { Untrusted, Low, Medium, High, System }
impl Default for Integrity { fn default() -> Self { Integrity::Untrusted } }
impl Integrity {
    pub fn sid(self) -> sid::Ptr<'static> { match self {
        Self::Untrusted => sid!(S-1-16-0x0000),
        Self::Low       => sid!(S-1-16-0x1000),
        Self::Medium    => sid!(S-1-16-0x2000),
        Self::High      => sid!(S-1-16-0x3000),
        Self::System    => sid!(S-1-16-0x4000),
    }}
}

struct TokenSettings {
    pub integrity:      Integrity,
    pub privileges:     HashSet<privilege::Luid>,
    pub enabled:        Vec<sid::Ptr<'static>>,
    pub restricted:     Option<Vec<sid::Ptr<'static>>>,
}

impl Default for TokenSettings {
    fn default() -> Self {
        Self {
            integrity:  Default::default(),
            privileges: Default::default(),
            enabled:    Default::default(),
            restricted: Some(vec![sid!(S-1-0-0)]),
        }
    }
}

struct Target {
    pub exe:            PathBuf,
    pub spawn:          TokenSettings, // Used to initialize non-delayed DLLs, pre-main code
    pub lockdown:       TokenSettings, // Eventual lockdown settings
}

impl Target {
    #[allow(unused_variables)]
    pub fn list() -> impl Iterator<Item = Self> {
        let sandbox_process_token = open_process_token(get_current_process(), token::ALL_ACCESS).unwrap();
        let all         = Box::leak(Box::new(sandbox_process_token.groups_and_privileges().unwrap())).sids().iter().map(|s| s.sid).collect::<Vec<_>>();
        let user        = Box::leak(Box::new(sandbox_process_token.user().unwrap())).user().sid;
        let session     = Box::leak(Box::new(sandbox_process_token.login_sid().unwrap())).groups().iter().next().unwrap().sid;
        let users       = sid!(S-1-5-32-545);
        let everyone    = sid!(S-1-1-0);
        let null        = sid!(S-1-0-0);

        let self_exe = std::path::PathBuf::from(std::env::args_os().next().expect("args[0] / exe"));
        let dir = self_exe.parent().unwrap();

        let se_change_notify_privilege = privilege::Luid::lookup_privilege_value_a(cstr!("SeChangeNotifyPrivilege")).unwrap();
        vec![
            Target {
                exe: dir.join("trivial.exe"),
                spawn:  TokenSettings {
                    // Minimal permissions to access e.g. `/KnownDlls` / `kernel32.dll` for init
                    privileges: [se_change_notify_privilege].into_iter().collect(),
                    enabled:    vec![everyone],
                    restricted: Some(vec![everyone]),
                    .. Default::default()
                },
                lockdown: TokenSettings {
                    .. Default::default()
                },
            },
            Target {
                exe: dir.join("less_trivial.exe"),
                spawn:  TokenSettings {
                    integrity:  Integrity::Low,
                    privileges: [se_change_notify_privilege].into_iter().collect(), // DLL access
                    enabled:    vec![user, users, everyone, session],
                    restricted: Some(vec![user, users, everyone, session, null]),
                    .. Default::default()
                },
                lockdown: TokenSettings {
                    // TODO: much if this can be lowered on rustc 1.61.0 but not 1.63.0
                    // partially because of threads?
                    // delay lockdown until debug string?
                    integrity:  Integrity::Low,
                    privileges: [se_change_notify_privilege].into_iter().collect(), // DLL access
                    enabled:    vec![user, users, everyone, session],
                    restricted: Some(vec![user, users, everyone, session, null]),
                    .. Default::default()
                },
            },
        ].into_iter()
    }
}

fn main() {
    for target in Target::list() {
        run(target);
    }
}

fn run(target: Target) {
    assert!(target.spawn.integrity >= target.lockdown.integrity, "target.lockdown.integrity cannot be more permissive than spawn integrity");

    let sandbox_process_token = open_process_token(get_current_process(), token::ALL_ACCESS).unwrap();
    let gap = sandbox_process_token.groups_and_privileges().unwrap();
    let all_group_sids = gap.sids().iter().map(|g| g.sid).collect::<Vec<_>>();

    let disable     = gap.sids().iter().copied().filter(|saa| !target.spawn.enabled.iter().copied().any(|e| *saa.sid == *e)).collect::<Vec<_>>();
    let permissive = unsafe { create_restricted_token(
        &sandbox_process_token,
        0,
        Some(&disable[..]),
        Some(&gap.privileges().iter().copied().filter(|p| !target.spawn.privileges.contains(&p.luid)).collect::<Vec<_>>()[..]),
        Some(&target.spawn.restricted.as_ref().unwrap_or(&all_group_sids).iter().copied().map(|r| sid::AndAttributes::new(r, 0)).collect::<Vec<_>>()[..]),
    )}.unwrap();

    let disable = gap.sids().iter().copied().filter(|saa| !target.lockdown.enabled.iter().copied().any(|e| *saa.sid == *e)).collect::<Vec<_>>();
    let restricted = unsafe { create_restricted_token(
        &sandbox_process_token,
        0,
        Some(&disable[..]),
        Some(&gap.privileges().iter().copied().filter(|p| !target.lockdown.privileges.contains(&p.luid)).collect::<Vec<_>>()[..]),
        Some(&target.lockdown.restricted.as_ref().unwrap_or(&all_group_sids).iter().copied().map(|r| sid::AndAttributes::new(r, 0)).collect::<Vec<_>>()[..]),
    )}.unwrap();

    permissive.set_integrity_level(sid::AndAttributes::new(target.spawn.integrity.sid(), 0)).unwrap();
    restricted.set_integrity_level(sid::AndAttributes::new(target.spawn.integrity.sid(), 0)).unwrap(); // lower child token to target.lockdown.integrity post-spawn
    let permissive = unsafe { duplicate_token_ex(&permissive, token::ALL_ACCESS, None, SecurityImpersonation, token::Impersonation) }; // primary -> impersonation token

    let mut command_line = abistr::CStrBuf::<u16, 32768>::from_truncate(&target.exe.as_os_str().encode_wide().chain(Some(0)).collect::<Vec<_>>());
    let mut si = STARTUPINFOW { lpDesktop: std::ptr::null_mut(), dwFlags: STARTF_UNTRUSTEDSOURCE, .. unsafe { std::mem::zeroed() } };
    si.cb = std::mem::size_of_val(&si) as u32;
    let pi = unsafe { create_process_as_user_w(&restricted, (), Some(command_line.buffer_mut()), None, None, false, CREATE_SEPARATE_WOW_VDM | CREATE_SUSPENDED, None, (), &si)}.unwrap();
    set_thread_token(&pi.thread, &permissive).unwrap();
    resume_thread(&pi.thread).unwrap();
    // XXX: wait for dlls to be initialized before doing this
    //open_process_token(&pi.process, token::ADJUST_DEFAULT).unwrap().set_integrity_level(sid::AndAttributes::new(target.lockdown.integrity.sid(), 0)).unwrap();

    let exit = wait_for_process(pi.process).unwrap();
    assert!(exit == 0, "exit code: 0x{exit:08x} {}", LastError::from(exit).friendly());
}
