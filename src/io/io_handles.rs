#[cfg(doc)] use crate as firehazard;
use crate::*;

use winapi::ctypes::c_void;

use core::marker::PhantomData;

#[doc(hidden)] #[deprecated = "use `firehazard::io::FileNN` instead"        ] pub type File         = FileNN;
#[doc(hidden)] #[deprecated = "use `firehazard::io::PipeReaderNN` instead"  ] pub type ReadPipe     = PipeReaderNN;
#[doc(hidden)] #[deprecated = "use `firehazard::io::PipeWriterNN` instead"  ] pub type WritePipe    = PipeWriterNN;



#[doc(alias = "HANDLE")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilea)\] Owned non-null file `HANDLE`
///
/// ### Alternatives
/// *   [`std::fs::File`](https://doc.rust-lang.org/std/fs/struct.File.html) &mdash; cross platform, no ABI guarantees
/// *   [`std::os::windows::io::OwnedHandle`] &mdash; untyped, permits null/invalid
/// *   [`firehazard::handle::Owned`] &mdash; untyped
/// *   [`firehazard::io::FileHandle`] &mdash; borrowed instead of owned
///
#[repr(transparent)] pub struct FileNN(pub(super) HANDLENN);

#[doc(alias = "HANDLE")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createpipe)\] Owned anonymous non-null pipe `HANDLE` ([io::Read]able end)
///
/// ### Alternatives
/// *   [`std::io::PipeReader`](https://doc.rust-lang.org/beta/std/io/struct.PipeReader.html) &mdash; no ABI guarantees, cross platform, not yet stable
/// *   [`std::os::windows::io::OwnedHandle`] &mdash; untyped, permits null/invalid
/// *   [`firehazard::handle::Owned`] &mdash; untyped
/// *   [`firehazard::io::ReadHandle`] &mdash; borrowed instead of owned
///
#[repr(transparent)] pub struct PipeReaderNN(pub(crate) HANDLENN);

#[doc(alias = "HANDLE")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createpipe)\] Owned anonymous non-null pipe `HANDLE` ([io::Write]able end)
///
/// ### Alternatives
/// *   [`std::io::PipeWriter`](https://doc.rust-lang.org/beta/std/io/struct.PipeWriter.html) &mdash; no ABI guarantees, cross platform, not yet stable
/// *   [`std::os::windows::io::OwnedHandle`] &mdash; untyped, permits null/invalid
/// *   [`firehazard::handle::Owned`] &mdash; untyped
/// *   [`firehazard::io::WriteHandle`] &mdash; borrowed instead of owned
///
#[repr(transparent)] pub struct PipeWriterNN(pub(crate) HANDLENN);



#[doc(alias = "HANDLE")]
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

#[doc(alias = "HANDLE")]
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

#[doc(alias = "HANDLE")]
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

#[doc = r"\[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle)\] CloseHandle"] impl Drop for FileNN          { fn drop(&mut self) { unsafe { drop_close_handle_nn(self) } } }
#[doc = r"\[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle)\] CloseHandle"] impl Drop for PipeReaderNN    { fn drop(&mut self) { unsafe { drop_close_handle_nn(self) } } }
#[doc = r"\[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle)\] CloseHandle"] impl Drop for PipeWriterNN    { fn drop(&mut self) { unsafe { drop_close_handle_nn(self) } } }

impl io::Read   for FileNN          { fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { unsafe { Ok(read_file(self, buf, None).map(usize::from32)?) } } }
impl io::Read   for FileHandle<'_>  { fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { unsafe { Ok(read_file(self, buf, None).map(usize::from32)?) } } }
impl io::Read   for PipeReaderNN    { fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { unsafe { Ok(read_file(self, buf, None).map(usize::from32)?) } } }
impl io::Read   for ReadHandle<'_>  { fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { unsafe { Ok(read_file(self, buf, None).map(usize::from32)?) } } }

impl io::Seek   for FileNN          { fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> { unsafe { Ok(set_file_pointer_ex(self, pos)?) } } }
impl io::Seek   for FileHandle<'_>  { fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> { unsafe { Ok(set_file_pointer_ex(self, pos)?) } } }

