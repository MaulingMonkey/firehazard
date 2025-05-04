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

#[cfg(doc)] pub use _doc::*;
#[cfg(doc)] mod _doc {
    use crate::prelude::*;
    #[doc = include_str!(r"..\doc\environment.md"               )] pub mod _environment {}
}

#[path = r"macros\_macros.rs"] #[macro_use] mod macros;
#[path = r"access\_access.rs"]              pub mod access;         //#[doc(hidden)] pub use access::constants::*;
#[path = r"ace\_ace.rs"]                    pub mod ace;            //#[doc(hidden)] pub use ace::funcs::*;
#[path = r"acl\_acl.rs"]                    pub mod acl;            //#[doc(hidden)] pub use acl::funcs::*;
#[path = r"alloc\_alloc.rs"]                pub mod alloc;          #[doc(hidden)] pub use alloc::funcs::*;
#[path = r"appcontainer\_appcontainer.rs"]  pub mod appcontainer;   #[doc(hidden)] pub use appcontainer::funcs::*;
#[path = r"capability\_capability.rs"]      pub mod capability;
#[path = r"debug\_debug.rs"]                pub mod debug;          #[doc(hidden)] pub use debug::funcs::*;
#[path = r"desktop\_desktop.rs"]            pub mod desktop;        #[doc(hidden)] pub use desktop::funcs::*;
#[path = r"dlls\_dlls.rs"]                  mod dlls;               pub(crate) use dlls::*;
#[path = r"error\_error.rs"]                mod error;              pub use error::*;
#[path = r"file\_file.rs"]                  pub mod file;           #[doc(hidden)] pub use file::funcs::*;
#[path = r"handle\_handle.rs"]              pub mod handle;         #[doc(hidden)] pub use handle::{funcs::*, traits::*};
#[path = r"io\_io.rs"]                      pub mod io;             #[doc(hidden)] #[allow(unused_imports)] pub use io::funcs::*;
#[path = r"job\_job.rs"]                    pub mod job;            #[doc(hidden)] pub use job::funcs::*;
#[path = r"time\_time.rs"]                  pub mod time;           #[doc(hidden)] pub use time::funcs::*;
#[path = r"os\_os.rs"]                      pub(crate) mod os;
#[path = r"pipe\_pipe.rs"]                  pub mod pipe;           #[doc(hidden)] pub use pipe::NMPWAIT;
#[path = r"prelude\_prelude.rs"]            pub mod prelude;
#[path = r"privilege\_privilege.rs"]        pub mod privilege;      #[doc(hidden)] pub use privilege::funcs::*;
#[path = r"process\_process.rs"]            pub mod process;        #[doc(hidden)] pub use process::funcs::*;
#[path = r"pseudoconsole\_pseudoconsole.rs"]pub mod pseudoconsole;  #[doc(hidden)] pub use pseudoconsole::funcs::*;
#[path = r"security\_security.rs"]          pub mod security;       //#[doc(hidden)] pub use security::funcs::*;
#[path = r"sid\_sid.rs"]                    pub mod sid;            #[doc(hidden)] pub use sid::funcs::*;
#[path = r"thread\_thread.rs"]              pub mod thread;         #[doc(hidden)] pub use thread::funcs::*;
#[path = r"token\_token.rs"]                pub mod token;          #[doc(hidden)] pub use token::funcs::*;
#[path = r"type_check\_type_check.rs"]      pub(crate) mod type_check;
#[path = r"util\_util.rs"]                  mod util;               pub(crate) use util::*;
#[path = r"values\_values.rs"]              pub mod values;         #[doc(hidden)] pub use values::*;
#[path = r"volume\_volume.rs"]              pub mod volume;
#[path = r"winsta\_winsta.rs"]              pub mod winsta;         #[doc(hidden)] pub use winsta::funcs::*;
