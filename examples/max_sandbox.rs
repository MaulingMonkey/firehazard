use sandbox::windows::ffi::*;

use abistr::*;

use winapi::shared::minwindef::FALSE;
use winapi::um::handleapi::DuplicateHandle;
use winapi::um::minwinbase::*;
use winapi::um::winbase::*;
use winapi::um::winnt::*;

use std::collections::*;
use std::ffi::OsString;
use std::mem::MaybeUninit;
use std::mem::zeroed;
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

#[derive(Default)] struct Allow {
    pub same_desktop:   bool,
}

struct Target {
    pub exe:            PathBuf,
    pub allow:          Allow,
    pub spawn:          TokenSettings, // Used to initialize non-delayed DLLs, pre-main code
    pub lockdown:       TokenSettings, // Eventual lockdown settings
}

impl Target {
    #[allow(unused_variables)]
    pub fn list() -> impl Iterator<Item = Self> {
        let sandbox_process_token = open_process_token(get_current_process(), token::ALL_ACCESS).unwrap();
        let all         = Box::leak(Box::new(sandbox_process_token.groups_and_privileges().unwrap())).sids().iter().map(|s| s.sid).collect::<Vec<_>>();
        let user        = Box::leak(Box::new(sandbox_process_token.user().unwrap())).user().sid;
        let session     = Box::leak(Box::new(sandbox_process_token.logon_sid().unwrap())).groups().iter().next().unwrap().sid;
        let users       = sid!(S-1-5-32-545);
        let everyone    = sid!(S-1-1-0);
        let null        = sid!(S-1-0-0);

        let self_exe = std::path::PathBuf::from(std::env::args_os().next().expect("args[0] / exe"));
        let dir = self_exe.parent().unwrap();

        let se_change_notify_privilege = lookup_privilege_value_a(cstr!("SeChangeNotifyPrivilege")).unwrap();
        vec![
            Target {
                exe: dir.join("trivial.exe"),
                allow: Allow::default(),
                spawn: TokenSettings {
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
                allow: Allow::default(),
                spawn: TokenSettings {
                    integrity:  Integrity::Low,
                    privileges: [se_change_notify_privilege].into_iter().collect(), // DLL access
                    enabled:    vec![user, users, everyone, session],
                    restricted: Some(vec![user, users, everyone, session]),
                    .. Default::default()
                },
                lockdown: TokenSettings {
                    // `BCryptGenRandom` is unreliable unless the process token has `user` or `session` available.
                    // Specifically, the second call to it can fail with GetLastError()==ERROR_ACCESS_DENIED, even with
                    // identical params - due to `RegisterReopenWait` / `ReopenSystemPreferredRngCallback` failing.
                    // ```
                    // thread 'main' panicked at 'couldn't generate random bytes with preferred RNG: Os { code: 5, kind: PermissionDenied, message: "Access is denied." }'
                    // ```
                    // In rust 1.61.0, this is kinda fine, since `std::sys::windows::rand::hashmap_random_keys` only calls it once... per thread, which is less fine.
                    // In rust 1.63.0, this is broken, since `hashmap_random_keys` calls it once to check if it should use it, then again to actually use it.
                    // In rust ~nightly, this is fine, since `RtlGenRandom` will be used as a fallback whenever it fails thanks to this simplification PR:
                    // <https://github.com/rust-lang/rust/commit/46673bb08ffa22f21287349d966d875038e41b37>
                    // <https://github.com/rust-lang/rust/blob/1.61.0/library/std/src/sys/windows/rand.rs#L18>
                    // <https://github.com/rust-lang/rust/blob/1.63.0/library/std/src/sys/windows/rand.rs#L16>
                    // <https://github.com/rust-lang/rust/blob/2fbc08e2ce64dee45a29cb6133da6b32366268aa/library/std/src/sys/windows/rand.rs#L16>
                    enabled:    vec![session],
                    restricted: Some(vec![session]),
                    .. Default::default()
                },
            },
        ].into_iter()
    }
}

fn main() {
    let context = Context {
        // TODO: make desktop available to low/untrusted integrity processes (currently requires Medium integrity)
        _alt_desktop:   create_desktop_a(cstr!("max_sandbox_desktop"), (), None, 0, access::GENERIC_ALL, None).unwrap(),
    };
    for target in Target::list() {
        run(&context, target);
    }
}

struct Context {
    _alt_desktop:   desktop::OwnedHandle,
}

fn run(_context: &Context, target: Target) {
    assert!(target.spawn.integrity >= target.lockdown.integrity, "target.lockdown.integrity cannot be more permissive than spawn integrity");

    let sandbox_process_token = open_process_token(get_current_process(), token::ALL_ACCESS).unwrap();
    let gap = sandbox_process_token.groups_and_privileges().unwrap();
    let all_group_sids = gap.sids().iter().map(|g| g.sid).collect::<Vec<_>>();

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

    let mut command_line = abistr::CStrBuf::<u16, 32768>::from_truncate(&target.exe.as_os_str().encode_wide().collect::<Vec<_>>());

    let policy1 = 0u64
        | process::creation::mitigation_policy::DEP_ENABLE
        //| process::creation::mitigation_policy::DEP_ATL_THUNK_ENABLE
        | process::creation::mitigation_policy::SEHOP_ENABLE
        | process::creation::mitigation_policy::force_relocate_images::ALWAYS_ON_REQ_RELOCS // chrome.exe doesn't bother with this
        | process::creation::mitigation_policy::heap_terminate::ALWAYS_ON
        | process::creation::mitigation_policy::bottom_up_aslr::ALWAYS_ON
        | process::creation::mitigation_policy::high_entropy_aslr::ALWAYS_ON
        | process::creation::mitigation_policy::strict_handle_checks::ALWAYS_ON
        | if target.allow.same_desktop { 0 } else { process::creation::mitigation_policy::win32k_system_call_disable::ALWAYS_ON } // user32.dll(?) requires access on init
        | process::creation::mitigation_policy::extension_point_disable::ALWAYS_ON
        | process::creation::mitigation_policy::prohibit_dynamic_code::ALWAYS_ON
        | process::creation::mitigation_policy::control_flow_guard::ALWAYS_ON               // Redundant?
        | process::creation::mitigation_policy::control_flow_guard::EXPORT_SUPPRESSION      // https://docs.microsoft.com/en-us/windows/win32/secbp/pe-metadata#export-suppression
        | process::creation::mitigation_policy::block_non_microsoft_binaries::ALWAYS_ON     // Redundant?
        | process::creation::mitigation_policy::block_non_microsoft_binaries::ALLOW_STORE   // ?
        | if target.allow.same_desktop { 0 } else { process::creation::mitigation_policy::font_disable::ALWAYS_ON } // user32.dll(?) requires access on init
        | process::creation::mitigation_policy::image_load_no_remote::ALWAYS_ON
        | process::creation::mitigation_policy::image_load_no_low_label::ALWAYS_ON
        | process::creation::mitigation_policy::image_load_prefer_system32::ALWAYS_ON
        ;

    let policy2 = 0u64
        | process::creation::mitigation_policy2::loader_integrity_continuity::ALWAYS_ON
        //| process::creation::mitigation_policy2::strict_control_flow_guard::ALWAYS_ON         // causes ERROR_STRICT_CFG_VIOLATION, even if our executables are built with -Zbuild-std and -Ccontrol-flow-guard=checks
        | process::creation::mitigation_policy2::module_tampering_protection::ALWAYS_ON
        | process::creation::mitigation_policy2::restrict_indirect_branch_prediction::ALWAYS_ON
        | process::creation::mitigation_policy2::allow_downgrade_dynamic_code_policy::ALWAYS_OFF
        | process::creation::mitigation_policy2::speculative_store_bypass_disable::ALWAYS_ON
        | process::creation::mitigation_policy2::cet_user_shadow_stacks::ALWAYS_ON      // Redundant
        | process::creation::mitigation_policy2::cet_user_shadow_stacks::STRICT_MODE
        | process::creation::mitigation_policy2::user_cet_set_context_ip_validation::ALWAYS_ON
        //| process::creation::mitigation_policy2::block_non_cet_binaries::ALWAYS_ON            // causes ERROR_ACCESS_DENIED (are our executables not built with CET?)
        //| process::creation::mitigation_policy2::xtended_control_flow_guard::ALWAYS_ON        // causes ERROR_INVALID_PARAMETER - not built with XFG? see https://connormcgarr.github.io/examining-xfg/ / https://query.prod.cms.rt.microsoft.com/cms/api/am/binary/RE37dMC
        //| process::creation::mitigation_policy2::pointer_auth_user_ip::ALWAYS_ON              // causes ERROR_INVALID_PARAMETER - ARM64 (not x64/AMD64!) only?
        | process::creation::mitigation_policy2::cet_dynamic_apis_out_of_proc_only::ALWAYS_ON
        //| process::creation::mitigation_policy2::restrict_core_sharing::ALWAYS_ON             // causes ERROR_INVALID_PARAMETER (do I need to specify cores to hog?)
        ;

    let mitigation_policy = [policy1, policy2];
    let child_policy = process::creation::child_process::RESTRICTED;
    let dab_policy = process::creation::desktop_app_breakaway::ENABLE_PROCESS_TREE;
    let mut job = create_job();
    let job_list = [job.clone()];

    let attribute_list = process::ThreadAttributeList::try_from(&[
        process::ThreadAttributeRef::mitigation_policy_dword64_2(&mitigation_policy),
        process::ThreadAttributeRef::child_process_policy(&child_policy),
        process::ThreadAttributeRef::desktop_app_policy(&dab_policy),
        #[cfg(nope)] {
            // will cause create_process_as_user_w to fail with ERROR_INVALID_PARAMETER
            // Also completely pointless, as we're almost certainly not running as a protected app ourselves
            const PROTECTION_LEVEL_SAME : u32 = 0xFFFFFFFF;
            process::ThreadAttributeRef::protection_level(&PROTECTION_LEVEL_SAME)
        },
        process::ThreadAttributeRef::job_list(&job_list[..]),
        // TODO: ThreadAttributeRef::handle_list ?
        // TODO: ThreadAttributeRef::security_capabilities ? app container / capability sids related: https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-security_capabilities
    ][..]).unwrap();

