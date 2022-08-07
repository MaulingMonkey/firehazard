//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/secauthz/access-tokens)\]
//! Access token handle types and related functions

mod handle;                     pub use handle::*;
mod psuedo_handle;              pub use psuedo_handle::*;

pub(crate) mod funcs {
    #[doc(hidden)] pub fn _placeholder() {}
}
