#![doc = include_str!("../Readme.md")]
#![cfg(windows)]

#![no_std]
//[debugger_visualizer(natvis_file = r"debug\firehazard.natvis")] // XXX: currently specified via launch.json
#![cfg_attr(not(std), allow(unused_imports))]
#![forbid(unsafe_op_in_unsafe_fn)]
#![deny(unreachable_patterns)]

#[cfg(std)] extern crate std;
#[doc(hidden)] pub extern crate abistr;
#[doc(hidden)] pub extern crate ialloc;
#[doc(hidden)] pub extern crate winapi;
#[cfg(alloc)]  extern crate alloc as alloc_;

#[cfg(doc)] pub use _doc::*;
#[cfg(doc)] mod _doc {
    use crate::prelude::*;
    #[doc = include_str!(r"..\doc\environment.md"               )] pub mod _environment {}
}

#[path = r"macros\_macros.rs"] #[macro_use] mod macros;
#[path = r"access\_access.rs"]              pub mod access;         //#[doc(no_inline)] #[doc(hidden)] pub use access::constants::*;
#[path = r"ace\_ace.rs"]                    pub mod ace;            //#[doc(no_inline)] #[doc(hidden)] pub use ace::funcs::*;
#[path = r"acl\_acl.rs"]                    pub mod acl;            //#[doc(no_inline)] #[doc(hidden)] pub use acl::funcs::*;
#[path = r"alloc\_alloc.rs"]                pub mod alloc;          #[doc(no_inline)] #[doc(hidden)] pub use alloc::funcs::*;
#[path = r"appcontainer\_appcontainer.rs"]  pub mod appcontainer;   #[doc(no_inline)] #[doc(hidden)] pub use appcontainer::funcs::*;
#[path = r"capability\_capability.rs"]      pub mod capability;
#[path = r"debug\_debug.rs"]                pub mod debug;          #[doc(no_inline)] #[doc(hidden)] pub use debug::funcs::*;
#[path = r"desktop\_desktop.rs"]            pub mod desktop;        #[doc(no_inline)] #[doc(hidden)] pub use desktop::funcs::*;
#[path = r"dlls\_dlls.rs"]                  mod dlls;               pub(crate) use dlls::*;
#[path = r"error\_error.rs"]                pub mod error;          #[doc(no_inline)] #[doc(hidden)] pub use error::*; pub use error::{Error, Result};
#[path = r"file\_file.rs"]                  pub mod file;           #[doc(no_inline)] #[doc(hidden)] pub use file::funcs::*;
#[path = r"handle\_handle.rs"]              pub mod handle;         #[doc(no_inline)] #[doc(hidden)] pub use handle::{funcs::*, traits::*};
#[path = r"io\_io.rs"]                      pub mod io;             #[doc(no_inline)] #[doc(hidden)] #[allow(unused_imports)] pub use io::funcs::*;
#[path = r"job\_job.rs"]                    pub mod job;            #[doc(no_inline)] #[doc(hidden)] pub use job::funcs::*;
#[path = r"time\_time.rs"]                  pub mod time;           #[doc(no_inline)] #[doc(hidden)] pub use time::funcs::*;
#[path = r"limit\_limit.rs"]                pub(crate) mod limit;
#[path = r"os\_os.rs"]                      pub(crate) mod os;
#[path = r"pipe\_pipe.rs"]                  pub mod pipe;           #[doc(no_inline)] #[doc(hidden)] pub use pipe::NMPWAIT;
#[path = r"prelude\_prelude.rs"]            pub mod prelude;
#[path = r"privilege\_privilege.rs"]        pub mod privilege;      #[doc(no_inline)] #[doc(hidden)] pub use privilege::funcs::*;
#[path = r"process\_process.rs"]            pub mod process;        #[doc(no_inline)] #[doc(hidden)] pub use process::funcs::*;
#[path = r"pseudoconsole\_pseudoconsole.rs"]pub mod pseudoconsole;  #[doc(no_inline)] #[doc(hidden)] pub use pseudoconsole::funcs::*;
#[path = r"security\_security.rs"]          pub mod security;       //#[doc(no_inline)] #[doc(hidden)] pub use security::funcs::*;
#[path = r"sid\_sid.rs"]                    pub mod sid;            #[doc(no_inline)] #[doc(hidden)] pub use sid::funcs::*;
#[path = r"string\_string.rs"]              pub mod string;         //#[doc(no_inline)] #[doc(hidden)] pub use string::funcs::*;
#[path = r"thread\_thread.rs"]              pub mod thread;         #[doc(no_inline)] #[doc(hidden)] pub use thread::funcs::*;
#[path = r"token\_token.rs"]                pub mod token;          #[doc(no_inline)] #[doc(hidden)] pub use token::funcs::*;
#[path = r"type_check\_type_check.rs"]      pub(crate) mod type_check;
#[path = r"util\_util.rs"]                  mod util;               pub(crate) use util::*;
#[path = r"values\_values.rs"]              pub mod values;         #[doc(no_inline)] #[doc(hidden)] pub use values::*;
#[path = r"volume\_volume.rs"]              pub mod volume;
#[path = r"winsta\_winsta.rs"]              pub mod winsta;         #[doc(no_inline)] #[doc(hidden)] pub use winsta::funcs::*;
