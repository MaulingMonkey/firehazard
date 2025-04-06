//! I/O APIs ([pipe](https://learn.microsoft.com/en-us/windows/win32/ipc/pipes) handles and fns)
//!
//! ### References
//! *   <https://learn.microsoft.com/en-us/windows/win32/ipc/pipes>
//! *   <https://learn.microsoft.com/en-us/windows/win32/ipc/multithreaded-pipe-server>

#[cfg(    std )] pub use std::io::{Read, Write, Seek, Error, ErrorKind, Result};
#[cfg(    std )] mod io_yes_std;
#[cfg(not(std))] mod io_not_std;
#[cfg(not(std))] pub use io_not_std::*;



#[path = "io_funcs.rs"]             pub(crate) mod funcs;       pub use funcs::*;
mod io_counters;                    pub use io_counters::*;
mod io_handles;                     pub use io_handles::*;
