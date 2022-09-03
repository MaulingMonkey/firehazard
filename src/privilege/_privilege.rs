#![doc = include_str!("_privilege.md")]

#[allow(unused_imports)] use crate::privilege; // doc purpouses
mod privilege_luid;                 pub use privilege_luid::*;
// TODO: privilege::Name that heavily hints at converting to privilege::Luid s?

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_privileges)\] LUID_AND_ATTRIBUTES, in the context of TOKEN_PRIVILEGES specifically
pub type LuidAndAttributes = crate::LuidAndAttributes<crate::privilege::Luid>;



pub use funcs::*;
pub(crate) mod funcs {
    mod adjust_token_privileges;        pub use adjust_token_privileges::*;
    mod lookup_privilege;               pub use lookup_privilege::*;
}
