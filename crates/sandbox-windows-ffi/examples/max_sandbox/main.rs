mod attribute;
mod debugger;
mod job;
mod settings;
mod tokens;

use sandbox_windows_ffi::*;

use abistr::*;

use winapi::um::winbase::*;

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

    let tokens = tokens::create(&target);
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
        &tokens.restricted, (), Some(unsafe { command_line.buffer_mut() }), None, None, false,
        process::DEBUG_PROCESS | process::CREATE_SEPARATE_WOW_VDM | process::CREATE_SUSPENDED | process::EXTENDED_STARTUPINFO_PRESENT, process::environment::Clear, (), &si
    ).unwrap();
    set_thread_token(&pi.thread, &tokens.permissive).unwrap();
    job::relimit(&job, 0);
    resume_thread(&pi.thread).unwrap();

    debugger::debug_loop(&tokens, &pi);

    let exit = wait_for_process(&pi.process).unwrap();
    assert!(exit == 0, "exit code: 0x{exit:08x} {}", Error::from(exit).friendly());
}
