//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/procthread/job-objects)\]
//! Job Object types and fns

mod job_owned_handle;                   pub use job_owned_handle::*;
mod job_information;                    pub use job_information::*;

#[path = "job_funcs.rs"] pub(crate) mod funcs; pub use funcs::*;
