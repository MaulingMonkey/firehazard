use crate::*;
use crate::policy::*;
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
    fn into_policy(self) -> (process::mitigation::Policy, Self::Policy) { (process::ChildProcessPolicy, self) }
}

unsafe impl IntoPolicy for ChildProcessPolicy {
    type Policy = PROCESS_MITIGATION_CHILD_PROCESS_POLICY;
    fn into_policy(self) -> (process::mitigation::Policy, Self::Policy) { (process::ChildProcessPolicy, self.into()) }
}

impl From<ChildProcessPolicy> for PROCESS_MITIGATION_CHILD_PROCESS_POLICY {
    fn from(i: ChildProcessPolicy) -> Self {
        let mut o = PROCESS_MITIGATION_CHILD_PROCESS_POLICY::default();
        o.set_NoChildProcessCreation(i.no_child_process_creation as u32);
        o.set_AuditNoChildProcessCreation(i.audit_no_child_process_creation as u32);
        o.set_AllowSecureProcessCreation(i.allow_secure_process_creation as u32);
        o
    }
}
