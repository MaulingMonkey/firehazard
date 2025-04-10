use super::*;
use crate::*;
use bytemuck::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntddk/ns-ntddk-process_mitigation_sehop_policy)\]
/// ≈ PROCESS_MITIGATION_SEHOP_POLICY
///
#[derive(Clone, Copy, Debug)]
#[derive(Default, Zeroable)]
pub struct SehopPolicy {
    pub enable_sehop: bool,
    #[doc(hidden)] pub _reserved_flags: ()
}

unsafe impl GetPolicy for PROCESS_MITIGATION_SEHOP_POLICY {
    type Raw = Self;
    fn ty() -> process::mitigation::Policy { process::SEHOPPolicy }
    fn from_policy(p: Self::Raw) -> Self { p }
}

impl SetPolicy for PROCESS_MITIGATION_SEHOP_POLICY {
    fn into_policy(self) -> Self::Raw { self }
}

unsafe impl GetPolicy for SehopPolicy {
    type Raw = u32; // XXX
    fn ty() -> process::mitigation::Policy { process::SEHOPPolicy }
    fn from_policy(p: Self::Raw) -> Self { PROCESS_MITIGATION_SEHOP_POLICY { Flags: p }.into() }
}

impl SetPolicy for SehopPolicy {
    fn into_policy(self) -> Self::Raw { PROCESS_MITIGATION_SEHOP_POLICY::from(self).Flags }
}

impl From<SehopPolicy> for PROCESS_MITIGATION_SEHOP_POLICY {
    fn from(i: SehopPolicy) -> Self {
        let mut o = Self::default();
        o.set_EnableSehop(i.enable_sehop as u32);
        o
    }
}

impl From<PROCESS_MITIGATION_SEHOP_POLICY> for SehopPolicy {
    fn from(i: PROCESS_MITIGATION_SEHOP_POLICY) -> Self {
        let mut o = Self::default();
        o.enable_sehop = i.EnableSehop() != 0;
        o
    }
}

// XXX: not (yet?) defined by winapi
#[allow(non_snake_case)] #[derive(Clone, Copy, Debug, Default)] #[repr(C)] struct PROCESS_MITIGATION_SEHOP_POLICY { Flags: u32 }
#[allow(non_snake_case)] impl PROCESS_MITIGATION_SEHOP_POLICY {
    pub fn set_EnableSehop(&mut self, value: u32) { const M : u32 = 1 << 0; if value == 0 { self.Flags &= !M } else { self.Flags |= M } }

    pub fn EnableSehop(&self) -> u32 { (self.Flags >> 0) & 1 }
}
