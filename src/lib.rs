#![doc = include_str!("../Readme.md")]
#![cfg(windows)]

#![no_std]
#![cfg_attr(not(std), allow(unused_imports))]
#![forbid(unsafe_op_in_unsafe_fn)]
#![deny(unreachable_patterns)]

#[cfg(std)] extern crate std;
#[doc(hidden)] pub extern crate abistr;
#[doc(hidden)] pub extern crate winapi;

#[path = "macros/_macros.rs"] #[macro_use] mod macros;
#[path = "access/_access.rs"]           pub mod access;         //#[doc(hidden)] pub use access::constants::*;
#[path = "ace/_ace.rs"]                 pub mod ace;            //#[doc(hidden)] pub use ace::funcs::*;
#[path = "acl/_acl.rs"]                 pub mod acl;            //#[doc(hidden)] pub use acl::funcs::*;
#[path = "alloc/_alloc.rs"]             pub mod alloc;          #[doc(hidden)] pub use alloc::funcs::*;
#[path = "appcontainer/_appcontainer.rs"] pub mod appcontainer; #[doc(hidden)] pub use appcontainer::funcs::*;
#[path = "capability/_capability.rs"]   pub mod capability;
#[path = "debug/_debug.rs"]             pub mod debug;          #[doc(hidden)] pub use debug::funcs::*;
#[path = "desktop/_desktop.rs"]         pub mod desktop;        #[doc(hidden)] pub use desktop::funcs::*;
#[path = "handle/_handle.rs"]           pub mod handle;         #[doc(hidden)] pub use handle::{funcs::*, traits::*};
#[path = "job/_job.rs"]                 pub mod job;            #[doc(hidden)] pub use job::funcs::*;
#[path = "misc/_misc.rs"]               mod misc;               #[doc(hidden)] pub use misc::funcs::*;
#[path = "io/_io.rs"]                   pub mod io;             #[doc(hidden)] pub use io::funcs::*;
#[path = "privilege/_privilege.rs"]     pub mod privilege;      #[doc(hidden)] pub use privilege::funcs::*;
#[path = "process/_process.rs"]         pub mod process;        #[doc(hidden)] pub use process::funcs::*;
#[path = "security/_security.rs"]       pub mod security;       //#[doc(hidden)] pub use security::funcs::*;
#[path = "sid/_sid.rs"]                 pub mod sid;            #[doc(hidden)] pub use sid::funcs::*;
#[path = "thread/_thread.rs"]           pub mod thread;         #[doc(hidden)] pub use thread::funcs::*;
#[path = "token/_token.rs"]             pub mod token;          #[doc(hidden)] pub use token::funcs::*;
#[path = "util/_util.rs"]               mod util;               pub(crate) use util::*;
#[path = "winsta/_winsta.rs"]           pub mod winsta;         #[doc(hidden)] pub use winsta::funcs::*;

mod error;                              pub use error::*;


pub use values::*;
mod values {
    mod luid;                       pub use luid::*;
}
