//! \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/secauthz/access-tokens)\]
//! Access token handle types and related functions

mod token_handles;                      pub use token_handles::*;

pub use boxes::*;
mod boxes {
    mod boxes_util;                     use boxes_util::*;
    mod token_appcontainer_information; pub use token_appcontainer_information::*;
    mod token_default_dacl;             pub use token_default_dacl::*;
    mod token_groups_and_privileges;    pub use token_groups_and_privileges::*;
    mod token_groups;                   pub use token_groups::*;
    mod token_mandatory_label;          pub use token_mandatory_label::*;
    mod token_owner;                    pub use token_owner::*;
    mod token_primary_group;            pub use token_primary_group::*;
    mod token_privileges;               pub use token_privileges::*;
    mod token_user;                     pub use token_user::*;
}

pub use values::*;
mod values {
    mod token_access_rights;            pub use token_access_rights::*;
    mod token_elevation_type;           pub use token_elevation_type::*;
    mod token_elevation;                pub use token_elevation::*;
    mod token_mandatory_policy;         pub use token_mandatory_policy::*;
    mod token_restricted_flags;         pub use token_restricted_flags::*;
    mod token_source;                   pub use token_source::*;
    mod token_type;                     pub use token_type::*;
}

pub use funcs::*;
pub(crate) mod funcs {
    use crate::prelude::*;
    include!(r"funcs\create_restricted_token.rs");
    include!(r"funcs\duplicate_token_ex.rs");
    include!(r"funcs\get_current_x_token.rs");
    pub mod get_token_information;
    include!(r"funcs\is_token_restricted.rs");
    include!(r"funcs\open_process_token.rs");
    include!(r"funcs\open_thread_token.rs");
    include!(r"funcs\revert_to_self.rs");
    include!(r"funcs\set_thread_token.rs");
    pub mod set_token_information;
}
