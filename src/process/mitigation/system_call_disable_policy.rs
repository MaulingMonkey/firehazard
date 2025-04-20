use super::*;
use crate::prelude::*;
use bytemuck::*;
use winapi::um::winnt::*;



#[doc(alias = "PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-process_mitigation_system_call_disable_policy)\]
/// ≈ PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY
///
#[derive(Clone, Copy, Debug)]
#[derive(Default, Zeroable)]
pub struct SystemCallDisablePolicy {
    pub disallow_win32k_system_calls:           bool,
    pub audit_disallow_win32k_system_calls:     bool,
    #[doc(hidden)] pub _reserved_flags: ()
}

unsafe impl GetPolicy for PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY {
    type Raw = Self;
    fn ty() -> process::mitigation::Policy { process::SystemCallDisablePolicy }
    fn from_policy(p: Self::Raw) -> Self { p }
}

impl SetPolicy for PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY {
    fn into_policy(self) -> Self::Raw { self }
}

unsafe impl GetPolicy for SystemCallDisablePolicy {
    type Raw = PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY;
    fn ty() -> process::mitigation::Policy { process::SystemCallDisablePolicy }
    fn from_policy(p: Self::Raw) -> Self { p.into() }
}

impl SetPolicy for SystemCallDisablePolicy {
    fn into_policy(self) -> Self::Raw { self.into() }
}

impl From<SystemCallDisablePolicy> for PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY {
    fn from(i: SystemCallDisablePolicy) -> Self {
        let mut o = Self::default();
        o.Flags = 0
            | ((i.disallow_win32k_system_calls          as u32) << 0)
            | ((i.audit_disallow_win32k_system_calls    as u32) << 1)
            ;
        o
    }
}

impl From<PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY> for SystemCallDisablePolicy {
    fn from(i: PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY) -> Self {
        let mut o = Self::default();
        o.disallow_win32k_system_calls          = (i.Flags & (1 << 0)) != 0;
        o.audit_disallow_win32k_system_calls    = (i.Flags & (1 << 1)) != 0;
        o
    }
}
