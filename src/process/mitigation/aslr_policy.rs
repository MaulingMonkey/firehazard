use super::*;
use crate::*;
use bytemuck::*;
use winapi::um::winnt::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-process_mitigation_aslr_policy)\]
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
    type Raw = Self;
    fn ty() -> process::mitigation::Policy { process::ASLRPolicy }
    fn into_policy(self) -> Self::Raw { self }
    fn from_policy(p: Self::Raw) -> Self { p }
}

unsafe impl IntoPolicy for AslrPolicy {
    type Raw = PROCESS_MITIGATION_ASLR_POLICY;
    fn ty() -> process::mitigation::Policy { process::ASLRPolicy }
    fn into_policy(self) -> Self::Raw { self.into() }
    fn from_policy(p: Self::Raw) -> Self { p.into() }
}

impl From<AslrPolicy> for PROCESS_MITIGATION_ASLR_POLICY {
    fn from(i: AslrPolicy) -> Self {
        let mut o = Self::default();
        o.set_EnableBottomUpRandomization   (i.enable_bottom_up_randomization   as u32);
        o.set_EnableForceRelocateImages     (i.enable_force_relocate_images     as u32);
        o.set_EnableHighEntropy             (i.enable_high_entropy              as u32);
        o.set_DisallowStrippedImages        (i.disallow_stripped_images         as u32);
        o
    }
}

impl From<PROCESS_MITIGATION_ASLR_POLICY> for AslrPolicy {
    fn from(i: PROCESS_MITIGATION_ASLR_POLICY) -> Self {
        let mut o = Self::default();
        o.enable_bottom_up_randomization    = i.EnableBottomUpRandomization()   != 0;
        o.enable_force_relocate_images      = i.EnableForceRelocateImages()     != 0;
        o.enable_high_entropy               = i.EnableHighEntropy()             != 0;
        o.disallow_stripped_images          = i.DisallowStrippedImages()        != 0;
        o
    }
}
