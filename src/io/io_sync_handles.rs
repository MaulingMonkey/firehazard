//! `HANDLE`s to [file](mod@file)s, [pipe]s, or sockets (created without [`file::FLAG_OVERLAPPED`])
//!
//! Note that while any of these handle types *can* wrap any of the above types of kernel objects:
//! *   [`Duplex`]([`Handle`](DuplexHandle)) makes little sense for [file](mod@file)s, as read and write share a seek position
//! *   [`File`]([`Handle`](FileHandle)) makes little sense for [pipe]s or sockets, since those cannot be seeked
//!
//! As such, <code>impl [From]<[OwnedFile]> for [OwnedDuplex]</code> and similar is not implemented, as it'd usually be a bug.
//! If you need this typically nonsensical operation, use <code>{[as](BorrowedFile::as_duplex_despite_shared_seek_position),[into}_duplex_despite_shared_seek_position](OwnedFile::into_duplex_despite_shared_seek_position)</code> instead.
//! Note that it's legal and sound to wrap an unreadable handle in [`OwnedReader`], and unwritable handle in [`OwnedWriter`],
//! an unseekable handle in [`OwnedFile`], etc. - this menagerie of handle types is a bunch of best-effort guard rails, not a promise of access.
//!
//! | Implements                            | Owned ([`io::sync`]) <br> (non-null) <br> `#[repr(transparent)]`  | Borrowed ([`io::sync`]) <br> (non-null) <br> `#[repr(transparent)]`   | Owned ([`std`]) <br> (may be overlapped) <br> (not `#[repr(transparent)]`)    |
//! | --------------------------------------| ------------------------------------------------------------------| ----------------------------------------------------------------------| ------------------------------------------------------------------------------|
//! | [`Read`]                              | [`OwnedReader`]                                                   | [`BorrowedReader`]                                                    | [`std::io::PipeReader`]                                                       |
//! | [`Write`]                             | [`OwnedWriter`]                                                   | [`BorrowedWriter`]                                                    | [`std::io::PipeWriter`]                                                       |
//! | [`Read`] + [`Write`]                  | [`OwnedDuplex`]                                                   | [`BorrowedDuplex`]                                                    | [`std::net::TcpStream`]                                                       |
//! | [`Read`] + [`Write`] + [`Seek`]       | [`OwnedFile`]                                                     | [`BorrowedFile`]                                                      | [`std::fs::File`]                                                             |
//! | [`Read`] + [`Seek`]                   | <strike style="opacity: 25%">`ReadOnlyFile`?</strike>             | <strike style="opacity: 25%">`ReadOnlyFileHandle`?</strike>           | <span style="opacity: 25%">N/A</span>                                         |
//! | [`Write`] + [`Seek`]                  | <strike style="opacity: 25%">`WriteOnlyFile`?</strike>            | <strike style="opacity: 25%">`WriteOnlyFileHandle`?</strike>          | <span style="opacity: 25%">N/A</span>                                         |
//! | [`Seek`]                              | <span style="opacity: 25%">N/A</span>                             | <span style="opacity: 25%">N/A</span>                                 | <span style="opacity: 25%">N/A</span>                                         |



use crate::prelude::*;
use crate::io::{Read, Write, Seek};

#[doc(alias = "HANDLE")] #[doc = "<code>= [OwnedFile]</code>"    ] pub type File      = OwnedFile;
#[doc(alias = "HANDLE")] #[doc = "<code>= [OwnedDuplex]</code>"  ] pub type Duplex    = OwnedDuplex;
#[doc(alias = "HANDLE")] #[doc = "<code>= [OwnedReader]</code>"  ] pub type Reader    = OwnedReader;
#[doc(alias = "HANDLE")] #[doc = "<code>= [OwnedWriter]</code>"  ] pub type Writer    = OwnedWriter;



#[doc(alias = "HANDLE")]
/// Owned non-null non-OVERLAPPED file `HANDLE`
///
/// ### Alternatives
/// *   [`firehazard::io::sync::BorrowedFile`]      &mdash; borrowed instead of owned
/// *   [`firehazard::handle::Owned`]               &mdash; untyped, possibly marked [`file::FLAG_OVERLAPPED`]
/// *   [`std::fs::File`]                           &mdash; cross platform, not `#[repr(transparent)]`, possibly marked [`file::FLAG_OVERLAPPED`]
/// *   [`std::os::windows::io::OwnedHandle`]       &mdash; [`std`], untyped, permits null/invalid, possibly marked [`file::FLAG_OVERLAPPED`]
///
#[repr(transparent)] pub struct OwnedFile(HANDLENN);

