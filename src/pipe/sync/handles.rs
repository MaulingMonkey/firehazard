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



handles!(unsafe impl {Send, Sync}                   for pipe::sync::{OwnedDuplex, OwnedReader, OwnedWriter, BorrowedDuplex<'_>, BorrowedReader<'_>, BorrowedWriter<'_>}); // SAFETY: `std::io::Pipe*` is `Send+Sync` despite `try_clone(&self)`, `impl Read for &Pipe*`, etc. all sharing a `HANDLE` - if this is unsound, so is `std`.
handles!(       impl Debug                          for pipe::sync::{OwnedDuplex, OwnedReader, OwnedWriter, BorrowedDuplex<'_>, BorrowedReader<'_>, BorrowedWriter<'_>});
