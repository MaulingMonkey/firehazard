use super::*;
use crate::*;
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
    type Raw = Self;
    fn ty() -> process::mitigation::Policy { process::ExtensionPointDisablePolicy }
    fn into_policy(self) -> Self::Raw { self }
    fn from_policy(p: Self::Raw) -> Self { p }
}

unsafe impl IntoPolicy for ExtensionPointDisablePolicy {
    type Raw = PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY;
    fn ty() -> process::mitigation::Policy { process::ExtensionPointDisablePolicy }
    fn into_policy(self) -> Self::Raw { self.into() }
    fn from_policy(p: Self::Raw) -> Self { p.into() }
}

impl From<ExtensionPointDisablePolicy> for PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY {
    fn from(i: ExtensionPointDisablePolicy) -> Self {
        let mut o = Self::default();
        o.set_DisableExtensionPoints(i.disable_extension_points as u32);
        o
    }
}

impl From<PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY> for ExtensionPointDisablePolicy {
    fn from(i: PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY) -> Self {
        let mut o = Self::default();
        o.disable_extension_points = i.DisableExtensionPoints() != 0;
        o
    }
}
