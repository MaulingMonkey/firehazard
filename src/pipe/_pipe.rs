//! \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/ipc/pipes)\]
//! Anonymous and named pipe APIs
//!
//!
//!
//! ### Handle Types
//!
//! | [`firehazard`]        | [`std`]                                                                               | Notes                                                                                 |
//! | ----------------------| --------------------------------------------------------------------------------------| --------------------------------------------------------------------------------------|
//! | **Owned Handles**     |                                                                                       | [`std`] equivalents are cross-platform, nullable, not FFI-friendly                    |
//! | [`pipe::DuplexNN`]    | <span style="opacity: 50%">[`std::fs::File`]?</span>                                  | + [`File`](std::fs::File) is [`Seek`](std::io::Seek)able, which is inappropriate      |
//! | [`pipe::ReaderNN`]    | [`std::io::PipeReader`](https://doc.rust-lang.org/beta/std/io/struct.PipeReader.html) | + [`std`] is beta                                                                     |
//! | [`pipe::WriterNN`]    | [`std::io::PipeWriter`](https://doc.rust-lang.org/beta/std/io/struct.PipeWriter.html) | + [`std`] is beta                                                                     |
//! | [`handle::Owned`]     | [`…::OwnedHandle`](std::os::windows::io::OwnedHandle)                                 | entirely generic, [`std`] *is* FFI-friendly (although nullable)                       |
//! | |
//! | **Borrowed Handles**  |                                                                                       | Borrowed handles are semi-generic (could be sockets or files)                         |
//! | [`io::ReadHandle`]    | <code>&dyn [std::io::Read]</code>                                                     | [`std`] isn't FFI-friendly                                                            |
//! | [`io::WriteHandle`]   | <code>&dyn [std::io::Write]</code>                                                    | [`std`] isn't FFI-friendly                                                            |
//! | [`handle::Borrowed`]  |                                                                                       | entirely generic, but Pseudo-handles are forbidden                                    |
//! | [`handle::Pseudo`]    | [`…::BorrowedHandle`](std::os::windows::io::BorrowedHandle)                           | entirely generic, [`std`] *is* FFI-friendly (although nullable)                       |
//!
//!
//!
//! ### References
//! *   <https://learn.microsoft.com/en-us/windows/win32/ipc/pipes>
//! *   <https://learn.microsoft.com/en-us/windows/win32/ipc/multithreaded-pipe-server>

#[cfg(doc)] use crate::{self as firehazard, *};



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/ipc/anonymous-pipes)\]
/// Anonymous Pipes
pub mod anonymous {
    use crate as firehazard;
    include!(r"anonymous\funcs\create.rs");
}
pub use anonymous::create;

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/ipc/named-pipes)\]
/// Named Pipes
pub mod named {
    use crate as firehazard;
    include!(r"named\funcs\call.rs");
    include!(r"named\funcs\create.rs");
    include!(r"named\funcs\connect.rs");
    include!(r"named\funcs\disconnect.rs");
    include!(r"named\funcs\get_client_computer_name.rs");
    include!(r"named\funcs\get_client_process_id.rs");
    include!(r"named\funcs\get_client_session_id.rs");
    include!(r"named\funcs\get_handle_state.rs");
    include!(r"named\funcs\get_info.rs");
    include!(r"named\funcs\get_server_process_id.rs");
    include!(r"named\funcs\get_server_session_id.rs");
    include!(r"named\funcs\impersonate_client.rs");
    include!(r"named\funcs\peek.rs");
    include!(r"named\funcs\set_handle_state.rs");
    include!(r"named\funcs\transact.rs");
    include!(r"named\funcs\wait.rs");
}

pub use values::*;
pub(crate) mod values {
    mod nmpwait;                pub use nmpwait::*;
    mod pipe_flags;             pub use pipe_flags::*;
    mod pipe_max_instances;     pub use pipe_max_instances::*;
}

mod pipe_handles;               pub use pipe_handles::*;
