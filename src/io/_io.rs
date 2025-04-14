//! \[<strike>microsoft.com</strike>\] Generic I/O APIs
//!
//! ### References
//! *   <https://learn.microsoft.com/en-us/windows/win32/ipc/pipes>
//! *   <https://learn.microsoft.com/en-us/windows/win32/ipc/multithreaded-pipe-server>

#[cfg(    std )] pub use std::io::{Read, Seek, Write, Error, ErrorKind, Result, SeekFrom};
#[cfg(    std )] mod io_yes_std;
#[cfg(not(std))] mod io_not_std;
#[cfg(not(std))] pub use io_not_std::*;



mod io_counters;                    pub use io_counters::*;
mod io_handles;                     pub use io_handles::*;

pub use funcs::*;
pub(crate) mod funcs {
    use crate as firehazard;
    include!(r"funcs\get_final_path_name_by_handle.rs");
    include!(r"funcs\read_file.rs");
    include!(r"funcs\set_file_pointer.rs");
    include!(r"funcs\write_file.rs");
}

#[deprecated = "moved to firehazard::pipe::create"] #[doc(hidden)] pub use crate::pipe::create as create_pipe;
