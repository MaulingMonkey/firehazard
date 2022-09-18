use super::*;
use crate::*;
use bytemuck::*;
use winapi::um::winnt::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-process_mitigation_system_call_disable_policy)\]
/// ~ [PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY]
#[derive(Clone, Copy, Debug)]
#[derive(Default, Zeroable)]
pub struct SystemCallDisablePolicy {
    pub disallow_win32k_system_calls: bool,
    #[doc(hidden)] pub _reserved_flags: ()
}

unsafe impl IntoPolicy for PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY {
    type Policy = Self;
    fn ty() -> process::mitigation::Policy { process::SystemCallDisablePolicy }
    fn into_policy(self) -> Self::Policy { self }
    fn from_policy(p: Self::Policy) -> Self { p }
}

unsafe impl IntoPolicy for SystemCallDisablePolicy {
    type Policy = PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY;
    fn ty() -> process::mitigation::Policy { process::SystemCallDisablePolicy }
    fn into_policy(self) -> Self::Policy { self.into() }
    fn from_policy(p: Self::Policy) -> Self { p.into() }
}

impl From<SystemCallDisablePolicy> for PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY {
    fn from(i: SystemCallDisablePolicy) -> Self {
        let mut o = Self::default();
        o.set_DisallowWin32kSystemCalls(i.disallow_win32k_system_calls as u32);
        o
    }
}

impl From<PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY> for SystemCallDisablePolicy {
    fn from(i: PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY) -> Self {
        let mut o = Self::default();
        o.disallow_win32k_system_calls = i.DisallowWin32kSystemCalls() != 0;
        o
    }
}
