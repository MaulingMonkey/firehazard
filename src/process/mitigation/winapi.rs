use crate::*;
use crate::policy::*;
use winapi::um::winnt::*;



unsafe impl IntoPolicy for PROCESS_MITIGATION_DEP_POLICY                        { type Policy = Self; fn into(self) -> (process::mitigation::Policy, Self::Policy) { (process::DEPPolicy,                   self) } }
unsafe impl IntoPolicy for PROCESS_MITIGATION_ASLR_POLICY                       { type Policy = Self; fn into(self) -> (process::mitigation::Policy, Self::Policy) { (process::ASLRPolicy,                  self) } }
unsafe impl IntoPolicy for PROCESS_MITIGATION_DYNAMIC_CODE_POLICY               { type Policy = Self; fn into(self) -> (process::mitigation::Policy, Self::Policy) { (process::DynamicCodePolicy,           self) } }
unsafe impl IntoPolicy for PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY        { type Policy = Self; fn into(self) -> (process::mitigation::Policy, Self::Policy) { (process::StrictHandleCheckPolicy,     self) } }
unsafe impl IntoPolicy for PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY        { type Policy = Self; fn into(self) -> (process::mitigation::Policy, Self::Policy) { (process::SystemCallDisablePolicy,     self) } }
//unsafe impl IntoPolicy for PROCESS_MITIGATION_OPTIONS_MASK                      { type Policy = Self; fn into(self) -> (process::mitigation::Policy, Self::Policy) { (process::MitigatoinOptionsMask,       self) } }
unsafe impl IntoPolicy for PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY    { type Policy = Self; fn into(self) -> (process::mitigation::Policy, Self::Policy) { (process::ExtensionPointDisablePolicy, self) } }
unsafe impl IntoPolicy for PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY         { type Policy = Self; fn into(self) -> (process::mitigation::Policy, Self::Policy) { (process::ControlFlowGuardPolicy,      self) } }
unsafe impl IntoPolicy for PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY           { type Policy = Self; fn into(self) -> (process::mitigation::Policy, Self::Policy) { (process::SignaturePolicy,             self) } }
unsafe impl IntoPolicy for PROCESS_MITIGATION_FONT_DISABLE_POLICY               { type Policy = Self; fn into(self) -> (process::mitigation::Policy, Self::Policy) { (process::FontDisablePolicy,           self) } }
unsafe impl IntoPolicy for PROCESS_MITIGATION_IMAGE_LOAD_POLICY                 { type Policy = Self; fn into(self) -> (process::mitigation::Policy, Self::Policy) { (process::ImageLoadPolicy,             self) } }
unsafe impl IntoPolicy for PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY         { type Policy = Self; fn into(self) -> (process::mitigation::Policy, Self::Policy) { (process::SystemCallFilterPolicy,      self) } }
unsafe impl IntoPolicy for PROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY        { type Policy = Self; fn into(self) -> (process::mitigation::Policy, Self::Policy) { (process::PayloadRestrictionPolicy,    self) } }
unsafe impl IntoPolicy for PROCESS_MITIGATION_CHILD_PROCESS_POLICY              { type Policy = Self; fn into(self) -> (process::mitigation::Policy, Self::Policy) { (process::ChildProcessPolicy,          self) } }
//unsafe impl IntoPolicy for PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY     { type Policy = Self; fn into(self) -> (process::mitigation::Policy, Self::Policy) { (process::SideChannelIsolationPolicy,  self) } }
//unsafe impl IntoPolicy for PROCESS_MITIGATION_USER_SHADOW_STACK_POLICY          { type Policy = Self; fn into(self) -> (process::mitigation::Policy, Self::Policy) { (process::UserShadowStackPolicy,       self) } }
//unsafe impl IntoPolicy for PROCESS_MITIGATION_REDIRECTION_TRUST_POLICY          { type Policy = Self; fn into(self) -> (process::mitigation::Policy, Self::Policy) { (process::RedirectionTrustPolicy,      self) } }
//unsafe impl IntoPolicy for PROCESS_MITIGATION_USER_POINTER_AUTH_POLICY          { type Policy = Self; fn into(self) -> (process::mitigation::Policy, Self::Policy) { (process::UserPointerAuthPolicy,       self) } }
//unsafe impl IntoPolicy for PROCESS_MITIGATION_SEHO_POLICY                       { type Policy = Self; fn into(self) -> (process::mitigation::Policy, Self::Policy) { (process::SEHOPPolicy,                 self) } }
