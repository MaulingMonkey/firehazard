//! \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/ipc/pipes)\]
//! Anonymous and named pipe APIs
//!
//!
//!
//! ### Handle Types
//!
//! These handles can wrap [file](mod@file)s, [pipe]s, and sockets, with the following caveats:
//! -   Pipe-specific APIs will likely fail on [file](mod@file)s, and possibly sockets (although [`get_file_type`] reports that sockets are pipes?)
//!     Consider using equivalent [`firehazard::io`] handle types instead if you wish to handle files and sockets without being baited into thinking pipe APIs are available.
//! -   `Duplex` can wrap a file, but reading and writing without seeking is unlikely to be what you want.
//! -   `Duplex` pipes are more deadlock prone than [`TcpStream`](std::net::TcpStream)s (see ranting @ [`pipe::named`])
//! -   You can wrap a read-only handle in a [`Write`]able type.  Access differentiation only exists to discourage misuse.
//! -   You can wrap a write-only handle in a [`Read`]able type.  Access differentiation only exists to discourage misuse.
//! -   Wrapping a [`file::FLAG_OVERLAPPED`] handle in a `firehazard::pipe::sync::*` type is undefined behavior!
//! -   Wrapping a [`file::FLAG_OVERLAPPED`] handle in a `std::*` type *may* be sound since 1.62.0 per [rust-lang/rust#81357](https://github.com/rust-lang/rust/issues/81357)?
//!
//! | Implements                        | Owned                                                                         | Borrowed                                                              |
//! | ----------------------------------| ------------------------------------------------------------------------------| ----------------------------------------------------------------------|
//! |                                   | **without `FILE_FLAG_OVERLAPPED`:**                                           |
//! | [`Read`]                          | [`firehazard::pipe::sync::OwnedReader`]                                       | [`firehazard::pipe::sync::BorrowedReader`]                              |
//! | [`Write`]                         | [`firehazard::pipe::sync::OwnedWriter`]                                       | [`firehazard::pipe::sync::BorrowedWriter`]                              |
//! | [`Read`] + [`Write`]              | [`firehazard::pipe::sync::OwnedDuplex`]                                       | [`firehazard::pipe::sync::BorrowedDuplex`]                              |
//! |                                   | **with `FILE_FLAG_OVERLAPPED`:**                                              |
//! | [`Read`]                          | <span style="opacity: 25%">N/A</span>                                         | <span style="opacity: 25%">N/A</span>                                 |
//! | [`Write`]                         | <span style="opacity: 25%">N/A</span>                                         | <span style="opacity: 25%">N/A</span>                                 |
//! | [`Read`] + [`Write`]              | <span style="opacity: 25%">N/A</span>                                         | <span style="opacity: 25%">N/A</span>                                 |
//! |                                   | **with or without `FILE_FLAG_OVERLAPPED`:**                                   |
//! | No I/O traits                     | [`firehazard::handle::Owned`] <br> [`std::os::windows::io::OwnedHandle`] <br> [`std::os::windows::io::OwnedSocket`] <br> [`std::net::TcpListener`] ⚠️ | [`firehazard::handle::Borrowed`] <br> [`std::os::windows::io::BorrowedHandle`] <br> [`std::os::windows::io::BorrowedSocket`] |
//! | [`Read`]                          | [`std::io::PipeReader`] ⚠️                                                    | <code>&amp;(impl [Read])</code> ⚠️                                    |
//! | [`Write`]                         | [`std::io::PipeWriter`] ⚠️                                                    | <code>&amp;(impl [Write])</code> ⚠️                                   |
//! | [`Read`] + [`Write`]              | [`std::net::TcpStream`] ⚠️ <br> [`std::fs::File`] ⚠️                          | <code>&amp;(impl [Read] + [Write])</code> ⚠️                          |
//!
//! Legend:<br>
//! ⚠️ No `#[repr(transparent)]` for FFI
//!
//! ### References
//! *   <https://learn.microsoft.com/en-us/windows/win32/ipc/pipes>
//! *   <https://learn.microsoft.com/en-us/windows/win32/ipc/multithreaded-pipe-server>
//!
//! [`std::io::PipeReader`]:    https://doc.rust-lang.org/std/io/struct.PipeReader.html
//! [`std::io::PipeWriter`]:    https://doc.rust-lang.org/std/io/struct.PipeWriter.html

use crate::prelude::*;
#[cfg(doc)] use crate::io::{Read, Write, Seek};



#[deprecated = "use `firehazard::pipe::sync::OwnedDuplex` instead"              ] #[doc(hidden)] pub type DuplexNN      = pipe::sync::OwnedDuplex;
#[deprecated = "use `firehazard::pipe::sync::OwnedReader` instead"              ] #[doc(hidden)] pub type ReaderNN      = pipe::sync::OwnedReader;
#[deprecated = "use `firehazard::pipe::sync::OwnedWriter` instead"              ] #[doc(hidden)] pub type WriterNN      = pipe::sync::OwnedWriter;

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/ipc/anonymous-pipes)\]
/// Anonymous Pipes
pub mod anonymous {
    use crate::prelude::*;
    include!(r"anonymous\funcs\create.rs");
}
pub use anonymous::create;

#[path = r"named\_named.rs"] pub mod named;

pub use values::*;
pub(crate) mod values {
    mod nmpwait;                pub use nmpwait::*;
    mod pipe_flags;             pub use pipe_flags::*;
    mod pipe_max_instances;     pub use pipe_max_instances::*;
}

/// <strike style="opacity: 25%">`HANDLE`s to [pipe]s (created with [`file::FLAG_OVERLAPPED`])</strike> NYI
pub mod overlapped {}
#[path = "pipe_handles.rs"] pub mod sync; // TODO: rename file (in separate commit for nicer diff.)
