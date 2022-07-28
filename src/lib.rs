#![forbid(unsafe_op_in_unsafe_fn)]
#![deny(unreachable_patterns)]

pub mod error;

pub mod handle {
    mod access_token;               pub use access_token::*;
    mod psuedo_access_token;        pub use psuedo_access_token::*;
}

pub(crate) use util::*;
mod util {
    mod bits32;                     pub(crate) use bits32::*;
}
