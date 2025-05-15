//! `HANDLE`s to [pipe]s (created without [`file::FLAG_OVERLAPPED`])

use crate::prelude::*;
use io::{Read, Write};



#[doc(alias = "HANDLE")] #[doc = "<code>= [OwnedDuplex]</code>"  ] pub type Duplex    = OwnedDuplex;
#[doc(alias = "HANDLE")] #[doc = "<code>= [OwnedReader]</code>"  ] pub type Reader    = OwnedReader;
#[doc(alias = "HANDLE")] #[doc = "<code>= [OwnedWriter]</code>"  ] pub type Writer    = OwnedWriter;



#[doc(alias = "HANDLE")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createpipe)\]
/// Owned non-null pipe `HANDLE` ([Read] and [Write]able)
///
/// ### Hard Requirements
/// *   `HANDLE` cannot be null, on pain of undefined behavior
/// *   `HANDLE` must not have been created with [`file::FLAG_OVERLAPPED`], on pain of undefined behavior
/// *   `HANDLE` must be `CloseHandle`able, on pain of panics on drop (or undefined behavior?)
///
/// ### Soft Requirements
/// *   `HANDLE` should probably be a byte-oriented pipe, or *maybe* a socket.
/// *   `HANDLE` should probably be [`Read`]able and [`Write`]able.  Consider [`OwnedReader`] or [`OwnedWriter`] otherwise.
///
/// ### Alternatives
/// *   [`firehazard::pipe::sync::OwnedReader`] &mdash; read-only
/// *   [`firehazard::pipe::sync::OwnedWriter`] &mdash; write-only
/// *   [`firehazard::pipe::sync::BorrowedDuplex`] &mdash; borrowed instead of owned
/// *   [`firehazard::handle::Owned`] &mdash; untyped
/// *   [`std::io::PipeReader`] &mdash; not `#[repr(transparent)]`, read-only, cross platform
/// *   [`std::io::PipeWriter`] &mdash; not `#[repr(transparent)]`, write-only, cross platform
/// *   [`std::os::windows::io::OwnedHandle`] &mdash; untyped, permits null/invalid
///
#[repr(transparent)] pub struct OwnedDuplex(pub(super) HANDLENN);

#[doc(alias = "HANDLE")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createpipe)\]
/// Owned non-null pipe `HANDLE` ([Read]able end)
///
/// ### Hard Requirements
/// *   `HANDLE` cannot be null, on pain of undefined behavior
/// *   `HANDLE` must not have been created with [`file::FLAG_OVERLAPPED`], on pain of undefined behavior
/// *   `HANDLE` must be `CloseHandle`able, on pain of panics on drop (or undefined behavior?)
///
/// ### Soft Requirements
/// *   `HANDLE` should probably be a byte-oriented pipe, or *maybe* a socket.
/// *   `HANDLE` should probably be [`Read`]able.
///
/// ### Alternatives
/// *   [`firehazard::pipe::sync::OwnedDuplex`] &mdash; also writable
/// *   [`firehazard::pipe::sync::BorrowedReader`] &mdash; borrowed instead of owned
/// *   [`firehazard::handle::Owned`] &mdash; untyped
/// *   [`std::io::PipeReader`] &mdash; not `#[repr(transparent)]`, cross platform
/// *   [`std::os::windows::io::OwnedHandle`] &mdash; untyped, permits null/invalid
///
#[repr(transparent)] pub struct OwnedReader(pub(super) HANDLENN);

#[doc(alias = "HANDLE")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createpipe)\]
/// Owned non-null pipe `HANDLE` ([Write]able end)
///
/// ### Hard Requirements
/// *   `HANDLE` cannot be null, on pain of undefined behavior
/// *   `HANDLE` must not have been created with [`file::FLAG_OVERLAPPED`], on pain of undefined behavior
/// *   `HANDLE` must be `CloseHandle`able, on pain of panics on drop (or undefined behavior?)
///
/// ### Soft Requirements
/// *   `HANDLE` should probably be a byte-oriented pipe, or *maybe* a socket.
/// *   `HANDLE` should probably be [`Write`]able.
///
/// ### Alternatives
/// *   [`firehazard::pipe::sync::OwnedDuplex`] &mdash; also readable
/// *   [`firehazard::pipe::sync::BorrowedWriter`] &mdash; borrowed instead of owned
/// *   [`firehazard::handle::Owned`] &mdash; untyped
/// *   [`std::io::PipeWriter`] &mdash; not `#[repr(transparent)]`, cross platform
/// *   [`std::os::windows::io::OwnedHandle`] &mdash; untyped, permits null/invalid
///
#[repr(transparent)] pub struct OwnedWriter(pub(super) HANDLENN);

