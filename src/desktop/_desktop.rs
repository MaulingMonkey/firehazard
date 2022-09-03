//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/winstation/desktops)\]
//! Desktop APIs

mod desktop_access_rights;              pub use desktop_access_rights::*;
mod desktop_flags;                      pub use desktop_flags::*;
mod desktop_handles;                    pub use desktop_handles::*;

pub use funcs::*;
#[path = "desktop_funcs.rs"] pub(crate) mod funcs;
