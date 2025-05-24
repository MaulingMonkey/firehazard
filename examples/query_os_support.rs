use abistr::cstr16;
use firehazard::prelude::*;
use winresult::ERROR;

const COL1_WIDTH : usize = 95;



fn main() {
    println!();
    test_component_filter_flags_support();
    println!();
    test_mitigation_policy_support();
    println!();
    // TODO: pseudoconsoles?
}

fn test_component_filter_flags_support() {
    // 10.0.17763.7240  (x64 Windows Server 2019 or 2025):  unsupported
    // 10.0.19045.5854  (x64 Windows 10 Professional):      supported
    // 10.0.19043.2251  (x64 Windows 10 Professional):      supported
    // 10.0.20348.3561  (x64 Windows Server 2022):          supported

    let col1 = "process::ThreadAttributeRef::component_filter_flags(&…) ";
    match process::ThreadAttributeList::try_from(&[process::ThreadAttributeRef::component_filter_flags(&0)][..]) {
        Ok(_)                                   =>        println!("{col1:…<COL1_WIDTH$} supported"),
        Err(err) if err == ERROR::NOT_SUPPORTED => return println!("{col1:…<COL1_WIDTH$} unsupported"),
        Err(err)                                => panic!("process::ThreadAttributeRef::component_filter_flags: unsupported, unexpected error creating attribute list: {err:?}"),
    };

    for (desc,              component   ) in [
        ("0",               0           ),
        ("COMPONENT_KTM",   1 <<  0     ), // C:\Program Files (x86)\Windows Kits\10\Include\10.0.22621.0\um\winnt.h
        ("(1 <<  1)",       1 <<  1     ), // Not yet implemented/documented?
        ("(1 << 31)",       1 << 31     ), // Not yet implemented, but still succeeds
    ] {
        let col1 = format!("    process::ThreadAttributeRef::component_filter_flags(&{desc}) ");
        let mut si = process::StartupInfoExW::default();
        si.attribute_list = Some(process::ThreadAttributeList::try_from(&[process::ThreadAttributeRef::component_filter_flags(&component)][..]).expect("should be able to create *any* mitigation policy thread attribute list if we can create one"));
        let r = create_process_w(cstr16!(""), None, None, None, false, process::EXTENDED_STARTUPINFO_PRESENT, process::environment::Clear, (), &si);
        match r.as_ref() {
            //r(&err) if err == ERROR::FILE_NOT_FOUND       => println!("{col1:…<COL1_WIDTH$} supported"),   // never observed
            Err(&err) if err == ERROR::PATH_NOT_FOUND       => println!("{col1:…<COL1_WIDTH$} supported"),   // failed because "" isn't an executable, not because the attribute was invalid
            Err(&err) if err == ERROR::INVALID_PARAMETER    => println!("{col1:…<COL1_WIDTH$} unsupported"), // failed because the attribute list contained "invalid" component filters (unrecognized by the current version of Windows)
            Err(&err)                                       => panic!("{col1} unexpected error while testing for support: {err:?}"),
            Ok(process)                                     => panic!("{col1} unexpectedly created process {pid} for \"\" while testing OS support", pid = process.process_id),
        }
    }
}

