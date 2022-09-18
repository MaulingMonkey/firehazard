use crate::*;
use crate::policy::*;
use bytemuck::*;
use winapi::um::winnt::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-process_mitigation_dep_policy)\]
/// ~ [PROCESS_MITIGATION_DEP_POLICY]
#[derive(Clone, Copy, Debug)]
#[derive(Default, Zeroable)]
pub struct DepPolicy {
    pub enable:                         bool,
    pub disable_atl_thunk_emulation:    bool,
    pub permanent:                      bool,
    #[doc(hidden)] pub _reserved_flags: ()
}

unsafe impl IntoPolicy for PROCESS_MITIGATION_DEP_POLICY {
    type Policy = Self;
    fn into_policy(self) -> (process::mitigation::Policy, Self::Policy) { (process::DEPPolicy, self) }
}

unsafe impl IntoPolicy for DepPolicy {
    type Policy = PROCESS_MITIGATION_DEP_POLICY;
    fn into_policy(self) -> (process::mitigation::Policy, Self::Policy) { (process::DEPPolicy, self.into()) }
}

impl From<DepPolicy> for PROCESS_MITIGATION_DEP_POLICY {
    fn from(i: DepPolicy) -> Self {
        let mut o = PROCESS_MITIGATION_DEP_POLICY::default();
        o.set_Enable(i.enable as u32);
        o.set_DisableAtlThunkEmulation(i.disable_atl_thunk_emulation as u32);
        o.Permanent = i.permanent as _;
        o
    }
}
