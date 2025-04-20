#[cfg(doc)] use crate as firehazard;
use crate::*;

use winapi::ctypes::c_void;



#[doc(alias = "HANDLE")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createpipe)\] Owned non-null pipe `HANDLE` ([io::Read] and [io::Write]able)
///
/// ### Alternatives
/// *   [`std::io::PipeReader`](https://doc.rust-lang.org/beta/std/io/struct.PipeReader.html) &mdash; no ABI guarantees, read-only, cross platform, not yet stable
/// *   [`std::io::PipeWriter`](https://doc.rust-lang.org/beta/std/io/struct.PipeWriter.html) &mdash; no ABI guarantees, write-only, cross platform, not yet stable
/// *   [`std::os::windows::io::OwnedHandle`] &mdash; untyped, permits null/invalid
/// *   [`firehazard::handle::Owned`] &mdash; untyped
/// *   [`firehazard::io::WriteHandle`] &mdash; borrowed instead of owned
/// *   [`firehazard::pipe::ReaderNN`] &mdash; read-only
/// *   [`firehazard::pipe::WriterNN`] &mdash; write-only
///
#[repr(transparent)] pub struct DuplexNN(pub(super) HANDLENN);

#[doc(alias = "HANDLE")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createpipe)\] Owned non-null pipe `HANDLE` ([io::Read]able end)
///
/// ### Alternatives
/// *   [`std::io::PipeReader`](https://doc.rust-lang.org/beta/std/io/struct.PipeReader.html) &mdash; no ABI guarantees, cross platform, not yet stable
/// *   [`std::os::windows::io::OwnedHandle`] &mdash; untyped, permits null/invalid
/// *   [`firehazard::handle::Owned`] &mdash; untyped
/// *   [`firehazard::io::ReadHandle`] &mdash; borrowed instead of owned
/// *   [`firehazard::pipe::DuplexNN`] &mdash; also writeable
///
#[repr(transparent)] pub struct ReaderNN(pub(super) HANDLENN);

#[doc(alias = "HANDLE")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createpipe)\] Owned non-null pipe `HANDLE` ([io::Write]able end)
///
/// ### Alternatives
/// *   [`std::io::PipeWriter`](https://doc.rust-lang.org/beta/std/io/struct.PipeWriter.html) &mdash; no ABI guarantees, cross platform, not yet stable
/// *   [`std::os::windows::io::OwnedHandle`] &mdash; untyped, permits null/invalid
/// *   [`firehazard::handle::Owned`] &mdash; untyped
/// *   [`firehazard::io::WriteHandle`] &mdash; borrowed instead of owned
/// *   [`firehazard::pipe::DuplexNN`] &mdash; also readable
///
#[repr(transparent)] pub struct WriterNN(pub(super) HANDLENN);

// TODO: Listener/Server Pipes?
// TODO: Message Pipes?



handles!(unsafe impl *LocalHandleNN<c_void>         for pipe::{DuplexNN});
handles!(       impl AsRef<Self>                    for pipe::{DuplexNN});
handles!(unsafe impl {Send, Sync}                   for pipe::{DuplexNN}); // SAFETY: `std::io::PipeReader` is `Send+Sync` despite `try_clone(&self)`, `impl Read for &PipeReader`, etc. all sharing a `HANDLE` - if this is unsound, so is `std`.
handles!(       impl Debug                          for pipe::{DuplexNN});

handles!(unsafe impl *LocalHandleNN<c_void>         for pipe::{ReaderNN});
handles!(       impl AsRef<Self>                    for pipe::{ReaderNN});
handles!(unsafe impl {Send, Sync}                   for pipe::{ReaderNN}); // SAFETY: `std::io::PipeReader` is `Send+Sync` despite `try_clone(&self)`, `impl Read for &PipeReader`, etc. all sharing a `HANDLE` - if this is unsound, so is `std`.
handles!(       impl Debug                          for pipe::{ReaderNN});

