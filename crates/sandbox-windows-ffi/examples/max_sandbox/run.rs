use crate::{*, job};
use sandbox_windows_ffi::*;
use abistr::*;
use winapi::um::winbase::*;
use std::os::windows::prelude::*;



pub fn all() {
    // TODO: make desktop available to low/untrusted integrity processes (currently requires Medium integrity)
    let _alt_desktop = create_desktop_a(cstr!("max_sandbox_desktop"), (), None, 0, access::GENERIC_ALL, None).unwrap();
    for target in settings::Target::list() { one(target) }
}

pub fn one(target: settings::Target) {
    assert!(target.spawn.integrity >= target.lockdown.integrity, "target.lockdown.integrity cannot be more permissive than spawn integrity");

    let tokens = tokens::create(&target);
    let mut command_line = abistr::CStrBuf::<u16, 32768>::from_truncate(&target.exe.as_os_str().encode_wide().collect::<Vec<_>>());

    let (_read, write) = io::create_pipe(Some(&security::Attributes::new(None, true)), 0).unwrap();
    let job = job::create();
    let attribute_list = attribute::List::new(&target, &job, vec![(&write).into()]);
    let mut si = process::StartupInfoExW::default();
    // N.B.: this will cause user32.dll(?) to STATUS_DLL_INIT_FAILED unless the child process can access the named desktop
    // specifying a nonexistant desktop is also an option
    si.startup_info.desktop     = Some(cstr16!("max_sandbox_desktop")).filter(|_| !target.allow.same_desktop);
    si.startup_info.flags       = STARTF_UNTRUSTEDSOURCE | STARTF_USESTDHANDLES;
    si.startup_info.std_output  = Some((&write).into());
    si.startup_info.std_error   = Some((&write).into());
    si.attribute_list           = Some(attribute_list.to_list());

    let environment = format!(
        concat!(
            "WRITE_HANDLE={write}\0",
            "READ_HANDLE_NOINHERIT={read}\0",
            "\0"
        ),
        write = write.as_handle_nn().as_ptr() as usize,
        read  = _read.as_handle_nn().as_ptr() as usize,
    );

    let pi = create_process_as_user_w(
        &tokens.restricted, (), Some(unsafe { command_line.buffer_mut() }), None, None, true,
        process::DEBUG_PROCESS | process::CREATE_SEPARATE_WOW_VDM | process::CREATE_SUSPENDED | process::EXTENDED_STARTUPINFO_PRESENT,
        environment.as_bytes(), (), &si
    ).unwrap();
    set_thread_token(&pi.thread, &tokens.permissive).unwrap();
    job::relimit(&job, 0);
    resume_thread(&pi.thread).unwrap();

    #[cfg(std)] let conio = {
        use std::io::{BufReader, BufRead};
        let thread_id  = pi.thread_id;
        let process_id = pi.process_id;
        std::thread::spawn(move || -> io::Result<()> {
            let read = BufReader::new(_read);
            for line in read.lines() {
                println!("[{process_id}:{thread_id}] console i/o: {}", line?); // guessing at the process/thread here TBH
            }
            Ok(())
        })
    };

    debugger::debug_loop(&tokens, &pi);

    let exit = wait_for_process(&pi.process).unwrap();
    drop(si);
    drop(write); // break pipe
    #[cfg(std)] match conio.join().unwrap() {
        Err(err) if err.kind() == io::ErrorKind::BrokenPipe => {},
        other => other.unwrap(),
    }
    assert!(exit == 0, "exit code: 0x{exit:08x} {}", Error::from(exit).friendly());
}
