use sandbox::windows::ffi::*;
use winapi::um::winnt::*;

fn main() {
    heap_enable_termination_on_corruption().unwrap();

    let mut policy = PROCESS_MITIGATION_ASLR_POLICY { Flags: 0 };
    policy.set_DisallowStrippedImages(1);
    policy.set_EnableBottomUpRandomization(1);
    policy.set_EnableForceRelocateImages(1);
    policy.set_EnableHighEntropy(1);
    set_process_mitigation_policy(policy).unwrap();

    let mut policy = PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY { Flags: 0 };
    policy.set_MicrosoftSignedOnly(1);  // but this doesn't include our exe?
    policy.set_MitigationOptIn(1);      // "prevent the process from loading images that are not signed by Microsoft, the Windows Store and the Windows Hardware Quality Labs (WHQL)"
    //policy.set_StoreSignedOnly(1);      // desktop app isn't store signed - causes ERROR_INVALID_PARAMETER
    set_process_mitigation_policy(policy).unwrap();

    let mut policy = PROCESS_MITIGATION_CHILD_PROCESS_POLICY { Flags: 0 };
    policy.set_NoChildProcessCreation(1);
    policy.set_AllowSecureProcessCreation(0);
    set_process_mitigation_policy(policy).unwrap();

    let mut policy = PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY { Flags: 0 };
    policy.set_EnableControlFlowGuard(1);   // "This field cannot be changed via SetProcessMitigationPolicy."
    policy.set_EnableExportSuppression(1);  // "This field cannot be changed via SetProcessMitigationPolicy."
    policy.set_StrictMode(1);               // "If TRUE, all DLLs that are loaded must enable CFG. If a DLL does not enable CFG then the image will fail to load. This policy can be enabled after a process has started by calling SetProcessMitigationPolicy. It cannot be disabled once enabled."
    //set_process_mitigation_policy(policy).unwrap(); // ERROR_ACCESS_DENIED no matter what I pass - baked into executable perhaps?

    let mut policy = PROCESS_MITIGATION_DEP_POLICY { Flags: 0, Permanent: 0 };
    policy.set_Enable(1);
    policy.set_DisableAtlThunkEmulation(1);
    policy.Permanent = 1;
    assert_eq!(core::mem::size_of::<usize>()==8, set_process_mitigation_policy(policy).is_err()); // ERROR_NOT_SUPPORTED - possibly because it's force-enabled on x64?

    let mut policy = PROCESS_MITIGATION_DYNAMIC_CODE_POLICY { Flags: 0 };
    policy.set_ProhibitDynamicCode(1);
    policy.set_AllowThreadOptOut(0);
    policy.set_AllowRemoteDowngrade(0);
    set_process_mitigation_policy(policy).unwrap();

    let mut policy = PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY { Flags: 0 };
    policy.set_DisableExtensionPoints(1);
    set_process_mitigation_policy(policy).unwrap();

    let mut policy = PROCESS_MITIGATION_FONT_DISABLE_POLICY { Flags: 0 };
    policy.set_DisableNonSystemFonts(1);
    set_process_mitigation_policy(policy).unwrap();

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
