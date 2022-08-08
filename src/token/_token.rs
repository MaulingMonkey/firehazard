//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/secauthz/access-tokens)\]
//! Access token handle types and related functions

mod access_rights;              pub use access_rights::*;
mod handle;                     pub use handle::*;
mod psuedo_handle;              pub use psuedo_handle::*;

pub use boxes::*;
mod boxes {
    mod token_appcontainer_information; pub use token_appcontainer_information::*;
    mod token_groups_and_privileges;    pub use token_groups_and_privileges::*;
    mod token_groups;                   pub use token_groups::*;
    mod token_mandatory_label;          pub use token_mandatory_label::*;
    mod token_owner;                    pub use token_owner::*;
    mod token_primary_group;            pub use token_primary_group::*;
    mod token_privileges;               pub use token_privileges::*;
    mod token_user;                     pub use token_user::*;
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
