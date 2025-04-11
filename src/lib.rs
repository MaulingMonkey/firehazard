#![doc = include_str!("../Readme.md")]
#![cfg(windows)]

#![no_std]
#![cfg_attr(not(std), allow(unused_imports))]
#![forbid(unsafe_op_in_unsafe_fn)]
#![deny(unreachable_patterns)]

#[cfg(std)] extern crate std;
#[doc(hidden)] pub extern crate abistr;
#[doc(hidden)] pub extern crate ialloc;
#[doc(hidden)] pub extern crate winapi;

#[path = r"macros\_macros.rs"] #[macro_use] mod macros;
#[path = r"access\_access.rs"]              pub mod access;         //#[doc(hidden)] pub use access::constants::*;
#[path = r"ace\_ace.rs"]                    pub mod ace;            //#[doc(hidden)] pub use ace::funcs::*;
#[path = r"acl\_acl.rs"]                    pub mod acl;            //#[doc(hidden)] pub use acl::funcs::*;
#[path = r"alloc\_alloc.rs"]                pub mod alloc;          #[doc(hidden)] pub use alloc::funcs::*;
#[path = r"appcontainer\_appcontainer.rs"]  pub mod appcontainer;   #[doc(hidden)] pub use appcontainer::funcs::*;
#[path = r"capability\_capability.rs"]      pub mod capability;
#[path = r"debug\_debug.rs"]                pub mod debug;          #[doc(hidden)] pub use debug::funcs::*;
#[path = r"desktop\_desktop.rs"]            pub mod desktop;        #[doc(hidden)] pub use desktop::funcs::*;
#[path = r"file\_file.rs"]                  pub mod file;
#[path = r"handle\_handle.rs"]              pub mod handle;         #[doc(hidden)] pub use handle::{funcs::*, traits::*};
#[path = r"io\_io.rs"]                      pub mod io;             #[doc(hidden)] pub use io::funcs::*;
#[path = r"job\_job.rs"]                    pub mod job;            #[doc(hidden)] pub use job::funcs::*;
#[path = r"misc\_misc.rs"]                  mod misc;               #[doc(hidden)] pub use misc::funcs::*;
#[path = r"os\_os.rs"]                      pub(crate) mod os;
#[path = r"privilege\_privilege.rs"]        pub mod privilege;      #[doc(hidden)] pub use privilege::funcs::*;
#[path = r"process\_process.rs"]            pub mod process;        #[doc(hidden)] pub use process::funcs::*;
#[path = r"security\_security.rs"]          pub mod security;       //#[doc(hidden)] pub use security::funcs::*;
#[path = r"sid\_sid.rs"]                    pub mod sid;            #[doc(hidden)] pub use sid::funcs::*;
#[path = r"thread\_thread.rs"]              pub mod thread;         #[doc(hidden)] pub use thread::funcs::*;
#[path = r"token\_token.rs"]                pub mod token;          #[doc(hidden)] pub use token::funcs::*;
#[path = r"util\_util.rs"]                  mod util;               pub(crate) use util::*;
#[path = r"volume\_volume.rs"]              pub mod volume;
#[path = r"winsta\_winsta.rs"]              pub mod winsta;         #[doc(hidden)] pub use winsta::funcs::*;

mod error;                              pub use error::*;


pub use values::*;
mod values {
    mod luid;                       pub use luid::*;
}
