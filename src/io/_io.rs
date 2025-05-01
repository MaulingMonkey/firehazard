//! \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/fileio/synchronous-and-asynchronous-i-o)] Generic I/O APIs
//!
//! ### Handle Types
//!
//! These handles can wrap [file](mod@file)s, [pipe]s, and sockets, with the following caveats:
//! -   `Duplex` can wrap a file, but reading and writing without seeking is unlikely to be what you want.
//! -   `Duplex` pipes are more deadlock prone than [`TcpStream`](std::net::TcpStream)s (see ranting @ [`pipe::named`])
//! -   `File` can wrap a pipe or socket, but [`Seek`]ing will always fail - `Duplex` is probably more appropriate.
//! -   You can wrap a read-only handle in a [`Write`] + [`Seek`]able type.  Differentiation from `File` only exists to discourage misuse.
//! -   You can wrap a write-only handle in a [`Read`] + [`Seek`]able type.  Differentiation from `File` only exists to discourage misuse.
//! -   Wrapping a [`file::FLAG_OVERLAPPED`] handle in a `firehazard::io::sync::*` type is undefined behavior!
//! -   Wrapping a [`file::FLAG_OVERLAPPED`] handle in a `std::*` type *may* be sound since 1.62.0 per [rust-lang/rust#81357](https://github.com/rust-lang/rust/issues/81357)?
//!
//! | Implements                        | Owned                                                                         | Borrowed                                                              |
//! | ----------------------------------| ------------------------------------------------------------------------------| ----------------------------------------------------------------------|
//! |                                   | **without `FILE_FLAG_OVERLAPPED`:**                                           |
//! | [`Read`]                          | [`firehazard::io::sync::OwnedReader`]                                         | [`firehazard::io::sync::BorrowedReader`]                              |
//! | [`Write`]                         | [`firehazard::io::sync::OwnedWriter`]                                         | [`firehazard::io::sync::BorrowedWriter`]                              |
//! | [`Read`] + [`Write`]              | [`firehazard::io::sync::OwnedDuplex`]                                         | [`firehazard::io::sync::BorrowedDuplex`]                              |
//! | [`Read`] + [`Write`] + [`Seek`]   | [`firehazard::io::sync::OwnedFile`]                                           | [`firehazard::io::sync::BorrowedFile`]                                |
//! | [`Read`] + [`Seek`]               | <strike style="opacity: 25%">`firehazard::io::sync::ReadOnlyFile`</strike>    | <span style="opacity: 25%">N/A</span>                                 |
//! | [`Write`] + [`Seek`]              | <strike style="opacity: 25%">`firehazard::io::sync::WriteOnlyFile`</strike>   | <span style="opacity: 25%">N/A</span>                                 |
//! |                                   | **with `FILE_FLAG_OVERLAPPED`:**                                              |
//! | [`Read`]                          | <span style="opacity: 25%">N/A</span>                                         | <span style="opacity: 25%">N/A</span>                                 |
//! | [`Write`]                         | <span style="opacity: 25%">N/A</span>                                         | <span style="opacity: 25%">N/A</span>                                 |
//! | [`Read`] + [`Write`]              | <span style="opacity: 25%">N/A</span>                                         | <span style="opacity: 25%">N/A</span>                                 |
//! | [`Read`] + [`Write`] + [`Seek`]   | <span style="opacity: 25%">N/A</span>                                         | <span style="opacity: 25%">N/A</span>                                 |
//! | [`Read`] + [`Seek`]               | <span style="opacity: 25%">N/A</span>                                         | <span style="opacity: 25%">N/A</span>                                 |
//! | [`Write`] + [`Seek`]              | <span style="opacity: 25%">N/A</span>                                         | <span style="opacity: 25%">N/A</span>                                 |
//! |                                   | **with or without `FILE_FLAG_OVERLAPPED`:**                                   |
//! | No I/O traits                     | [`firehazard::handle::Owned`] <br> [`std::os::windows::io::OwnedHandle`] <br> [`std::os::windows::io::OwnedSocket`] <br> [`std::net::TcpListener`] ⚠️ | [`firehazard::handle::Borrowed`] <br> [`std::os::windows::io::BorrowedHandle`] <br> [`std::os::windows::io::BorrowedSocket`] |
//! | [`Read`]                          | [`std::io::PipeReader`] ⚠️                                                    | <code>&amp;(impl [Read])</code> ⚠️                                    |
//! | [`Write`]                         | [`std::io::PipeWriter`] ⚠️                                                    | <code>&amp;(impl [Write])</code> ⚠️                                   |
//! | [`Read`] + [`Write`]              | [`std::net::TcpStream`] ⚠️                                                    | <code>&amp;(impl [Read] + [Write])</code> ⚠️                          |
//! | [`Read`] + [`Write`] + [`Seek`]   | [`std::fs::File`] ⚠️                                                          | <code>&amp;(impl [Read] + [Write] + [Seek])</code> ⚠️                 |
//! | [`Read`] + [`Seek`]               | ≈ <code>impl [Read] + [Seek]</code> ⚠️                                        | <code>&amp;(impl [Read] + [Seek])</code> ⚠️                           |
//! | [`Write`] + [`Seek`]              | ≈ <code>impl [Write] + [Seek]</code> ⚠️                                       | <code>&amp;(impl [Write] + [Seek])</code> ⚠️                          |
//!
//! Legend:<br>
//! ⚠️ No `#[repr(transparent)]` for FFI
//!
//! ### References
//! *   <https://learn.microsoft.com/en-us/windows/win32/fileio/synchronous-and-asynchronous-i-o>
//! *   <https://learn.microsoft.com/en-us/windows/win32/ipc/pipes>
//! *   <https://learn.microsoft.com/en-us/windows/win32/ipc/multithreaded-pipe-server>
//!
//! [`std::io::PipeReader`]:    https://doc.rust-lang.org/std/io/struct.PipeReader.html
//! [`std::io::PipeWriter`]:    https://doc.rust-lang.org/std/io/struct.PipeWriter.html

