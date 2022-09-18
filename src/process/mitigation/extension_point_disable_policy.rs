use crate::*;
use crate::policy::*;
use bytemuck::*;
use winapi::um::winnt::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-process_mitigation_extension_point_disable_policy)\]
/// ~ [PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY]
#[derive(Clone, Copy, Debug)]
#[derive(Default, Zeroable)]
pub struct ExtensionPointDisablePolicy {
    pub disable_extension_points: bool,
    #[doc(hidden)] pub _reserved_flags: ()
}

unsafe impl IntoPolicy for PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY {
    type Policy = Self;
    fn into_policy(self) -> (process::mitigation::Policy, Self::Policy) { (process::ExtensionPointDisablePolicy, self) }
}

unsafe impl IntoPolicy for ExtensionPointDisablePolicy {
    type Policy = PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY;
    fn into_policy(self) -> (process::mitigation::Policy, Self::Policy) { (process::ExtensionPointDisablePolicy, self.into()) }
}

impl From<ExtensionPointDisablePolicy> for PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY {
    fn from(i: ExtensionPointDisablePolicy) -> Self {
        let mut o = PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY::default();
        o.set_DisableExtensionPoints(i.disable_extension_points as u32);
        o
    }
}