#[doc(alias = "HANDLE")]
/// Owned non-null non-OVERLAPPED readable and writable pipe or socket `HANDLE`
///
/// ### Alternatives
/// *   [`firehazard::io::sync::BorrowedDuplex`]    &mdash; borrowed instead of owned
/// *   [`firehazard::handle::Owned`]               &mdash; untyped, possibly marked [`file::FLAG_OVERLAPPED`]
/// *   [`std::fs::File`]                           &mdash; cross platform, not `#[repr(transparent)]`, [`Seek`]able, possibly marked [`file::FLAG_OVERLAPPED`]
/// *   [`std::net::TcpStream`]                     &mdash; cross platform, not `#[repr(transparent)]`, possibly marked [`file::FLAG_OVERLAPPED`], socket specific fns
/// *   [`std::os::windows::io::OwnedHandle`]       &mdash; [`std`], untyped, permits null/invalid, possibly marked [`file::FLAG_OVERLAPPED`]
///
#[repr(transparent)] pub struct OwnedDuplex(HANDLENN);

#[doc(alias = "HANDLE")]
/// Owned non-null non-OVERLAPPED readable file, pipe, or socket `HANDLE`
///
/// ### Alternatives
/// *   [`firehazard::io::sync::BorrowedReader`]    &mdash; borrowed instead of owned
/// *   [`firehazard::handle::Owned`]               &mdash; untyped, possibly marked [`file::FLAG_OVERLAPPED`]
/// *   [`std::io::PipeReader`](https://doc.rust-lang.org/beta/std/io/struct.PipeReader.html) &mdash; cross platform, not `#[repr(transparent)]`, not yet stable, possibly marked [`file::FLAG_OVERLAPPED`]
/// *   [`std::os::windows::io::OwnedHandle`]       &mdash; [`std`], untyped, permits null/invalid, possibly marked [`file::FLAG_OVERLAPPED`]
///
#[repr(transparent)] pub struct OwnedReader(HANDLENN);

#[doc(alias = "HANDLE")]
/// Owned non-null non-OVERLAPPED writable file, pipe, or socket `HANDLE`
///
/// ### Alternatives
/// *   [`firehazard::io::sync::BorrowedWriter`]    &mdash; borrowed instead of owned
/// *   [`firehazard::handle::Owned`]               &mdash; untyped, possibly marked [`file::FLAG_OVERLAPPED`]
/// *   [`std::io::PipeWriter`](https://doc.rust-lang.org/beta/std/io/struct.PipeWriter.html) &mdash; cross platform, not `#[repr(transparent)]`, not yet stable, possibly marked [`file::FLAG_OVERLAPPED`]
/// *   [`std::os::windows::io::OwnedHandle`]       &mdash; [`std`], untyped, permits null/invalid, possibly marked [`file::FLAG_OVERLAPPED`]
///
#[repr(transparent)] pub struct OwnedWriter(HANDLENN);



#[doc(alias = "HANDLE")]
/// Borrowed non-null non-OVERLAPPED file `HANDLE`
///
/// ### Alternatives
/// *   [`firehazard::io::sync::OwnedFile`]         &mdash; owned instead of borrowed
/// *   [`firehazard::handle::Borrowed`]            &mdash; untyped, possibly marked [`file::FLAG_OVERLAPPED`]
/// *   <code>&amp;[std::fs::File]</code>           &mdash; cross platform, not `#[repr(transparent)]`, possibly marked [`file::FLAG_OVERLAPPED`]
/// *   [`std::os::windows::io::BorrowedHandle`]    &mdash; [`std`], untyped, permits null/invalid, possibly marked [`file::FLAG_OVERLAPPED`]
///
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct BorrowedFile<'a>(HANDLENN, PhantomData<&'a HANDLENN>);

#[doc(alias = "HANDLE")]
/// Borrowed non-null non-OVERLAPPED pipe or socket `HANDLE`
///
/// ### Alternatives
/// *   [`firehazard::io::sync::OwnedDuplex`]       &mdash; owned instead of borrowed
/// *   [`firehazard::handle::Borrowed`]            &mdash; untyped, possibly marked [`file::FLAG_OVERLAPPED`]
/// *   <code>&amp;[std::fs::File]</code>           &mdash; cross platform, not `#[repr(transparent)]`, [`Seek`]able, possibly marked [`file::FLAG_OVERLAPPED`]
/// *   <code>&amp;[std::net::TcpStream]</code>     &mdash; cross platform, not `#[repr(transparent)]`, possibly marked [`file::FLAG_OVERLAPPED`], socket specific fns
/// *   [`std::os::windows::io::BorrowedHandle`]    &mdash; [`std`], untyped, permits null/invalid, possibly marked [`file::FLAG_OVERLAPPED`]
///
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct BorrowedDuplex<'a>(HANDLENN, PhantomData<&'a HANDLENN>);

