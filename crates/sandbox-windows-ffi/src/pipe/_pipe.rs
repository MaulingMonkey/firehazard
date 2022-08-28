//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/ipc/pipes)\]
//! Pipe handles and fns
//!
//! ### References
//! *   <https://docs.microsoft.com/en-us/windows/win32/ipc/pipes>
//! *   <https://docs.microsoft.com/en-us/windows/win32/ipc/multithreaded-pipe-server>

#[path = "pipe_funcs.rs"] pub(crate) mod funcs;
mod pipe_handles;                   pub use pipe_handles::*;
