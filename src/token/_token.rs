//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/secauthz/access-tokens)\]
//! Access token handle types and related functions

mod token_handle;                       pub use token_handle::*;
mod token_psuedo_handle;                pub use token_psuedo_handle::*;

pub use boxes::*;
mod boxes {
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
    mod token_source;                   pub use token_source::*;
    mod token_type;                     pub use token_type::*;
}

pub use funcs::*;
pub(crate) mod funcs {
    include!("funcs/create_restricted_token.rs");
    include!("funcs/duplicate_token_ex.rs");
    include!("funcs/get_current_x_token.rs");
    pub mod get_token_information;
    include!("funcs/is_token_restricted.rs");
    pub mod open_process_token;
    pub mod open_thread_token;
    include!("funcs/revert_to_self.rs");
    include!("funcs/set_thread_token.rs");
    pub mod set_token_information;
}
