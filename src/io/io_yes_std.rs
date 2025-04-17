#![cfg(std)]

use crate::{AsLocalHandle, HANDLE, HANDLENN};
use crate::io::FileNN;
use crate::os::windows::prelude::*;



impl From<std::fs::File> for FileNN { fn from(file: std::fs::File) -> Self {
    // [`FromRawHandle::from_raw_handle`](https://doc.rust-lang.org/std/os/windows/io/trait.FromRawHandle.html#tymethod.from_raw_handle) reads:
    // "The handle passed in must: [...] be an owned handle; in particular, it must be open."
    //
    // As such, I believe `File::from_raw_handle(null_ptr())` is undefined behavior. This is further evidenced by
    // [`HandleOrNull::from_raw_handle`](https://doc.rust-lang.org/std/os/windows/io/struct.HandleOrNull.html#method.from_raw_handle), which reads:
    // "The passed handle value must either satisfy the safety requirements of FromRawHandle::from_raw_handle, or be null."
    //
    // This implies null does *not* meet the baseline requirements of `FromRawHandle::from_raw_handle`.
    //
    Self(HANDLENN::new(file.into_raw_handle()).expect("undefined behavior: `std::fs::File::from_raw_handle(null_ptr())` was presumably called earlier, but null is not an open, owned handle")) }
}

impl From<FileNN> for std::fs::File { fn from(file: FileNN) -> Self { unsafe { std::fs::File::from_raw_handle(file.into_handle()) } } }

impl    AsLocalHandle for std::fs::File                 { fn as_handle(&self) -> HANDLE { self.as_raw_handle().cast() } }
impl    AsLocalHandle for std::io::Stderr               { fn as_handle(&self) -> HANDLE { self.as_raw_handle().cast() } }
impl    AsLocalHandle for std::io::Stdin                { fn as_handle(&self) -> HANDLE { self.as_raw_handle().cast() } }
impl    AsLocalHandle for std::io::Stdout               { fn as_handle(&self) -> HANDLE { self.as_raw_handle().cast() } }
impl    AsLocalHandle for std::process::Child           { fn as_handle(&self) -> HANDLE { self.as_raw_handle().cast() } }
impl    AsLocalHandle for std::process::ChildStderr     { fn as_handle(&self) -> HANDLE { self.as_raw_handle().cast() } }
impl    AsLocalHandle for std::process::ChildStdin      { fn as_handle(&self) -> HANDLE { self.as_raw_handle().cast() } }
impl    AsLocalHandle for std::process::ChildStdout     { fn as_handle(&self) -> HANDLE { self.as_raw_handle().cast() } }
impl<T> AsLocalHandle for std::thread::JoinHandle<T>    { fn as_handle(&self) -> HANDLE { self.as_raw_handle().cast() } }

#[cfg(test)] mod tests {
    use super::*;
    use core::ptr::null_mut;

    #[test] #[should_panic = "undefined behavior"] fn null_std_fs_file() {
        let null = unsafe { std::fs::File::from_raw_handle(null_mut()) }; // arguably u.b.
        let _panic = crate::io::FileNN::from(null); // u.b. detected
    }
}
