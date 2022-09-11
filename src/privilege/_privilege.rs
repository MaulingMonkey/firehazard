#![doc = include_str!("_privilege.md")]

#[allow(unused_imports)] use crate::privilege; // doc purpouses
mod privilege_attributes;           pub use privilege_attributes::*;
mod privilege_luid;                 pub use privilege_luid::*;
mod privilege_name;                 pub use privilege_name::*;



pub use funcs::*;
pub(crate) mod funcs {
    mod adjust_token_privileges;        pub use adjust_token_privileges::*;
    mod lookup_privilege;               pub use lookup_privilege::*;
}
