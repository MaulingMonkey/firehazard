#![forbid(unsafe_op_in_unsafe_fn)]
#![deny(unreachable_patterns)]

#[path = "access/_access.rs"]           pub mod access;         //#[doc(hidden)] pub use access::constants::*;
#[path = "ace/_ace.rs"]                 pub mod ace;            //#[doc(hidden)] pub use ace::funcs::*;
#[path = "acl/_acl.rs"]                 pub mod acl;            //#[doc(hidden)] pub use acl::funcs::*;
#[path = "alloc/_alloc.rs"]             pub mod alloc;
#[path = "debug/_debug.rs"]             pub mod debug;          #[doc(hidden)] pub use debug::funcs::*;
#[path = "desktop/_desktop.rs"]         pub mod desktop;        #[doc(hidden)] pub use desktop::funcs::*;
#[path = "handle/_handle.rs"]           pub mod handle;         #[doc(hidden)] pub use handle::funcs::*;
#[path = "job/_job.rs"]                 pub mod job;            #[doc(hidden)] pub use job::funcs::*;
#[path = "policy/_policy.rs"]           pub mod policy;         #[doc(hidden)] pub use policy::funcs::*;
#[path = "privilege/_privilege.rs"]     pub mod privilege;      #[doc(hidden)] pub use privilege::funcs::*;
#[path = "process/_process.rs"]         pub mod process;        #[doc(hidden)] pub use process::funcs::*;
#[path = "security/_security.rs"]       pub mod security;       //#[doc(hidden)] pub use security::funcs::*;
#[path = "sid/_sid.rs"]                 pub mod sid;            #[doc(hidden)] pub use sid::funcs::*;
#[path = "thread/_thread.rs"]           pub mod thread;         #[doc(hidden)] pub use thread::funcs::*;
#[path = "token/_token.rs"]             pub mod token;          #[doc(hidden)] pub use token::funcs::*;
#[path = "winsta/_winsta.rs"]           pub mod winsta;         #[doc(hidden)] pub use winsta::funcs::*;

pub mod error;                  #[doc(hidden)] pub use error::*;

pub(crate) use util::*;
mod util {
    mod bits32;                     pub(crate) use bits32::*;
    pub fn none2null<T>(_: Option<std::convert::Infallible>) -> *mut T { std::ptr::null_mut() }
}

pub use values::*;
mod values {
    mod local_string;               pub use local_string::*;
    mod luid_and_attributes;        pub use luid_and_attributes::*;
    mod luid;                       pub use luid::*;
}
