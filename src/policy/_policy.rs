//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessmitigationpolicy)\]
//! Process mitigation policy types and functions

use winapi::um::winnt::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessmitigationpolicy)\]
/// SetProcessMitigationPolicy parameters
pub unsafe trait IntoPolicy                                                     { type Policy;        fn into(self) -> (PROCESS_MITIGATION_POLICY, Self::Policy); }
unsafe impl IntoPolicy for PROCESS_MITIGATION_DEP_POLICY                        { type Policy = Self; fn into(self) -> (PROCESS_MITIGATION_POLICY, Self::Policy) { (ProcessDEPPolicy,                   self) } }
unsafe impl IntoPolicy for PROCESS_MITIGATION_ASLR_POLICY                       { type Policy = Self; fn into(self) -> (PROCESS_MITIGATION_POLICY, Self::Policy) { (ProcessASLRPolicy,                  self) } }
unsafe impl IntoPolicy for PROCESS_MITIGATION_DYNAMIC_CODE_POLICY               { type Policy = Self; fn into(self) -> (PROCESS_MITIGATION_POLICY, Self::Policy) { (ProcessDynamicCodePolicy,           self) } }
unsafe impl IntoPolicy for PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY        { type Policy = Self; fn into(self) -> (PROCESS_MITIGATION_POLICY, Self::Policy) { (ProcessStrictHandleCheckPolicy,     self) } }
unsafe impl IntoPolicy for PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY        { type Policy = Self; fn into(self) -> (PROCESS_MITIGATION_POLICY, Self::Policy) { (ProcessSystemCallDisablePolicy,     self) } }
// TODO: ProcessMitigationOptionsMask
unsafe impl IntoPolicy for PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY    { type Policy = Self; fn into(self) -> (PROCESS_MITIGATION_POLICY, Self::Policy) { (ProcessExtensionPointDisablePolicy, self) } }
unsafe impl IntoPolicy for PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY         { type Policy = Self; fn into(self) -> (PROCESS_MITIGATION_POLICY, Self::Policy) { (ProcessControlFlowGuardPolicy,      self) } }
unsafe impl IntoPolicy for PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY           { type Policy = Self; fn into(self) -> (PROCESS_MITIGATION_POLICY, Self::Policy) { (ProcessSignaturePolicy,             self) } }
unsafe impl IntoPolicy for PROCESS_MITIGATION_FONT_DISABLE_POLICY               { type Policy = Self; fn into(self) -> (PROCESS_MITIGATION_POLICY, Self::Policy) { (ProcessFontDisablePolicy,           self) } }
unsafe impl IntoPolicy for PROCESS_MITIGATION_IMAGE_LOAD_POLICY                 { type Policy = Self; fn into(self) -> (PROCESS_MITIGATION_POLICY, Self::Policy) { (ProcessImageLoadPolicy,             self) } }
//unsafe impl IntoPolicy for PROCESS_MITIGATION_REDIRECTION_TRUST_POLICY          { type Policy = Self; fn into(self) -> (PROCESS_MITIGATION_POLICY, Self::Policy) { (_,                                  self) } }
//unsafe impl IntoPolicy for PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY     { type Policy = Self; fn into(self) -> (PROCESS_MITIGATION_POLICY, Self::Policy) { (_,                                  self) } }
//unsafe impl IntoPolicy for PROCESS_MITIGATION_USER_SHADOW_STACK_POLICY          { type Policy = Self; fn into(self) -> (PROCESS_MITIGATION_POLICY, Self::Policy) { (_,                                  self) } }

pub use funcs::*;
pub(crate) mod funcs {
    include!("funcs/set_process_mitigation_policy.rs");
}
