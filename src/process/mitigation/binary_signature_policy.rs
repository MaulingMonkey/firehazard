use crate::*;
use crate::policy::*;
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
    fn into_policy(self) -> (process::mitigation::Policy, Self::Policy) { (process::SignaturePolicy, self) }
}

unsafe impl IntoPolicy for BinarySignaturePolicy {
    type Policy = PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY;
    fn into_policy(self) -> (process::mitigation::Policy, Self::Policy) { (process::SignaturePolicy, self.into()) }
}

impl From<BinarySignaturePolicy> for PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY {
    fn from(i: BinarySignaturePolicy) -> Self {
        let mut o = PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY::default();
        o.set_MicrosoftSignedOnly(i.microsoft_signed_only as u32);
        o.set_StoreSignedOnly(i.store_signed_only as u32);
        o.set_MitigationOptIn(i.mitigation_opt_in as u32);
        o
    }
}
