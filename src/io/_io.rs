//! I/O APIs ([pipe](https://learn.microsoft.com/en-us/windows/win32/ipc/pipes) handles and fns)
//!
//! ### References
//! *   <https://learn.microsoft.com/en-us/windows/win32/ipc/pipes>
//! *   <https://learn.microsoft.com/en-us/windows/win32/ipc/multithreaded-pipe-server>

#[cfg(    std )] pub use std::io::{Read, Write, Seek, Error, ErrorKind, Result};
#[cfg(    std )] mod io_yes_std;
#[cfg(not(std))] mod io_not_std;
#[cfg(not(std))] pub use io_not_std::*;



mod io_counters;                    pub use io_counters::*;
mod io_handles;                     pub use io_handles::*;

pub use funcs::*;
pub(crate) mod funcs {
    use crate as firehazard;
    // TODO: CallNamedPipeA
    // TODO: CallNamedPipeW
    // TODO: ConnectNamedPipe
    // TODO: CreateNamedPipeA
    // TODO: CreateNamedPipeW
    include!("funcs/create_pipe.rs");
    // TODO: DisconnectNamedPipe
    include!("funcs/get_final_path_name_by_handle.rs");
    // TODO: GetNamedPipeClientComputerNameA
    // TODO: GetNamedPipeClientComputerNameW
    // TODO: GetNamedPipeClientProcessId
    // TODO: GetNamedPipeClientSessionId
    // TODO: GetNamedPipeHandleStateA
    // TODO: GetNamedPipeHandleStateW
    // TODO: GetNamedPipeInfo
    // TODO: GetNamedPipeServerProcessId
    // TODO: GetNamedPipeServerSessionId
    // TODO: ImpersonateNamedPipeClient
    // TODO: PeekNamedPipe
    // TODO: SetNamedPipeHandleState
    // TODO: TransactNamedPipe
    // TODO: WaitNamedPipeA
    // TODO: WaitNamedPipeW
}
