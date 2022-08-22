//! [Microsoft Windows](https://en.wikipedia.org/wiki/Microsoft_Windows) specific sandboxing APIs

#![cfg_attr(not(std), no_std)]
#![forbid(unsafe_op_in_unsafe_fn)]
#![deny(unreachable_patterns)]

#[doc(inline)] pub extern crate sandbox_windows_ffi as ffi;
