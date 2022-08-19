use win32_security_playground::*;

use abistr::cstr16;

use winapi::um::processthreadsapi::STARTUPINFOW;
use winapi::um::winbase::*;
use winapi::um::winnt::*;

use std::ffi::OsStr;
use std::mem::size_of_val;
use std::os::windows::prelude::OsStrExt;
use std::ptr::null_mut;



fn main() {
    let mut args = std::env::args_os();
    let exe = args.next().expect("args[0] / exe");
    let command = args.next().unwrap_or_default();

    match &*command.to_string_lossy() {
        "" | "default"              => default(&exe),
        "launched_low_integrity"    => launched_low_integrity(),
        command                     => panic!("unrecognized command {command:?}"),
    }
}

fn default(exe: &OsStr) {
    let t = open_process_token(get_current_process(), token::ALL_ACCESS).unwrap();
    //let t = unsafe { duplicate_token_ex(&t, token::ALL_ACCESS, None, SecurityImpersonation, token::Primary) };

    let privileges = t.privileges().unwrap();
    let privileges_to_remove = Some(privileges.privileges());
    //let privileges_to_remove = None;

    let groups = t.groups().unwrap();
    let mut logon_session_sids = groups.groups().iter().filter(|g| g.attributes & SE_GROUP_LOGON_ID != 0).copied();
    let logon_session_sid = logon_session_sids.next().expect("logon_session_sid").sid;
    assert!(logon_session_sids.next().is_none(), "multiple logon session SIDs?");

    // XXX: consider using the user + all enabled/logon sids from groups for permissive_to_restrict?
    let permissive_to_restrict = vec![
        // Users - required to load e.g. `C:\Windows\System32\cryptbase.dll`, which isn't in `/KnownDlls`.
        // This in turn is required by `ADVAPI32.dll` which forwards `SystemFunction036` (`RtlGenRandom`) to it.
        // That - at least at one point - was used to initialize Rust stdlib hash seeding for DoS attack resistance.
        // Required in rustc 1.63.0, but not in 1.61.0 (haven't tested 1.62.0)
        // Additionally: SeChangeNotifyPrivilege is required (hence why permissive only uses `DISABLE_MAX_PRIVILEGE`,
        // which keeps SeChangeNotifyPrivilege, instead of `privileges_to_remove`, which removes it.)
        sid::AndAttributes::new(sid!(S-1-5-32-545), 0), // Users

        // Everyone - required to load... `/KnownDlls` ?  Not actually quite sure, and procmon isn't super helpful here.
        sid::AndAttributes::new(sid!(S-1-1-0), 0), // Everyone

        // Logon Session - not required for launch?  But I'm currently using this to allow the child
        // process to open it's own token to lower it's integrity level from "Low" to "Untrusted".
        sid::AndAttributes::new(logon_session_sid,  0), // LogonSessionId_x_yyyyyyy
    ];

    let restrictive_to_restrict = vec![
        sid::AndAttributes::new(sid!(S-1-0-0), 0), // NULL SID
    ];

    //  1. Create the more permissive token used to initialize DLLs and run pre-main stuff.
    let permissive = unsafe { create_restricted_token(&t, DISABLE_MAX_PRIVILEGE, None, None, Some(&permissive_to_restrict)) }.unwrap();
    // untrusted integrity will cause `bcrypt.dll` to fail to load with 0xC0000142 / ERROR_DLL_INIT_FAILED, so launch with low integrity instead
    let low_integrity = sid::AndAttributes::new(sid!(S-1-16-4096), 0);
    permissive.set_integrity_level(low_integrity).unwrap();
    let permissive = unsafe { duplicate_token_ex(&permissive, token::ALL_ACCESS, None, SecurityImpersonation, token::Impersonation) }.unwrap();


    //  2. Create the more restrictive token used after `RevertToSelf()`.
    let crt_static  = cfg!(target_feature = "crt-static");
    let crt_dynamic = !crt_static;
    let groups      = t.groups().unwrap();
    let to_disable  = groups.groups().iter().copied().filter(|g| !(false
        // Keep "LogonSessionId_0_1288468" / S-1-5-5-0-1288468
        // 'cause chromium sandbox docs said so? something something network mounts?
        //|| (g.attributes & SE_GROUP_LOGON_ID != 0)

        // Prevent STATUS_DLL_NOT_FOUND for rust executables that dynamically link the CRT (from a non-main thread?)
        || (crt_dynamic && equal_sid(g.sid, sid!(S-1-1-0)))         // Everyone
        || (crt_dynamic && equal_sid(g.sid, sid!(S-1-5-32-545)))    // Users
    )).collect::<Vec<_>>();

    let restricted = unsafe { create_restricted_token(&t, 0, Some(&to_disable), privileges_to_remove, Some(&restrictive_to_restrict)) }.unwrap();
    //let untrusted_integrity = sid::AndAttributes::new(sid!(S-1-16-0), 0);
    restricted.set_integrity_level(low_integrity).unwrap(); // going directly to untrusted seems to cause the child to exit STATUS_BAD_IMPERSONATION_LEVEL
    //let restricted = unsafe { duplicate_token_ex(&restricted, token::ALL_ACCESS, None, SecurityImpersonation, token::Primary) };

    // For the child process to lower itself to untrusted integrity, in needs `token::ADJUST_DEFAULT` access
    // under the thread's current access token (currently done before `revert_to_self()`, so `permissive`).
    // `permissive` is currently restricted to only "Everyone" and "LogonSession_x_yyyyyyy" - the latter seems narrower, so we grant access to it.
    let mut acl = acl::Builder::new(acl::REVISION);
    acl.add_acl(acl::REVISION, 0, restricted.default_dacl().unwrap().default_dacl()).unwrap(); // allow debuggers to attach, task managers to kill, etc.
    acl.add_access_allowed_ace(acl::REVISION, token::ADJUST_DEFAULT | token::QUERY, logon_session_sid).unwrap();
    acl.finish().unwrap();
    restricted.set_default_dacl(&mut acl).unwrap();



    // "The maximum length of this string is 32K characters."
    // "The Unicode version of this function, CreateProcessAsUserW, can modify the contents of this string."
    // let exe = OsStr::new(r"C:\local\minimal\main.exe");
    let mut command_line = abistr::CStrBuf::<u16, 32768>::from_truncate(&exe.encode_wide().chain(cstr16!(" launched_low_integrity").to_units_with_nul().iter().copied()).collect::<Vec<_>>());

    let creation_flags = 0
        // | CREATE_DEFAULT_ERROR_MODE // hard errors? consider setting the local thread's error mode instead for finer control?
        //| DETACHED_PROCESS // | CREATE_NEW_CONSOLE | CREATE_NO_WINDOW // TODO: use one of these to secure console access?
        // | CREATE_PRESERVE_CODE_AUTHZ_LEVEL
        // | CREATE_SECURE_PROCESS // TODO: use to secure process more?
        | CREATE_SEPARATE_WOW_VDM // secure 16-bit nonsense
        | CREATE_SUSPENDED
        ;

    // TODO: use STARTUPINFOEXW to specify thread attributes
    // https://docs.microsoft.com/en-us/windows/win32/api/winbase/ns-winbase-startupinfoexw
    // https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-updateprocthreadattribute
    let mut startup_info = STARTUPINFOW {
        lpDesktop:  null_mut(), // TODO: use a new secured desktop
        dwFlags: 0
            | STARTF_UNTRUSTEDSOURCE // untrusted command line
            // | STARTF_USESTDHANDLES // stdin/stdout/stderr - might require inheriting handles?
        ,
        .. unsafe { std::mem::zeroed() }
    };
    startup_info.cb = size_of_val(&startup_info) as u32;

    let process_info = unsafe { create_process_as_user_w(
        &restricted,
        &*exe.to_string_lossy(), // XXX: simplify if/when abistr gets OsStr support: https://github.com/MaulingMonkey/abistr/issues/9
        Some(command_line.buffer_mut()),
        None, // process security attributes
        None, // thread security attributes
        false, // inherit inheriable handles
        creation_flags,
        None, // environment
        (), // working dir
        &startup_info,
    )}.expect("CreateProcessAsUserW");

    // temporarilly allow the child process's main thread to have more permissions to initialize DLLs etc.s
    set_thread_token(&process_info.thread, &permissive).unwrap();
    open_process_token(&process_info.process, token::ADJUST_DEFAULT).unwrap().set_integrity_level(sid::AndAttributes::new(sid!(S-1-16-0), 0)).unwrap(); // lower to untrusted integrity
    resume_thread(&process_info.thread).unwrap();

    let exit_code = wait_for_process(process_info.process).unwrap();
    if exit_code != 0 {
        use winapi::shared::ntstatus::*;
        let friendly = match exit_code as _ {
            STATUS_ACCESS_DENIED            => "STATUS_ACCESS_DENIED",              // 0xC0000022
            STATUS_BAD_IMPERSONATION_LEVEL  => "STATUS_BAD_IMPERSONATION_LEVEL",    // 0xC00000A5
            STATUS_DLL_NOT_FOUND            => "STATUS_DLL_NOT_FOUND",              // 0xC0000135
            STATUS_DLL_INIT_FAILED          => "STATUS_DLL_INIT_FAILED",            // 0xC0000142
            _                               => "STATUS_???",
        };
        panic!("{command_line:?} failed with exit code: 0x{exit_code:08x} ({friendly})");
    }
}

