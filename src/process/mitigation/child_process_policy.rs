use super::*;
use crate::*;
use bytemuck::*;
use winapi::um::winnt::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-process_mitigation_child_process_policy)\]
/// ~ [PROCESS_MITIGATION_CHILD_PROCESS_POLICY]
#[derive(Clone, Copy, Debug)]
#[derive(Default, Zeroable)]
pub struct ChildProcessPolicy {
    pub no_child_process_creation:          bool,
    pub audit_no_child_process_creation:    bool,
    pub allow_secure_process_creation:      bool,
    #[doc(hidden)] pub _reserved_flags: ()
}

unsafe impl IntoPolicy for PROCESS_MITIGATION_CHILD_PROCESS_POLICY {
    type Policy = Self;
    fn ty() -> process::mitigation::Policy { process::ChildProcessPolicy }
    fn into_policy(self) -> Self::Policy { self }
    fn from_policy(p: Self::Policy) -> Self { p }
}

unsafe impl IntoPolicy for ChildProcessPolicy {
    type Policy = PROCESS_MITIGATION_CHILD_PROCESS_POLICY;
    fn ty() -> process::mitigation::Policy { process::ChildProcessPolicy }
    fn into_policy(self) -> Self::Policy { self.into() }
    fn from_policy(p: Self::Policy) -> Self { p.into() }
}

impl From<ChildProcessPolicy> for PROCESS_MITIGATION_CHILD_PROCESS_POLICY {
    fn from(i: ChildProcessPolicy) -> Self {
        let mut o = Self::default();
        o.set_NoChildProcessCreation        (i.no_child_process_creation        as u32);
        o.set_AuditNoChildProcessCreation   (i.audit_no_child_process_creation  as u32);
        o.set_AllowSecureProcessCreation    (i.allow_secure_process_creation    as u32);
        o
    }
}

impl From<PROCESS_MITIGATION_CHILD_PROCESS_POLICY> for ChildProcessPolicy {
    fn from(i: PROCESS_MITIGATION_CHILD_PROCESS_POLICY) -> Self {
        let mut o = Self::default();
        o.no_child_process_creation         = i.NoChildProcessCreation()        != 0;
        o.audit_no_child_process_creation   = i.AuditNoChildProcessCreation()   != 0;
        o.allow_secure_process_creation     = i.AllowSecureProcessCreation()    != 0;
        o
    }
}
