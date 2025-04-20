#![doc = include_str!("_privilege.md")]

#[allow(unused_imports)] use crate::privilege; // doc purpouses
mod privilege_attributes;           pub use privilege_attributes::*;
mod privilege_luid;                 pub use privilege_luid::*;
mod privilege_name;                 pub use privilege_name::*;



pub use funcs::*;
pub(crate) mod funcs {
    use crate::prelude::*;
    include!(r"funcs\adjust_token_privileges.rs");
    include!(r"funcs\lookup_privilege.rs");
}
