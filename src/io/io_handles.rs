#[cfg(doc)] use crate as firehazard;
use crate::*;

use winapi::ctypes::c_void;
use winapi::shared::minwindef::FALSE;
use winapi::um::fileapi::{ReadFile, WriteFile};

#[cfg(std)] use std::os::windows::prelude::*;

use core::marker::PhantomData;
use core::ptr::null_mut;

#[doc = "use [`firehazard::io::FileNN`] instead"        ] #[deprecated = "use `firehazard::io::FileNN` instead"        ] pub type File         = FileNN;
#[doc = "use [`firehazard::io::PipeReaderNN`] instead"  ] #[deprecated = "use `firehazard::io::PipeReaderNN` instead"  ] pub type ReadPipe     = PipeReaderNN;
#[doc = "use [`firehazard::io::PipeWriterNN`] instead"  ] #[deprecated = "use `firehazard::io::PipeWriterNN` instead"  ] pub type WritePipe    = PipeWriterNN;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilea)\] Owned non-null file `HANDLE`
///
/// ### Alternatives
/// *   [`std::fs::File`](https://doc.rust-lang.org/std/fs/struct.File.html) &mdash; cross platform, no ABI guarantees
/// *   [`std::os::windows::io::OwnedHandle`] &mdash; untyped, permits null/invalid
/// *   [`firehazard::handle::Owned`] &mdash; untyped
/// *   [`firehazard::io::FileHandle`] &mdash; borrowed instead of owned
///
#[repr(transparent)] pub struct FileNN(pub(super) HANDLENN);

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createpipe)\] Owned anonymous non-null pipe `HANDLE` ([io::Read]able end)
///
/// ### Alternatives
/// *   [`std::io::PipeReader`](https://doc.rust-lang.org/beta/std/io/struct.PipeReader.html) &mdash; no ABI guarantees, cross platform, not yet stable
/// *   [`std::os::windows::io::OwnedHandle`] &mdash; untyped, permits null/invalid
/// *   [`firehazard::handle::Owned`] &mdash; untyped
/// *   [`firehazard::io::ReadHandle`] &mdash; borrowed instead of owned
///
#[repr(transparent)] pub struct PipeReaderNN(pub(super) HANDLENN);

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createpipe)\] Owned anonymous non-null pipe `HANDLE` ([io::Write]able end)
///
/// ### Alternatives
/// *   [`std::io::PipeWriter`](https://doc.rust-lang.org/beta/std/io/struct.PipeWriter.html) &mdash; no ABI guarantees, cross platform, not yet stable
/// *   [`std::os::windows::io::OwnedHandle`] &mdash; untyped, permits null/invalid
/// *   [`firehazard::handle::Owned`] &mdash; untyped
/// *   [`firehazard::io::WriteHandle`] &mdash; borrowed instead of owned
///
#[repr(transparent)] pub struct PipeWriterNN(pub(super) HANDLENN);



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilea)\] Borrowed non-null file `HANDLE`
///
/// ### Alternatives
/// *   [`std::fs::File`](https://doc.rust-lang.org/std/fs/struct.File.html) &mdash; owned instead of borrowed, cross platform, no ABI guarantees
/// *   [`std::os::windows::io::BorrowedHandle`] &mdash; untyped, permits null/invalid
/// *   [`firehazard::handle::Borrowed`] &mdash; untyped
/// *   [`firehazard::io::FileNN`] &mdash; owned instead of borrowed
///
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct FileHandle<'a>(pub(super) HANDLENN, PhantomData<&'a HANDLENN>);

/// Borrowed non-null readable pipe or file `HANDLE`
///
/// ### Alternatives
/// *   [`std::io::PipeReader`](https://doc.rust-lang.org/beta/std/io/struct.PipeReader.html) &mdash; owned instead of borrowed, cross platform, no ABI guarantees, not yet stable
/// *   [`std::os::windows::io::BorrowedHandle`] &mdash; untyped, permits null/invalid
/// *   [`firehazard::handle::Borrowed`] &mdash; untyped
/// *   [`firehazard::io::PipeReaderNN`] &mdash; owned instead of borrowed
///
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct ReadHandle<'a>(pub(super) HANDLENN, PhantomData<&'a HANDLENN>);

