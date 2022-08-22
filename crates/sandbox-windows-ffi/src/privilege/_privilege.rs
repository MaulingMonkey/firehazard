//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/secauthz/privilege-constants)\]
//! Privilege related types and functions

mod privilege_luid;                 pub use privilege_luid::*;

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_privileges)\] LUID_AND_ATTRIBUTES, in the context of TOKEN_PRIVILEGES specifically
pub type LuidAndAttributes = crate::LuidAndAttributes<crate::privilege::Luid>;



pub use funcs::*;
pub(crate) mod funcs {
    mod adjust_token_privileges;        pub use adjust_token_privileges::*;
}