    let mut si = process::StartupInfoExW::default();
    // N.B.: this will cause user32.dll(?) to STATUS_DLL_INIT_FAILED unless the child process can access the named desktop
    // specifying a nonexistant desktop is also an option
    si.startup_info.desktop = Some(cstr16!("max_sandbox_desktop")).filter(|_| !target.allow.same_desktop);
    si.startup_info.flags   = STARTF_UNTRUSTEDSOURCE;
    si.attribute_list       = Some(attribute_list);

    let pi = create_process_as_user_w(
        &restricted, (), Some(unsafe { command_line.buffer_mut() }), None, None, false,
        process::DEBUG_PROCESS | process::CREATE_SEPARATE_WOW_VDM | process::CREATE_SUSPENDED | process::EXTENDED_STARTUPINFO_PRESENT, process::environment::Clear, (), &si
    ).unwrap();
    set_thread_token(&pi.thread, &permissive).unwrap();
    relimit_job(&mut job, 0);
    resume_thread(&pi.thread).unwrap();

    let mut sandboxed = false;
    let mut threads = HashMap::<thread::Id, thread::OwnedHandle>::new();
    loop {
        let event = wait_for_debug_event_ex(None).unwrap();
        let DEBUG_EVENT { dwProcessId, dwThreadId, .. } = *event;
        let dbg_continue                = move || continue_debug_event(dwProcessId, dwThreadId, DBG_CONTINUE).unwrap();
        let dbg_exception_not_handled   = move || continue_debug_event(dwProcessId, dwThreadId, DBG_EXCEPTION_NOT_HANDLED).unwrap();
        use debug::DebugEventU::*;
        match event.u() {
            Exception(event) => {
                let code = event.ExceptionRecord.ExceptionCode;
                let ty = match code {
                    EXCEPTION_ACCESS_VIOLATION      => "EXCEPTION_ACCESS_VIOLATION",
                    EXCEPTION_BREAKPOINT            => "EXCEPTION_BREAKPOINT",
                    EXCEPTION_DATATYPE_MISALIGNMENT => "EXCEPTION_DATATYPE_MISALIGNMENT",
                    EXCEPTION_SINGLE_STEP           => "EXCEPTION_SINGLE_STEP",
                    DBG_CONTROL_C                   => "DBG_CONTROL_C",
                    // Ref: https://docs.microsoft.com/en-us/troubleshoot/developer/visualstudio/cpp/libraries/fatal-error-thread-exit-fls-callback
                    0xE06D7363                      => "Microsoft C++ Exception",
                    //0x8007045A                      => "ERROR_DLL_INIT_FAILED",
                    _                               => "???",
                };
                eprintln!("[{dwProcessId}:{dwThreadId}] exception: {ty} ({code})");
                dbg_exception_not_handled();
            },
            CreateThread(event) => {
                eprintln!("[{dwProcessId}:{dwThreadId}] thread created");
                let mut thread = event.hThread;

                let process = get_current_process().as_handle();
                assert!(FALSE != unsafe { DuplicateHandle(process, thread, process, &mut thread, access::GENERIC_ALL.into(), false as _, 0) });
                let thread = unsafe { thread::OwnedHandle::clone_from_raw(thread) };

                set_thread_token(&thread, &permissive).unwrap();
                let _prev_thread = threads.insert(dwThreadId, thread);
                debug_assert!(_prev_thread.is_none());
                dbg_continue();
            },
            CreateProcess(event) => {
                eprintln!("[{dwProcessId}:{dwThreadId}] process created");
                let mut thread = event.hThread;

                let process = get_current_process().as_handle();
                assert!(FALSE != unsafe { DuplicateHandle(process, thread, process, &mut thread, access::GENERIC_ALL.into(), false as _, 0) });
                let thread = unsafe { thread::OwnedHandle::clone_from_raw(thread) };

                set_thread_token(&thread, &permissive).unwrap(); // already set?
                let _prev_thread = threads.insert(dwThreadId, thread);
                debug_assert!(_prev_thread.is_none());
                dbg_continue();
            },
            ExitThread(event) => {
                eprintln!("[{dwProcessId}:{dwThreadId}] thread exited with code: {:?}", Error::from(event.dwExitCode));
                let _thread = threads.remove(&dwThreadId);
                debug_assert!(_thread.is_some());
                dbg_continue();
            },
            ExitProcess(event) => {
                eprintln!("[{dwProcessId}:{dwThreadId}] process exited with code: {:?}", Error::from(event.dwExitCode));
                let _thread = threads.remove(&dwThreadId);
                debug_assert!(_thread.is_some());
                dbg_continue();
                break;
            },
            LoadDll(_event) => {
                eprintln!("[{dwProcessId}:{dwThreadId}] dll loaded");
                dbg_continue();
            },
            UnloadDll(_event)  => {
                eprintln!("[{dwProcessId}:{dwThreadId}] dll unloaded");
                dbg_continue();
            },
            DebugString(event) => {
                let bytes = usize::from(event.nDebugStringLength);
                let mut buffer_wide;
                let mut buffer_narrow;
                let buffer_osstring;
                let narrow = if event.fUnicode != 0 {
                    // Unicode
                    buffer_wide = vec![MaybeUninit::<u16>::uninit(); (bytes+1)/2];
                    let buffer = read_process_memory(&pi.process, event.lpDebugStringData.cast(), &mut buffer_wide[..]).unwrap();
                    let nul = buffer.iter().position(|ch| *ch == 0).unwrap_or(buffer.len());
                    buffer_osstring = OsString::from_wide(buffer.split_at(nul).0);
                    buffer_osstring.to_string_lossy()
                } else {
                    buffer_narrow = vec![MaybeUninit::<u8>::uninit(); bytes];
                    let buffer = read_process_memory(&pi.process, event.lpDebugStringData.cast(), &mut buffer_narrow[..]).unwrap();
                    let nul = buffer.iter().position(|ch| *ch == 0).unwrap_or(buffer.len());
                    String::from_utf8_lossy(buffer.split_at(nul).0)
                };
                eprintln!("[{dwProcessId}:{dwThreadId}] debug string: {:?}", &*narrow);
                if narrow == "sandbox" {
                    for thread in threads.values() { suspend_thread(thread).unwrap(); }
                    debug_active_process_stop(pi.process_id).unwrap();
                    // XXX: This seems to cause the child process to die with 101 / ERROR_EXCL_SEM_ALREADY_OWNED ?
                    //open_process_token(&pi.process, token::ADJUST_DEFAULT).unwrap().set_integrity_level(sid::AndAttributes::new(target.lockdown.integrity.sid(), 0)).unwrap();
                    for thread in threads.values() { set_thread_token(thread, None).unwrap(); }
                    for thread in threads.values() { resume_thread(thread).unwrap(); }
                    threads.clear();
                    sandboxed = true;
                    eprintln!("[{dwProcessId}:{dwThreadId}] sandboxed");
                    break;
                } else {
                    dbg_continue();
                }
            },
            Rip(_event) => {
                eprintln!("[{dwProcessId}:{dwThreadId}] rip event: {{ dwError: {}, dwType: {} }}", _event.dwError, _event.dwType);
                dbg_continue();
            },
            _ => {},
        }
    }

