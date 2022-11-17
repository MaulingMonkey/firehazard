use firehazard::*;
use firehazard::process::ThreadAttributeList;
use winapi::shared::winerror::ERROR_NOT_SUPPORTED;



pub struct List<'s> {
    pub mitigation_policy:  process::creation::MitigationPolicy,
    pub child_policy:       process::creation::ChildProcessPolicyFlags,
    pub dab_policy:         process::creation::DesktopAppPolicyFlags,
    pub component_filter:   u32,
    pub job_list:           Vec<job::Handle<'s>>,
    pub inherit:            Vec<handle::Borrowed<'s>>,
    // ...
}

impl<'s> List<'s> {
    pub fn new(target: &crate::settings::Target, job: impl Into<job::Handle<'s>>, inherit: impl IntoIterator<Item = handle::Borrowed<'s>>) -> Self {
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
            | (!target.allow.dynamic_code * process::creation::mitigation_policy::prohibit_dynamic_code::ALWAYS_ON)
            | process::creation::mitigation_policy::control_flow_guard::ALWAYS_ON               // Redundant?
            | process::creation::mitigation_policy::control_flow_guard::EXPORT_SUPPRESSION      // https://learn.microsoft.com/en-us/windows/win32/secbp/pe-metadata#export-suppression
            | process::creation::mitigation_policy::block_non_microsoft_binaries::ALWAYS_ON     // Redundant?
            | process::creation::mitigation_policy::block_non_microsoft_binaries::ALLOW_STORE   // ?
            | (!target.allow.same_desktop * process::creation::mitigation_policy::font_disable::ALWAYS_ON) // user32.dll(?) requires access on init
            | process::creation::mitigation_policy::image_load_no_remote::ALWAYS_ON
            | process::creation::mitigation_policy::image_load_no_low_label::ALWAYS_ON
            | process::creation::mitigation_policy::image_load_prefer_system32::ALWAYS_ON
            ;

        let policy2 = None
            | process::creation::mitigation_policy2::loader_integrity_continuity::ALWAYS_ON
            | process::creation::mitigation_policy2::strict_control_flow_guard::ALWAYS_ON
            | process::creation::mitigation_policy2::module_tampering_protection::ALWAYS_ON
            | process::creation::mitigation_policy2::restrict_indirect_branch_prediction::ALWAYS_ON
            | (!target.allow.dynamic_code * process::creation::mitigation_policy2::allow_downgrade_dynamic_code_policy::ALWAYS_OFF)
            | process::creation::mitigation_policy2::speculative_store_bypass_disable::ALWAYS_ON
            | process::creation::mitigation_policy2::cet_user_shadow_stacks::ALWAYS_ON      // Redundant
            | process::creation::mitigation_policy2::cet_user_shadow_stacks::STRICT_MODE
            | process::creation::mitigation_policy2::user_cet_set_context_ip_validation::ALWAYS_ON
            | (!target.allow.missing_cet * process::creation::mitigation_policy2::block_non_cet_binaries::ALWAYS_ON)
            //| process::creation::mitigation_policy2::xtended_control_flow_guard::ALWAYS_ON        // causes ERROR_INVALID_PARAMETER - not built with XFG? see https://connormcgarr.github.io/examining-xfg/ / https://query.prod.cms.rt.microsoft.com/cms/api/am/binary/RE37dMC
            //| process::creation::mitigation_policy2::pointer_auth_user_ip::ALWAYS_ON              // causes ERROR_INVALID_PARAMETER - ARM64 (not x64/AMD64!) only?
            | process::creation::mitigation_policy2::cet_dynamic_apis_out_of_proc_only::ALWAYS_ON
            //| process::creation::mitigation_policy2::restrict_core_sharing::ALWAYS_ON             // causes ERROR_INVALID_PARAMETER (do I need to specify cores to hog?)
            ;

        let mitigation_policy   = process::creation::MitigationPolicy::from((policy1, policy2));
        let child_policy        = process::creation::child_process::RESTRICTED;
        let dab_policy          = process::creation::desktop_app_breakaway::ENABLE_PROCESS_TREE;
        const COMPONENT_KTM : u32 = 1; // C:\Program Files (x86)\Windows Kits\10\Include\10.0.22621.0\um\winnt.h
        let component_filter    = COMPONENT_KTM;
        let job_list            = vec![job.into()];
        let inherit             = inherit.into_iter().collect();

        Self { mitigation_policy, child_policy, dab_policy, component_filter, job_list, inherit }
    }

    pub fn to_list(&self) -> ThreadAttributeList {
        let mitigation_policy   = process::ThreadAttributeRef::mitigation_policy(&self.mitigation_policy);
        let child_policy        = process::ThreadAttributeRef::child_process_policy(&self.child_policy);
        let dab_policy          = process::ThreadAttributeRef::desktop_app_policy(&self.dab_policy);
        let component_filter    = process::ThreadAttributeRef::component_filter_flags(&self.component_filter);
        let _protection_level   = {
            // will cause create_process_as_user_w to fail with ERROR_INVALID_PARAMETER
            // Also completely pointless, as we're almost certainly not running as a protected app ourselves
            const PROTECTION_LEVEL_SAME : u32 = 0xFFFFFFFF;
            process::ThreadAttributeRef::protection_level(&PROTECTION_LEVEL_SAME)
        };
        let job_list            = process::ThreadAttributeRef::job_list(&self.job_list[..]);
        let inherit             = process::ThreadAttributeRef::handle_list(&self.inherit[..]);
        // TODO: ThreadAttributeRef::security_capabilities ? app container / capability sids related: https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-security_capabilities

        let list = [mitigation_policy, child_policy, dab_policy, #[cfg(nope)] _protection_level, job_list, inherit, component_filter];
        let mut n = list.len();
        let min = n - 1; // component_filter is optional

        loop {
            match process::ThreadAttributeList::try_from(&list[..n-1]) {
                Ok(list) => return list,
                Err(err) if err == ERROR_NOT_SUPPORTED && n > min => n -= 1,
                err => {
                    dbg!(process::ThreadAttributeList::try_from(&[mitigation_policy ][..]).err());
                    dbg!(process::ThreadAttributeList::try_from(&[child_policy      ][..]).err());
                    dbg!(process::ThreadAttributeList::try_from(&[dab_policy        ][..]).err());
                    dbg!(process::ThreadAttributeList::try_from(&[component_filter  ][..]).err());
                    dbg!(process::ThreadAttributeList::try_from(&[_protection_level ][..]).err());
                    dbg!(process::ThreadAttributeList::try_from(&[job_list          ][..]).err());
                    dbg!(process::ThreadAttributeList::try_from(&[inherit           ][..]).err());
                    err.unwrap();
                },
            }
        }
    }
}
