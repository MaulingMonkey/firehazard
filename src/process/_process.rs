//! \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessw)\]
//! Process [`OwnedHandle`] and related fns

#[path = "process_funcs.rs"]
pub(crate) mod funcs;                   pub use funcs::*;
mod process_creation_flags;             pub use process_creation_flags::*;
mod process_handles;                    pub use process_handles::*;
mod process_information;                pub use process_information::*;
mod process_startup_info;               pub use process_startup_info::*;
mod process_thread_attribute_list;      pub use process_thread_attribute_list::*;

#[path = "creation/_creation.rs"]       pub mod creation;
#[path = "mitigation/_mitigation.rs"]   pub mod mitigation;
#[path = "process_environment.rs"]      pub mod environment;

/// DWORD / u32 process identifier.  In practice, only 16 bits are generally used - reuse / collisions can occur quickly.
pub type Id = u32;

pub use mitigation::Policy::*; // export e.g. `ProcessDEPPolicy` as `process::DEPPolicy`
