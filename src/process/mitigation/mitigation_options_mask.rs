use super::*;
use crate::*;



#[doc(alias = "ProcessMitigationOptionsMask")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocessmitigationpolicy)\]
/// ProcessMitigationOptionsMask
///
/// Per the observed usage in
/// [chromium/sandbox/win/src/process_mitigations.cc](https://github.com/chromium/chromium/blob/ddcdad76569bdac5d28889fdd1b3dc96ed6b7ab4/sandbox/win/src/process_mitigations.cc),
/// this is actually a mask of valid [`process::creation::MitigationPolicy`] bits.
///
/// When interpreting said bits, many values are `..._RESERVED` and should not be used as is - so I've chosen [`process::creation::MitigationPolicyMask`] is the least awkward fit.
///
pub type OptionsMask = process::creation::MitigationPolicyMask;

unsafe impl GetPolicy for OptionsMask {
    type Raw = Self;
    fn ty() -> process::mitigation::Policy { process::MitigationOptionsMask }
    fn from_policy(p: Self::Raw) -> Self { p }
}

// intentionally omitted: impl SetPolicy for OptionsMask
