#![cfg(std)]

use crate::prelude::*;
use crate::io::FileNN;
use crate::os::windows::prelude::*;



/// XXX: FileNN currently assumes the handle was created without using `FILE_FLAG_OVERLAPPED`.
/// However, std::fs::File may wrap a handle that *was* created using `FILE_FLAG_OVERLAPPED`.
/// See https://github.com/rust-lang/rust/issues/81357 for notes on the required workarounds for soundness.
#[cfg(nope)] // See also disabled unit test bellow
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

// FileNN → File should be sound since the former currently forbids `FILE_FLAG_OVERLAPPED`.
impl From<FileNN> for std::fs::File { fn from(file: FileNN) -> Self { unsafe { std::fs::File::from_raw_handle(file.into_handle()) } } }

impl    AsLocalHandle for std::fs::File                 { fn as_handle(&self) -> HANDLE { self.as_raw_handle().cast() } }
impl    AsLocalHandle for std::io::Stderr               { fn as_handle(&self) -> HANDLE { self.as_raw_handle().cast() } }
impl    AsLocalHandle for std::io::Stdin                { fn as_handle(&self) -> HANDLE { self.as_raw_handle().cast() } }
impl    AsLocalHandle for std::io::Stdout               { fn as_handle(&self) -> HANDLE { self.as_raw_handle().cast() } }
impl    AsLocalHandle for std::process::ChildStderr     { fn as_handle(&self) -> HANDLE { self.as_raw_handle().cast() } }
impl    AsLocalHandle for std::process::ChildStdin      { fn as_handle(&self) -> HANDLE { self.as_raw_handle().cast() } }
impl    AsLocalHandle for std::process::ChildStdout     { fn as_handle(&self) -> HANDLE { self.as_raw_handle().cast() } }

#[cfg(test)] mod tests {
    #[allow(unused_imports)] use crate::prelude::*;

    #[cfg(nope)] // File → FileNN currently disabled
    #[test] #[should_panic = "undefined behavior"] fn null_std_fs_file() {
        let null = unsafe { std::fs::File::from_raw_handle(null_mut()) }; // arguably u.b.
        let _panic = io::FileNN::from(null); // u.b. detected
    }
}