fn launched_low_integrity() {
    assert!(std::path::Path::new(r"C:\Windows\System32\kernel32.dll").exists());
    assert!(std::path::Path::new(r"C:\Windows\System32\cryptbase.dll").exists());

    // lower access
    let t = open_process_token(get_current_process(), token::ADJUST_DEFAULT | token::QUERY).unwrap();
    t.set_integrity_level(sid::AndAttributes::new(sid!(S-1-16-0), 0)).expect("should have lowered to untrusted integrity");
    t.set_integrity_level(sid::AndAttributes::new(sid!(S-1-16-4096), 0)).expect_err("shouldn't be able to raise from untrusted integrity to low");

    revert_to_self().expect("should have discarded our less restricted token");

    // we can still use the opened handle after revert_to_self, although we can't open new ones:
    t.set_integrity_level(sid::AndAttributes::new(sid!(S-1-16-0), 0)).unwrap();
    drop(t); // don't leave the handle open for abuse after revert_to_self()

    open_process_token(get_current_process(), token::ADJUST_DEFAULT).expect_err("shouldn't be able to re-open the process token from untrusted integrity, or with incompatible restricted SIDs");

    assert!(!std::path::Path::new(r"C:\Windows\System32\kernel32.dll").exists());
    assert!(!std::path::Path::new(r"C:\Windows\System32\cryptbase.dll").exists());
}
