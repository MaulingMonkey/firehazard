use sandbox_windows_ffi::*;
use sandbox_windows_ffi::process::ThreadAttributeList;



pub struct List<'s> {
    pub mitigation_policy:  process::creation::MitigationPolicy,
    pub child_policy:       process::creation::ChildProcessPolicyFlags,
    pub dab_policy:         process::creation::DesktopAppPolicyFlags,
    pub job_list:           Vec<job::Handle<'s>>,
    // ...
}

impl<'s> List<'s> {
    pub fn new(target: &crate::settings::Target, job: impl Into<job::Handle<'s>>) -> Self {
        let policy1 = ()
            | process::creation::mitigation_policy::DEP_ENABLE
            //| process::creation::mitigation_policy::DEP_ATL_THUNK_ENABLE
            | process::creation::mitigation_policy::SEHOP_ENABLE
            | process::creation::mitigation_policy::force_relocate_images::ALWAYS_ON_REQ_RELOCS // chrome.exe doesn't bother with this
            | process::creation::mitigation_policy::heap_terminate::ALWAYS_ON
            | process::creation::mitigation_policy::bottom_up_aslr::ALWAYS_ON
            | process::creation::mitigation_policy::high_entropy_aslr::ALWAYS_ON
            | process::creation::mitigation_policy::strict_handle_checks::ALWAYS_ON
            | (!target.allow.same_desktop * process::creation::mitigation_policy::win32k_system_call_disable::ALWAYS_ON) // user32.dll(?) requires access on init
            | process::creation::mitigation_policy::extension_point_disable::ALWAYS_ON
            | process::creation::mitigation_policy::prohibit_dynamic_code::ALWAYS_ON
            | process::creation::mitigation_policy::control_flow_guard::ALWAYS_ON               // Redundant?
            | process::creation::mitigation_policy::control_flow_guard::EXPORT_SUPPRESSION      // https://docs.microsoft.com/en-us/windows/win32/secbp/pe-metadata#export-suppression
            | process::creation::mitigation_policy::block_non_microsoft_binaries::ALWAYS_ON     // Redundant?
            | process::creation::mitigation_policy::block_non_microsoft_binaries::ALLOW_STORE   // ?
            | (!target.allow.same_desktop * process::creation::mitigation_policy::font_disable::ALWAYS_ON) // user32.dll(?) requires access on init
            | process::creation::mitigation_policy::image_load_no_remote::ALWAYS_ON
            | process::creation::mitigation_policy::image_load_no_low_label::ALWAYS_ON
            | process::creation::mitigation_policy::image_load_prefer_system32::ALWAYS_ON
            ;

        let policy2 = None
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

        let mitigation_policy   = process::creation::MitigationPolicy::from((policy1, policy2));
        let child_policy        = process::creation::child_process::RESTRICTED;
        let dab_policy          = process::creation::desktop_app_breakaway::ENABLE_PROCESS_TREE;
        let job_list            = vec![job.into()];

        Self { mitigation_policy, child_policy, dab_policy, job_list }
    }

    pub fn to_list(&self) -> ThreadAttributeList {
        process::ThreadAttributeList::try_from(&[
            process::ThreadAttributeRef::mitigation_policy(&self.mitigation_policy),
            process::ThreadAttributeRef::child_process_policy(&self.child_policy),
            process::ThreadAttributeRef::desktop_app_policy(&self.dab_policy),
            #[cfg(nope)] {
                // will cause create_process_as_user_w to fail with ERROR_INVALID_PARAMETER
                // Also completely pointless, as we're almost certainly not running as a protected app ourselves
                const PROTECTION_LEVEL_SAME : u32 = 0xFFFFFFFF;
                process::ThreadAttributeRef::protection_level(&PROTECTION_LEVEL_SAME)
            },
            process::ThreadAttributeRef::job_list(&self.job_list[..]),
            // TODO: ThreadAttributeRef::handle_list ?
            // TODO: ThreadAttributeRef::security_capabilities ? app container / capability sids related: https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-security_capabilities
        ][..]).unwrap()
    }
}
