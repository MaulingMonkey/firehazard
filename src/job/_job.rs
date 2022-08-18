//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/procthread/job-objects)\]
//! Job Object types and fns

mod job_owned_handle;                   pub use job_owned_handle::*;

#[path = "job_funcs.rs"] pub(crate) mod funcs; pub use funcs::*;
