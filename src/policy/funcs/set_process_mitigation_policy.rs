use crate::error::LastError;
use winapi::shared::minwindef::FALSE;
use winapi::um::processthreadsapi::*;
use winapi::um::winnt::*;
use std::mem::size_of_val;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessmitigationpolicy)\]
/// SetProcessMitigationPolicy(ProcessDEPPolicy, [PROCESS_MITIGATION_DEP_POLICY](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-process_mitigation_dep_policy) { ... })
pub fn dep(policy: &PROCESS_MITIGATION_DEP_POLICY) -> Result<(), LastError> { unsafe { set(ProcessDEPPolicy, policy) } }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessmitigationpolicy)\]
/// SetProcessMitigationPolicy(ProcessASLRPolicy, [PROCESS_MITIGATION_ASLR_POLICY](https://docs.microsoft.com/en-us/windows/desktop/api/winnt/ns-winnt-process_mitigation_aslr_policy) { ... })
pub fn aslr(policy: &PROCESS_MITIGATION_ASLR_POLICY) -> Result<(), LastError> { unsafe { set(ProcessASLRPolicy, policy) } }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessmitigationpolicy)\]
/// SetProcessMitigationPolicy(ProcessDynamicCodePolicy, [PROCESS_MITIGATION_DYNAMIC_CODE_POLICY](https://docs.microsoft.com/en-us/windows/desktop/api/winnt/ns-winnt-process_mitigation_dynamic_code_policy) { ... })
pub fn dynamic_code(policy: &PROCESS_MITIGATION_DYNAMIC_CODE_POLICY) -> Result<(), LastError> { unsafe { set(ProcessDynamicCodePolicy, policy) } }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessmitigationpolicy)\]
/// SetProcessMitigationPolicy(ProcessStrictHandleCheckPolicy, [PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY](https://docs.microsoft.com/en-us/windows/desktop/api/winnt/ns-winnt-process_mitigation_strict_handle_check_policy) { ... })
pub fn strict_handle_check(policy: &PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY) -> Result<(), LastError> { unsafe { set(ProcessStrictHandleCheckPolicy, policy) } }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessmitigationpolicy)\]
/// SetProcessMitigationPolicy(ProcessSystemCallDisablePolicy, [PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY](https://docs.microsoft.com/en-us/windows/desktop/api/winnt/ns-winnt-process_mitigation_system_call_disable_policy) { ... })
pub fn system_call_disable(policy: &PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY) -> Result<(), LastError> { unsafe { set(ProcessSystemCallDisablePolicy, policy) } }

// TODO: ProcessMitigationOptionsMask

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessmitigationpolicy)\]
/// SetProcessMitigationPolicy(ProcessExtensionPointDisablePolicy, [PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY](https://docs.microsoft.com/en-us/windows/desktop/api/winnt/ns-winnt-process_mitigation_extension_point_disable_policy) { ... })
pub fn extension_point_disable(policy: &PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY) -> Result<(), LastError> { unsafe { set(ProcessExtensionPointDisablePolicy, policy) } }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessmitigationpolicy)\]
/// SetProcessMitigationPolicy(ProcessControlFlowGuardPolicy, [PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY](https://docs.microsoft.com/en-us/windows/desktop/api/winnt/ns-winnt-process_mitigation_control_flow_guard_policy) { ... })
pub fn control_flow_guard(policy: &PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY) -> Result<(), LastError> { unsafe { set(ProcessControlFlowGuardPolicy, policy) } }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessmitigationpolicy)\]
/// SetProcessMitigationPolicy(ProcessSignaturePolicy, [PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY](https://docs.microsoft.com/en-us/windows/desktop/api/winnt/ns-winnt-process_mitigation_binary_signature_policy) { ... })
pub fn binary_signature(policy: &PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY) -> Result<(), LastError> { unsafe { set(ProcessSignaturePolicy, policy) } }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessmitigationpolicy)\]
/// SetProcessMitigationPolicy(ProcessFontDisablePolicy, [PROCESS_MITIGATION_FONT_DISABLE_POLICY](https://docs.microsoft.com/en-us/windows/desktop/api/winnt/ns-winnt-process_mitigation_font_disable_policy) { ... })
pub fn font_disable(policy: &PROCESS_MITIGATION_FONT_DISABLE_POLICY) -> Result<(), LastError> { unsafe { set(ProcessFontDisablePolicy, policy) } }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessmitigationpolicy)\]
/// SetProcessMitigationPolicy(ProcessImageLoadPolicy, [PROCESS_MITIGATION_IMAGE_LOAD_POLICY](https://docs.microsoft.com/en-us/windows/desktop/api/winnt/ns-winnt-process_mitigation_image_load_policy) { ... })
pub fn image_load(policy: &PROCESS_MITIGATION_IMAGE_LOAD_POLICY) -> Result<(), LastError> { unsafe { set(ProcessImageLoadPolicy, policy) } }

#[cfg(nyi)] // types/enums not exposed by winapi
/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessmitigationpolicy)\]
/// SetProcessMitigationPolicy(ProcessRedirectionTrustPolicy, [PROCESS_MITIGATION_REDIRECTION_TRUST_POLICY](https://docs.microsoft.com/en-us/windows/desktop/api/winnt/ns-winnt-process_mitigation_redirection_trust_policy) { ... })
pub fn redirection_trust(policy: &PROCESS_MITIGATION_REDIRECTION_TRUST_POLICY) -> Result<(), LastError> { unsafe { set(ProcessRedirectionTrustPolicy, policy) } }

#[cfg(nyi)] // types/enums not exposed by winapi
/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessmitigationpolicy)\]
/// SetProcessMitigationPolicy(ProcessSideChannelIsolationPolicy, [PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY](https://docs.microsoft.com/en-us/windows/desktop/api/winnt/ns-winnt-process_mitigation_side_channel_isolation_policy) { ... })
pub fn side_channel_isolation(policy: &PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY) -> Result<(), LastError> { unsafe { set(ProcessSideChannelIsolationPolicy, policy) } }

#[cfg(nyi)] // types/enums not exposed by winapi
/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessmitigationpolicy)\]
/// SetProcessMitigationPolicy(ProcessUserShadowStackPolicy, [PROCESS_MITIGATION_USER_SHADOW_STACK_POLICY](https://docs.microsoft.com/en-us/windows/desktop/api/winnt/ns-winnt-process_mitigation_user_shadow_stack_policy) { ... })
pub fn user_shadow_stack(policy: &PROCESS_MITIGATION_USER_SHADOW_STACK_POLICY) -> Result<(), LastError> { unsafe { set(ProcessUserShadowStackPolicy, policy) } }



unsafe fn set<T>(ty: PROCESS_MITIGATION_POLICY, value: &T) -> Result<(), LastError> {
    LastError::get_if(FALSE == unsafe { SetProcessMitigationPolicy(ty, value as *const T as *mut _, size_of_val(value)) })
}
