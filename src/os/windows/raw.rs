//! [`std::os::windows::raw`] re-export or [`no_std`](https://docs.rust-embedded.org/book/intro/no-std.html) placeholders

use core::ffi::c_void;

pub type HANDLE = *mut c_void;
//b type SOCKET = *mut c_void;