    assert!(sandboxed, "process was never sandboxed");

    let exit = wait_for_process(pi.process).unwrap();
    assert!(exit == 0, "exit code: 0x{exit:08x} {}", Error::from(exit).friendly());
}

fn create_job() -> job::OwnedHandle {
    let mut job = create_job_object_a(None, ()).unwrap();
    relimit_job(&mut job, 1);

    // TODO: consider UserHandleGrantAccess to... do what, exactly?
    // https://docs.microsoft.com/en-us/windows/win32/api/jobapi2/nf-jobapi2-setinformationjobobject
    set_information_job_object(&mut job, JOBOBJECT_BASIC_UI_RESTRICTIONS { UIRestrictionsClass: 0
        | JOB_OBJECT_UILIMIT_DESKTOP            // Prevents processes associated with the job from creating desktops and switching desktops using the CreateDesktop and SwitchDesktop functions.
        | JOB_OBJECT_UILIMIT_DISPLAYSETTINGS    // Prevents processes associated with the job from calling the ChangeDisplaySettings function.
        | JOB_OBJECT_UILIMIT_EXITWINDOWS        // Prevents processes associated with the job from calling the ExitWindows or ExitWindowsEx function.
        | JOB_OBJECT_UILIMIT_GLOBALATOMS        // Prevents processes associated with the job from accessing global atoms. When this flag is used, each job has its own atom table.
        | JOB_OBJECT_UILIMIT_HANDLES            // Prevents processes associated with the job from using USER handles owned by processes not associated with the same job.
        | JOB_OBJECT_UILIMIT_READCLIPBOARD      // Prevents processes associated with the job from reading data from the clipboard.
        | JOB_OBJECT_UILIMIT_SYSTEMPARAMETERS   // Prevents processes associated with the job from changing system parameters by using the SystemParametersInfo function.
        | JOB_OBJECT_UILIMIT_WRITECLIPBOARD     // Prevents processes associated with the job from writing data to the clipboard.
    }).unwrap();
    #[cfg(nope)] // TODO: the pointers in this type would require set_information_job_object to be `unsafe`, replace with a safer type
    set_information_job_object(&mut job, JOBOBJECT_SECURITY_LIMIT_INFORMATION {
        SecurityLimitFlags: 0
            | JOB_OBJECT_SECURITY_NO_ADMIN          // Prevents any process in the job from using a token that specifies the local administrators group.
            | JOB_OBJECT_SECURITY_RESTRICTED_TOKEN  // Prevents any process in the job from using a token that was not created with the CreateRestrictedToken function.
            // | JOB_OBJECT_SECURITY_FILTER_TOKENS    // Applies a filter to the token when a process impersonates a client. Requires at least one of the following members to be set: SidsToDisable, PrivilegesToDelete, or RestrictedSids.
            // | JOB_OBJECT_SECURITY_ONLY_TOKEN       // I don't have JobToken set - and would this prevent SetThreadToken ?
            ,
        .. unsafe { zeroed() }
    }).unwrap();
    // TODO: JOBOBJECT_END_OF_JOB_TIME_INFORMATION to hard-terminate the processes of the job?
    // TODO: JobObjectGroupInformation processor groups?
    // TODO: JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2 limits?
    // TODO: JOBOBJECT_NET_RATE_CONTROL_INFORMATION to disable network?
    // TODO: JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION[_2] ?
    // TODO: JOBOBJECT_LIMIT_VIOLATION_INFORMATION ?
    // TODO: SetIoRateControlInformationJobObject ?
    job
}

fn relimit_job(job: &mut job::OwnedHandle, processes: u32) {
    set_information_job_object(job, JOBOBJECT_EXTENDED_LIMIT_INFORMATION {
        BasicLimitInformation: JOBOBJECT_BASIC_LIMIT_INFORMATION {
            LimitFlags: 0
                | JOB_OBJECT_LIMIT_ACTIVE_PROCESS
                | JOB_OBJECT_LIMIT_DIE_ON_UNHANDLED_EXCEPTION
                | JOB_OBJECT_LIMIT_JOB_MEMORY
                | JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE
                // | JOB_OBJECT_LIMIT_JOB_TIME // ?
                ,
            ActiveProcessLimit: processes,
            //PerJobUserTimeLimit: ..., // ?
            .. unsafe { zeroed() }
        },
        JobMemoryLimit: 4 * 1024*1024*1024, // 4 GiB
        .. unsafe { zeroed() }
    }).unwrap();
}
