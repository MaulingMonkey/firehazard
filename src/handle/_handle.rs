//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/sysinfo/kernel-objects)\]
//! Kernel Object `HANDLE` wrapping types and functions

//#[path = "handle_borrowed.rs"]  mod borrowed;           pub use borrowed::*;
#[path = "handle_owned.rs"]     mod owned;              pub use owned::*;
#[path = "handle_handle.rs"]    mod value;              pub use value::*;
//#[path = "handle_psuedo.rs"]    mod psuedo;             pub use psuedo::*;
#[path = "handle_funcs.rs"]     pub(crate) mod funcs;   pub use funcs::*;