#[doc = r"\[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle)\] CloseHandle"] impl Drop for OwnedDuplex { fn drop(&mut self) { unsafe { drop_close_handle_nn(self) } } }
#[doc = r"\[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle)\] CloseHandle"] impl Drop for OwnedReader { fn drop(&mut self) { unsafe { drop_close_handle_nn(self) } } }
#[doc = r"\[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle)\] CloseHandle"] impl Drop for OwnedWriter { fn drop(&mut self) { unsafe { drop_close_handle_nn(self) } } }



#[doc(alias = "HANDLE")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createpipe)\]
/// Borrowed non-null pipe `HANDLE` ([Read] and [Write]able)
///
/// ### Hard Requirements
/// *   `HANDLE` cannot be null, on pain of undefined behavior
/// *   `HANDLE` must not have been created with [`file::FLAG_OVERLAPPED`], on pain of undefined behavior
/// *   Any `DuplicateHandle`d clones of `HANDLE` must be `CloseHandle`able, on pain of panics on drop (or undefined behavior?)
///
/// ### Soft Requirements
/// *   `HANDLE` should probably be a byte-oriented pipe, or *maybe* a socket.
/// *   `HANDLE` should probably be [`Read`]able and [`Write`]able.  Consider [`BorrowedReader`] or [`BorrowedWriter`] otherwise.
///
/// ### Alternatives
/// *   [`firehazard::pipe::sync::BorrowedReader`] &mdash; read-only
/// *   [`firehazard::pipe::sync::BorrowedWriter`] &mdash; write-only
/// *   [`firehazard::pipe::sync::OwnedDuplex`] &mdash; owned instead of borrowed
/// *   [`firehazard::handle::Borrowed`] &mdash; untyped
/// *   [`std::io::PipeReader`] &mdash; not `#[repr(transparent)]`, read-only, cross platform
/// *   [`std::io::PipeWriter`] &mdash; not `#[repr(transparent)]`, write-only, cross platform
/// *   [`std::os::windows::io::BorrowedHandle`] &mdash; untyped, permits null/invalid
///
#[repr(transparent)] pub struct BorrowedDuplex<'a>(pub(super) HANDLENN, PhantomData<&'a HANDLENN>);

#[doc(alias = "HANDLE")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createpipe)\]
/// Borrowed non-null pipe `HANDLE` ([Read]able end)
///
/// ### Hard Requirements
/// *   `HANDLE` cannot be null, on pain of undefined behavior
/// *   `HANDLE` must not have been created with [`file::FLAG_OVERLAPPED`], on pain of undefined behavior
/// *   Any `DuplicateHandle`d clones of `HANDLE` must be `CloseHandle`able, on pain of panics on drop (or undefined behavior?)
///
/// ### Soft Requirements
/// *   `HANDLE` should probably be a byte-oriented pipe, or *maybe* a socket.
/// *   `HANDLE` should probably be [`Read`]able.
///
/// ### Alternatives
/// *   [`firehazard::pipe::sync::BorrowedDuplex`] &mdash; also writable
/// *   [`firehazard::pipe::sync::OwnedReader`] &mdash; owned instead of borrowed
/// *   [`firehazard::handle::Borrowed`] &mdash; untyped
/// *   [`std::io::PipeReader`] &mdash; not `#[repr(transparent)]`, cross platform
/// *   [`std::os::windows::io::BorrowedHandle`] &mdash; untyped, permits null/invalid
///
#[repr(transparent)] pub struct BorrowedReader<'a>(pub(super) HANDLENN, PhantomData<&'a HANDLENN>);

