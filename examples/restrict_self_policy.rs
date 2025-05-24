use bytemuck::Zeroable;
use firehazard::*;
use winapi::shared::winerror::*;
use winapi::um::winnt::*;

fn main() {
    heap_enable_termination_on_corruption().unwrap();

    let mask : process::mitigation::OptionsMask = get_process_mitigation_policy(get_current_process()).unwrap();
    dbg!(mask);

    set_process_mitigation_policy(process::mitigation::AslrPolicy::strict_v1()).unwrap();

    set_process_mitigation_policy(process::mitigation::BinarySignaturePolicy {
        microsoft_signed_only:      true, // but this doesn't include our exe?
        mitigation_opt_in:          true, // "prevent the process from loading images that are not signed by Microsoft, the Windows Store and the Windows Hardware Quality Labs (WHQL)"
        //store_signed_only:          true, // cannot use in a desktop app, even in audit mode? causes ERROR_INVALID_PARAMETER
        .. Zeroable::zeroed()
    }).unwrap();

    set_process_mitigation_policy(process::mitigation::ChildProcessPolicy::strict_v1()).unwrap();

    let old = get_process_mitigation_policy::<process::mitigation::ControlFlowGuardPolicy>(get_current_process()).unwrap();
    set_process_mitigation_policy(process::mitigation::ControlFlowGuardPolicy {
        strict_mode: true,              // block non-CFG binaries
        enable_xfg_audit_mode: false,   // if XFG is enabled, enforce it?
        //enable_xfg: true,             // ERROR_INVALID_PARAMETER - also probably can't be changed?
        .. Zeroable::zeroed()
    }).unwrap();
    let new = get_process_mitigation_policy::<process::mitigation::ControlFlowGuardPolicy>(get_current_process()).unwrap();
    assert_eq!(old.enable_control_flow_guard,   new.enable_control_flow_guard,  "not as read-only as I thought?");
    assert_eq!(old.enable_export_suppression,   new.enable_export_suppression,  "not as read-only as I thought?");
    assert_eq!(old.enable_xfg,                  new.enable_xfg,                 "not as read-only as I thought?");

    let policy = process::mitigation::DepPolicy {
        enable:                         true,
        disable_atl_thunk_emulation:    true,
        permanent:                      true,
        .. Zeroable::zeroed()
    };
    assert_eq!(ERROR_NOT_SUPPORTED, set_process_mitigation_policy(policy).unwrap_err()); // already permanently enabled

    set_process_mitigation_policy(process::mitigation::DynamicCodePolicy {
        prohibit_dynamic_code:  true,
        allow_thread_opt_out:   false,
        allow_remote_downgrade: false,
        .. Default::default()
    }).unwrap();

    set_process_mitigation_policy(process::mitigation::ExtensionPointDisablePolicy {
        disable_extension_points: true,
        .. Default::default()
    }).unwrap();

    set_process_mitigation_policy(process::mitigation::FontDisablePolicy {
        disable_non_system_fonts:       true,
        audit_non_system_font_loading:  false,
        .. Default::default()
    }).unwrap();

    set_process_mitigation_policy(process::mitigation::ImageLoadPolicy {
        no_remote_images:               true,
        no_low_mandatory_label_images:  true,
        prefer_system32_images:         true,
        .. Default::default()
    }).unwrap();

    set_process_mitigation_policy(process::mitigation::PayloadRestrictionPolicy {
        enable_export_address_filter:       true,
        enable_export_address_filter_plus:  true,
        enable_import_address_filter:       true,
        enable_rop_caller_check:            true,
        enable_rop_sim_exec:                true,
        enable_rop_stack_pivot:             true,
        .. Default::default()
    }).unwrap_err(); // ERROR_INVALID_PARAMETER no matter the flags set

    set_process_mitigation_policy(process::mitigation::RedirectionTrustPolicy {
        enforce_redirection_trust: true,
        .. Default::default()
    }).unwrap();

    set_process_mitigation_policy(process::mitigation::StrictHandleCheckPolicy {
        handle_exceptions_permanently_enabled:          true,
        raise_exception_on_invalid_handle_reference:    true,
        .. Default::default()
    }).unwrap();

    set_process_mitigation_policy(process::mitigation::SystemCallDisablePolicy {
        disallow_win32k_system_calls: true,
        .. Default::default()
    }).unwrap();

    let mut policy = PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY { Flags: 0 };
    policy.set_FilterId(0);
    set_process_mitigation_policy(policy).unwrap_err(); // ERROR_INVALID_PARAMETER - n.b. docs: "This structure is not supported."

    set_process_mitigation_policy(process::mitigation::SideChannelIsolationPolicy {
        disable_page_combine:               true,
        isolate_security_domain:            true,
        //restrict_core_sharing:              true, // ERROR_INVALID_PARAMETER - n.b. poorly documented
        smt_branch_target_isolation:        true,
        speculative_store_bypass_disable:   true,
        .. Default::default()
    }).unwrap();

    set_process_mitigation_policy(process::mitigation::SehopPolicy { // "Structured Exception Handling Overwrite Protection"
        enable_sehop: true,
        .. Default::default()
    }).unwrap_err(); // ERROR_INVALID_PARAMETER regardless of settings - older windows?

    match set_process_mitigation_policy(process::mitigation::UserPointerAuthPolicy {
        enable_pointer_auth_user_ip: true,
        .. Default::default()
    }) {
        Ok(())                                      => {}, // succeeds on Windows 10 Professional x64 (10.0.19045.5854)
        Err(err) if err == ERROR_INVALID_PARAMETER  => {}, // failed   on Windows 10 Professional x64 (previous versions)?
        result                                      => result.expect("unexpected error setting process::mitigation::UserPointerAuthPolicy"),
    }

    set_process_mitigation_policy(process::mitigation::UserShadowStackPolicy {
        block_non_cet_binaries:                 true,
        cet_dynamic_apis_out_of_proc_only:      true,
        //enable_user_shadow_stack:               true, // ERROR_ACCESS_DENIED
        //enable_user_shadow_stack_strict_mode:   true, // ERROR_ACCESS_DENIED
        //set_context_ip_validation:              true, // ERROR_ACCESS_DENIED
        audit_block_non_cet_binaries:           false,
        audit_set_context_ip_validation:        false,
        audit_user_shadow_stack:                false,
        .. get_process_mitigation_policy(get_current_process()).unwrap()
    }).unwrap();
}
