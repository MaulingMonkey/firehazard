#[cfg(doc)] use crate as firehazard;
use crate::*;

use winapi::ctypes::c_void;

use core::marker::PhantomData;

#[doc(hidden)] #[deprecated = "use `firehazard::io::FileNN` instead"        ] pub type File         = FileNN;
#[doc(hidden)] #[deprecated = "use `firehazard::pipe::ReaderNN` instead"    ] pub type ReadPipe     = pipe::ReaderNN;
#[doc(hidden)] #[deprecated = "use `firehazard::pipe::WriterNN` instead"    ] pub type WritePipe    = pipe::WriterNN;



#[doc(alias = "HANDLE")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilea)\] Owned non-null non-OVERLAPPED file `HANDLE`
///
/// ### Alternatives
/// *   [`std::fs::File`](https://doc.rust-lang.org/std/fs/struct.File.html) &mdash; cross platform, no ABI guarantees
/// *   [`std::os::windows::io::OwnedHandle`] &mdash; untyped, permits null/invalid
/// *   [`firehazard::handle::Owned`] &mdash; untyped
/// *   [`firehazard::io::FileHandle`] &mdash; borrowed instead of owned
///
#[repr(transparent)] pub struct FileNN(pub(super) HANDLENN);

#[doc(hidden)] #[deprecated = "moved to firehazard::pipe::ReaderNN"] pub type PipeReaderNN = pipe::ReaderNN;
#[doc(hidden)] #[deprecated = "moved to firehazard::pipe::WriterNN"] pub type PipeWriterNN = pipe::WriterNN;



#[doc(alias = "HANDLE")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilea)\] Borrowed non-null non-OVERLAPPED file `HANDLE`
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
/// Borrowed non-null non-OVERLAPPED readable pipe or file `HANDLE`
///
/// ### Alternatives
/// *   [`std::io::PipeReader`](https://doc.rust-lang.org/beta/std/io/struct.PipeReader.html) &mdash; owned instead of borrowed, cross platform, no ABI guarantees, not yet stable
/// *   [`std::os::windows::io::BorrowedHandle`] &mdash; untyped, permits null/invalid
/// *   [`firehazard::handle::Borrowed`] &mdash; untyped
/// *   [`firehazard::pipe::ReaderNN`] &mdash; owned instead of borrowed
///
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct ReadHandle<'a>(pub(super) HANDLENN, PhantomData<&'a HANDLENN>);

#[doc(alias = "HANDLE")]
/// Borrowed non-null non-OVERLAPPED writeable pipe or file `HANDLE`
///
/// ### Alternatives
/// *   [`std::io::PipeWriter`](https://doc.rust-lang.org/beta/std/io/struct.PipeWriter.html) &mdash; owned instead of borrowed, cross platform, no ABI guarantees, not yet stable
/// *   [`std::os::windows::io::BorrowedHandle`] &mdash; untyped, permits null/invalid
/// *   [`firehazard::handle::Borrowed`] &mdash; untyped
/// *   [`firehazard::pipe::WriterNN`] &mdash; owned instead of borrowed
///
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct WriteHandle<'a>(pub(super) HANDLENN, PhantomData<&'a HANDLENN>);



