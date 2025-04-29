//! \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/fileio/synchronous-and-asynchronous-i-o)] Generic I/O APIs
//!
//! ### References
//! *   <https://learn.microsoft.com/en-us/windows/win32/fileio/synchronous-and-asynchronous-i-o>
//! *   <https://learn.microsoft.com/en-us/windows/win32/ipc/pipes>
//! *   <https://learn.microsoft.com/en-us/windows/win32/ipc/multithreaded-pipe-server>

#[cfg(    std )] pub use std::io::{Read, Seek, Write, Error, ErrorKind, Result, SeekFrom};
#[cfg(    std )] mod io_yes_std;
#[cfg(not(std))] mod io_not_std;
#[cfg(not(std))] pub use io_not_std::*;



mod io_counters;                    pub use io_counters::*;
mod io_handles;                     pub use io_handles::*;
mod io_status_block;                pub(crate) use io_status_block::StatusBlock;

#[allow(unused_imports)] pub use funcs::*;
pub(crate) mod funcs {
    use crate::prelude::*;
    include!(r"funcs\read_file.rs");
    include!(r"funcs\write_file.rs");
}

#[deprecated = "use `firehazard::io::FileNN` instead"       ] #[doc(hidden)] pub type File         = FileNN;
#[deprecated = "use `firehazard::pipe::ReaderNN` instead"   ] #[doc(hidden)] pub type ReadPipe     = crate::pipe::ReaderNN;
#[deprecated = "use `firehazard::pipe::WriterNN` instead"   ] #[doc(hidden)] pub type WritePipe    = crate::pipe::WriterNN;
#[deprecated = "use `firehazard::pipe::create` instead"     ] #[doc(hidden)] pub use crate::pipe::create as create_pipe;
