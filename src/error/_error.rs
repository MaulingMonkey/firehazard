//! \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi/)\]
//! Error handling types and functions

include!(r"fast_fail.rs");

mod error;                          pub use error::*;
mod handle_conversion_error;        pub use handle_conversion_error::*;
mod preserve_error_scope;           pub(crate) use preserve_error_scope::*;
