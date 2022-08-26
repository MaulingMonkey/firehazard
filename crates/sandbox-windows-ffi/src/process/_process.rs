//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessw)\]
//! Process [`OwnedHandle`] and related fns

#[path = "process_funcs.rs"]
pub(crate) mod funcs;                   pub use funcs::*;
mod process_creation_flags;             pub use process_creation_flags::*;
mod process_handle;                     pub use process_handle::*;
mod process_information;                pub use process_information::*;
mod process_owned_handle;               pub use process_owned_handle::*;
mod process_psuedo_handle;              pub use process_psuedo_handle::*;
mod process_startup_info;               pub use process_startup_info::*;
mod process_thread_attribute_list;      pub use process_thread_attribute_list::*;

#[path = "creation/_creation.rs"]       pub mod creation;
#[path = "process_environment.rs"]      pub mod environment;

pub type Id = u32;
