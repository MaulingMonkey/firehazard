//! \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/ipc/named-pipes)\]
//! Named Pipes

use crate::prelude::*;

include!(r"funcs\call.rs");
include!(r"funcs\create.rs");
include!(r"funcs\connect.rs");
include!(r"funcs\disconnect.rs");
include!(r"funcs\get_client_computer_name.rs");
include!(r"funcs\get_client_process_id.rs");
include!(r"funcs\get_client_session_id.rs");
include!(r"funcs\get_handle_state.rs");
include!(r"funcs\get_info.rs");
include!(r"funcs\get_server_process_id.rs");
include!(r"funcs\get_server_session_id.rs");
include!(r"funcs\impersonate_client.rs");
include!(r"funcs\peek.rs");
include!(r"funcs\set_handle_state.rs");
include!(r"funcs\transact.rs");
include!(r"funcs\wait.rs");
