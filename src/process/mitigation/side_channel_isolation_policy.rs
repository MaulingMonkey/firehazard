use crate::*;
use crate::policy::*;
use bytemuck::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-process_mitigation_side_channel_isolation_policy)\]
/// ~ [PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY]
#[derive(Clone, Copy, Debug)]
#[derive(Default, Zeroable)]
pub struct SideChannelIsolationPolicy {
    pub smt_branch_target_isolation:        bool,
    pub isolate_security_domain:            bool,
    pub disable_page_combine:               bool,
    pub speculative_store_bypass_disable:   bool,
    pub restrict_core_sharing:              bool,
    #[doc(hidden)] pub _reserved_flags: ()
}

unsafe impl IntoPolicy for PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY {
    type Policy = Self;
    fn into_policy(self) -> (process::mitigation::Policy, Self::Policy) { (process::SideChannelIsolationPolicy, self) }
}

unsafe impl IntoPolicy for SideChannelIsolationPolicy {
    type Policy = u32; // XXX
    fn into_policy(self) -> (process::mitigation::Policy, Self::Policy) { (process::SideChannelIsolationPolicy, PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY::from(self).Flags) }
}

impl From<SideChannelIsolationPolicy> for PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY {
    fn from(i: SideChannelIsolationPolicy) -> Self {
        let mut o = PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY::default();
        o.set_SmtBranchTargetIsolation      (i.smt_branch_target_isolation      as u32);
        o.set_IsolateSecurityDomain         (i.isolate_security_domain          as u32);
        o.set_DisablePageCombine            (i.disable_page_combine             as u32);
        o.set_SpeculativeStoreBypassDisable (i.speculative_store_bypass_disable as u32);
        o.set_RestrictCoreSharing           (i.restrict_core_sharing            as u32);
        o
    }
}

// XXX: not (yet?) defined by winapi
#[allow(non_snake_case)] #[derive(Clone, Copy, Debug, Default)] #[repr(C)] struct PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY { Flags: u32 }
#[allow(non_snake_case)] impl PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY {
    pub fn set_SmtBranchTargetIsolation         (&mut self, value: u32) { const M : u32 = 1 << 0; if value == 0 { self.Flags &= !M } else { self.Flags |= M } }
    pub fn set_IsolateSecurityDomain            (&mut self, value: u32) { const M : u32 = 1 << 1; if value == 0 { self.Flags &= !M } else { self.Flags |= M } }
    pub fn set_DisablePageCombine               (&mut self, value: u32) { const M : u32 = 1 << 2; if value == 0 { self.Flags &= !M } else { self.Flags |= M } }
    pub fn set_SpeculativeStoreBypassDisable    (&mut self, value: u32) { const M : u32 = 1 << 3; if value == 0 { self.Flags &= !M } else { self.Flags |= M } }
    pub fn set_RestrictCoreSharing              (&mut self, value: u32) { const M : u32 = 1 << 4; if value == 0 { self.Flags &= !M } else { self.Flags |= M } }
}
