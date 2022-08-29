use sandbox_windows_ffi::*;

use winapi::shared::minwindef::FALSE;
use winapi::um::handleapi::DuplicateHandle;
use winapi::um::minwinbase::*;
use winapi::um::winnt::*;

use std::collections::*;
use std::ffi::OsString;
use std::mem::MaybeUninit;
use std::os::windows::prelude::*;



pub fn debug_loop(
    tokens:     &crate::tokens::Tokens,
    pi:         &process::Information,
) {
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

                set_thread_token(&thread, &tokens.permissive).unwrap();
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

                set_thread_token(&thread, &tokens.permissive).unwrap(); // already set?
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
}
