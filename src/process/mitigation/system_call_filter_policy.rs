use super::*;
use crate::*;
use bytemuck::*;
use winapi::um::winnt::*;
use core::fmt::{self, Debug, Display, Formatter};



#[doc(alias = "PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntddk/ns-ntddk-_process_mitigation_system_call_filter_policy)\]
/// ≈ [PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY]
///
#[derive(Clone, Copy, Debug)]
#[derive(Default, Zeroable)]
pub struct SystemCallFilterPolicy {
    pub filter_id: SystemCallFilterId,
    #[doc(hidden)] pub _reserved_flags: ()
}

unsafe impl GetPolicy for PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY {
    type Raw = Self;
    fn ty() -> process::mitigation::Policy { process::SystemCallFilterPolicy }
    fn from_policy(p: Self::Raw) -> Self { p }
}

impl SetPolicy for PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY {
    fn into_policy(self) -> Self::Raw { self }
}

unsafe impl GetPolicy for SystemCallFilterPolicy {
    type Raw = PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY;
    fn ty() -> process::mitigation::Policy { process::SystemCallFilterPolicy }
    fn from_policy(p: Self::Raw) -> Self { p.into() }
}

impl SetPolicy for SystemCallFilterPolicy {
    fn into_policy(self) -> Self::Raw { self.into() }
}

impl From<SystemCallFilterPolicy> for PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY {
    fn from(i: SystemCallFilterPolicy) -> Self {
        let mut o = Self::default();
        o.Flags = 0
            | (i.filter_id.get() << 0)
            ;
        o
    }
}

impl From<PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY> for SystemCallFilterPolicy {
    fn from(i: PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY) -> Self {
        let mut o = Self::default();
        o.filter_id = SystemCallFilterId::from_truncate(i.Flags >> 0);
        o
    }
}



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntddk/ns-ntddk-_process_mitigation_system_call_filter_policy)\]
/// ≈ typeof([PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY]::FilterId)
///
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Default, Zeroable)]
pub struct SystemCallFilterId(u32);

impl SystemCallFilterId {
    pub const fn from_truncate(filter_id: u32) -> Self { Self(filter_id & 0b_1111) }

    #[track_caller] pub const fn from_constant<const N: u32>() -> Self {
        const { assert!(N <= 15, "SystemCallFilterId::from_constant::<5+ bit integer>(): PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY::FilterId only has 4 bits") };
        Self::from_truncate(N)
    }

    pub(crate) const fn get(&self) -> u32 { self.0 }
}

impl Debug   for SystemCallFilterId { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Debug  ::fmt(&self.0, fmt) } }
impl Display for SystemCallFilterId { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Display::fmt(&self.0, fmt) } }

impl From<SystemCallFilterId> for u32 { fn from(filter_id: SystemCallFilterId) -> u32 { filter_id.0 } }
