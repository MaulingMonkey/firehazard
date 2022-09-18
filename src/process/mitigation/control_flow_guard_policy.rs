use crate::*;
use crate::policy::*;
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
    type Policy = Self;
    fn into_policy(self) -> (process::mitigation::Policy, Self::Policy) { (process::ControlFlowGuardPolicy, self) }
}

unsafe impl IntoPolicy for ControlFlowGuardPolicy {
    type Policy = PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY;
    fn into_policy(self) -> (process::mitigation::Policy, Self::Policy) { (process::ControlFlowGuardPolicy, self.into()) }
}

impl From<ControlFlowGuardPolicy> for PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY {
    fn from(i: ControlFlowGuardPolicy) -> Self {
        let mut o = PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY::default();
        o.set_EnableControlFlowGuard(i.enable_control_flow_guard as u32);
        o.set_EnableExportSuppression(i.enable_export_suppression as u32);
        o.set_StrictMode(i.strict_mode as u32);
        o
    }
}
