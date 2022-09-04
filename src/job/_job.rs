//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/procthread/job-objects)\]
//! Job Object types and fns
//!
//! Note that jobs aren't inescapeable:
//! *   [project-zero: Blog: In-Console-Able (2015)](https://googleprojectzero.blogspot.com/2015/05/in-console-able.html)
//! *   [project-zero: Issue 213: Windows: Console Driver Job Object Process Limit Bypass](https://bugs.chromium.org/p/project-zero/issues/detail?id=213&redir=1)



mod job_handles;                        pub use job_handles::*;
mod job_information;                    pub use job_information::*;

#[path = "job_funcs.rs"] pub(crate) mod funcs; pub use funcs::*;

/// JOB_OBJECT_* and JOBOBJECT_* stuff
pub mod object {
    pub mod limit;
    pub mod uilimit;

    mod basic_accounting_information;   pub use basic_accounting_information::*;
    mod basic_ui_restrictions;          pub use basic_ui_restrictions::*;
    mod cpu_rate_control;               pub use cpu_rate_control::*;
    mod end_of_job_time;                pub use end_of_job_time::*;
    mod group_information;              pub use group_information::*;
    mod limit_information;              pub use limit_information::*;
    mod net_rate_control;               pub use net_rate_control::*;
    mod security_limit_information;     //pub use security_limit_information::*;
}
