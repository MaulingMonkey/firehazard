//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessmitigationpolicy)\]
//! Process mitigation policy types and functions

use crate::*;
use winapi::um::winnt::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessmitigationpolicy)\]
/// SetProcessMitigationPolicy parameters
///
/// ### Safety
/// [`IntoPolicy::Policy`] must be ABI-compatible with whatever policy enumerand [`IntoPolicy::into`] returns.
pub unsafe trait IntoPolicy {
    /// POD-ish type that will be passed directly to SetProcessMitigationPolicy
    type Policy;

    fn into(self) -> (process::mitigation::Policy, Self::Policy);
}

pub use funcs::*;
pub(crate) mod funcs {
    include!("funcs/heap_enable_termination_on_corruption.rs");
    include!("funcs/set_process_mitigation_policy.rs");
}
