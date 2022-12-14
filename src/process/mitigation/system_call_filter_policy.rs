use super::*;
use crate::*;
use winapi::um::winnt::*;



unsafe impl GetPolicy for PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY {
    type Raw = Self;
    fn ty() -> process::mitigation::Policy { process::SystemCallFilterPolicy }
    fn from_policy(p: Self::Raw) -> Self { p }
}

impl SetPolicy for PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY {
    fn into_policy(self) -> Self::Raw { self }
}
