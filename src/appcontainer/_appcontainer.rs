//! \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/secauthz/appcontainer-isolation)\]
//! AppContainer isolation management functions

pub use funcs::*;
#[path = "appcontainer_funcs.rs"] pub(crate) mod funcs;
