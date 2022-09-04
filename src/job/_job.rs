//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/procthread/job-objects)\]
//! Job Object types and fns

mod job_handles;                        pub use job_handles::*;
mod job_information;                    pub use job_information::*;

#[path = "job_funcs.rs"] pub(crate) mod funcs; pub use funcs::*;

/// JOB_OBJECT_* and JOBOBJECT_* stuff
pub mod object {
    pub mod limit;
    pub mod uilimit;

    mod basic_ui_restrictions;          pub use basic_ui_restrictions::*;
    mod end_of_job_time;                pub use end_of_job_time::*;
    mod group_information;              pub use group_information::*;
    mod limit_information;              pub use limit_information::*;
    mod security_limit_information;     //pub use security_limit_information::*;
}