handles!(unsafe impl *LocalHandleNN<c_void>         for pipe::{WriterNN});
handles!(       impl AsRef<Self>                    for pipe::{WriterNN});
handles!(unsafe impl {Send, Sync}                   for pipe::{WriterNN}); // SAFETY: `std::io::PipeWriter` is `Send+Sync` despite `try_clone(&self)`, `impl Write for &PipeWriter`, etc. all sharing a `HANDLE` - if this is unsound, so is `std`.
handles!(       impl Debug                          for pipe::{WriterNN});

handles!(unsafe impl @convert     pipe::DuplexNN => pipe::ReaderNN          );
handles!(unsafe impl @convert     pipe::DuplexNN => pipe::WriterNN          );
handles!(unsafe impl @convert &'_ pipe::DuplexNN => io::ReadHandle<'_>      );
handles!(unsafe impl @convert &'_ pipe::DuplexNN => io::WriteHandle<'_>     );
handles!(unsafe impl @convert     pipe::DuplexNN => handle::Owned           );
handles!(unsafe impl @convert &'_ pipe::DuplexNN => handle::Borrowed<'_>    );
handles!(unsafe impl @convert &'_ pipe::DuplexNN => handle::Pseudo<'_>      );

handles!(unsafe impl @convert &'_ pipe::ReaderNN => io::ReadHandle<'_>      );
handles!(unsafe impl @convert     pipe::ReaderNN => handle::Owned           );
handles!(unsafe impl @convert &'_ pipe::ReaderNN => handle::Borrowed<'_>    );
handles!(unsafe impl @convert &'_ pipe::ReaderNN => handle::Pseudo<'_>      );

handles!(unsafe impl @convert &'_ pipe::WriterNN => io::WriteHandle<'_>     );
handles!(unsafe impl @convert     pipe::WriterNN => handle::Owned           );
handles!(unsafe impl @convert &'_ pipe::WriterNN => handle::Borrowed<'_>    );
handles!(unsafe impl @convert &'_ pipe::WriterNN => handle::Pseudo<'_>      );

#[doc = r"\[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle)\] CloseHandle"] impl Drop for DuplexNN { fn drop(&mut self) { unsafe { drop_close_handle_nn(self) } } }
#[doc = r"\[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle)\] CloseHandle"] impl Drop for ReaderNN { fn drop(&mut self) { unsafe { drop_close_handle_nn(self) } } }
#[doc = r"\[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle)\] CloseHandle"] impl Drop for WriterNN { fn drop(&mut self) { unsafe { drop_close_handle_nn(self) } } }

impl io::Read   for &'_ pipe::DuplexNN  { fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { unsafe { Ok(read_file(*self, buf, None).map(usize::from32)?) } } }
impl io::Read   for     pipe::DuplexNN  { fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { unsafe { Ok(read_file( self, buf, None).map(usize::from32)?) } } }
impl io::Read   for &'_ pipe::ReaderNN  { fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { unsafe { Ok(read_file(*self, buf, None).map(usize::from32)?) } } }
impl io::Read   for     pipe::ReaderNN  { fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { unsafe { Ok(read_file( self, buf, None).map(usize::from32)?) } } }

// noop flush sane: https://github.com/rust-lang/rust/blob/c2110769cd58cd3b0c31f308c8cfeab5e19340fd/library/std/src/io/pipe.rs#L271-L274
impl io::Write  for &'_ pipe::DuplexNN  { fn write(&mut self, buf: &[u8]) -> io::Result<usize> { unsafe { Ok(write_file(*self, buf, None).map(usize::from32)?) } } fn flush(&mut self) -> io::Result<()> { Ok(()) } }
impl io::Write  for     pipe::DuplexNN  { fn write(&mut self, buf: &[u8]) -> io::Result<usize> { unsafe { Ok(write_file( self, buf, None).map(usize::from32)?) } } fn flush(&mut self) -> io::Result<()> { Ok(()) } }
impl io::Write  for &'_ pipe::WriterNN  { fn write(&mut self, buf: &[u8]) -> io::Result<usize> { unsafe { Ok(write_file(*self, buf, None).map(usize::from32)?) } } fn flush(&mut self) -> io::Result<()> { Ok(()) } }
impl io::Write  for     pipe::WriterNN  { fn write(&mut self, buf: &[u8]) -> io::Result<usize> { unsafe { Ok(write_file( self, buf, None).map(usize::from32)?) } } fn flush(&mut self) -> io::Result<()> { Ok(()) } }

