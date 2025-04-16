use crate::{*, job};
use firehazard::*;
use abistr::*;
use winapi::shared::winerror::ERROR_INVALID_PARAMETER;
use winapi::um::winbase::*;



pub fn all() {
    // TODO: make desktop available to low/untrusted integrity processes (currently requires Medium integrity)
    let _alt_desktop = create_desktop_a(cstr!("max_sandbox_desktop"), (), None, None, access::GENERIC_ALL, None).unwrap();
    for target in settings::Target::list() {
        if std::env::var_os("CI").is_some() {
            dbg!(&target);
        } else {
            println!("sandboxing {}", target.exe.display());
        }
        one(target);
    }
}

pub fn one(target: settings::Target) {
    assert!(target.spawn.integrity >= target.lockdown.integrity, "target.lockdown.integrity cannot be more permissive than spawn integrity");

    let tokens = tokens::create(&target);
    let exe = &target.exe;
    let mut command_line = exe_to_command_line_0(exe);

    let (_read, write) = pipe::create(Some(&security::Attributes::new(None, true)), 0).unwrap();
    let job = job::create();
    let original_attribute_list = attribute::List::new(&target, &job, vec![(&write).into()]);
    let mut attribute_list = original_attribute_list.clone();

    let environment = format!(
        concat!(
            "WRITE_HANDLE={write}\0",
            "READ_HANDLE_NOINHERIT={read}\0",
            "\0"
        ),
        write = write.as_handle_nn().as_ptr() as usize,
        read  = _read.as_handle_nn().as_ptr() as usize,
    );

    let mut modifications = [
        ("", (|_original, _modified| {}) as for<'a> fn(_: &attribute::List<'a>, _: &mut attribute::List<'a>)), // typing

        // N.B. there's a weird sandwich of versions:
        // github runner    → cmd /C ver
        // windows-2019     → 10.0.17763.7009
        // windows-2022     → 10.0.20348.3328       yes, higher than windows-2025
        // windows-2025     → 10.0.17763.7009
        // (local desktop)  → 10.0.19045.5737

        // Windows 10.0.19045.5737 needs none of these modifications... possibly because some are already disabled in `attribute::List::new(...)`
        // Windows 10.0.20348.3328 needs none of these modifications... possibly because some are already disabled in `attribute::List::new(...)`
        ("removing process::creation::mitigation_policy2::restrict_core_sharing::*",                    |_, list| { list.mitigation_policy.as_mut().map(|p| *p = process::creation::MitigationPolicy::from((p.flag1(), p.flag2() & !process::creation::mitigation_policy2::restrict_core_sharing                   ::MASK))); }), // Win 10+
        ("removing process::creation::mitigation_policy2::cet_dynamic_apis_out_of_proc_only::*",        |_, list| { list.mitigation_policy.as_mut().map(|p| *p = process::creation::MitigationPolicy::from((p.flag1(), p.flag2() & !process::creation::mitigation_policy2::cet_dynamic_apis_out_of_proc_only       ::MASK))); }), // Win 10+
        ("removing process::creation::mitigation_policy2::pointer_auth_user_ip::*",                     |_, list| { list.mitigation_policy.as_mut().map(|p| *p = process::creation::MitigationPolicy::from((p.flag1(), p.flag2() & !process::creation::mitigation_policy2::pointer_auth_user_ip                    ::MASK))); }), // Win 10+, ARM64
        ("removing process::creation::mitigation_policy2::xtended_control_flow_guard::*",               |_, list| { list.mitigation_policy.as_mut().map(|p| *p = process::creation::MitigationPolicy::from((p.flag1(), p.flag2() & !process::creation::mitigation_policy2::xtended_control_flow_guard              ::MASK))); }), // Win 10+
        ("removing process::creation::mitigation_policy2::block_non_cet_binaries::*",                   |_, list| { list.mitigation_policy.as_mut().map(|p| *p = process::creation::MitigationPolicy::from((p.flag1(), p.flag2() & !process::creation::mitigation_policy2::block_non_cet_binaries                  ::MASK))); }), // Win 10+
        ("removing process::creation::mitigation_policy2::user_cet_set_context_ip_validation::*",       |_, list| { list.mitigation_policy.as_mut().map(|p| *p = process::creation::MitigationPolicy::from((p.flag1(), p.flag2() & !process::creation::mitigation_policy2::user_cet_set_context_ip_validation      ::MASK))); }), // Win 10+
        ("removing process::creation::mitigation_policy2::cet_user_shadow_stacks::*",                   |_, list| { list.mitigation_policy.as_mut().map(|p| *p = process::creation::MitigationPolicy::from((p.flag1(), p.flag2() & !process::creation::mitigation_policy2::cet_user_shadow_stacks                  ::MASK))); }), // Win 10+

        // Windows 10.0.17763.7009 needs at least some of the above modifications, but none of those that follow
        ("removing process::creation::mitigation_policy2::speculative_store_bypass_disable::*",         |_, list| { list.mitigation_policy.as_mut().map(|p| *p = process::creation::MitigationPolicy::from((p.flag1(), p.flag2() & !process::creation::mitigation_policy2::speculative_store_bypass_disable        ::MASK))); }), // Win 10+
        ("removing process::creation::mitigation_policy2::allow_downgrade_dynamic_code_policy::*",      |_, list| { list.mitigation_policy.as_mut().map(|p| *p = process::creation::MitigationPolicy::from((p.flag1(), p.flag2() & !process::creation::mitigation_policy2::allow_downgrade_dynamic_code_policy     ::MASK))); }), // Win 10+
        ("removing process::creation::mitigation_policy2::restrict_indirect_branch_prediction::*",      |_, list| { list.mitigation_policy.as_mut().map(|p| *p = process::creation::MitigationPolicy::from((p.flag1(), p.flag2() & !process::creation::mitigation_policy2::restrict_indirect_branch_prediction     ::MASK))); }), // Win 10+
        ("removing process::creation::mitigation_policy2::module_tampering_protection::*",              |_, list| { list.mitigation_policy.as_mut().map(|p| *p = process::creation::MitigationPolicy::from((p.flag1(), p.flag2() & !process::creation::mitigation_policy2::module_tampering_protection             ::MASK))); }), // Win 10+
        ("removing process::creation::mitigation_policy2::strict_control_flow_guard::*",                |_, list| { list.mitigation_policy.as_mut().map(|p| *p = process::creation::MitigationPolicy::from((p.flag1(), p.flag2() & !process::creation::mitigation_policy2::strict_control_flow_guard               ::MASK))); }), // Win 10+
        ("removing process::creation::mitigation_policy2::loader_integrity_continuity::*",              |_, list| { list.mitigation_policy.as_mut().map(|p| *p = process::creation::MitigationPolicy::from((p.flag1(), p.flag2() & !process::creation::mitigation_policy2::loader_integrity_continuity             ::MASK))); }), // Win 10+

        ("removing process::creation::mitigation_policy::image_load_prefer_system32::*",                |_, list| { list.mitigation_policy.as_mut().map(|p| *p = process::creation::MitigationPolicy::from((p.flag1() & !process::creation::mitigation_policy::image_load_prefer_system32              ::MASK, p.flag2()))); }), // Win 10+
        ("removing process::creation::mitigation_policy::image_load_no_low_label::*",                   |_, list| { list.mitigation_policy.as_mut().map(|p| *p = process::creation::MitigationPolicy::from((p.flag1() & !process::creation::mitigation_policy::image_load_no_low_label                 ::MASK, p.flag2()))); }), // Win 10+
        ("removing process::creation::mitigation_policy::image_load_no_remote::*",                      |_, list| { list.mitigation_policy.as_mut().map(|p| *p = process::creation::MitigationPolicy::from((p.flag1() & !process::creation::mitigation_policy::image_load_no_remote                    ::MASK, p.flag2()))); }), // Win 10+
        ("removing process::creation::mitigation_policy::font_disable::*",                              |_, list| { list.mitigation_policy.as_mut().map(|p| *p = process::creation::MitigationPolicy::from((p.flag1() & !process::creation::mitigation_policy::font_disable                            ::MASK, p.flag2()))); }), // Win 10+
        ("removing process::creation::mitigation_policy::block_non_microsoft_binaries::*",              |_, list| { list.mitigation_policy.as_mut().map(|p| *p = process::creation::MitigationPolicy::from((p.flag1() & !process::creation::mitigation_policy::block_non_microsoft_binaries            ::MASK, p.flag2()))); }), // Win 8.1+
        ("removing process::creation::mitigation_policy::control_flow_guard::*",                        |_, list| { list.mitigation_policy.as_mut().map(|p| *p = process::creation::MitigationPolicy::from((p.flag1() & !process::creation::mitigation_policy::control_flow_guard                      ::MASK, p.flag2()))); }), // Win 8.1+
        ("removing process::creation::mitigation_policy::prohibit_dynamic_code::*",                     |_, list| { list.mitigation_policy.as_mut().map(|p| *p = process::creation::MitigationPolicy::from((p.flag1() & !process::creation::mitigation_policy::prohibit_dynamic_code                   ::MASK, p.flag2()))); }), // Win 8.1+
        ("removing process::creation::mitigation_policy::extension_point_disable::*",                   |_, list| { list.mitigation_policy.as_mut().map(|p| *p = process::creation::MitigationPolicy::from((p.flag1() & !process::creation::mitigation_policy::extension_point_disable                 ::MASK, p.flag2()))); }), // Win 8+
        ("removing process::creation::mitigation_policy::win32k_system_call_disable::*",                |_, list| { list.mitigation_policy.as_mut().map(|p| *p = process::creation::MitigationPolicy::from((p.flag1() & !process::creation::mitigation_policy::win32k_system_call_disable              ::MASK, p.flag2()))); }), // Win 8+
        ("removing process::creation::mitigation_policy::strict_handle_checks::*",                      |_, list| { list.mitigation_policy.as_mut().map(|p| *p = process::creation::MitigationPolicy::from((p.flag1() & !process::creation::mitigation_policy::strict_handle_checks                    ::MASK, p.flag2()))); }), // Win 8+
        ("removing process::creation::mitigation_policy::high_entropy_aslr::*",                         |_, list| { list.mitigation_policy.as_mut().map(|p| *p = process::creation::MitigationPolicy::from((p.flag1() & !process::creation::mitigation_policy::high_entropy_aslr                       ::MASK, p.flag2()))); }), // Win 8+
        ("removing process::creation::mitigation_policy::bottom_up_aslr::*",                            |_, list| { list.mitigation_policy.as_mut().map(|p| *p = process::creation::MitigationPolicy::from((p.flag1() & !process::creation::mitigation_policy::bottom_up_aslr                          ::MASK, p.flag2()))); }), // Win 8+
        ("removing process::creation::mitigation_policy::heap_terminate::*",                            |_, list| { list.mitigation_policy.as_mut().map(|p| *p = process::creation::MitigationPolicy::from((p.flag1() & !process::creation::mitigation_policy::heap_terminate                          ::MASK, p.flag2()))); }), // Win 8+
        ("removing process::creation::mitigation_policy::force_relocate_images::*",                     |_, list| { list.mitigation_policy.as_mut().map(|p| *p = process::creation::MitigationPolicy::from((p.flag1() & !process::creation::mitigation_policy::force_relocate_images                   ::MASK, p.flag2()))); }), // Win 8+

        ("removing process::creation::mitigation_policy::legacy::SEHOP_ENABLE",                         |_, list| { list.mitigation_policy.as_mut().map(|p| *p = process::creation::MitigationPolicy::from((p.flag1() & !process::creation::mitigation_policy::legacy::SEHOP_ENABLE,                           p.flag2()))); }), // Win 7+
        ("removing process::creation::mitigation_policy::legacy::DEP_ATL_THUNK_ENABLE",                 |_, list| { list.mitigation_policy.as_mut().map(|p| *p = process::creation::MitigationPolicy::from((p.flag1() & !process::creation::mitigation_policy::legacy::DEP_ATL_THUNK_ENABLE,                   p.flag2()))); }), // Win 7+
        ("removing process::creation::mitigation_policy::legacy::DEP_ENABLE",                           |_, list| { list.mitigation_policy.as_mut().map(|p| *p = process::creation::MitigationPolicy::from((p.flag1() & !process::creation::mitigation_policy::legacy::DEP_ENABLE,                             p.flag2()))); }), // Win 7+

        ("removing process::creation::mitigation_policy2::*",                                           |o, list| { *list = o.clone(); list.mitigation_policy.as_mut().map(|p| *p = process::creation::MitigationPolicy::from((true  * p.flag1(), false * p.flag2()))); }),
        ("removing process::creation::mitigation_policy::*",                                            |o, list| { *list = o.clone(); list.mitigation_policy.as_mut().map(|p| *p = process::creation::MitigationPolicy::from((false * p.flag1(), true  * p.flag2()))); }),
        ("removing the mitigation policy outright",                                                     |o, list| *list = attribute::List { mitigation_policy:  None, .. o.clone() }),
        ("removing the child policy",                                                                   |o, list| *list = attribute::List { child_policy:       None, .. o.clone() }),
        ("removing the dab policy",                                                                     |o, list| *list = attribute::List { dab_policy:         None, .. o.clone() }),
        ("removing the component filter",                                                               |o, list| *list = attribute::List { component_filter:   None, .. o.clone() }),
        ("removing the protection level",                                                               |o, list| *list = attribute::List { protection_level:   None, .. o.clone() }),
        ("removing the job list",                                                                       |o, list| *list = attribute::List { job_list:           None, .. o.clone() }),
        ("removing the inherited handles list",                                                         |o, list| *list = attribute::List { inherit:            None, .. o.clone() }),

        ("removing most attributes", |o, list| {
            *list = o.clone();
            list.mitigation_policy  = None;
            list.child_policy       = None;
            list.dab_policy         = None;
            list.component_filter   = None;
            list.protection_level   = None;
            list.job_list           = None;
            list.inherit            = None;
        }),
    ].iter();
    let _ = modifications.next(); // skip typing

    let pi = loop {
        let mut si = process::StartupInfoExW::default();
        // N.B.: this will cause user32.dll(?) to STATUS_DLL_INIT_FAILED unless the child process can access the named desktop
        // specifying a nonexistant desktop is also an option
        si.startup_info.desktop     = Some(cstr16!("max_sandbox_desktop")).filter(|_| !target.allow.same_desktop);
        si.startup_info.flags       = STARTF_UNTRUSTEDSOURCE | STARTF_USESTDHANDLES;
        si.startup_info.std_output  = Some((&write).into());
        si.startup_info.std_error   = Some((&write).into());
        si.attribute_list           = Some(attribute_list.to_list());

        match create_process_as_user_w(
            &tokens.restricted, (), Some(&mut command_line[..]), None, None, true,
            process::DEBUG_PROCESS | process::CREATE_SEPARATE_WOW_VDM | process::CREATE_SUSPENDED | process::EXTENDED_STARTUPINFO_PRESENT,
            &*environment, (), &si,
        ) {
            Ok(pi) => break pi,
            Err(e) if e == ERROR_INVALID_PARAMETER => {
                drop(si); // unborrow?
                let (desc, f) = modifications.next().unwrap_or_else(|| panic!("process {exe:?} failed: ERROR_INVALID_PARAMETER - no modifications to the attribute list were able to resolve the issue"));
                eprintln!("process {exe:?} failed: ERROR_INVALID_PARAMETER - attempting to modify the attribute list for better success by {desc}");
                f(&original_attribute_list, &mut attribute_list);
                continue; // failed - continue attempting new combinations until it succeeds
            },
            Err(err) => panic!("process {exe:?} failed: {err:?}"),
        }
    };
    set_thread_token(&pi.thread, &tokens.permissive).unwrap();
    job::relimit(&job, 0);
    unsafe { resume_thread(&pi.thread) }.unwrap(); // SAFETY: this resumes the main thread in another process that was started in a suspended state via create_process_as_user_w(... | process::CREATE_SUSPENDED | ...)

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
    //drop(si);
    drop(write); // break pipe
    #[cfg(std)] match conio.join().unwrap() {
        Err(err) if err.kind() == io::ErrorKind::BrokenPipe => {},
        other => other.unwrap(),
    }
    assert!(exit == 0, "exit code: 0x{exit:08x} {}", Error::from(exit).friendly());
}
