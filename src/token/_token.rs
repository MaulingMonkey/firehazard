//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/secauthz/access-tokens)\]
//! Access token handle types and related functions

mod handle;                     pub use handle::*;
mod psuedo_handle;              pub use psuedo_handle::*;

pub use funcs::*;
pub(crate) mod funcs {
    include!("funcs/create_restricted_token.rs");
    include!("funcs/is_token_restricted.rs");
    pub mod open_process_token;
    pub mod open_thread_token;
    include!("funcs/revert_to_self.rs");
}
