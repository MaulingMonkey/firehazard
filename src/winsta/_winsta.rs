//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/winstation/window-stations)\]
//! Window Station APIs

mod winsta_owned_handle;                pub use winsta_owned_handle::*;

pub use funcs::*;
#[path = "winsta_funcs.rs"] pub(crate) mod funcs;
