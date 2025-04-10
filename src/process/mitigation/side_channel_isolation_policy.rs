use super::*;
use crate::*;
use bytemuck::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-process_mitigation_side_channel_isolation_policy)\]
/// ≈ PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY
///
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

unsafe impl GetPolicy for PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY {
    type Raw = Self;
    fn ty() -> process::mitigation::Policy { process::SideChannelIsolationPolicy }
    fn from_policy(p: Self::Raw) -> Self { p }
}

impl SetPolicy for PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY {
    fn into_policy(self) -> Self::Raw { self }
}

unsafe impl GetPolicy for SideChannelIsolationPolicy {
    type Raw = u32; // XXX
    fn ty() -> process::mitigation::Policy { process::SideChannelIsolationPolicy }
    fn from_policy(p: Self::Raw) -> Self { PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY { Flags: p }.into() }
}

impl SetPolicy for SideChannelIsolationPolicy {
    fn into_policy(self) -> Self::Raw { PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY::from(self).Flags }
}

impl From<SideChannelIsolationPolicy> for PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY {
    fn from(i: SideChannelIsolationPolicy) -> Self {
        let mut o = Self::default();
        o.set_SmtBranchTargetIsolation      (i.smt_branch_target_isolation      as u32);
        o.set_IsolateSecurityDomain         (i.isolate_security_domain          as u32);
        o.set_DisablePageCombine            (i.disable_page_combine             as u32);
        o.set_SpeculativeStoreBypassDisable (i.speculative_store_bypass_disable as u32);
        o.set_RestrictCoreSharing           (i.restrict_core_sharing            as u32);
        o
    }
}

impl From<PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY> for SideChannelIsolationPolicy {
    fn from(i: PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY) -> Self {
        let mut o = Self::default();
        o.smt_branch_target_isolation       = i.SmtBranchTargetIsolation()      != 0;
        o.isolate_security_domain           = i.IsolateSecurityDomain()         != 0;
        o.disable_page_combine              = i.DisablePageCombine()            != 0;
        o.speculative_store_bypass_disable  = i.SpeculativeStoreBypassDisable() != 0;
        o.restrict_core_sharing             = i.RestrictCoreSharing()           != 0;
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

    pub fn SmtBranchTargetIsolation         (&self) -> u32 { (self.Flags >> 0) & 1 }
    pub fn IsolateSecurityDomain            (&self) -> u32 { (self.Flags >> 1) & 1 }
    pub fn DisablePageCombine               (&self) -> u32 { (self.Flags >> 2) & 1 }
    pub fn SpeculativeStoreBypassDisable    (&self) -> u32 { (self.Flags >> 3) & 1 }
    pub fn RestrictCoreSharing              (&self) -> u32 { (self.Flags >> 4) & 1 }
}
