#![forbid(unsafe_op_in_unsafe_fn)]
#![deny(unreachable_patterns)]

#[path = "ace/_ace.rs"]         pub mod ace;    //#[doc(hidden)] pub use ace::funcs::*;
#[path = "acl/_acl.rs"]         pub mod acl;    //#[doc(hidden)] pub use acl::funcs::*;
#[path = "alloc/_alloc.rs"]     pub mod alloc;
#[path = "privilege/_privilege.rs"] pub mod privilege;  #[doc(hidden)] pub use privilege::funcs::*;
#[path = "sid/_sid.rs"]         pub mod sid;    #[doc(hidden)] pub use sid::funcs::*;
#[path = "token/_token.rs"]     pub mod token;  #[doc(hidden)] pub use token::funcs::*;

pub mod error;

pub(crate) use util::*;
mod util {
    mod bits32;                     pub(crate) use bits32::*;
}

pub use values::*;
mod values {
    mod local_string;               pub use local_string::*;
    mod luid_and_attributes;        pub use luid_and_attributes::*;
    mod luid;                       pub use luid::*;
}
