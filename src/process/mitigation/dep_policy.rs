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
    fn ty() -> process::mitigation::Policy { process::DEPPolicy }
    fn into_policy(self) -> Self::Policy { self }
    fn from_policy(p: Self::Policy) -> Self { p }
}

unsafe impl IntoPolicy for DepPolicy {
    type Policy = PROCESS_MITIGATION_DEP_POLICY;
    fn ty() -> process::mitigation::Policy { process::DEPPolicy }
    fn into_policy(self) -> Self::Policy { self.into() }
    fn from_policy(p: Self::Policy) -> Self { p.into() }
}

impl From<DepPolicy> for PROCESS_MITIGATION_DEP_POLICY {
    fn from(i: DepPolicy) -> Self {
        let mut o = Self::default();
        o.set_Enable                    (i.enable                       as u32);
        o.set_DisableAtlThunkEmulation  (i.disable_atl_thunk_emulation  as u32);
        o.Permanent = i.permanent as _;
        o
    }
}

impl From<PROCESS_MITIGATION_DEP_POLICY> for DepPolicy {
    fn from(i: PROCESS_MITIGATION_DEP_POLICY) -> Self {
        let mut o = Self::default();
        o.enable                        = i.Enable()                    != 0;
        o.disable_atl_thunk_emulation   = i.DisableAtlThunkEmulation()  != 0;
        o.permanent                     = i.Permanent                   != 0;
        o
    }
}