#[doc(alias = "HANDLE")]
/// Borrowed non-null non-OVERLAPPED readable file, pipe, or socket `HANDLE`
///
/// ### Alternatives
/// *   [`firehazard::io::sync::OwnedReader`]       &mdash; owned instead of borrowed
/// *   [`firehazard::handle::Borrowed`]            &mdash; untyped, possibly marked [`file::FLAG_OVERLAPPED`]
/// *   <code>&amp;[std::io::PipeReader](https://doc.rust-lang.org/beta/std/io/struct.PipeReader.html)</code> &mdash; cross platform, not `#[repr(transparent)]`, not yet stable, possibly marked [`file::FLAG_OVERLAPPED`]
/// *   [`std::os::windows::io::BorrowedHandle`]    &mdash; [`std`], untyped, permits null/invalid, possibly marked [`file::FLAG_OVERLAPPED`]
///
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct BorrowedReader<'a>(HANDLENN, PhantomData<&'a HANDLENN>);

#[doc(alias = "HANDLE")]
/// Borrowed non-null non-OVERLAPPED writable file, pipe, or socket `HANDLE`
///
/// ### Alternatives
/// *   [`firehazard::io::sync::OwnedWriter`]       &mdash; owned instead of borrowed
/// *   [`firehazard::handle::Borrowed`]            &mdash; untyped, possibly marked [`file::FLAG_OVERLAPPED`]
/// *   <code>&amp;[std::io::PipeWriter](https://doc.rust-lang.org/beta/std/io/struct.PipeWriter.html)</code> &mdash; cross platform, not `#[repr(transparent)]`, not yet stable, possibly marked [`file::FLAG_OVERLAPPED`]
/// *   [`std::os::windows::io::BorrowedHandle`]    &mdash; [`std`], untyped, permits null/invalid, possibly marked [`file::FLAG_OVERLAPPED`]
///
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct BorrowedWriter<'a>(HANDLENN, PhantomData<&'a HANDLENN>);



impl FromLocalHandle<c_void> for OwnedFile {
    unsafe fn from_raw_nn       (handle:  HANDLENN) ->  Self { unsafe { type_check_owned_from_raw   ( handle) }; Self(handle) }
    unsafe fn borrow_from_raw_nn(handle: &HANDLENN) -> &Self { unsafe { type_check_borrowed_from_raw(*handle); transmute(handle) } }
}

impl FromLocalHandle<c_void> for OwnedDuplex {
    unsafe fn from_raw_nn       (handle:  HANDLENN) ->  Self { unsafe { type_check_owned_from_raw   ( handle) }; Self(handle) }
    unsafe fn borrow_from_raw_nn(handle: &HANDLENN) -> &Self { unsafe { type_check_borrowed_from_raw(*handle); transmute(handle) } }
}

impl FromLocalHandle<c_void> for OwnedReader {
    unsafe fn from_raw_nn       (handle:  HANDLENN) ->  Self { unsafe { type_check_owned_from_raw   ( handle) }; Self(handle) }
    unsafe fn borrow_from_raw_nn(handle: &HANDLENN) -> &Self { unsafe { type_check_borrowed_from_raw(*handle); transmute(handle) } }
}

impl FromLocalHandle<c_void> for OwnedWriter {
    unsafe fn from_raw_nn       (handle:  HANDLENN) ->  Self { unsafe { type_check_owned_from_raw   ( handle) }; Self(handle) }
    unsafe fn borrow_from_raw_nn(handle: &HANDLENN) -> &Self { unsafe { type_check_borrowed_from_raw(*handle); transmute(handle) } }
}

impl FromLocalHandle<c_void> for BorrowedFile<'_> {
    unsafe fn from_raw_nn       (handle:  HANDLENN) ->  Self { unsafe { type_check_borrowed_from_raw( handle) }; Self(handle, PhantomData) }
    unsafe fn borrow_from_raw_nn(handle: &HANDLENN) -> &Self { unsafe { type_check_borrowed_from_raw(*handle); transmute(handle) } }
}

impl FromLocalHandle<c_void> for BorrowedDuplex<'_> {
    unsafe fn from_raw_nn       (handle:  HANDLENN) -> Self  { unsafe { type_check_borrowed_from_raw( handle) }; Self(handle, PhantomData) }
    unsafe fn borrow_from_raw_nn(handle: &HANDLENN) -> &Self { unsafe { type_check_borrowed_from_raw(*handle); transmute(handle) } }
}

impl FromLocalHandle<c_void> for BorrowedReader<'_> {
    unsafe fn from_raw_nn       (handle:  HANDLENN) -> Self  { unsafe { type_check_borrowed_from_raw( handle) }; Self(handle, PhantomData) }
    unsafe fn borrow_from_raw_nn(handle: &HANDLENN) -> &Self { unsafe { type_check_borrowed_from_raw(*handle); transmute(handle) } }
}

impl FromLocalHandle<c_void> for BorrowedWriter<'_> {
    unsafe fn from_raw_nn       (handle:  HANDLENN) ->  Self { unsafe { type_check_borrowed_from_raw( handle) }; Self(handle, PhantomData) }
    unsafe fn borrow_from_raw_nn(handle: &HANDLENN) -> &Self { unsafe { type_check_borrowed_from_raw(*handle); transmute(handle) } }
}



