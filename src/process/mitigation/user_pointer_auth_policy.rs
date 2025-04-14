use super::*;
use crate::*;
use bytemuck::*;



#[doc(alias = "PROCESS_MITIGATION_USER_POINTER_AUTH_POLICY")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntddk/ns-ntddk-process_mitigation_user_pointer_auth_policy)\]
/// ≈ PROCESS_MITIGATION_USER_POINTER_AUTH_POLICY
///
#[derive(Clone, Copy, Debug)]
#[derive(Default, Zeroable)]
pub struct UserPointerAuthPolicy {
    pub enable_pointer_auth_user_ip: bool,
    #[doc(hidden)] pub _reserved_flags: ()
}

unsafe impl GetPolicy for PROCESS_MITIGATION_USER_POINTER_AUTH_POLICY {
    type Raw = Self;
    fn ty() -> process::mitigation::Policy { process::UserPointerAuthPolicy }
    fn from_policy(p: Self::Raw) -> Self { p }
}

impl SetPolicy for PROCESS_MITIGATION_USER_POINTER_AUTH_POLICY {
    fn into_policy(self) -> Self::Raw { self }
}

unsafe impl GetPolicy for UserPointerAuthPolicy {
    type Raw = u32; // XXX
    fn ty() -> process::mitigation::Policy { process::UserPointerAuthPolicy }
    fn from_policy(p: Self::Raw) -> Self { PROCESS_MITIGATION_USER_POINTER_AUTH_POLICY { Flags: p }.into() }
}

impl SetPolicy for UserPointerAuthPolicy {
    fn into_policy(self) -> Self::Raw { PROCESS_MITIGATION_USER_POINTER_AUTH_POLICY::from(self).Flags }
}

impl From<UserPointerAuthPolicy> for PROCESS_MITIGATION_USER_POINTER_AUTH_POLICY {
    fn from(i: UserPointerAuthPolicy) -> Self {
        let mut o = PROCESS_MITIGATION_USER_POINTER_AUTH_POLICY::default();
        o.set_EnablePointerAuthUserIp(i.enable_pointer_auth_user_ip as u32);
        o
    }
}

impl From<PROCESS_MITIGATION_USER_POINTER_AUTH_POLICY> for UserPointerAuthPolicy {
    fn from(i: PROCESS_MITIGATION_USER_POINTER_AUTH_POLICY) -> Self {
        let mut o = Self::default();
        o.enable_pointer_auth_user_ip = i.EnablePointerAuthUserIp() != 0;
        o
    }
}

// XXX: not (yet?) defined by winapi
#[allow(non_snake_case)] #[derive(Clone, Copy, Debug, Default)] #[repr(C)] struct PROCESS_MITIGATION_USER_POINTER_AUTH_POLICY { Flags: u32 }
#[allow(non_snake_case)] impl PROCESS_MITIGATION_USER_POINTER_AUTH_POLICY {
    pub fn set_EnablePointerAuthUserIp(&mut self, value: u32) { const M : u32 = 1 << 0; if value == 0 { self.Flags &= !M } else { self.Flags |= M } }

    pub fn EnablePointerAuthUserIp(&self) -> u32 { (self.Flags >> 0) & 1 }
}
