use crate::*;
use crate::policy::*;
use bytemuck::*;
use winapi::um::winnt::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-process_mitigation_aslr_policy)\]
/// ~ [PROCESS_MITIGATION_ASLR_POLICY]
#[derive(Clone, Copy, Debug)]
#[derive(Default, Zeroable)]
pub struct AslrPolicy {
    pub enable_bottom_up_randomization:     bool,
    pub enable_force_relocate_images:       bool,
    pub enable_high_entropy:                bool,
    pub disallow_stripped_images:           bool,
    #[doc(hidden)] pub _reserved_flags:     ()
}

unsafe impl IntoPolicy for PROCESS_MITIGATION_ASLR_POLICY {
    type Policy = Self;
    fn into_policy(self) -> (process::mitigation::Policy, Self::Policy) { (process::ASLRPolicy, self) }
}

unsafe impl IntoPolicy for AslrPolicy {
    type Policy = PROCESS_MITIGATION_ASLR_POLICY;
    fn into_policy(self) -> (process::mitigation::Policy, Self::Policy) { (process::ASLRPolicy, self.into()) }
}

impl From<AslrPolicy> for PROCESS_MITIGATION_ASLR_POLICY {
    fn from(i: AslrPolicy) -> Self {
        let mut o = PROCESS_MITIGATION_ASLR_POLICY::default();
        o.set_EnableBottomUpRandomization(i.enable_bottom_up_randomization as u32);
        o.set_EnableForceRelocateImages(i.enable_force_relocate_images as u32);
        o.set_EnableHighEntropy(i.enable_high_entropy as u32);
        o.set_DisallowStrippedImages(i.disallow_stripped_images as u32);
        o
    }
}
