//! \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/ipc/pipes)\]
//! Anonymous and named pipe APIs
//!
//! ### References
//! *   <https://learn.microsoft.com/en-us/windows/win32/ipc/pipes>
//! *   <https://learn.microsoft.com/en-us/windows/win32/ipc/multithreaded-pipe-server>

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
