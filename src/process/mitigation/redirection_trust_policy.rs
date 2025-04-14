use super::*;
use crate::*;
use bytemuck::*;



#[doc(alias = "PROCESS_MITIGATION_REDIRECTION_TRUST_POLICY")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-process-mitigation-redirection-trust-policy)\]
/// ≈ PROCESS_MITIGATION_REDIRECTION_TRUST_POLICY
///
#[derive(Clone, Copy, Debug)]
#[derive(Default, Zeroable)]
pub struct RedirectionTrustPolicy {
    pub enforce_redirection_trust:  bool,
    pub audit_redirection_trust:    bool,
    #[doc(hidden)] pub _reserved_flags: ()
}

unsafe impl GetPolicy for PROCESS_MITIGATION_REDIRECTION_TRUST_POLICY {
    type Raw = Self;
    fn ty() -> process::mitigation::Policy { process::RedirectionTrustPolicy }
    fn from_policy(p: Self::Raw) -> Self { p }
}

impl SetPolicy for PROCESS_MITIGATION_REDIRECTION_TRUST_POLICY {
    fn into_policy(self) -> Self::Raw { self }
}

unsafe impl GetPolicy for RedirectionTrustPolicy {
    type Raw = u32; // XXX
    fn ty() -> process::mitigation::Policy { process::RedirectionTrustPolicy }
    fn from_policy(p: Self::Raw) -> Self { PROCESS_MITIGATION_REDIRECTION_TRUST_POLICY { Flags: p }.into() }
}

impl SetPolicy for RedirectionTrustPolicy {
    fn into_policy(self) -> Self::Raw { PROCESS_MITIGATION_REDIRECTION_TRUST_POLICY::from(self).Flags }
}

impl From<RedirectionTrustPolicy> for PROCESS_MITIGATION_REDIRECTION_TRUST_POLICY {
    fn from(i: RedirectionTrustPolicy) -> Self {
        let mut o = PROCESS_MITIGATION_REDIRECTION_TRUST_POLICY::default();
        o.set_EnforceRedirectionTrust   (i.enforce_redirection_trust    as u32);
        o.set_AuditRedirectionTrust     (i.audit_redirection_trust      as u32);
        o
    }
}

impl From<PROCESS_MITIGATION_REDIRECTION_TRUST_POLICY> for RedirectionTrustPolicy {
    fn from(i: PROCESS_MITIGATION_REDIRECTION_TRUST_POLICY) -> Self {
        let mut o = Self::default();
        o.enforce_redirection_trust = i.EnforceRedirectionTrust() != 0;
        o.audit_redirection_trust   = i.AuditRedirectionTrust()   != 0;
        o
    }
}

// XXX: not (yet?) defined by winapi
#[allow(non_snake_case)] #[derive(Clone, Copy, Debug, Default)] #[repr(C)] struct PROCESS_MITIGATION_REDIRECTION_TRUST_POLICY { Flags: u32 }
#[allow(non_snake_case)] impl PROCESS_MITIGATION_REDIRECTION_TRUST_POLICY {
    pub fn set_EnforceRedirectionTrust  (&mut self, value: u32) { const M : u32 = 1 << 0; if value == 0 { self.Flags &= !M } else { self.Flags |= M } }
    pub fn set_AuditRedirectionTrust    (&mut self, value: u32) { const M : u32 = 1 << 1; if value == 0 { self.Flags &= !M } else { self.Flags |= M } }

    pub fn EnforceRedirectionTrust  (&self) -> u32 { (self.Flags >> 0) & 1 }
    pub fn AuditRedirectionTrust    (&self) -> u32 { (self.Flags >> 1) & 1 }
}
