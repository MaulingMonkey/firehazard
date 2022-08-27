//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/sysinfo/kernel-objects)\]
//! Kernel Object `HANDLE` wrapping types and functions

#[path = "handle_handles.rs"]   mod handles;            pub use handles::*;
#[path = "handle_funcs.rs"]     pub(crate) mod funcs;   pub use funcs::*;
#[path = "handle_traits.rs"]    pub(crate) mod traits;  pub use traits::*;