handles!(unsafe impl AsLocalHandleNN<c_void>                for io::sync::{OwnedFile, OwnedDuplex, OwnedReader, OwnedWriter, BorrowedFile<'_>, BorrowedDuplex<'_>, BorrowedReader<'_>, BorrowedWriter<'_>});
handles!(unsafe impl {Send, Sync}                           for io::sync::{OwnedFile, OwnedDuplex, OwnedReader, OwnedWriter, BorrowedFile<'_>, BorrowedDuplex<'_>, BorrowedReader<'_>, BorrowedWriter<'_>}); // SAFETY: `std::fs::File` is `Send+Sync` despite `try_clone(&self)`, `impl Read for &sync::File`, etc. all sharing a `HANDLE` - if this is unsound, so is `std`.
handles!(       impl Debug                                  for io::sync::{OwnedFile, OwnedDuplex, OwnedReader, OwnedWriter, BorrowedFile<'_>, BorrowedDuplex<'_>, BorrowedReader<'_>, BorrowedWriter<'_>});

handles!(unsafe impl TryCloneToOwned<io::sync::OwnedFile>   for io::sync::{OwnedFile,   BorrowedFile  <'_>});
handles!(unsafe impl TryCloneToOwned<io::sync::OwnedDuplex> for io::sync::{OwnedDuplex, BorrowedDuplex<'_>});
handles!(unsafe impl TryCloneToOwned<io::sync::OwnedReader> for io::sync::{OwnedReader, BorrowedReader<'_>});
handles!(unsafe impl TryCloneToOwned<io::sync::OwnedWriter> for io::sync::{OwnedWriter, BorrowedWriter<'_>});



impl io::sync::OwnedFile {
    /// This conversion usually makes little sense, so it requires a more explicit method.
    pub fn into_duplex_despite_shared_seek_position(self) -> io::sync::OwnedDuplex { io::sync::OwnedDuplex(core::mem::ManuallyDrop::new(self).0) }

    /// This conversion usually makes little sense, so it requires a more explicit method.
    pub fn as_duplex_despite_shared_seek_position(&self) -> io::sync::BorrowedDuplex { io::sync::BorrowedDuplex(self.0, PhantomData) }
}

impl<'a> io::sync::BorrowedFile<'a> {
    /// This conversion usually makes little sense, so it requires a more explicit method.
    pub fn into_duplex_despite_shared_seek_position(self) -> io::sync::BorrowedDuplex<'a> { io::sync::BorrowedDuplex(self.0, PhantomData) }

    /// This conversion usually makes little sense, so it requires a more explicit method.
    pub fn as_duplex_despite_shared_seek_position(&self) -> io::sync::BorrowedDuplex<'a> { io::sync::BorrowedDuplex(self.0, PhantomData) }
}



//ndles!(unsafe impl @convert io::sync::OwnedFile           => io::sync::OwnedDuplex            ); // requires explicit into_duplex_despite_shared_seek_position
handles!(unsafe impl @convert io::sync::OwnedFile           => io::sync::OwnedReader            );
handles!(unsafe impl @convert io::sync::OwnedFile           => io::sync::OwnedWriter            );
//ndles!(unsafe impl @convert io::sync::OwnedFile           => pipe::sync::OwnedDuplex          );
handles!(unsafe impl @convert io::sync::OwnedFile           => pipe::sync::OwnedReader          );
handles!(unsafe impl @convert io::sync::OwnedFile           => pipe::sync::OwnedWriter          );
handles!(unsafe impl @convert io::sync::OwnedFile           => handle::Owned                    );

handles!(unsafe impl @convert io::sync::OwnedDuplex         => io::sync::OwnedReader            );
handles!(unsafe impl @convert io::sync::OwnedDuplex         => io::sync::OwnedWriter            );
handles!(unsafe impl @convert io::sync::OwnedDuplex         => pipe::sync::OwnedDuplex          );
handles!(unsafe impl @convert io::sync::OwnedDuplex         => pipe::sync::OwnedReader          );
handles!(unsafe impl @convert io::sync::OwnedDuplex         => pipe::sync::OwnedWriter          );
handles!(unsafe impl @convert io::sync::OwnedDuplex         => handle::Owned                    );

handles!(unsafe impl @convert io::sync::OwnedReader         => pipe::sync::OwnedReader          );
handles!(unsafe impl @convert io::sync::OwnedReader         => handle::Owned                    );

handles!(unsafe impl @convert io::sync::OwnedWriter         => pipe::sync::OwnedWriter          );
handles!(unsafe impl @convert io::sync::OwnedWriter         => handle::Owned                    );



handles!(unsafe impl @convert &'_ io::sync::OwnedFile       => io::sync::BorrowedFile<'_>       );
//ndles!(unsafe impl @convert &'_ io::sync::OwnedFile       => io::sync::BorrowedDuplex<'_>     ); // requires explicit as_duplex_despite_shared_seek_position
handles!(unsafe impl @convert &'_ io::sync::OwnedFile       => io::sync::BorrowedReader<'_>     );
handles!(unsafe impl @convert &'_ io::sync::OwnedFile       => io::sync::BorrowedWriter<'_>     );
//ndles!(unsafe impl @convert &'_ io::sync::OwnedFile       => pipe::sync::BorrowedDuplex<'_>   );
handles!(unsafe impl @convert &'_ io::sync::OwnedFile       => pipe::sync::BorrowedReader<'_>   );
handles!(unsafe impl @convert &'_ io::sync::OwnedFile       => pipe::sync::BorrowedWriter<'_>   );
handles!(unsafe impl @convert &'_ io::sync::OwnedFile       => handle::Borrowed<'_>             );
handles!(unsafe impl @convert &'_ io::sync::OwnedFile       => handle::Pseudo<'_>               );

handles!(unsafe impl @convert &'_ io::sync::OwnedDuplex     => io::sync::BorrowedDuplex<'_>     );
handles!(unsafe impl @convert &'_ io::sync::OwnedDuplex     => io::sync::BorrowedReader<'_>     );
handles!(unsafe impl @convert &'_ io::sync::OwnedDuplex     => io::sync::BorrowedWriter<'_>     );
handles!(unsafe impl @convert &'_ io::sync::OwnedDuplex     => pipe::sync::BorrowedDuplex<'_>   );
handles!(unsafe impl @convert &'_ io::sync::OwnedDuplex     => pipe::sync::BorrowedReader<'_>   );
handles!(unsafe impl @convert &'_ io::sync::OwnedDuplex     => pipe::sync::BorrowedWriter<'_>   );
handles!(unsafe impl @convert &'_ io::sync::OwnedDuplex     => handle::Borrowed<'_>             );
handles!(unsafe impl @convert &'_ io::sync::OwnedDuplex     => handle::Pseudo<'_>               );

handles!(unsafe impl @convert &'_ io::sync::OwnedReader     => io::sync::BorrowedReader<'_>     );
handles!(unsafe impl @convert &'_ io::sync::OwnedReader     => pipe::sync::BorrowedReader<'_>   );
handles!(unsafe impl @convert &'_ io::sync::OwnedReader     => handle::Borrowed<'_>             );
handles!(unsafe impl @convert &'_ io::sync::OwnedReader     => handle::Pseudo<'_>               );

handles!(unsafe impl @convert &'_ io::sync::OwnedWriter     => io::sync::BorrowedWriter<'_>     );
handles!(unsafe impl @convert &'_ io::sync::OwnedWriter     => pipe::sync::BorrowedWriter<'_>   );
handles!(unsafe impl @convert &'_ io::sync::OwnedWriter     => handle::Borrowed<'_>             );
handles!(unsafe impl @convert &'_ io::sync::OwnedWriter     => handle::Pseudo<'_>               );



//ndles!(unsafe impl @convert io::sync::BorrowedFile<'_>    => io::sync::BorrowedDuplex<'_>     ); // requires explicit as_duplex_despite_shared_seek_position
handles!(unsafe impl @convert io::sync::BorrowedFile<'_>    => io::sync::BorrowedReader<'_>     );
handles!(unsafe impl @convert io::sync::BorrowedFile<'_>    => io::sync::BorrowedWriter<'_>     );
//ndles!(unsafe impl @convert io::sync::BorrowedFile<'_>    => pipe::sync::BorrowedDuplex<'_>   );
handles!(unsafe impl @convert io::sync::BorrowedFile<'_>    => pipe::sync::BorrowedReader<'_>   );
handles!(unsafe impl @convert io::sync::BorrowedFile<'_>    => pipe::sync::BorrowedWriter<'_>   );
handles!(unsafe impl @convert io::sync::BorrowedFile<'_>    => handle::Borrowed<'_>             );
handles!(unsafe impl @convert io::sync::BorrowedFile<'_>    => handle::Pseudo<'_>               );

handles!(unsafe impl @convert io::sync::BorrowedDuplex<'_>  => io::sync::BorrowedReader<'_>     );
handles!(unsafe impl @convert io::sync::BorrowedDuplex<'_>  => io::sync::BorrowedWriter<'_>     );
handles!(unsafe impl @convert io::sync::BorrowedDuplex<'_>  => pipe::sync::BorrowedDuplex<'_>   );
handles!(unsafe impl @convert io::sync::BorrowedDuplex<'_>  => pipe::sync::BorrowedReader<'_>   );
handles!(unsafe impl @convert io::sync::BorrowedDuplex<'_>  => pipe::sync::BorrowedWriter<'_>   );
handles!(unsafe impl @convert io::sync::BorrowedDuplex<'_>  => handle::Borrowed<'_>             );
handles!(unsafe impl @convert io::sync::BorrowedDuplex<'_>  => handle::Pseudo<'_>               );

handles!(unsafe impl @convert io::sync::BorrowedReader<'_>  => pipe::sync::BorrowedReader<'_>   );
handles!(unsafe impl @convert io::sync::BorrowedReader<'_>  => handle::Borrowed<'_>             );
handles!(unsafe impl @convert io::sync::BorrowedReader<'_>  => handle::Pseudo<'_>               );

handles!(unsafe impl @convert io::sync::BorrowedWriter<'_>  => pipe::sync::BorrowedWriter<'_>   );
handles!(unsafe impl @convert io::sync::BorrowedWriter<'_>  => handle::Borrowed<'_>             );
handles!(unsafe impl @convert io::sync::BorrowedWriter<'_>  => handle::Pseudo<'_>               );



#[doc = r"\[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle)\] CloseHandle"] impl Drop for OwnedFile     { fn drop(&mut self) { unsafe { drop_close_handle_nn(self) } } }
#[doc = r"\[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle)\] CloseHandle"] impl Drop for OwnedDuplex   { fn drop(&mut self) { unsafe { drop_close_handle_nn(self) } } }
#[doc = r"\[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle)\] CloseHandle"] impl Drop for OwnedReader   { fn drop(&mut self) { unsafe { drop_close_handle_nn(self) } } }
#[doc = r"\[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle)\] CloseHandle"] impl Drop for OwnedWriter   { fn drop(&mut self) { unsafe { drop_close_handle_nn(self) } } }



impl Read   for &'_ OwnedFile           { fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { unsafe { Ok(read_file(*self, buf, None).map(usize::from32)?) } } }
impl Read   for     OwnedFile           { fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { unsafe { Ok(read_file( self, buf, None).map(usize::from32)?) } } }
impl Read   for &'_ OwnedDuplex         { fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { unsafe { Ok(read_file(*self, buf, None).map(usize::from32)?) } } }
impl Read   for     OwnedDuplex         { fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { unsafe { Ok(read_file( self, buf, None).map(usize::from32)?) } } }
impl Read   for &'_ OwnedReader         { fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { unsafe { Ok(read_file(*self, buf, None).map(usize::from32)?) } } }
impl Read   for     OwnedReader         { fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { unsafe { Ok(read_file( self, buf, None).map(usize::from32)?) } } }
impl Read   for &'_ BorrowedFile<'_>    { fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { unsafe { Ok(read_file(*self, buf, None).map(usize::from32)?) } } }
impl Read   for     BorrowedFile<'_>    { fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { unsafe { Ok(read_file( self, buf, None).map(usize::from32)?) } } }
impl Read   for &'_ BorrowedDuplex<'_>  { fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { unsafe { Ok(read_file(*self, buf, None).map(usize::from32)?) } } }
impl Read   for     BorrowedDuplex<'_>  { fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { unsafe { Ok(read_file( self, buf, None).map(usize::from32)?) } } }
impl Read   for &'_ BorrowedReader<'_>  { fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { unsafe { Ok(read_file(*self, buf, None).map(usize::from32)?) } } }
impl Read   for     BorrowedReader<'_>  { fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { unsafe { Ok(read_file( self, buf, None).map(usize::from32)?) } } }

impl Seek   for &'_ OwnedFile           { fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> { unsafe { Ok(set_file_pointer_ex(*self, pos)?) } } }
impl Seek   for     OwnedFile           { fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> { unsafe { Ok(set_file_pointer_ex( self, pos)?) } } }
impl Seek   for &'_ BorrowedFile<'_>    { fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> { unsafe { Ok(set_file_pointer_ex(*self, pos)?) } } }
impl Seek   for     BorrowedFile<'_>    { fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> { unsafe { Ok(set_file_pointer_ex( self, pos)?) } } }

// noop flush sane: https://github.com/rust-lang/rust/blob/c2110769cd58cd3b0c31f308c8cfeab5e19340fd/library/std/src/sys/fs/windows.rs#L604-L606
impl Write  for &'_ OwnedFile           { fn write(&mut self, buf: &[u8]) -> io::Result<usize> { unsafe { Ok(write_file(*self, buf, None).map(usize::from32)?) } } fn flush(&mut self) -> io::Result<()> { Ok(()) } }
impl Write  for     OwnedFile           { fn write(&mut self, buf: &[u8]) -> io::Result<usize> { unsafe { Ok(write_file( self, buf, None).map(usize::from32)?) } } fn flush(&mut self) -> io::Result<()> { Ok(()) } }
impl Write  for &'_ OwnedDuplex         { fn write(&mut self, buf: &[u8]) -> io::Result<usize> { unsafe { Ok(write_file(*self, buf, None).map(usize::from32)?) } } fn flush(&mut self) -> io::Result<()> { Ok(()) } }
impl Write  for     OwnedDuplex         { fn write(&mut self, buf: &[u8]) -> io::Result<usize> { unsafe { Ok(write_file( self, buf, None).map(usize::from32)?) } } fn flush(&mut self) -> io::Result<()> { Ok(()) } }
impl Write  for &'_ OwnedWriter         { fn write(&mut self, buf: &[u8]) -> io::Result<usize> { unsafe { Ok(write_file(*self, buf, None).map(usize::from32)?) } } fn flush(&mut self) -> io::Result<()> { Ok(()) } }
impl Write  for     OwnedWriter         { fn write(&mut self, buf: &[u8]) -> io::Result<usize> { unsafe { Ok(write_file( self, buf, None).map(usize::from32)?) } } fn flush(&mut self) -> io::Result<()> { Ok(()) } }
impl Write  for &'_ BorrowedFile<'_>    { fn write(&mut self, buf: &[u8]) -> io::Result<usize> { unsafe { Ok(write_file(*self, buf, None).map(usize::from32)?) } } fn flush(&mut self) -> io::Result<()> { Ok(()) } }
impl Write  for     BorrowedFile<'_>    { fn write(&mut self, buf: &[u8]) -> io::Result<usize> { unsafe { Ok(write_file( self, buf, None).map(usize::from32)?) } } fn flush(&mut self) -> io::Result<()> { Ok(()) } }
impl Write  for &'_ BorrowedDuplex<'_>  { fn write(&mut self, buf: &[u8]) -> io::Result<usize> { unsafe { Ok(write_file(*self, buf, None).map(usize::from32)?) } } fn flush(&mut self) -> io::Result<()> { Ok(()) } }
impl Write  for     BorrowedDuplex<'_>  { fn write(&mut self, buf: &[u8]) -> io::Result<usize> { unsafe { Ok(write_file( self, buf, None).map(usize::from32)?) } } fn flush(&mut self) -> io::Result<()> { Ok(()) } }
impl Write  for &'_ BorrowedWriter<'_>  { fn write(&mut self, buf: &[u8]) -> io::Result<usize> { unsafe { Ok(write_file(*self, buf, None).map(usize::from32)?) } } fn flush(&mut self) -> io::Result<()> { Ok(()) } }
impl Write  for     BorrowedWriter<'_>  { fn write(&mut self, buf: &[u8]) -> io::Result<usize> { unsafe { Ok(write_file( self, buf, None).map(usize::from32)?) } } fn flush(&mut self) -> io::Result<()> { Ok(()) } }

impl crate::os::windows::io::FromRawHandle for OwnedFile      { unsafe fn from_raw_handle(handle: crate::os::windows::io::RawHandle) -> Self { let handle = HANDLENN::new(handle.cast()).expect("undefined behavior: null is not an open, owned handle"); unsafe { type_check_owned_from_raw(handle) }; Self(handle) } }
impl crate::os::windows::io::FromRawHandle for OwnedDuplex    { unsafe fn from_raw_handle(handle: crate::os::windows::io::RawHandle) -> Self { let handle = HANDLENN::new(handle.cast()).expect("undefined behavior: null is not an open, owned handle"); unsafe { type_check_owned_from_raw(handle) }; Self(handle) } }
impl crate::os::windows::io::FromRawHandle for OwnedReader    { unsafe fn from_raw_handle(handle: crate::os::windows::io::RawHandle) -> Self { let handle = HANDLENN::new(handle.cast()).expect("undefined behavior: null is not an open, owned handle"); unsafe { type_check_owned_from_raw(handle) }; Self(handle) } }
impl crate::os::windows::io::FromRawHandle for OwnedWriter    { unsafe fn from_raw_handle(handle: crate::os::windows::io::RawHandle) -> Self { let handle = HANDLENN::new(handle.cast()).expect("undefined behavior: null is not an open, owned handle"); unsafe { type_check_owned_from_raw(handle) }; Self(handle) } }

impl crate::os::windows::io::IntoRawHandle for OwnedFile      { fn into_raw_handle(self) -> crate::os::windows::io::RawHandle { self.into_handle().cast() } }
impl crate::os::windows::io::IntoRawHandle for OwnedDuplex    { fn into_raw_handle(self) -> crate::os::windows::io::RawHandle { self.into_handle().cast() } }
impl crate::os::windows::io::IntoRawHandle for OwnedReader    { fn into_raw_handle(self) -> crate::os::windows::io::RawHandle { self.into_handle().cast() } }
impl crate::os::windows::io::IntoRawHandle for OwnedWriter    { fn into_raw_handle(self) -> crate::os::windows::io::RawHandle { self.into_handle().cast() } }

unsafe impl valrow::Borrowable for OwnedFile        { type Abi = HANDLENN; }
unsafe impl valrow::Borrowable for OwnedDuplex      { type Abi = HANDLENN; }
unsafe impl valrow::Borrowable for OwnedReader      { type Abi = HANDLENN; }
unsafe impl valrow::Borrowable for OwnedWriter      { type Abi = HANDLENN; }
//safe impl valrow::Borrowable for Borrowed*        { type Abi = HANDLENN; } // valid but pointless

impl CloneToOwned for OwnedFile             {}
impl CloneToOwned for OwnedDuplex           {}
impl CloneToOwned for OwnedReader           {}
impl CloneToOwned for OwnedWriter           {}

impl CloneToOwned for BorrowedFile<'_>      {}
impl CloneToOwned for BorrowedDuplex<'_>    {}
impl CloneToOwned for BorrowedReader<'_>    {}
impl CloneToOwned for BorrowedWriter<'_>    {}

// It might be appropriate to impl TryFrom<OwnedHandle> for non-sync io::File, io::Duplex, io::Reader, io::WriterNN?
// ~~Constructing `crate::os::windows::io::NullHandleError` is awkward though.~~ Just use OwnedHandle::try_from(HandleOrNull)?
// Deferring until I have a concrete use case, if I ever do.



// https://stackoverflow.com/questions/42513027/has-been-a-file-handle-opened-with-file-flag-sequential-scan-flag
// https://community.osr.com/t/win32-handle-vs-nt-handle/21915/20?page=2
// https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/wdm/ns-wdm-_file_object

#[inline(never)] pub(crate) unsafe fn assert_is_synchronous_file(raw: HANDLENN) {
    let _preserve_error_scope = PreserveErrorScope::new();

    let handle = unsafe { handle::Pseudo::from_raw_nn(raw) };

    assert!(
        get_file_type(&handle).map_or_else(|err| err != ERROR_INVALID_HANDLE, |_| true),
        "undefined behavior: `io::sync::{{Owned,Borrowed}}{{File,Duplex,Reader,Writer}}::from_raw[_nn]`: handle {raw:p} is not a file handle, and this debug check isn't guaranteed to work"
    );

    const FILE_SYNCHRONOUS_IO_ALERT     : u32 = 16;
    const FILE_SYNCHRONOUS_IO_NONALERT  : u32 = 32;
    const FILE_SYNCHRONOUS_IO_          : u32 = FILE_SYNCHRONOUS_IO_ALERT | FILE_SYNCHRONOUS_IO_NONALERT;

    assert!(
        nt_query_information_file::<file::ModeInformation>(handle).map_or(true, |info| 0 != (info.Mode & FILE_SYNCHRONOUS_IO_)),
        "undefined behavior: `io::sync::{{Owned,Borrowed}}{{File,Duplex,Reader,Writer}}::from_raw[_nn]`: handle {raw:p} is not synchronous (e.g. it was created with `FILE_FLAG_OVERLAPPED`, and this debug check isn't guaranteed to work)"
    );
}

#[allow(dead_code)] // XXX: casting handles not yet implemented
#[inline(always)] pub(crate) unsafe fn type_check_cast_handle(handle: HANDLENN) {
    if type_check::cast_handle() { unsafe { assert_is_synchronous_file(handle) } }
}

#[inline(always)] pub(crate) unsafe fn type_check_owned_from_raw(handle: HANDLENN) {
    if type_check::owned_from_raw() { unsafe { assert_is_synchronous_file(handle) } }
}

#[inline(always)] pub(crate) unsafe fn type_check_borrowed_from_raw(handle: HANDLENN) {
    if type_check::borrowed_from_raw() { unsafe { assert_is_synchronous_file(handle) } }
}



#[cfg(test)] mod tests {
    #[allow(unused_imports)] use crate::prelude::*;
    use crate::io::*;
    use crate::os::windows::io::FromRawHandle;

    #[test] fn non_overlapped_to_firehazard_io_file_nn() {
        let file = create_file_w(cstr16!("Readme.md"), access::GENERIC_READ, file::Share::READ, None, winapi::um::fileapi::OPEN_EXISTING, 0,                     None).unwrap();
        let _sync = unsafe { io::sync::OwnedFile::from_raw_handle(file.into_handle()) };
    }

    #[test] #[should_panic = "undefined behavior"] fn overlapped_to_firehazard_io_file_nn() {
        let file = create_file_w(cstr16!("Readme.md"), access::GENERIC_READ, file::Share::READ, None, winapi::um::fileapi::OPEN_EXISTING, file::FLAG_OVERLAPPED, None).unwrap();
        let _sync = unsafe { io::sync::OwnedFile::from_raw_handle(file.into_handle()) };
    }

    #[test] #[should_panic = "undefined behavior"] fn null_to_firehazard_io_file() {
        let _null = unsafe { io::sync::OwnedFile::from_raw_handle(null_mut()) };
    }

    #[test] #[should_panic = "undefined behavior"] fn process_to_firehazard_io_file_nn() {
        let _invalid = unsafe { io::sync::OwnedFile::from_raw_handle(get_current_process().into_handle()) };
    }

    #[test] #[should_panic = "undefined behavior"] fn thread_to_firehazard_io_file_nn() {
        let _invalid = unsafe { io::sync::OwnedFile::from_raw_handle(get_current_thread().into_handle()) };
    }
}