/// Testing if the current version of Windows supports a given mitigation policy is a little awkward:
/// 1. We get no error until we get all the way to the point of calling [`create_process_w`] (or similar), which might have significant side effects!
/// 2. [`create_process_w`] takes an obscene number of parameters and indirect parameters that could generate [`ERROR::INVALID_PARAMETER`] beyond unrecognized mitigation policies.
///
/// Fortunately, attribute parameter checking *does* happen *before* we actually attempt to access the underlying application path, so we can simplify specify an invalid application path.
/// While we can't specify [`()`](https://doc.rust-lang.org/std/primitive.unit.html) for the `application_name` without also getting [`ERROR::INVALID_PARAMETER`],
/// we *can* specify <code>abistr::[cstr16\!](https://docs.rs/abistr/0.2.0-rc3/abistr/macro.cstr16.html)\(\"\"\)</code> and get [`ERROR::PATH_NOT_FOUND`] if the other parameters were valid.
/// This *shouldn't* ever launch an executable, right?  Probably?  I hope?
///
fn test_mitigation_policy_support() {
    match process::ThreadAttributeList::try_from(&[process::ThreadAttributeRef::mitigation_policy(&Default::default())][..]) {
        Err(err) if err == ERROR::NOT_SUPPORTED     => return println!("process::ThreadAttributeRef::mitigation_policy: unsupported"),
        Err(err)                                    => panic!("process::ThreadAttributeRef::mitigation_policy: unsupported, unexpected error creating attribute list: {err:?}"),
        Ok(_) => {
            println!("{:…<COL1_WIDTH$} supported", "process::ThreadAttributeRef::mitigation_policy(&…) ");

            for (desc, policy) in [
                ("process::creation::MitigationPolicy::default()",                                              process::creation::MitigationPolicy::default()),

                ("process::creation::mitigation_policy::block_non_microsoft_binaries",                          process::creation::mitigation_policy::block_non_microsoft_binaries::ALWAYS_ON               .into()),
                ("process::creation::mitigation_policy::block_non_microsoft_binaries::ALLOW_STORE",             process::creation::mitigation_policy::block_non_microsoft_binaries::ALLOW_STORE             .into()),
                ("process::creation::mitigation_policy::bottom_up_aslr",                                        process::creation::mitigation_policy::bottom_up_aslr::ALWAYS_ON                             .into()),
                ("process::creation::mitigation_policy::control_flow_guard",                                    process::creation::mitigation_policy::control_flow_guard::ALWAYS_ON                         .into()),
                ("process::creation::mitigation_policy::control_flow_guard::EXPORT_SUPPRESSION",                process::creation::mitigation_policy::control_flow_guard::EXPORT_SUPPRESSION                .into()),
                ("process::creation::mitigation_policy::extension_point_disable",                               process::creation::mitigation_policy::extension_point_disable::ALWAYS_ON                    .into()),
                ("process::creation::mitigation_policy::font_disable",                                          process::creation::mitigation_policy::font_disable::ALWAYS_ON                               .into()),
                ("process::creation::mitigation_policy::font_disable::AUDIT_NONSYSTEM_FONTS",                   process::creation::mitigation_policy::font_disable::AUDIT_NONSYSTEM_FONTS                   .into()),
                ("process::creation::mitigation_policy::force_relocate_images",                                 process::creation::mitigation_policy::force_relocate_images::ALWAYS_ON                      .into()),
                ("process::creation::mitigation_policy::force_relocate_images::ALWAYS_ON_REQ_RELOCS",           process::creation::mitigation_policy::force_relocate_images::ALWAYS_ON_REQ_RELOCS           .into()),
                ("process::creation::mitigation_policy::heap_terminate",                                        process::creation::mitigation_policy::heap_terminate::ALWAYS_ON                             .into()),
                ("process::creation::mitigation_policy::high_entropy_aslr",                                     process::creation::mitigation_policy::high_entropy_aslr::ALWAYS_ON                          .into()),
                ("process::creation::mitigation_policy::image_load_no_low_label",                               process::creation::mitigation_policy::image_load_no_low_label::ALWAYS_ON                    .into()),
                ("process::creation::mitigation_policy::image_load_no_remote",                                  process::creation::mitigation_policy::image_load_no_remote::ALWAYS_ON                       .into()),
                ("process::creation::mitigation_policy::image_load_prefer_system32",                            process::creation::mitigation_policy::image_load_prefer_system32::ALWAYS_ON                 .into()),
                ("process::creation::mitigation_policy::legacy::DEP_ENABLE",                                    process::creation::mitigation_policy::legacy::DEP_ENABLE                                    .into()),
                ("process::creation::mitigation_policy::legacy::DEP_ATL_THUNK_ENABLE",                          process::creation::mitigation_policy::legacy::DEP_ATL_THUNK_ENABLE                          .into()),
                ("process::creation::mitigation_policy::legacy::SEHOP_ENABLE",                                  process::creation::mitigation_policy::legacy::SEHOP_ENABLE                                  .into()),
                ("process::creation::mitigation_policy::prohibit_dynamic_code",                                 process::creation::mitigation_policy::prohibit_dynamic_code::ALWAYS_ON                      .into()),
                ("process::creation::mitigation_policy::prohibit_dynamic_code::ALWAYS_ON_ALLOW_OPT_OUT",        process::creation::mitigation_policy::prohibit_dynamic_code::ALWAYS_ON_ALLOW_OPT_OUT        .into()),
                ("process::creation::mitigation_policy::strict_handle_checks",                                  process::creation::mitigation_policy::strict_handle_checks::ALWAYS_ON                       .into()),
                ("process::creation::mitigation_policy::win32k_system_call_disable",                            process::creation::mitigation_policy::win32k_system_call_disable::ALWAYS_ON                 .into()),

                ("process::creation::mitigation_policy2::allow_downgrade_dynamic_code_policy",                  process::creation::mitigation_policy2::allow_downgrade_dynamic_code_policy::ALWAYS_ON       .into()),
                ("process::creation::mitigation_policy2::block_non_cet_binaries",                               process::creation::mitigation_policy2::block_non_cet_binaries::ALWAYS_ON                    .into()), // 10.0.17763.7240 (x64 Windows Server 2019 or 2025): unsupported
                ("process::creation::mitigation_policy2::block_non_cet_binaries::NON_EHCONT",                   process::creation::mitigation_policy2::block_non_cet_binaries::NON_EHCONT                   .into()), // 10.0.17763.7240 (x64 Windows Server 2019 or 2025): unsupported
                ("process::creation::mitigation_policy2::cet_dynamic_apis_out_of_proc_only",                    process::creation::mitigation_policy2::cet_dynamic_apis_out_of_proc_only::ALWAYS_ON         .into()), // 10.0.17763.7240 (x64 Windows Server 2019 or 2025): unsupported
                ("process::creation::mitigation_policy2::cet_user_shadow_stacks",                               process::creation::mitigation_policy2::cet_user_shadow_stacks::ALWAYS_ON                    .into()), // 10.0.17763.7240 (x64 Windows Server 2019 or 2025): unsupported
                ("process::creation::mitigation_policy2::cet_user_shadow_stacks::STRICT_MODE",                  process::creation::mitigation_policy2::cet_user_shadow_stacks::STRICT_MODE                  .into()), // 10.0.17763.7240 (x64 Windows Server 2019 or 2025): unsupported
                ("process::creation::mitigation_policy2::loader_integrity_continuity",                          process::creation::mitigation_policy2::loader_integrity_continuity::ALWAYS_ON               .into()),
                ("process::creation::mitigation_policy2::loader_integrity_continuity::AUDIT",                   process::creation::mitigation_policy2::loader_integrity_continuity::AUDIT                   .into()),
                ("process::creation::mitigation_policy2::module_tampering_protection",                          process::creation::mitigation_policy2::module_tampering_protection::ALWAYS_ON               .into()),
                ("process::creation::mitigation_policy2::module_tampering_protection::NOINHERIT",               process::creation::mitigation_policy2::module_tampering_protection::NOINHERIT               .into()),

                // 10.0.17763.7240  (x64 Windows Server 2019 or 2025):  unsupported
                // 10.0.19045.5854  (x64 Windows 10 Professional):      unsupported
                // 10.0.20348.3561  (x64 Windows Server 2022):          unsupported
                // (expected: ARM64 only)
                ("process::creation::mitigation_policy2::pointer_auth_user_ip",                                 process::creation::mitigation_policy2::pointer_auth_user_ip::ALWAYS_ON                      .into()),

                // 10.0.17763.7240  (x64 Windows Server 2019 or 2025):  unsupported
                // 10.0.19045.5854  (x64 Windows 10 Professional):      unsupported
                // 10.0.20348.3561  (x64 Windows Server 2022):          unsupported
                ("process::creation::mitigation_policy2::restrict_core_sharing",                                process::creation::mitigation_policy2::restrict_core_sharing::ALWAYS_ON                     .into()),

                ("process::creation::mitigation_policy2::restrict_indirect_branch_prediction",                  process::creation::mitigation_policy2::restrict_indirect_branch_prediction::ALWAYS_ON       .into()),
                ("process::creation::mitigation_policy2::speculative_store_bypass_disable",                     process::creation::mitigation_policy2::speculative_store_bypass_disable::ALWAYS_ON          .into()),
                ("process::creation::mitigation_policy2::strict_control_flow_guard",                            process::creation::mitigation_policy2::strict_control_flow_guard::ALWAYS_ON                 .into()),
                ("process::creation::mitigation_policy2::user_cet_set_context_ip_validation",                   process::creation::mitigation_policy2::user_cet_set_context_ip_validation::ALWAYS_ON        .into()), // 10.0.17763.7240 (x64 Windows Server 2019 or 2025): unsupported
                ("process::creation::mitigation_policy2::user_cet_set_context_ip_validation::RELAXED_MODE",     process::creation::mitigation_policy2::user_cet_set_context_ip_validation::RELAXED_MODE     .into()), // 10.0.17763.7240 (x64 Windows Server 2019 or 2025): unsupported

                // 10.0.17763.7240  (x64 Windows Server 2019 or 2025):  unsupported
                // 10.0.19045.5854  (x64 Windows 10 Professional):      unsupported
                // 10.0.20348.3561  (x64 Windows Server 2022):          supported
                ("process::creation::mitigation_policy2::xtended_control_flow_guard",                           process::creation::mitigation_policy2::xtended_control_flow_guard::ALWAYS_ON                .into()),
            ] {
                let col1 = format!("    {desc} ");
                let mut si = process::StartupInfoExW::default();
                si.attribute_list = Some(process::ThreadAttributeList::try_from(&[process::ThreadAttributeRef::mitigation_policy(&policy)][..]).expect("should be able to create *any* mitigation policy thread attribute list if we can create one"));
                match create_process_w(cstr16!(""), None, None, None, false, process::EXTENDED_STARTUPINFO_PRESENT, process::environment::Clear, (), &si) {
                    //r(err) if err == ERROR::FILE_NOT_FOUND        => println!("{col1:…<COL1_WIDTH$} supported"),   // never observed
                    Err(err) if err == ERROR::PATH_NOT_FOUND        => println!("{col1:…<COL1_WIDTH$} supported"),   // failed because "" isn't an executable, not because the attribute was invalid
                    Err(err) if err == ERROR::INVALID_PARAMETER     => println!("{col1:…<COL1_WIDTH$} unsupported"), // failed because the attribute list contained "invalid" mitigation policies (unrecognized by the current version of Windows)
                    Err(err)                                        => panic!("{desc}: unexpected error while testing for support: {err:?}"),
                    Ok(process)                                     => panic!("{desc}: unexpectedly created process {pid} for \"\" while testing OS support", pid = process.process_id),
                }
            }
        },
    }
}
