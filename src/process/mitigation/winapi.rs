use crate::*;
use crate::policy::*;
use winapi::um::winnt::*;



//unsafe impl IntoPolicy for PROCESS_MITIGATION_OPTIONS_MASK                      { type Policy = Self; fn into_policy(self) -> (process::mitigation::Policy, Self::Policy) { (process::MitigatoinOptionsMask,       self) } }
unsafe impl IntoPolicy for PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY         { type Policy = Self; fn into_policy(self) -> (process::mitigation::Policy, Self::Policy) { (process::SystemCallFilterPolicy,      self) } }
//unsafe impl IntoPolicy for PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY     { type Policy = Self; fn into_policy(self) -> (process::mitigation::Policy, Self::Policy) { (process::SideChannelIsolationPolicy,  self) } }
//unsafe impl IntoPolicy for PROCESS_MITIGATION_USER_SHADOW_STACK_POLICY          { type Policy = Self; fn into_policy(self) -> (process::mitigation::Policy, Self::Policy) { (process::UserShadowStackPolicy,       self) } }
//unsafe impl IntoPolicy for PROCESS_MITIGATION_REDIRECTION_TRUST_POLICY          { type Policy = Self; fn into_policy(self) -> (process::mitigation::Policy, Self::Policy) { (process::RedirectionTrustPolicy,      self) } }
//unsafe impl IntoPolicy for PROCESS_MITIGATION_USER_POINTER_AUTH_POLICY          { type Policy = Self; fn into_policy(self) -> (process::mitigation::Policy, Self::Policy) { (process::UserPointerAuthPolicy,       self) } }
//unsafe impl IntoPolicy for PROCESS_MITIGATION_SEHO_POLICY                       { type Policy = Self; fn into_policy(self) -> (process::mitigation::Policy, Self::Policy) { (process::SEHOPPolicy,                 self) } }