// noop flush sane: https://github.com/rust-lang/rust/blob/c2110769cd58cd3b0c31f308c8cfeab5e19340fd/library/std/src/sys/fs/windows.rs#L604-L606
impl io::Write  for FileNN          { fn write(&mut self, buf: &[u8]) -> io::Result<usize> { unsafe { Ok(write_file(self, buf, None).map(usize::from32)?) } } fn flush(&mut self) -> io::Result<()> { Ok(()) } }
impl io::Write  for FileHandle<'_>  { fn write(&mut self, buf: &[u8]) -> io::Result<usize> { unsafe { Ok(write_file(self, buf, None).map(usize::from32)?) } } fn flush(&mut self) -> io::Result<()> { Ok(()) } }

// noop flush sane: https://github.com/rust-lang/rust/blob/c2110769cd58cd3b0c31f308c8cfeab5e19340fd/library/std/src/io/pipe.rs#L271-L274
impl io::Write  for PipeWriterNN    { fn write(&mut self, buf: &[u8]) -> io::Result<usize> { unsafe { Ok(write_file(self, buf, None).map(usize::from32)?) } } fn flush(&mut self) -> io::Result<()> { Ok(()) } }
impl io::Write  for WriteHandle<'_> { fn write(&mut self, buf: &[u8]) -> io::Result<usize> { unsafe { Ok(write_file(self, buf, None).map(usize::from32)?) } } fn flush(&mut self) -> io::Result<()> { Ok(()) } }

impl crate::os::windows::io::FromRawHandle for FileNN         { unsafe fn from_raw_handle(handle: crate::os::windows::io::RawHandle) -> Self { Self(HANDLENN::new(handle.cast()).expect("undefined behavior: null is not an open, owned handle")) } }
impl crate::os::windows::io::FromRawHandle for PipeReaderNN   { unsafe fn from_raw_handle(handle: crate::os::windows::io::RawHandle) -> Self { Self(HANDLENN::new(handle.cast()).expect("undefined behavior: null is not an open, owned handle")) } }
impl crate::os::windows::io::FromRawHandle for PipeWriterNN   { unsafe fn from_raw_handle(handle: crate::os::windows::io::RawHandle) -> Self { Self(HANDLENN::new(handle.cast()).expect("undefined behavior: null is not an open, owned handle")) } }

impl crate::os::windows::io::IntoRawHandle for FileNN         { fn into_raw_handle(self) -> crate::os::windows::io::RawHandle { self.into_handle().cast() } }
impl crate::os::windows::io::IntoRawHandle for PipeReaderNN   { fn into_raw_handle(self) -> crate::os::windows::io::RawHandle { self.into_handle().cast() } }
impl crate::os::windows::io::IntoRawHandle for PipeWriterNN   { fn into_raw_handle(self) -> crate::os::windows::io::RawHandle { self.into_handle().cast() } }

unsafe impl valrow::Borrowable for FileNN            { type Abi = HANDLENN; }
unsafe impl valrow::Borrowable for PipeReaderNN      { type Abi = HANDLENN; }
unsafe impl valrow::Borrowable for PipeWriterNN      { type Abi = HANDLENN; }

unsafe impl valrow::Borrowable for FileHandle<'_>    { type Abi = HANDLENN; }
unsafe impl valrow::Borrowable for ReadHandle<'_>    { type Abi = HANDLENN; }
unsafe impl valrow::Borrowable for WriteHandle<'_>   { type Abi = HANDLENN; }

// It might be appropriate to impl TryFrom<OwnedHandle> for FileNN, PipeReaderNN, PipeWriterNN?
// ~~Constructing `crate::os::windows::io::NullHandleError` is awkward though.~~ Just use OwnedHandle::try_from(HandleOrNull)?
// Deferring until I have a concrete use case, if I ever do.



#[cfg(test)] mod tests {
    use crate::io::*;
    use crate::os::windows::io::FromRawHandle;
    use core::ptr::null_mut;

    #[test] #[should_panic = "undefined behavior"] fn null_firehazard_io_file() {
        let _null = unsafe { FileNN::from_raw_handle(null_mut()) };
    }

    #[test] #[should_panic = "undefined behavior"] fn null_firehazard_io_read_pipe() {
        let _null = unsafe { PipeReaderNN::from_raw_handle(null_mut()) };
    }

    #[test] #[should_panic = "undefined behavior"] fn null_firehazard_io_write_pipe() {
        let _null = unsafe { PipeWriterNN::from_raw_handle(null_mut()) };
    }
}
