//! \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/ipc/pipes)\]
//! Anonymous and named pipe APIs
//!
//! ### References
//! *   <https://learn.microsoft.com/en-us/windows/win32/ipc/pipes>
//! *   <https://learn.microsoft.com/en-us/windows/win32/ipc/multithreaded-pipe-server>

pub use funcs::*;
pub(crate) mod funcs {
    use crate as firehazard;
    include!(r"funcs\call_named_pipe.rs");
    include!(r"funcs\connect_named_pipe.rs");
    include!(r"funcs\create_named_pipe.rs");
    include!(r"funcs\create_pipe.rs");
    include!(r"funcs\disconnect_named_pipe.rs");
    include!(r"funcs\get_named_pipe_client_computer_name.rs");
    include!(r"funcs\get_named_pipe_client_process_id.rs");
    include!(r"funcs\get_named_pipe_client_session_id.rs");
    include!(r"funcs\get_named_pipe_handle_state.rs");
    include!(r"funcs\get_named_pipe_info.rs");
    include!(r"funcs\get_named_pipe_server_process_id.rs");
    include!(r"funcs\get_named_pipe_server_session_id.rs");
    include!(r"funcs\impersonate_named_pipe_client.rs");
    include!(r"funcs\peek_named_pipe.rs");
    include!(r"funcs\set_named_pipe_handle_state.rs");
    include!(r"funcs\transact_named_pipe.rs");
    include!(r"funcs\wait_named_pipe.rs");
}

pub use values::*;
pub(crate) mod values {
    mod nmpwait;                pub use nmpwait::*;
    mod pipe_flags;             pub use pipe_flags::*;
    mod pipe_max_instances;     pub use pipe_max_instances::*;
}