#[doc(alias = "HANDLE")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createpipe)\]
/// Borrowed non-null pipe `HANDLE` ([Write]able end)
///
/// ### Hard Requirements
/// *   `HANDLE` cannot be null, on pain of undefined behavior
/// *   `HANDLE` must not have been created with [`file::FLAG_OVERLAPPED`], on pain of undefined behavior
/// *   Any `DuplicateHandle`d clones of `HANDLE` must be `CloseHandle`able, on pain of panics on drop (or undefined behavior?)
///
/// ### Soft Requirements
/// *   `HANDLE` should probably be a byte-oriented pipe, or *maybe* a socket.
/// *   `HANDLE` should probably be [`Write`]able.
///
/// ### Alternatives
/// *   [`firehazard::pipe::sync::BorrowedDuplex`] &mdash; also readable
/// *   [`firehazard::pipe::sync::OwnedWriter`] &mdash; owned instead of borrowed
/// *   [`firehazard::handle::Borrowed`] &mdash; untyped
/// *   [`std::io::PipeWriter`] &mdash; not `#[repr(transparent)]`, cross platform
/// *   [`std::os::windows::io::BorrowedHandle`] &mdash; untyped, permits null/invalid
///
#[repr(transparent)] pub struct BorrowedWriter<'a>(pub(super) HANDLENN, PhantomData<&'a HANDLENN>);



impl FromLocalHandle<c_void> for OwnedDuplex {
    unsafe fn from_raw_nn       (handle:  HANDLENN) ->  Self { unsafe { io::sync::type_check_owned_from_raw   ( handle) }; Self(handle) }
    unsafe fn borrow_from_raw_nn(handle: &HANDLENN) -> &Self { unsafe { io::sync::type_check_borrowed_from_raw(*handle); transmute(handle) } }
}

impl FromLocalHandle<c_void> for OwnedReader {
    unsafe fn from_raw_nn       (handle:  HANDLENN) ->  Self { unsafe { io::sync::type_check_owned_from_raw   ( handle) }; Self(handle) }
    unsafe fn borrow_from_raw_nn(handle: &HANDLENN) -> &Self { unsafe { io::sync::type_check_borrowed_from_raw(*handle); transmute(handle) } }
}

impl FromLocalHandle<c_void> for OwnedWriter {
    unsafe fn from_raw_nn       (handle:  HANDLENN) ->  Self { unsafe { io::sync::type_check_owned_from_raw   ( handle) }; Self(handle) }
    unsafe fn borrow_from_raw_nn(handle: &HANDLENN) -> &Self { unsafe { io::sync::type_check_borrowed_from_raw(*handle); transmute(handle) } }
}

impl FromLocalHandle<c_void> for BorrowedDuplex<'_> {
    unsafe fn from_raw_nn       (handle:  HANDLENN) -> Self  { unsafe { io::sync::type_check_borrowed_from_raw( handle) }; Self(handle, PhantomData) }
    unsafe fn borrow_from_raw_nn(handle: &HANDLENN) -> &Self { unsafe { io::sync::type_check_borrowed_from_raw(*handle); transmute(handle) } }
}

impl FromLocalHandle<c_void> for BorrowedReader<'_> {
    unsafe fn from_raw_nn       (handle:  HANDLENN) -> Self  { unsafe { io::sync::type_check_borrowed_from_raw( handle) }; Self(handle, PhantomData) }
    unsafe fn borrow_from_raw_nn(handle: &HANDLENN) -> &Self { unsafe { io::sync::type_check_borrowed_from_raw(*handle); transmute(handle) } }
}

impl FromLocalHandle<c_void> for BorrowedWriter<'_> {
    unsafe fn from_raw_nn       (handle:  HANDLENN) ->  Self { unsafe { io::sync::type_check_borrowed_from_raw( handle) }; Self(handle, PhantomData) }
    unsafe fn borrow_from_raw_nn(handle: &HANDLENN) -> &Self { unsafe { io::sync::type_check_borrowed_from_raw(*handle); transmute(handle) } }
}



