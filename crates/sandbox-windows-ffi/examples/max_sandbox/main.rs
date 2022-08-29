mod attribute;
mod job;
mod settings;

use sandbox_windows_ffi::*;

use abistr::*;

use winapi::shared::minwindef::FALSE;
use winapi::um::handleapi::DuplicateHandle;
use winapi::um::minwinbase::*;
use winapi::um::winbase::*;
use winapi::um::winnt::*;

use std::collections::*;
use std::ffi::OsString;
use std::mem::MaybeUninit;
use std::os::windows::prelude::*;



fn main() {
    let context = Context {
        // TODO: make desktop available to low/untrusted integrity processes (currently requires Medium integrity)
        _alt_desktop:   create_desktop_a(cstr!("max_sandbox_desktop"), (), None, 0, access::GENERIC_ALL, None).unwrap(),
    };
    for target in settings::Target::list() {
        run(&context, target);
    }
}

struct Context {
    _alt_desktop:   desktop::OwnedHandle,
}

fn run(_context: &Context, target: settings::Target) {
    assert!(target.spawn.integrity >= target.lockdown.integrity, "target.lockdown.integrity cannot be more permissive than spawn integrity");

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

    let mut command_line = abistr::CStrBuf::<u16, 32768>::from_truncate(&target.exe.as_os_str().encode_wide().collect::<Vec<_>>());

    let job = job::create();
    let attribute_list = attribute::List::new(&target, &job);

    let mut si = process::StartupInfoExW::default();
    // N.B.: this will cause user32.dll(?) to STATUS_DLL_INIT_FAILED unless the child process can access the named desktop
    // specifying a nonexistant desktop is also an option
    si.startup_info.desktop = Some(cstr16!("max_sandbox_desktop")).filter(|_| !target.allow.same_desktop);
    si.startup_info.flags   = STARTF_UNTRUSTEDSOURCE;
    si.attribute_list       = Some(attribute_list.to_list());

    let pi = create_process_as_user_w(
        &restricted, (), Some(unsafe { command_line.buffer_mut() }), None, None, false,
        process::DEBUG_PROCESS | process::CREATE_SEPARATE_WOW_VDM | process::CREATE_SUSPENDED | process::EXTENDED_STARTUPINFO_PRESENT, process::environment::Clear, (), &si
    ).unwrap();
    set_thread_token(&pi.thread, &permissive).unwrap();
    job::relimit(&job, 0);
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
                let thread = unsafe { thread::OwnedHandle::from_raw(thread) }.unwrap();

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
                let thread = unsafe { thread::OwnedHandle::from_raw(thread) }.unwrap();

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

    let exit = wait_for_process(&pi.process).unwrap();
    assert!(exit == 0, "exit code: 0x{exit:08x} {}", Error::from(exit).friendly());
}
