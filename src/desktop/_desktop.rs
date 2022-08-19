//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/winstation/desktops)\]
//! Desktop APIs

mod desktop_access_rights;              pub use desktop_access_rights::*;
mod desktop_owned_handle;               pub use desktop_owned_handle::*;

pub use funcs::*;
#[path = "desktop_funcs.rs"] pub(crate) mod funcs;
