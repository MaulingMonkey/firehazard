use crate::*;
use crate::policy::*;
use bytemuck::*;



/// \[[docs.microsoft.com](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntddk/ns-ntddk-process_mitigation_sehop_policy)\]
/// ~ PROCESS_MITIGATION_SEHOP_POLICY
#[derive(Clone, Copy, Debug)]
#[derive(Default, Zeroable)]
pub struct SehopPolicy {
    pub enable_sehop: bool,
    #[doc(hidden)] pub _reserved_flags: ()
}

unsafe impl IntoPolicy for PROCESS_MITIGATION_SEHOP_POLICY {
    type Policy = Self;
    fn into_policy(self) -> (process::mitigation::Policy, Self::Policy) { (process::SEHOPPolicy, self) }
}

unsafe impl IntoPolicy for SehopPolicy {
    type Policy = u32; // XXX
    fn into_policy(self) -> (process::mitigation::Policy, Self::Policy) { (process::SEHOPPolicy, PROCESS_MITIGATION_SEHOP_POLICY::from(self).Flags) }
}

impl From<SehopPolicy> for PROCESS_MITIGATION_SEHOP_POLICY {
    fn from(i: SehopPolicy) -> Self {
        let mut o = PROCESS_MITIGATION_SEHOP_POLICY::default();
        o.set_EnableSehop(i.enable_sehop as u32);
        o
    }
}

// XXX: not (yet?) defined by winapi
#[allow(non_snake_case)] #[derive(Clone, Copy, Debug, Default)] #[repr(C)] struct PROCESS_MITIGATION_SEHOP_POLICY { Flags: u32 }
#[allow(non_snake_case)] impl PROCESS_MITIGATION_SEHOP_POLICY {
    pub fn set_EnableSehop(&mut self, value: u32) { const M : u32 = 1 << 0; if value == 0 { self.Flags &= !M } else { self.Flags |= M } }
}
