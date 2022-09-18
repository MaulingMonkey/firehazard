use super::*;
use crate::*;
use bytemuck::*;
use winapi::um::winnt::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-process_mitigation_control_flow_guard_policy)\]
/// ~ [PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY]
#[derive(Clone, Copy, Debug)]
#[derive(Default, Zeroable)]
pub struct ControlFlowGuardPolicy {
    pub enable_control_flow_guard:  bool,
    pub enable_export_suppression:  bool,
    pub strict_mode:                bool,
    #[doc(hidden)] pub _reserved_flags: ()
}

unsafe impl IntoPolicy for PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY {
    type Raw = Self;
    fn ty() -> process::mitigation::Policy { process::ControlFlowGuardPolicy }
    fn into_policy(self) -> Self::Raw { self }
    fn from_policy(p: Self::Raw) -> Self { p }
}

unsafe impl IntoPolicy for ControlFlowGuardPolicy {
    type Raw = PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY;
    fn ty() -> process::mitigation::Policy { process::ControlFlowGuardPolicy }
    fn into_policy(self) -> Self::Raw { self.into() }
    fn from_policy(p: Self::Raw) -> Self { p.into() }
}

impl From<ControlFlowGuardPolicy> for PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY {
    fn from(i: ControlFlowGuardPolicy) -> Self {
        let mut o = Self::default();
        o.set_EnableControlFlowGuard    (i.enable_control_flow_guard    as u32);
        o.set_EnableExportSuppression   (i.enable_export_suppression    as u32);
        o.set_StrictMode                (i.strict_mode                  as u32);
        o
    }
}

impl From<PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY> for ControlFlowGuardPolicy {
    fn from(i: PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY) -> Self {
        let mut o = Self::default();
        o.enable_control_flow_guard = i.EnableControlFlowGuard()    != 0;
        o.enable_export_suppression = i.EnableExportSuppression()   != 0;
        o.strict_mode               = i.StrictMode()                != 0;
        o
    }
}
