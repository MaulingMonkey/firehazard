//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessmitigationpolicy)\]
//! Process mitigation policy types and functions

pub use funcs::*;
pub(crate) mod funcs {
    pub mod set_process_mitigation_policy;
}
