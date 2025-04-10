use super::*;
use crate::*;
use bytemuck::*;
use winapi::um::winnt::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-process_mitigation_dynamic_code_policy)\]
/// ≈ [PROCESS_MITIGATION_DYNAMIC_CODE_POLICY]
///
#[derive(Clone, Copy, Debug)]
#[derive(Default, Zeroable)]
pub struct DynamicCodePolicy {
    pub prohibit_dynamic_code:  bool,
    pub allow_thread_opt_out:   bool,
    pub allow_remote_downgrade: bool,
    #[doc(hidden)] pub _reserved_flags: ()
}

unsafe impl GetPolicy for PROCESS_MITIGATION_DYNAMIC_CODE_POLICY {
    type Raw = Self;
    fn ty() -> process::mitigation::Policy { process::DynamicCodePolicy }
    fn from_policy(p: Self::Raw) -> Self { p }
}

impl SetPolicy for PROCESS_MITIGATION_DYNAMIC_CODE_POLICY {
    fn into_policy(self) -> Self::Raw { self }
}

unsafe impl GetPolicy for DynamicCodePolicy {
    type Raw = PROCESS_MITIGATION_DYNAMIC_CODE_POLICY;
    fn ty() -> process::mitigation::Policy { process::DynamicCodePolicy }
    fn from_policy(p: Self::Raw) -> Self { p.into() }
}

impl SetPolicy for DynamicCodePolicy {
    fn into_policy(self) -> Self::Raw { self.into() }
}

impl From<DynamicCodePolicy> for PROCESS_MITIGATION_DYNAMIC_CODE_POLICY {
    fn from(i: DynamicCodePolicy) -> Self {
        let mut o = Self::default();
        o.set_ProhibitDynamicCode   (i.prohibit_dynamic_code    as u32);
        o.set_AllowThreadOptOut     (i.allow_thread_opt_out     as u32);
        o.set_AllowRemoteDowngrade  (i.allow_remote_downgrade   as u32);
        o
    }
}

impl From<PROCESS_MITIGATION_DYNAMIC_CODE_POLICY> for DynamicCodePolicy {
    fn from(i: PROCESS_MITIGATION_DYNAMIC_CODE_POLICY) -> Self {
        let mut o = Self::default();
        o.prohibit_dynamic_code     = i.ProhibitDynamicCode()   != 0;
        o.allow_thread_opt_out      = i.AllowThreadOptOut()     != 0;
        o.allow_remote_downgrade    = i.AllowRemoteDowngrade()  != 0;
        o
    }
}