impl crate::os::windows::io::FromRawHandle for pipe::DuplexNN   { unsafe fn from_raw_handle(handle: crate::os::windows::io::RawHandle) -> Self { Self(HANDLENN::new(handle.cast()).expect("undefined behavior: null is not an open, owned handle")) } }
impl crate::os::windows::io::FromRawHandle for pipe::ReaderNN   { unsafe fn from_raw_handle(handle: crate::os::windows::io::RawHandle) -> Self { Self(HANDLENN::new(handle.cast()).expect("undefined behavior: null is not an open, owned handle")) } }
impl crate::os::windows::io::FromRawHandle for pipe::WriterNN   { unsafe fn from_raw_handle(handle: crate::os::windows::io::RawHandle) -> Self { Self(HANDLENN::new(handle.cast()).expect("undefined behavior: null is not an open, owned handle")) } }

impl crate::os::windows::io::IntoRawHandle for pipe::DuplexNN   { fn into_raw_handle(self) -> crate::os::windows::io::RawHandle { self.into_handle().cast() } }
impl crate::os::windows::io::IntoRawHandle for pipe::ReaderNN   { fn into_raw_handle(self) -> crate::os::windows::io::RawHandle { self.into_handle().cast() } }
impl crate::os::windows::io::IntoRawHandle for pipe::WriterNN   { fn into_raw_handle(self) -> crate::os::windows::io::RawHandle { self.into_handle().cast() } }

unsafe impl valrow::Borrowable for pipe::DuplexNN      { type Abi = HANDLENN; }
unsafe impl valrow::Borrowable for pipe::ReaderNN      { type Abi = HANDLENN; }
unsafe impl valrow::Borrowable for pipe::WriterNN      { type Abi = HANDLENN; }

impl pipe::DuplexNN { #[doc(alias = "DuplicateHandle")] #[doc = r"\[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-duplicatehandle)\] DuplicateHandle"] pub fn try_clone(&self) -> Result<Self, Error> { Ok(Self(duplicate_handle_local_same_access(self, false)?.into_handle_nn())) } }
impl pipe::ReaderNN { #[doc(alias = "DuplicateHandle")] #[doc = r"\[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-duplicatehandle)\] DuplicateHandle"] pub fn try_clone(&self) -> Result<Self, Error> { Ok(Self(duplicate_handle_local_same_access(self, false)?.into_handle_nn())) } }
impl pipe::WriterNN { #[doc(alias = "DuplicateHandle")] #[doc = r"\[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-duplicatehandle)\] DuplicateHandle"] pub fn try_clone(&self) -> Result<Self, Error> { Ok(Self(duplicate_handle_local_same_access(self, false)?.into_handle_nn())) } }



#[cfg(test)] mod tests {
    use crate::*;
    use crate::os::windows::io::FromRawHandle;
    use core::ptr::null_mut;

    #[test] #[should_panic = "undefined behavior"] fn null_firehazard_pipe_duplex() {
        let _null = unsafe { pipe::DuplexNN::from_raw_handle(null_mut()) };
    }

    #[test] #[should_panic = "undefined behavior"] fn null_firehazard_pipe_reader() {
        let _null = unsafe { pipe::ReaderNN::from_raw_handle(null_mut()) };
    }

    #[test] #[should_panic = "undefined behavior"] fn null_firehazard_pipe_writer() {
        let _null = unsafe { pipe::WriterNN::from_raw_handle(null_mut()) };
    }
}
