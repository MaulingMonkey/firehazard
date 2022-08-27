//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/procthread/job-objects)\]
//! Job Object types and fns

mod job_handles;                        pub use job_handles::*;
mod job_information;                    pub use job_information::*;

#[path = "job_funcs.rs"] pub(crate) mod funcs; pub use funcs::*;