handles!(unsafe impl *LocalHandleNN<c_void>         for io::{FileNN, FileHandle<'_>});
handles!(unsafe impl AsRef<Self>                    for io::{FileNN, FileHandle<'_>});
handles!(unsafe impl {Send, Sync}                   for io::{FileNN, FileHandle<'_>}); // SAFETY: `std::fs::File` is `Send+Sync` despite `try_clone(&self)`, `impl Read for &FileNN`, etc. all sharing a `HANDLE` - if this is unsound, so is `std`.
handles!(       impl Debug                          for io::{FileNN, FileHandle<'_>});

handles!(unsafe impl *LocalHandleNN<c_void>         for io::{ReadHandle<'_>});
handles!(unsafe impl AsRef<Self>                    for io::{ReadHandle<'_>});
handles!(unsafe impl {Send, Sync}                   for io::{ReadHandle<'_>}); // SAFETY: `std::io::PipeReader` is `Send+Sync` despite `try_clone(&self)`, `impl Read for &PipeReader`, etc. all sharing a `HANDLE` - if this is unsound, so is `std`.
handles!(       impl Debug                          for io::{ReadHandle<'_>});

handles!(unsafe impl *LocalHandleNN<c_void>         for io::{WriteHandle<'_>});
handles!(unsafe impl AsRef<Self>                    for io::{WriteHandle<'_>});
handles!(unsafe impl {Send, Sync}                   for io::{WriteHandle<'_>}); // SAFETY: `std::io::PipeWriter` is `Send+Sync` despite `try_clone(&self)`, `impl Write for &PipeWriter`, etc. all sharing a `HANDLE` - if this is unsound, so is `std`.
handles!(       impl Debug                          for io::{WriteHandle<'_>});

handles!(unsafe impl @convert     io::FileNN        => pipe::ReaderNN           ); // XXX: technically not pipes, but I'm not sure that actually matters
handles!(unsafe impl @convert     io::FileNN        => pipe::WriterNN           ); // XXX: technically not pipes, but I'm not sure that actually matters
handles!(unsafe impl @convert     io::FileNN        => handle::Owned            );

handles!(unsafe impl @convert &'_ io::FileNN        => io::FileHandle<'_>       );
handles!(unsafe impl @convert &'_ io::FileNN        => io::ReadHandle<'_>       );
handles!(unsafe impl @convert &'_ io::FileNN        => io::WriteHandle<'_>      );
handles!(unsafe impl @convert &'_ io::FileNN        => handle::Borrowed<'_>     );
handles!(unsafe impl @convert &'_ io::FileNN        => handle::Pseudo<'_>       );

handles!(unsafe impl @convert io::FileHandle<'_>    => io::ReadHandle<'_>       );
handles!(unsafe impl @convert io::FileHandle<'_>    => io::WriteHandle<'_>      );
handles!(unsafe impl @convert io::FileHandle<'_>    => handle::Borrowed<'_>     );
handles!(unsafe impl @convert io::FileHandle<'_>    => handle::Pseudo<'_>       );

handles!(unsafe impl @convert io::ReadHandle<'_>    => handle::Borrowed<'_>     );
handles!(unsafe impl @convert io::ReadHandle<'_>    => handle::Pseudo<'_>       );

handles!(unsafe impl @convert io::WriteHandle<'_>   => handle::Borrowed<'_>     );
handles!(unsafe impl @convert io::WriteHandle<'_>   => handle::Pseudo<'_>       );



#[doc = r"\[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle)\] CloseHandle"] impl Drop for FileNN          { fn drop(&mut self) { unsafe { drop_close_handle_nn(self) } } }

impl io::Read   for &'_ FileNN          { fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { unsafe { Ok(read_file(*self, buf, None).map(usize::from32)?) } } }
impl io::Read   for     FileNN          { fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { unsafe { Ok(read_file( self, buf, None).map(usize::from32)?) } } }
impl io::Read   for &'_ FileHandle<'_>  { fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { unsafe { Ok(read_file(*self, buf, None).map(usize::from32)?) } } }
impl io::Read   for     FileHandle<'_>  { fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { unsafe { Ok(read_file( self, buf, None).map(usize::from32)?) } } }
impl io::Read   for &'_ ReadHandle<'_>  { fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { unsafe { Ok(read_file(*self, buf, None).map(usize::from32)?) } } }
impl io::Read   for     ReadHandle<'_>  { fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { unsafe { Ok(read_file( self, buf, None).map(usize::from32)?) } } }

impl io::Seek   for &'_ FileNN          { fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> { unsafe { Ok(set_file_pointer_ex(*self, pos)?) } } }
impl io::Seek   for     FileNN          { fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> { unsafe { Ok(set_file_pointer_ex( self, pos)?) } } }
impl io::Seek   for &'_ FileHandle<'_>  { fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> { unsafe { Ok(set_file_pointer_ex(*self, pos)?) } } }
impl io::Seek   for     FileHandle<'_>  { fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> { unsafe { Ok(set_file_pointer_ex( self, pos)?) } } }

// noop flush sane: https://github.com/rust-lang/rust/blob/c2110769cd58cd3b0c31f308c8cfeab5e19340fd/library/std/src/sys/fs/windows.rs#L604-L606
impl io::Write  for &'_ FileNN          { fn write(&mut self, buf: &[u8]) -> io::Result<usize> { unsafe { Ok(write_file(*self, buf, None).map(usize::from32)?) } } fn flush(&mut self) -> io::Result<()> { Ok(()) } }
impl io::Write  for     FileNN          { fn write(&mut self, buf: &[u8]) -> io::Result<usize> { unsafe { Ok(write_file( self, buf, None).map(usize::from32)?) } } fn flush(&mut self) -> io::Result<()> { Ok(()) } }
impl io::Write  for &'_ FileHandle<'_>  { fn write(&mut self, buf: &[u8]) -> io::Result<usize> { unsafe { Ok(write_file(*self, buf, None).map(usize::from32)?) } } fn flush(&mut self) -> io::Result<()> { Ok(()) } }
impl io::Write  for     FileHandle<'_>  { fn write(&mut self, buf: &[u8]) -> io::Result<usize> { unsafe { Ok(write_file( self, buf, None).map(usize::from32)?) } } fn flush(&mut self) -> io::Result<()> { Ok(()) } }

// noop flush sane: https://github.com/rust-lang/rust/blob/c2110769cd58cd3b0c31f308c8cfeab5e19340fd/library/std/src/io/pipe.rs#L271-L274
impl io::Write  for &'_ WriteHandle<'_> { fn write(&mut self, buf: &[u8]) -> io::Result<usize> { unsafe { Ok(write_file(*self, buf, None).map(usize::from32)?) } } fn flush(&mut self) -> io::Result<()> { Ok(()) } }
impl io::Write  for     WriteHandle<'_> { fn write(&mut self, buf: &[u8]) -> io::Result<usize> { unsafe { Ok(write_file( self, buf, None).map(usize::from32)?) } } fn flush(&mut self) -> io::Result<()> { Ok(()) } }

impl crate::os::windows::io::FromRawHandle for FileNN         { unsafe fn from_raw_handle(handle: crate::os::windows::io::RawHandle) -> Self { Self(HANDLENN::new(handle.cast()).expect("undefined behavior: null is not an open, owned handle")) } }
impl crate::os::windows::io::IntoRawHandle for FileNN         { fn into_raw_handle(self) -> crate::os::windows::io::RawHandle { self.into_handle().cast() } }

unsafe impl valrow::Borrowable for FileNN            { type Abi = HANDLENN; }
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
}
