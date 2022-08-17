use win32_security_playground::*;

use abistr::cstr;

use winapi::shared::minwindef::FALSE;
use winapi::um::handleapi::DuplicateHandle;
use winapi::um::minwinbase::*;
use winapi::um::processthreadsapi::STARTUPINFOW;
use winapi::um::winbase::*;
use winapi::um::winnt::*;

use std::collections::*;
use std::ffi::OsString;
use std::mem::MaybeUninit;
use std::mem::zeroed;
use std::os::windows::prelude::*;
use std::path::PathBuf;
use std::ptr::null_mut;

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
        let session     = Box::leak(Box::new(sandbox_process_token.logon_sid().unwrap())).groups().iter().next().unwrap().sid;
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
    let mut si = STARTUPINFOW { lpDesktop: null_mut(), dwFlags: STARTF_UNTRUSTEDSOURCE, .. unsafe { zeroed() } };
    si.cb = std::mem::size_of_val(&si) as u32;
    let pi = unsafe { create_process_as_user_w(&restricted, (), Some(command_line.buffer_mut()), None, None, false, DEBUG_PROCESS | CREATE_SEPARATE_WOW_VDM | CREATE_SUSPENDED, None, (), &si)}.unwrap();
    set_thread_token(&pi.thread, &permissive).unwrap();
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
                assert!(FALSE != unsafe { DuplicateHandle(process, thread, process, &mut thread, GENERIC_ALL, false as _, 0) });
                let thread = unsafe { thread::OwnedHandle::clone_from_raw(thread) };

                set_thread_token(&thread, &permissive).unwrap();
                let _prev_thread = threads.insert(dwThreadId, thread);
                debug_assert!(_prev_thread.is_none());
                dbg_continue();
            },
            CreateProcess(_event) => {
                eprintln!("[{dwProcessId}:{dwThreadId}] process created");
                dbg_continue();
            },
            ExitThread(_event) => {
                eprintln!("[{dwProcessId}:{dwThreadId}] thread exited with code: {}", _event.dwExitCode);
                let _thread = threads.remove(&dwThreadId);
                debug_assert!(_thread.is_some());
                dbg_continue();
            },
            ExitProcess(_event) => {
                eprintln!("[{dwProcessId}:{dwThreadId}] process exited with code: {}", _event.dwExitCode);
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
    assert!(exit == 0, "exit code: 0x{exit:08x} {}", LastError::from(exit).friendly());
}