/// Borrowed non-null writeable pipe or file `HANDLE`
///
/// ### Alternatives
/// *   [`std::io::PipeWriter`](https://doc.rust-lang.org/beta/std/io/struct.PipeWriter.html) &mdash; owned instead of borrowed, cross platform, no ABI guarantees, not yet stable
/// *   [`std::os::windows::io::BorrowedHandle`] &mdash; untyped, permits null/invalid
/// *   [`firehazard::handle::Borrowed`] &mdash; untyped
/// *   [`firehazard::io::PipeWriterNN`] &mdash; owned instead of borrowed
///
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct WriteHandle<'a>(pub(super) HANDLENN, PhantomData<&'a HANDLENN>);



handles!(unsafe impl *LocalHandleNN<c_void>         for io::{FileNN, FileHandle<'_>});
handles!(unsafe impl AsRef<Self>                    for io::{FileNN, FileHandle<'_>});
handles!(unsafe impl {Send, Sync}                   for io::{FileNN, FileHandle<'_>}); // SAFETY: `std::fs::File` is `Send+Sync` despite `try_clone(&self)`, `impl Read for &FileNN`, etc. all sharing a `HANDLE` - if this is unsound, so is `std`.
handles!(unsafe impl {AsRef, From}                  for io::{FileNN, FileHandle<'_>});
handles!(unsafe impl {AsRef<@base>, From<@base>}    for io::{FileNN, FileHandle<'_>});
handles!(       impl Debug                          for io::{FileNN, FileHandle<'_>});

handles!(unsafe impl *LocalHandleNN<c_void>         for io::{PipeReaderNN, ReadHandle<'_>});
handles!(unsafe impl AsRef<Self>                    for io::{PipeReaderNN, ReadHandle<'_>});
handles!(unsafe impl {Send, Sync}                   for io::{PipeReaderNN, ReadHandle<'_>}); // SAFETY: `std::io::PipeReader` is `Send+Sync` despite `try_clone(&self)`, `impl Read for &PipeReader`, etc. all sharing a `HANDLE` - if this is unsound, so is `std`.
handles!(unsafe impl {AsRef, From}                  for io::{PipeReaderNN, ReadHandle<'_>});
handles!(unsafe impl {AsRef<@base>, From<@base>}    for io::{PipeReaderNN, ReadHandle<'_>});
handles!(       impl Debug                          for io::{PipeReaderNN, ReadHandle<'_>});

handles!(unsafe impl *LocalHandleNN<c_void>         for io::{PipeWriterNN, WriteHandle<'_>});
handles!(unsafe impl AsRef<Self>                    for io::{PipeWriterNN, WriteHandle<'_>});
handles!(unsafe impl {Send, Sync}                   for io::{PipeWriterNN, WriteHandle<'_>}); // SAFETY: `std::io::PipeWriter` is `Send+Sync` despite `try_clone(&self)`, `impl Write for &PipeWriter`, etc. all sharing a `HANDLE` - if this is unsound, so is `std`.
handles!(unsafe impl {AsRef, From}                  for io::{PipeWriterNN, WriteHandle<'_>});
handles!(unsafe impl {AsRef<@base>, From<@base>}    for io::{PipeWriterNN, WriteHandle<'_>});
handles!(       impl Debug                          for io::{PipeWriterNN, WriteHandle<'_>});

handles!(unsafe impl @convert io::FileNN => io::PipeWriterNN);
handles!(unsafe impl @convert io::FileNN => io::PipeReaderNN);
handles!(unsafe impl @convert &'_ io::FileNN => io::WriteHandle<'_> );
handles!(unsafe impl @convert &'_ io::FileNN => io::ReadHandle<'_>  );

impl Drop       for FileNN          { fn drop(&mut self) { unsafe { drop_close_handle_nn(self) } } }
impl Drop       for PipeReaderNN    { fn drop(&mut self) { unsafe { drop_close_handle_nn(self) } } }
impl Drop       for PipeWriterNN    { fn drop(&mut self) { unsafe { drop_close_handle_nn(self) } } }

impl io::Read   for FileNN          { fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { unsafe { read_file(self.0, buf) } } }
impl io::Read   for FileHandle<'_>  { fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { unsafe { read_file(self.0, buf) } } }
impl io::Read   for PipeReaderNN    { fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { unsafe { read_file(self.0, buf) } } }
impl io::Read   for ReadHandle<'_>  { fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { unsafe { read_file(self.0, buf) } } }

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-readfile)\] ReadFile
unsafe fn read_file(h: HANDLENN, buf: &mut [u8]) -> io::Result<usize> {
    let mut read = 0;
    Error::get_last_if(FALSE == unsafe { ReadFile(h.as_ptr(), buf.as_mut_ptr().cast(), buf.len().try_into().unwrap_or(!0u32), &mut read, null_mut()) })?;
    Ok(usize::from32(read))
}

impl io::Write  for FileNN          { fn write(&mut self, buf: &[u8]) -> io::Result<usize> { unsafe { write_file(self.0, buf) } } fn flush(&mut self) -> io::Result<()> { Ok(()) } }
impl io::Write  for FileHandle<'_>  { fn write(&mut self, buf: &[u8]) -> io::Result<usize> { unsafe { write_file(self.0, buf) } } fn flush(&mut self) -> io::Result<()> { Ok(()) } }
impl io::Write  for PipeWriterNN    { fn write(&mut self, buf: &[u8]) -> io::Result<usize> { unsafe { write_file(self.0, buf) } } fn flush(&mut self) -> io::Result<()> { Ok(()) } }
impl io::Write  for WriteHandle<'_> { fn write(&mut self, buf: &[u8]) -> io::Result<usize> { unsafe { write_file(self.0, buf) } } fn flush(&mut self) -> io::Result<()> { Ok(()) } }

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-writefile)\] WriteFile
unsafe fn write_file(h: HANDLENN, buf: &[u8]) -> io::Result<usize> {
    let mut written = 0;
    Error::get_last_if(FALSE == unsafe { WriteFile(h.as_ptr(), buf.as_ptr().cast(), buf.len().try_into().unwrap_or(!0u32), &mut written, null_mut()) })?;
    Ok(usize::from32(written))
}

#[cfg(std)] impl From<std::fs::File> for FileNN { fn from(file: std::fs::File) -> Self {
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

#[cfg(std)] impl From<FileNN> for std::fs::File { fn from(file: FileNN) -> Self { unsafe { std::fs::File::from_raw_handle(file.into_handle()) } } }

#[cfg(std)] impl std::os::windows::io::FromRawHandle for FileNN         { unsafe fn from_raw_handle(handle: RawHandle) -> Self { Self(HANDLENN::new(handle).expect("undefined behavior: null is not an open, owned handle")) } }
#[cfg(std)] impl std::os::windows::io::FromRawHandle for PipeReaderNN   { unsafe fn from_raw_handle(handle: RawHandle) -> Self { Self(HANDLENN::new(handle).expect("undefined behavior: null is not an open, owned handle")) } }
#[cfg(std)] impl std::os::windows::io::FromRawHandle for PipeWriterNN   { unsafe fn from_raw_handle(handle: RawHandle) -> Self { Self(HANDLENN::new(handle).expect("undefined behavior: null is not an open, owned handle")) } }

#[cfg(std)] impl std::os::windows::io::IntoRawHandle for FileNN         { fn into_raw_handle(self) -> RawHandle { self.into_handle() } }
#[cfg(std)] impl std::os::windows::io::IntoRawHandle for PipeReaderNN   { fn into_raw_handle(self) -> RawHandle { self.into_handle() } }
#[cfg(std)] impl std::os::windows::io::IntoRawHandle for PipeWriterNN   { fn into_raw_handle(self) -> RawHandle { self.into_handle() } }

#[cfg(std)] impl std::os::windows::io::AsRawHandle for FileNN           { fn as_raw_handle(&self) -> RawHandle { self.0.as_ptr() } }
#[cfg(std)] impl std::os::windows::io::AsRawHandle for PipeReaderNN     { fn as_raw_handle(&self) -> RawHandle { self.0.as_ptr() } }
#[cfg(std)] impl std::os::windows::io::AsRawHandle for PipeWriterNN     { fn as_raw_handle(&self) -> RawHandle { self.0.as_ptr() } }
#[cfg(std)] impl std::os::windows::io::AsRawHandle for FileHandle<'_>   { fn as_raw_handle(&self) -> RawHandle { self.0.as_ptr() } }
#[cfg(std)] impl std::os::windows::io::AsRawHandle for ReadHandle<'_>   { fn as_raw_handle(&self) -> RawHandle { self.0.as_ptr() } }
#[cfg(std)] impl std::os::windows::io::AsRawHandle for WriteHandle<'_>  { fn as_raw_handle(&self) -> RawHandle { self.0.as_ptr() } }

#[cfg(std)] impl std::os::windows::io::AsHandle for FileNN              { fn as_handle(&self) -> BorrowedHandle { unsafe { BorrowedHandle::borrow_raw(self.0.as_ptr()) } } }
#[cfg(std)] impl std::os::windows::io::AsHandle for PipeReaderNN        { fn as_handle(&self) -> BorrowedHandle { unsafe { BorrowedHandle::borrow_raw(self.0.as_ptr()) } } }
#[cfg(std)] impl std::os::windows::io::AsHandle for PipeWriterNN        { fn as_handle(&self) -> BorrowedHandle { unsafe { BorrowedHandle::borrow_raw(self.0.as_ptr()) } } }
#[cfg(std)] impl std::os::windows::io::AsHandle for FileHandle<'_>      { fn as_handle(&self) -> BorrowedHandle { unsafe { BorrowedHandle::borrow_raw(self.0.as_ptr()) } } }
#[cfg(std)] impl std::os::windows::io::AsHandle for ReadHandle<'_>      { fn as_handle(&self) -> BorrowedHandle { unsafe { BorrowedHandle::borrow_raw(self.0.as_ptr()) } } }
#[cfg(std)] impl std::os::windows::io::AsHandle for WriteHandle<'_>     { fn as_handle(&self) -> BorrowedHandle { unsafe { BorrowedHandle::borrow_raw(self.0.as_ptr()) } } }

// It might be appropriate to impl TryFrom<OwnedHandle> for FileNN, PipeReaderNN, PipeWriterNN?
// ~~Constructing `std::os::windows::io::NullHandleError` is awkward though.~~ Just use OwnedHandle::try_from(HandleOrNull)
// Deferring until I have a concrete use case, if I ever do.



#[cfg(test)] mod evil {
    use super::*;

    #[test] #[should_panic = "undefined behavior"] fn null_std_fs_file() {
        let null = unsafe { std::fs::File::from_raw_handle(std::ptr::null_mut()) }; // arguably u.b.
        let _panic = crate::io::FileNN::from(null); // u.b. detected
    }

    #[test] #[should_panic = "undefined behavior"] fn null_firehazard_io_file() {
        let _null = unsafe { crate::io::FileNN::from_raw_handle(std::ptr::null_mut()) };
    }

    #[test] #[should_panic = "undefined behavior"] fn null_firehazard_io_read_pipe() {
        let _null = unsafe { crate::io::PipeReaderNN::from_raw_handle(std::ptr::null_mut()) };
    }

    #[test] #[should_panic = "undefined behavior"] fn null_firehazard_io_write_pipe() {
        let _null = unsafe { crate::io::PipeWriterNN::from_raw_handle(std::ptr::null_mut()) };
    }
}