use crate::prelude::*;



#[cfg(    std )] mod io_yes_std; #[cfg(    std )] #[doc(no_inline)] pub use std::io    ::{Read, Seek, Write, Error, ErrorKind, Result, SeekFrom};
#[cfg(not(std))] mod io_not_std; #[cfg(not(std))]                   pub use io_not_std ::{Read, Seek, Write, Error, ErrorKind, Result, SeekFrom};

/// <strike style="opacity: 25%">`HANDLE`s to [file](mod@file)s, [pipe]s, or sockets (created with [`file::FLAG_OVERLAPPED`])</strike> NYI
pub mod overlapped {} // I'd call it async, but that's a reserved keyword :(
#[path = "io_handles.rs"] pub mod sync; // TODO: rename file (in separate commit for nicer diff.)

mod io_counters;                    pub use io_counters::*;
mod io_status_block;                pub(crate) use io_status_block::StatusBlock;

#[allow(unused_imports)] pub use funcs::*;
pub(crate) mod funcs {
    use crate::prelude::*;
    include!(r"funcs\read_file.rs");
    include!(r"funcs\write_file.rs");
}

#[deprecated = "use `firehazard::io::sync::OwnedFile` instead"          ] #[doc(hidden)] pub type File              = crate::io::sync::OwnedFile;
#[deprecated = "use `firehazard::io::sync::OwnedFile` instead"          ] #[doc(hidden)] pub type FileNN            = crate::io::sync::OwnedFile;
#[deprecated = "use `firehazard::io::sync::BorrowedFile` instead"       ] #[doc(hidden)] pub type FileHandle<'a>    = crate::io::sync::BorrowedFile<'a>;
#[deprecated = "use `firehazard::io::sync::BorrowedDuplex` instead"     ] #[doc(hidden)] pub type DuplexHandle<'a>  = crate::io::sync::BorrowedDuplex<'a>;
#[deprecated = "use `firehazard::io::sync::BorrowedReader` instead"     ] #[doc(hidden)] pub type ReadHandle<'a>    = crate::io::sync::BorrowedReader<'a>;
#[deprecated = "use `firehazard::io::sync::BorrowedWriter` instead"     ] #[doc(hidden)] pub type WriteHandle<'a>   = crate::io::sync::BorrowedWriter<'a>;
#[deprecated = "use `firehazard::pipe::sync::OwnedReader` instead"      ] #[doc(hidden)] pub type ReadPipe          = crate::pipe::sync::OwnedReader;
#[deprecated = "use `firehazard::pipe::sync::OwnedReader` instead"      ] #[doc(hidden)] pub type PipeReaderNN      = crate::pipe::sync::OwnedReader;
#[deprecated = "use `firehazard::pipe::sync::OwnedWriter` instead"      ] #[doc(hidden)] pub type WritePipe         = crate::pipe::sync::OwnedWriter;
#[deprecated = "use `firehazard::pipe::sync::OwnedWriter` instead"      ] #[doc(hidden)] pub type PipeWriterNN      = crate::pipe::sync::OwnedWriter;
#[deprecated = "use `firehazard::pipe::create` instead"                 ] #[doc(hidden)] pub use crate::pipe::create as create_pipe;
