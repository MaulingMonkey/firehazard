use super::*;
use crate::*;
use bytemuck::*;
use winapi::um::winnt::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-process_mitigation_strict_handle_check_policy)\]
/// ~ [PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY]
#[derive(Clone, Copy, Debug)]
#[derive(Default, Zeroable)]
pub struct StrictHandleCheckPolicy {
    pub raise_exception_on_invalid_handle_reference:    bool,
    pub handle_exceptions_permanently_enabled:          bool,
    #[doc(hidden)] pub _reserved_flags: ()
}

unsafe impl GetPolicy for PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY {
    type Raw = Self;
    fn ty() -> process::mitigation::Policy { process::StrictHandleCheckPolicy }
    fn from_policy(p: Self::Raw) -> Self { p }
}

impl SetPolicy for PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY {
    fn into_policy(self) -> Self::Raw { self }
}

unsafe impl GetPolicy for StrictHandleCheckPolicy {
    type Raw = PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY;
    fn ty() -> process::mitigation::Policy { process::StrictHandleCheckPolicy }
    fn from_policy(p: Self::Raw) -> Self { p.into() }
}

impl SetPolicy for StrictHandleCheckPolicy {
    fn into_policy(self) -> Self::Raw { self.into() }
}

impl From<StrictHandleCheckPolicy> for PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY {
    fn from(i: StrictHandleCheckPolicy) -> Self {
        let mut o = Self::default();
        o.set_RaiseExceptionOnInvalidHandleReference    (i.raise_exception_on_invalid_handle_reference  as u32);
        o.set_HandleExceptionsPermanentlyEnabled        (i.handle_exceptions_permanently_enabled        as u32);
        o
    }
}

impl From<PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY> for StrictHandleCheckPolicy {
    fn from(i: PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY) -> Self {
        let mut o = Self::default();
        o.raise_exception_on_invalid_handle_reference   = i.RaiseExceptionOnInvalidHandleReference()    != 0;
        o.handle_exceptions_permanently_enabled         = i.HandleExceptionsPermanentlyEnabled()        != 0;
        o
    }
}