handles!(unsafe impl AsLocalHandleNN<c_void>        for pipe::sync::{OwnedDuplex, OwnedReader, OwnedWriter, BorrowedDuplex<'_>, BorrowedReader<'_>, BorrowedWriter<'_>});
handles!(unsafe impl {Send, Sync}                   for pipe::sync::{OwnedDuplex, OwnedReader, OwnedWriter, BorrowedDuplex<'_>, BorrowedReader<'_>, BorrowedWriter<'_>}); // SAFETY: `std::io::Pipe*` is `Send+Sync` despite `try_clone(&self)`, `impl Read for &Pipe*`, etc. all sharing a `HANDLE` - if this is unsound, so is `std`.
handles!(       impl Debug                          for pipe::sync::{OwnedDuplex, OwnedReader, OwnedWriter, BorrowedDuplex<'_>, BorrowedReader<'_>, BorrowedWriter<'_>});

handles!(unsafe impl TryCloneToOwned<OwnedDuplex>   for pipe::sync::{OwnedDuplex, BorrowedDuplex<'_>});
handles!(unsafe impl TryCloneToOwned<OwnedReader>   for pipe::sync::{OwnedReader, BorrowedReader<'_>});
handles!(unsafe impl TryCloneToOwned<OwnedWriter>   for pipe::sync::{OwnedWriter, BorrowedWriter<'_>});

handles!(unsafe impl @convert     pipe::sync::OwnedDuplex       => pipe::sync::OwnedReader         );
handles!(unsafe impl @convert     pipe::sync::OwnedDuplex       => pipe::sync::OwnedWriter         );
handles!(unsafe impl @convert &'_ pipe::sync::OwnedDuplex       => pipe::sync::BorrowedDuplex<'_>  );
handles!(unsafe impl @convert &'_ pipe::sync::OwnedDuplex       => pipe::sync::BorrowedReader<'_>  );
handles!(unsafe impl @convert &'_ pipe::sync::OwnedDuplex       => pipe::sync::BorrowedWriter<'_>  );
handles!(unsafe impl @convert     pipe::sync::OwnedDuplex       => io::sync::OwnedDuplex           );
handles!(unsafe impl @convert     pipe::sync::OwnedDuplex       => io::sync::OwnedReader           );
handles!(unsafe impl @convert     pipe::sync::OwnedDuplex       => io::sync::OwnedWriter           );
handles!(unsafe impl @convert &'_ pipe::sync::OwnedDuplex       => io::sync::BorrowedDuplex<'_>    );
handles!(unsafe impl @convert &'_ pipe::sync::OwnedDuplex       => io::sync::BorrowedReader<'_>    );
handles!(unsafe impl @convert &'_ pipe::sync::OwnedDuplex       => io::sync::BorrowedWriter<'_>    );
handles!(unsafe impl @convert     pipe::sync::OwnedDuplex       => handle::Owned                   );
handles!(unsafe impl @convert &'_ pipe::sync::OwnedDuplex       => handle::Borrowed<'_>            );
handles!(unsafe impl @convert &'_ pipe::sync::OwnedDuplex       => handle::Pseudo<'_>              );

handles!(unsafe impl @convert &'_ pipe::sync::OwnedReader       => pipe::sync::BorrowedReader<'_>  );
handles!(unsafe impl @convert     pipe::sync::OwnedReader       => io::sync::OwnedReader           );
handles!(unsafe impl @convert &'_ pipe::sync::OwnedReader       => io::sync::BorrowedReader<'_>    );
handles!(unsafe impl @convert     pipe::sync::OwnedReader       => handle::Owned                   );
handles!(unsafe impl @convert &'_ pipe::sync::OwnedReader       => handle::Borrowed<'_>            );
handles!(unsafe impl @convert &'_ pipe::sync::OwnedReader       => handle::Pseudo<'_>              );

handles!(unsafe impl @convert &'_ pipe::sync::OwnedWriter       => pipe::sync::BorrowedWriter<'_>  );
handles!(unsafe impl @convert     pipe::sync::OwnedWriter       => io::sync::OwnedWriter           );
handles!(unsafe impl @convert &'_ pipe::sync::OwnedWriter       => io::sync::BorrowedWriter<'_>    );
handles!(unsafe impl @convert     pipe::sync::OwnedWriter       => handle::Owned                   );
handles!(unsafe impl @convert &'_ pipe::sync::OwnedWriter       => handle::Borrowed<'_>            );
handles!(unsafe impl @convert &'_ pipe::sync::OwnedWriter       => handle::Pseudo<'_>              );

handles!(unsafe impl @convert pipe::sync::BorrowedDuplex<'_>    => pipe::sync::BorrowedReader<'_>  );
handles!(unsafe impl @convert pipe::sync::BorrowedDuplex<'_>    => pipe::sync::BorrowedWriter<'_>  );
handles!(unsafe impl @convert pipe::sync::BorrowedDuplex<'_>    => io::sync::BorrowedDuplex<'_>    );
handles!(unsafe impl @convert pipe::sync::BorrowedDuplex<'_>    => io::sync::BorrowedReader<'_>    );
handles!(unsafe impl @convert pipe::sync::BorrowedDuplex<'_>    => io::sync::BorrowedWriter<'_>    );
handles!(unsafe impl @convert pipe::sync::BorrowedDuplex<'_>    => handle::Borrowed<'_>            );
handles!(unsafe impl @convert pipe::sync::BorrowedDuplex<'_>    => handle::Pseudo<'_>              );

handles!(unsafe impl @convert pipe::sync::BorrowedReader<'_>    => io::sync::BorrowedReader<'_>    );
handles!(unsafe impl @convert pipe::sync::BorrowedReader<'_>    => handle::Borrowed<'_>            );
handles!(unsafe impl @convert pipe::sync::BorrowedReader<'_>    => handle::Pseudo<'_>              );

handles!(unsafe impl @convert pipe::sync::BorrowedWriter<'_>    => io::sync::BorrowedWriter<'_>    );
handles!(unsafe impl @convert pipe::sync::BorrowedWriter<'_>    => handle::Borrowed<'_>            );
handles!(unsafe impl @convert pipe::sync::BorrowedWriter<'_>    => handle::Pseudo<'_>              );



//pl Read   for &'_ pipe::sync::OwnedDuplex         { fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { unsafe { Ok(read_file(*self, buf, None).map(usize::from32)?) } } } // deadlock bait, do not implement - see "Quirks: Serialized I/O" rant on `mod pipe::named`
impl Read   for     pipe::sync::OwnedDuplex         { fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { unsafe { Ok(read_file( self, buf, None).map(usize::from32)?) } } }
impl Read   for &'_ pipe::sync::OwnedReader         { fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { unsafe { Ok(read_file(*self, buf, None).map(usize::from32)?) } } }
impl Read   for     pipe::sync::OwnedReader         { fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { unsafe { Ok(read_file( self, buf, None).map(usize::from32)?) } } }
//pl Read   for &'_ pipe::sync::BorrowedDuplex<'_>  { fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { unsafe { Ok(read_file(*self, buf, None).map(usize::from32)?) } } } // deadlock bait, do not implement - see "Quirks: Serialized I/O" rant on `mod pipe::named`
impl Read   for     pipe::sync::BorrowedDuplex<'_>  { fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { unsafe { Ok(read_file( self, buf, None).map(usize::from32)?) } } }
impl Read   for &'_ pipe::sync::BorrowedReader<'_>  { fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { unsafe { Ok(read_file(*self, buf, None).map(usize::from32)?) } } }
impl Read   for     pipe::sync::BorrowedReader<'_>  { fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { unsafe { Ok(read_file( self, buf, None).map(usize::from32)?) } } }

// noop flush sane: https://github.com/rust-lang/rust/blob/c2110769cd58cd3b0c31f308c8cfeab5e19340fd/library/std/src/io/pipe.rs#L271-L274
//pl Write  for &'_ pipe::sync::OwnedDuplex         { fn write(&mut self, buf: &[u8]) -> io::Result<usize> { unsafe { Ok(write_file(*self, buf, None).map(usize::from32)?) } } fn flush(&mut self) -> io::Result<()> { Ok(()) } } // deadlock bait, do not implement - see "Quirks: Serialized I/O" rant on `mod pipe::named`
impl Write  for     pipe::sync::OwnedDuplex         { fn write(&mut self, buf: &[u8]) -> io::Result<usize> { unsafe { Ok(write_file( self, buf, None).map(usize::from32)?) } } fn flush(&mut self) -> io::Result<()> { Ok(()) } }
impl Write  for &'_ pipe::sync::OwnedWriter         { fn write(&mut self, buf: &[u8]) -> io::Result<usize> { unsafe { Ok(write_file(*self, buf, None).map(usize::from32)?) } } fn flush(&mut self) -> io::Result<()> { Ok(()) } }
impl Write  for     pipe::sync::OwnedWriter         { fn write(&mut self, buf: &[u8]) -> io::Result<usize> { unsafe { Ok(write_file( self, buf, None).map(usize::from32)?) } } fn flush(&mut self) -> io::Result<()> { Ok(()) } }
//pl Write  for &'_ pipe::sync::BorrowedDuplex<'_>  { fn write(&mut self, buf: &[u8]) -> io::Result<usize> { unsafe { Ok(write_file(*self, buf, None).map(usize::from32)?) } } fn flush(&mut self) -> io::Result<()> { Ok(()) } } // deadlock bait, do not implement - see "Quirks: Serialized I/O" rant on `mod pipe::named`
impl Write  for     pipe::sync::BorrowedDuplex<'_>  { fn write(&mut self, buf: &[u8]) -> io::Result<usize> { unsafe { Ok(write_file( self, buf, None).map(usize::from32)?) } } fn flush(&mut self) -> io::Result<()> { Ok(()) } }
impl Write  for &'_ pipe::sync::BorrowedWriter<'_>  { fn write(&mut self, buf: &[u8]) -> io::Result<usize> { unsafe { Ok(write_file(*self, buf, None).map(usize::from32)?) } } fn flush(&mut self) -> io::Result<()> { Ok(()) } }
impl Write  for     pipe::sync::BorrowedWriter<'_>  { fn write(&mut self, buf: &[u8]) -> io::Result<usize> { unsafe { Ok(write_file( self, buf, None).map(usize::from32)?) } } fn flush(&mut self) -> io::Result<()> { Ok(()) } }

impl crate::os::windows::io::FromRawHandle for pipe::sync::OwnedDuplex { unsafe fn from_raw_handle(handle: crate::os::windows::io::RawHandle) -> Self { Self(HANDLENN::new(handle.cast()).expect("undefined behavior: null is not an open, owned handle")) } }
impl crate::os::windows::io::FromRawHandle for pipe::sync::OwnedReader { unsafe fn from_raw_handle(handle: crate::os::windows::io::RawHandle) -> Self { Self(HANDLENN::new(handle.cast()).expect("undefined behavior: null is not an open, owned handle")) } }
impl crate::os::windows::io::FromRawHandle for pipe::sync::OwnedWriter { unsafe fn from_raw_handle(handle: crate::os::windows::io::RawHandle) -> Self { Self(HANDLENN::new(handle.cast()).expect("undefined behavior: null is not an open, owned handle")) } }

impl crate::os::windows::io::IntoRawHandle for pipe::sync::OwnedDuplex { fn into_raw_handle(self) -> crate::os::windows::io::RawHandle { self.into_handle().cast() } }
impl crate::os::windows::io::IntoRawHandle for pipe::sync::OwnedReader { fn into_raw_handle(self) -> crate::os::windows::io::RawHandle { self.into_handle().cast() } }
impl crate::os::windows::io::IntoRawHandle for pipe::sync::OwnedWriter { fn into_raw_handle(self) -> crate::os::windows::io::RawHandle { self.into_handle().cast() } }

unsafe impl valrow::Borrowable for pipe::sync::OwnedDuplex  { type Abi = HANDLENN; }
unsafe impl valrow::Borrowable for pipe::sync::OwnedReader  { type Abi = HANDLENN; }
unsafe impl valrow::Borrowable for pipe::sync::OwnedWriter  { type Abi = HANDLENN; }
//safe impl valrow::Borrowable for pipe::sync::Borrowed*    { type Abi = HANDLENN; } // valid but pointless

impl CloneToOwned for pipe::sync::OwnedDuplex {}
impl CloneToOwned for pipe::sync::OwnedReader {}
impl CloneToOwned for pipe::sync::OwnedWriter {}

impl CloneToOwned for pipe::sync::BorrowedDuplex<'_> {}
impl CloneToOwned for pipe::sync::BorrowedReader<'_> {}
impl CloneToOwned for pipe::sync::BorrowedWriter<'_> {}



#[cfg(test)] mod tests {
    use crate::prelude::*;
    use crate::os::windows::io::FromRawHandle;

    #[test] #[should_panic = "undefined behavior"] fn null_firehazard_pipe_duplex() {
        let _null = unsafe { pipe::sync::OwnedDuplex::from_raw_handle(null_mut()) };
    }

    #[test] #[should_panic = "undefined behavior"] fn null_firehazard_pipe_reader() {
        let _null = unsafe { pipe::sync::OwnedReader::from_raw_handle(null_mut()) };
    }

    #[test] #[should_panic = "undefined behavior"] fn null_firehazard_pipe_writer() {
        let _null = unsafe { pipe::sync::OwnedWriter::from_raw_handle(null_mut()) };
    }
}
