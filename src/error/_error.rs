//! Error handling types and functions

mod error;                          pub use error::*;
mod handle_conversion_error;        pub use handle_conversion_error::*;
mod preserve_error_scope;           pub(crate) use preserve_error_scope::*;