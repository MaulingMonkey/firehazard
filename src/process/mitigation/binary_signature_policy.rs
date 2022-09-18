use super::*;
use crate::*;
use bytemuck::*;
use winapi::um::winnt::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-process_mitigation_binary_signature_policy)\]
/// ~ [PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY]
#[derive(Clone, Copy, Debug)]
#[derive(Default, Zeroable)]
pub struct BinarySignaturePolicy {
    pub microsoft_signed_only:  bool,
    pub store_signed_only:      bool,
    pub mitigation_opt_in:      bool,
    #[doc(hidden)] pub _reserved_flags: ()
}

unsafe impl IntoPolicy for PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY {
    type Policy = Self;
    fn ty() -> process::mitigation::Policy { process::SignaturePolicy }
    fn into_policy(self) -> Self::Policy { self }
    fn from_policy(p: Self::Policy) -> Self { p }
}

unsafe impl IntoPolicy for BinarySignaturePolicy {
    type Policy = PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY;
    fn ty() -> process::mitigation::Policy { process::SignaturePolicy }
    fn into_policy(self) -> Self::Policy { self.into() }
    fn from_policy(p: Self::Policy) -> Self { p.into() }
}

impl From<BinarySignaturePolicy> for PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY {
    fn from(i: BinarySignaturePolicy) -> Self {
        let mut o = Self::default();
        o.set_MicrosoftSignedOnly   (i.microsoft_signed_only    as u32);
        o.set_StoreSignedOnly       (i.store_signed_only        as u32);
        o.set_MitigationOptIn       (i.mitigation_opt_in        as u32);
        o
    }
}

impl From<PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY> for BinarySignaturePolicy {
    fn from(i: PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY) -> Self {
        let mut o = Self::default();
        o.microsoft_signed_only = i.MicrosoftSignedOnly()   != 0;
        o.store_signed_only     = i.StoreSignedOnly()       != 0;
        o.mitigation_opt_in     = i.MitigationOptIn()       != 0;
        o
    }
}
