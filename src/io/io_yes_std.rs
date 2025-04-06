#![cfg(std)]

use crate::{AsLocalHandle, HANDLENN};
use crate::io::{FileNN, PipeReaderNN, PipeWriterNN, FileHandle, ReadHandle, WriteHandle};

use std::os::windows::prelude::*;



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

impl std::os::windows::io::FromRawHandle for FileNN         { unsafe fn from_raw_handle(handle: RawHandle) -> Self { Self(HANDLENN::new(handle).expect("undefined behavior: null is not an open, owned handle")) } }
impl std::os::windows::io::FromRawHandle for PipeReaderNN   { unsafe fn from_raw_handle(handle: RawHandle) -> Self { Self(HANDLENN::new(handle).expect("undefined behavior: null is not an open, owned handle")) } }
impl std::os::windows::io::FromRawHandle for PipeWriterNN   { unsafe fn from_raw_handle(handle: RawHandle) -> Self { Self(HANDLENN::new(handle).expect("undefined behavior: null is not an open, owned handle")) } }

impl std::os::windows::io::IntoRawHandle for FileNN         { fn into_raw_handle(self) -> RawHandle { self.into_handle() } }
impl std::os::windows::io::IntoRawHandle for PipeReaderNN   { fn into_raw_handle(self) -> RawHandle { self.into_handle() } }
impl std::os::windows::io::IntoRawHandle for PipeWriterNN   { fn into_raw_handle(self) -> RawHandle { self.into_handle() } }

impl std::os::windows::io::AsRawHandle for FileNN           { fn as_raw_handle(&self) -> RawHandle { self.0.as_ptr() } }
impl std::os::windows::io::AsRawHandle for PipeReaderNN     { fn as_raw_handle(&self) -> RawHandle { self.0.as_ptr() } }
impl std::os::windows::io::AsRawHandle for PipeWriterNN     { fn as_raw_handle(&self) -> RawHandle { self.0.as_ptr() } }
impl std::os::windows::io::AsRawHandle for FileHandle<'_>   { fn as_raw_handle(&self) -> RawHandle { self.0.as_ptr() } }
impl std::os::windows::io::AsRawHandle for ReadHandle<'_>   { fn as_raw_handle(&self) -> RawHandle { self.0.as_ptr() } }
impl std::os::windows::io::AsRawHandle for WriteHandle<'_>  { fn as_raw_handle(&self) -> RawHandle { self.0.as_ptr() } }

impl std::os::windows::io::AsHandle for FileNN              { fn as_handle(&self) -> BorrowedHandle { unsafe { BorrowedHandle::borrow_raw(self.0.as_ptr()) } } }
impl std::os::windows::io::AsHandle for PipeReaderNN        { fn as_handle(&self) -> BorrowedHandle { unsafe { BorrowedHandle::borrow_raw(self.0.as_ptr()) } } }
impl std::os::windows::io::AsHandle for PipeWriterNN        { fn as_handle(&self) -> BorrowedHandle { unsafe { BorrowedHandle::borrow_raw(self.0.as_ptr()) } } }
impl std::os::windows::io::AsHandle for FileHandle<'_>      { fn as_handle(&self) -> BorrowedHandle { unsafe { BorrowedHandle::borrow_raw(self.0.as_ptr()) } } }
impl std::os::windows::io::AsHandle for ReadHandle<'_>      { fn as_handle(&self) -> BorrowedHandle { unsafe { BorrowedHandle::borrow_raw(self.0.as_ptr()) } } }
impl std::os::windows::io::AsHandle for WriteHandle<'_>     { fn as_handle(&self) -> BorrowedHandle { unsafe { BorrowedHandle::borrow_raw(self.0.as_ptr()) } } }

// It might be appropriate to impl TryFrom<OwnedHandle> for FileNN, PipeReaderNN, PipeWriterNN?
// ~~Constructing `std::os::windows::io::NullHandleError` is awkward though.~~ Just use OwnedHandle::try_from(HandleOrNull)?
// Deferring until I have a concrete use case, if I ever do.



#[cfg(test)] mod tests {
    use super::*;
    use core::ptr::null_mut;

    #[test] #[should_panic = "undefined behavior"] fn null_std_fs_file() {
        let null = unsafe { std::fs::File::from_raw_handle(null_mut()) }; // arguably u.b.
        let _panic = crate::io::FileNN::from(null); // u.b. detected
    }

    #[test] #[should_panic = "undefined behavior"] fn null_firehazard_io_file() {
        let _null = unsafe { crate::io::FileNN::from_raw_handle(null_mut()) };
    }

    #[test] #[should_panic = "undefined behavior"] fn null_firehazard_io_read_pipe() {
        let _null = unsafe { crate::io::PipeReaderNN::from_raw_handle(null_mut()) };
    }

    #[test] #[should_panic = "undefined behavior"] fn null_firehazard_io_write_pipe() {
        let _null = unsafe { crate::io::PipeWriterNN::from_raw_handle(null_mut()) };
    }
}
