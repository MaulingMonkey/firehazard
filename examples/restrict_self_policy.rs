use firehazard::*;
use winapi::um::winnt::*;

fn main() {
    heap_enable_termination_on_corruption().unwrap();

    set_process_mitigation_policy(process::mitigation::AslrPolicy {
        disallow_stripped_images:       true,
        enable_bottom_up_randomization: true,
        enable_force_relocate_images:   true,
        enable_high_entropy:            true,
        .. Default::default()
    }).unwrap();

    set_process_mitigation_policy(process::mitigation::BinarySignaturePolicy {
        microsoft_signed_only:  true, // but this doesn't include our exe?
        mitigation_opt_in:      true, // "prevent the process from loading images that are not signed by Microsoft, the Windows Store and the Windows Hardware Quality Labs (WHQL)"
        //store_signed_only:      true, // desktop app isn't store signed - causes ERROR_INVALID_PARAMETER
        .. Default::default()
    }).unwrap();

    set_process_mitigation_policy(process::mitigation::ChildProcessPolicy {
        no_child_process_creation:          true,
        allow_secure_process_creation:      false,
        audit_no_child_process_creation:    false,
        .. Default::default()
    }).unwrap();

    set_process_mitigation_policy(process::mitigation::ControlFlowGuardPolicy {
        enable_control_flow_guard:  false,  // "This field cannot be changed via SetProcessMitigationPolicy."
        enable_export_suppression:  false,  // "This field cannot be changed via SetProcessMitigationPolicy."
        strict_mode:                true,   // "If TRUE, all DLLs that are loaded must enable CFG. If a DLL does not enable CFG then the image will fail to load. This policy can be enabled after a process has started by calling SetProcessMitigationPolicy. It cannot be disabled once enabled."
        .. Default::default()
    }).unwrap_err(); // ERROR_ACCESS_DENIED no matter what I pass - baked into executable perhaps?

    let mut policy = PROCESS_MITIGATION_DEP_POLICY { Flags: 0, Permanent: 0 };
    policy.set_Enable(1);
    policy.set_DisableAtlThunkEmulation(1);
    policy.Permanent = 1;
    assert_eq!(core::mem::size_of::<usize>()==8, set_process_mitigation_policy(policy).is_err()); // ERROR_NOT_SUPPORTED - possibly because it's force-enabled on x64?

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

    let mut policy = PROCESS_MITIGATION_IMAGE_LOAD_POLICY { Flags: 0 };
    policy.set_NoRemoteImages(1);
    policy.set_NoLowMandatoryLabelImages(1);
    policy.set_PreferSystem32Images(1);
    set_process_mitigation_policy(policy).unwrap();

    let mut policy = PROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY { Flags: 0 };
    policy.set_EnableExportAddressFilter(1);
    policy.set_EnableExportAddressFilterPlus(1);
    policy.set_EnableImportAddressFilter(1);
    policy.set_EnableRopCallerCheck(1);
    policy.set_EnableRopSimExec(1);
    policy.set_EnableRopStackPivot(1);
    //set_process_mitigation_policy(policy).unwrap(); // ERROR_INVALID_PARAMETER no matter the flags set

    let mut policy = PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY { Flags: 0 };
    policy.set_HandleExceptionsPermanentlyEnabled(1);
    policy.set_RaiseExceptionOnInvalidHandleReference(1);
    set_process_mitigation_policy(policy).unwrap();

    let mut policy = PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY { Flags: 0 };
    policy.set_DisallowWin32kSystemCalls(1);
    set_process_mitigation_policy(policy).unwrap();

    let mut policy = PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY { Flags: 0 };
    policy.set_FilterId(0);
    set_process_mitigation_policy(policy).unwrap(); // XXX: "This structure is not supported."

    // TODO:
    // PROCESS_MITIGATION_OPTIONS_MASK
    // PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY
    // PROCESS_MITIGATION_USER_SHADOW_STACK_POLICY
    // PROCESS_MITIGATION_REDIRECTION_TRUST_POLICY
    // PROCESS_MITIGATION_USER_POINTER_AUTH_POLICY
    // PROCESS_MITIGATION_SEHO_POLICY
}
