//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessmitigationpolicy)\]
//! Process mitigation policy types and functions

use winapi::um::winnt::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessmitigationpolicy)\]
/// SetProcessMitigationPolicy parameters
pub unsafe trait IntoPolicy                                                     { type Policy;        fn into(self) -> (PROCESS_MITIGATION_POLICY, Self::Policy); }
unsafe impl IntoPolicy for PROCESS_MITIGATION_DEP_POLICY                        { type Policy = Self; fn into(self) -> (PROCESS_MITIGATION_POLICY, Self::Policy) { (Process::DEPPolicy as _,                    self) } }
unsafe impl IntoPolicy for PROCESS_MITIGATION_ASLR_POLICY                       { type Policy = Self; fn into(self) -> (PROCESS_MITIGATION_POLICY, Self::Policy) { (Process::ASLRPolicy as _,                   self) } }
unsafe impl IntoPolicy for PROCESS_MITIGATION_DYNAMIC_CODE_POLICY               { type Policy = Self; fn into(self) -> (PROCESS_MITIGATION_POLICY, Self::Policy) { (Process::DynamicCodePolicy as _,            self) } }
unsafe impl IntoPolicy for PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY        { type Policy = Self; fn into(self) -> (PROCESS_MITIGATION_POLICY, Self::Policy) { (Process::StrictHandleCheckPolicy as _,      self) } }
unsafe impl IntoPolicy for PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY        { type Policy = Self; fn into(self) -> (PROCESS_MITIGATION_POLICY, Self::Policy) { (Process::SystemCallDisablePolicy as _,      self) } }
//unsafe impl IntoPolicy for PROCESS_MITIGATION_OPTIONS_MASK                      { type Policy = Self; fn into(self) -> (PROCESS_MITIGATION_POLICY, Self::Policy) { (Process::MitigatoinOptionsMask as _,        self) } }
unsafe impl IntoPolicy for PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY    { type Policy = Self; fn into(self) -> (PROCESS_MITIGATION_POLICY, Self::Policy) { (Process::ExtensionPointDisablePolicy as _,  self) } }
unsafe impl IntoPolicy for PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY         { type Policy = Self; fn into(self) -> (PROCESS_MITIGATION_POLICY, Self::Policy) { (Process::ControlFlowGuardPolicy as _,       self) } }
unsafe impl IntoPolicy for PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY           { type Policy = Self; fn into(self) -> (PROCESS_MITIGATION_POLICY, Self::Policy) { (Process::SignaturePolicy as _,              self) } }
unsafe impl IntoPolicy for PROCESS_MITIGATION_FONT_DISABLE_POLICY               { type Policy = Self; fn into(self) -> (PROCESS_MITIGATION_POLICY, Self::Policy) { (Process::FontDisablePolicy as _,            self) } }
unsafe impl IntoPolicy for PROCESS_MITIGATION_IMAGE_LOAD_POLICY                 { type Policy = Self; fn into(self) -> (PROCESS_MITIGATION_POLICY, Self::Policy) { (Process::ImageLoadPolicy as _,              self) } }
unsafe impl IntoPolicy for PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY         { type Policy = Self; fn into(self) -> (PROCESS_MITIGATION_POLICY, Self::Policy) { (Process::SystemCallFilterPolicy as _,       self) } }
unsafe impl IntoPolicy for PROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY        { type Policy = Self; fn into(self) -> (PROCESS_MITIGATION_POLICY, Self::Policy) { (Process::PayloadRestrictionPolicy as _,     self) } }
unsafe impl IntoPolicy for PROCESS_MITIGATION_CHILD_PROCESS_POLICY              { type Policy = Self; fn into(self) -> (PROCESS_MITIGATION_POLICY, Self::Policy) { (Process::ChildProcessPolicy as _,           self) } }
//unsafe impl IntoPolicy for PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY     { type Policy = Self; fn into(self) -> (PROCESS_MITIGATION_POLICY, Self::Policy) { (SideChannelIsolationPolicy as _,            self) } }
//unsafe impl IntoPolicy for PROCESS_MITIGATION_USER_SHADOW_STACK_POLICY          { type Policy = Self; fn into(self) -> (PROCESS_MITIGATION_POLICY, Self::Policy) { (UserShadowStackPolicy as _,                 self) } }
//unsafe impl IntoPolicy for PROCESS_MITIGATION_REDIRECTION_TRUST_POLICY          { type Policy = Self; fn into(self) -> (PROCESS_MITIGATION_POLICY, Self::Policy) { (RedirectionTrustPolicy as _,                self) } }
//unsafe impl IntoPolicy for PROCESS_MITIGATION_USER_POINTER_AUTH_POLICY          { type Policy = Self; fn into(self) -> (PROCESS_MITIGATION_POLICY, Self::Policy) { (UserPointerAuthPolicy as _,                 self) } }
//unsafe impl IntoPolicy for PROCESS_MITIGATION_SEHO_POLICY                       { type Policy = Self; fn into(self) -> (PROCESS_MITIGATION_POLICY, Self::Policy) { (SEHOPPolicy as _,                           self) } }


pub use funcs::*;
pub(crate) mod funcs {
    include!("funcs/heap_enable_termination_on_corruption.rs");
    include!("funcs/set_process_mitigation_policy.rs");
}

/// `C:\Program Files (x86)\Windows Kits\10\Include\10.0.22621.0\um\winnt.h`
///
/// `typedef enum _PROCESS_MITIGATION_POLICY { ... };`
#[allow(dead_code)] #[repr(u32)] enum Process {
    DEPPolicy,
    ASLRPolicy,
    DynamicCodePolicy,
    StrictHandleCheckPolicy,
    SystemCallDisablePolicy,
    MitigationOptionsMask,
    ExtensionPointDisablePolicy,
    ControlFlowGuardPolicy,
    SignaturePolicy,
    FontDisablePolicy,
    ImageLoadPolicy,
    SystemCallFilterPolicy,
    PayloadRestrictionPolicy,
    ChildProcessPolicy,
    SideChannelIsolationPolicy,
    UserShadowStackPolicy,
    RedirectionTrustPolicy,
    UserPointerAuthPolicy,
    SEHOPPolicy,
    //MaxProcessMitigationPolicy
}
