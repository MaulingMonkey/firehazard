use crate::*;
use crate::policy::*;
use bytemuck::*;
use winapi::um::winnt::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-process_mitigation_dynamic_code_policy)\]
/// ~ [PROCESS_MITIGATION_DYNAMIC_CODE_POLICY]
#[derive(Clone, Copy, Debug)]
#[derive(Default, Zeroable)]
pub struct DynamicCodePolicy {
    pub prohibit_dynamic_code:  bool,
    pub allow_thread_opt_out:   bool,
    pub allow_remote_downgrade: bool,
    #[doc(hidden)] pub _reserved_flags: ()
}

unsafe impl IntoPolicy for PROCESS_MITIGATION_DYNAMIC_CODE_POLICY {
    type Policy = Self;
    fn into_policy(self) -> (process::mitigation::Policy, Self::Policy) { (process::DynamicCodePolicy, self) }
}

unsafe impl IntoPolicy for DynamicCodePolicy {
    type Policy = PROCESS_MITIGATION_DYNAMIC_CODE_POLICY;
    fn into_policy(self) -> (process::mitigation::Policy, Self::Policy) { (process::DynamicCodePolicy, self.into()) }
}

impl From<DynamicCodePolicy> for PROCESS_MITIGATION_DYNAMIC_CODE_POLICY {
    fn from(i: DynamicCodePolicy) -> Self {
        let mut o = PROCESS_MITIGATION_DYNAMIC_CODE_POLICY::default();
        o.set_ProhibitDynamicCode(i.prohibit_dynamic_code as u32);
        o.set_AllowThreadOptOut(i.allow_thread_opt_out as u32);
        o.set_AllowRemoteDowngrade(i.allow_remote_downgrade as u32);
        o
    }
}
