//! [`std::os`] re-export or [`no_std`](https://docs.rust-embedded.org/book/intro/no-std.html) placeholders

#![allow(dead_code, unused_imports)]



/// [`std::os::windows`] re-export or [`no_std`](https://docs.rust-embedded.org/book/intro/no-std.html) placeholders
pub mod windows {
    #[cfg(std)] pub use std::os::windows::{
        io,
        prelude,
        raw,
    };

    #[cfg(not(std))] pub mod io;
    #[cfg(not(std))] pub mod prelude;
    #[cfg(not(std))] pub mod raw;
}
