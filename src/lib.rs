#![forbid(unsafe_op_in_unsafe_fn)]
#![deny(unreachable_patterns)]

pub use boxes::*;
mod boxes {
    mod token_appcontainer_information; pub use token_appcontainer_information::*;
    mod token_groups_and_privileges;pub use token_groups_and_privileges::*;
    mod token_groups;               pub use token_groups::*;
    mod token_mandatory_label;      pub use token_mandatory_label::*;
    mod token_owner;                pub use token_owner::*;
    mod token_primary_group;        pub use token_primary_group::*;
    mod token_privileges;           pub use token_privileges::*;
    mod token_user;                 pub use token_user::*;
}

pub mod error;

pub mod handle {
    mod access_token;               pub use access_token::*;
    mod psuedo_access_token;        pub use psuedo_access_token::*;
}

pub mod refs {
    mod sid;                        pub use sid::*;
}

pub(crate) use util::*;
mod util {
    mod bits32;                     pub(crate) use bits32::*;
}

pub use values::*;
mod values {
    mod luid_and_attributes;        pub use luid_and_attributes::*;
    mod luid;                       pub use luid::*;
    mod privilege_luid;             pub use privilege_luid::*;
}
