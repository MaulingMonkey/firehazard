use crate::*;
use crate::policy::*;
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
    fn into_policy(self) -> (process::mitigation::Policy, Self::Policy) { (process::SystemCallDisablePolicy, self) }
}

unsafe impl IntoPolicy for SystemCallDisablePolicy {
    type Policy = PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY;
    fn into_policy(self) -> (process::mitigation::Policy, Self::Policy) { (process::SystemCallDisablePolicy, self.into()) }
}

impl From<SystemCallDisablePolicy> for PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY {
    fn from(i: SystemCallDisablePolicy) -> Self {
        let mut o = PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY::default();
        o.set_DisallowWin32kSystemCalls(i.disallow_win32k_system_calls as u32);
        o
    }
}
