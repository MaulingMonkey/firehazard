//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessmitigationpolicy)\]
//! Process mitigation policy types and functions

use crate::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocessmitigationpolicy)\] GetProcessMitigationPolicy /<br>
/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessmitigationpolicy)\] SetProcessMitigationPolicy parameters
///
/// ### Safety
/// [`IntoPolicy::Policy`] must be ABI-compatible with whatever policy enumerand [`IntoPolicy::ty`] returns.
pub unsafe trait IntoPolicy {
    /// POD-ish type that will be passed directly to SetProcessMitigationPolicy
    type Policy : Default;

    fn ty() -> process::mitigation::Policy;
    fn into_policy(self) -> Self::Policy;
    fn from_policy(p: Self::Policy) -> Self;
}
