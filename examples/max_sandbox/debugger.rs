use firehazard::*;

use winapi::ctypes::c_void;
use winapi::um::minwinbase::*;
use winapi::um::winnt::*;

use std::collections::*;
use std::mem::MaybeUninit;
use std::path::PathBuf;



pub fn debug_loop(
    tokens:     &crate::tokens::Tokens,
    pi:         &process::Information,
) {
    let mut sandboxed = false;
    let mut threads = HashMap::<thread::Id, thread::OwnedHandle>::new();
    let mut dlls    = HashMap::<*mut c_void, PathBuf>::new();
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
                    // Ref: https://learn.microsoft.com/en-us/troubleshoot/developer/visualstudio/cpp/libraries/fatal-error-thread-exit-fls-callback
                    0xE06D7363                      => "Microsoft C++ Exception",
                    //0x8007045A                      => "ERROR_DLL_INIT_FAILED",
                    _                               => "???",
                };
                println!("[{dwProcessId}:{dwThreadId}] exception: {ty} ({code})");
                dbg_exception_not_handled();
            },
            CreateThread(event) => {
                println!("[{dwProcessId}:{dwThreadId}] thread created");
                let thread = unsafe { thread::Handle::from_raw(event.hThread) }.unwrap();
                let thread = duplicate_handle_local(thread, access::GENERIC_ALL, false).unwrap(); // `thread` does not start with enough permission for `set_thread_token`
                set_thread_token(&thread, &tokens.permissive).unwrap();
                let _prev_thread = threads.insert(dwThreadId, thread);
                debug_assert!(_prev_thread.is_none());
                dbg_continue();
            },
            CreateProcess(event) => {
                println!("[{dwProcessId}:{dwThreadId}] process created");
                let thread = unsafe { thread::Handle::from_raw(event.hThread) }.unwrap();
                let thread = duplicate_handle_local(thread, access::GENERIC_ALL, false).unwrap(); // `thread` does not start with enough permission for `set_thread_token`
                set_thread_token(&thread, &tokens.permissive).unwrap(); // already set?
                let _prev_thread = threads.insert(dwThreadId, thread);
                debug_assert!(_prev_thread.is_none());
                dbg_continue();
            },
            ExitThread(event) => {
                println!("[{dwProcessId}:{dwThreadId}] thread exited with code: {:?}", Error::from(event.dwExitCode));
                let _thread = threads.remove(&dwThreadId);
                debug_assert!(_thread.is_some());
                dbg_continue();
            },
            ExitProcess(event) => {
                println!("[{dwProcessId}:{dwThreadId}] process exited with code: {:?}", Error::from(event.dwExitCode));
                let _thread = threads.remove(&dwThreadId);
                debug_assert!(_thread.is_some());
                dbg_continue();
                break;
            },
            LoadDll(event) => {
                // XXX: May fail with ERROR_INVALID_HANDLE if the debugeee's spawn token lacks `USERS` (DLL failed to open?)
                let hfile = unsafe { io::sync::BorrowedFile::from_raw(event.hFile) }.ok();
                // XXX: May fail with ERROR_ACCESS_DENIED if the debugee's restricted token lacks `SE_CHANGE_NOTIFY_NAME`
                let image_name = hfile.and_then(|hfile| get_final_path_name_by_handle(&hfile, 0).ok()).unwrap_or_else(|| PathBuf::from("???"));

                println!("[{dwProcessId}:{dwThreadId}] dll loaded: {image_name:?}");
                let _prev_name = dlls.insert(event.lpBaseOfDll, image_name);
                dbg_continue();
            },
            UnloadDll(event)  => {
                let image_name = dlls.remove(&event.lpBaseOfDll).unwrap_or_default();
                println!("[{dwProcessId}:{dwThreadId}] dll unloaded: {image_name:?}");
                dbg_continue();
            },
            DebugString(event) => {
                let bytes = usize::from(event.nDebugStringLength);
                let narrow = if event.fUnicode != 0 {
                    // Unicode
                    let mut buffer = vec![MaybeUninit::<u16>::uninit(); (bytes+1)/2];
                    let buffer = unsafe { read_process_memory(&pi.process, event.lpDebugStringData.cast(), &mut buffer[..]) }.unwrap();
                    let nul = buffer.iter().position(|ch| *ch == 0).unwrap_or(buffer.len());
                    String::from_utf16_lossy(buffer.split_at(nul).0)
                } else {
                    let mut buffer = vec![MaybeUninit::<u8>::uninit(); bytes];
                    let buffer = unsafe { read_process_memory(&pi.process, event.lpDebugStringData.cast(), &mut buffer[..]) }.unwrap();
                    let nul = buffer.iter().position(|ch| *ch == 0).unwrap_or(buffer.len());
                    String::from_utf8_lossy(buffer.split_at(nul).0).into_owned()
                };
                println!("[{dwProcessId}:{dwThreadId}] debug string: {:?}", narrow);
                if narrow == "sandbox" {
                    for thread in threads.values() { unsafe { suspend_thread(thread) }.unwrap(); }
                    debug_active_process_stop(pi.process_id).unwrap();
                    // XXX: This seems to cause the child process to die with 101 / ERROR_EXCL_SEM_ALREADY_OWNED ?
                    //open_process_token(&pi.process, token::ADJUST_DEFAULT).unwrap().set_integrity_level(sid::AndAttributes::new(target.lockdown.integrity.sid(), None)).unwrap();
                    for thread in threads.values() { set_thread_token(thread, None).unwrap(); }
                    for thread in threads.values() { unsafe { resume_thread(thread) }.unwrap(); }
                    threads.clear();
                    sandboxed = true;
                    println!("[{dwProcessId}:{dwThreadId}] sandboxed");
                    break;
                } else {
                    dbg_continue();
                }
            },
            Rip(_event) => {
                println!("[{dwProcessId}:{dwThreadId}] rip event: {{ dwError: {}, dwType: {} }}", _event.dwError, _event.dwType);
                dbg_continue();
            },
            _ => {},
        }
    }

    assert!(sandboxed, "process was never sandboxed");
}
