//! \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/ipc/named-pipes)\]
//! Named Pipes
//!
//!
//!
//! ### Comparison vs Sockets
//!
//! -   Pipes get full blown names, meaning you don't need to worry about accidental collisions with limited port numbers.
//! -   Pipes are subject to the windows security model, allowing you to require / rely on OS authentication.
//!
//!
//!
//! ### Names
//!
//! Pipe names take the form of:
//! -   `\\ServerName\pipe\PipeName`        &mdash; Traditional.
//! -   `\\ServerName\pipe\local\PipeName`  &mdash; AppContainer friendly (access will be limited to processes of the same app.)
//!
//! To reference pipes on the current machine, use `.` for `ServerName`, e.g.: `\\.\pipe\local\example`
//!
//!
//!
//! ### Quirks: Accepting Connections
//!
//! Where [`TcpListener::accept`] returns a new handle which is bound to the accepted connection for read and write,
//! [`pipe::named::Listener::accept`] instead *reuses* and binds *the original listening handle,* for read and/or write.
//! This can lead to some suprises with pipes "disappearing" when the last/only pipe is connected to.
//! "Connections" can include basic metadata checks, which are implemented in terms of opening a handle:
//!
//! -   [Calling GetFileAttributesW() removes a pipe](https://stackoverflow.com/questions/28769237/calling-getfileattributesw-removes-a-pipe) (stackoverflow.com)
//! -   [`dotnet/runtime#69604`: Using File.Exists to check the pipe created will make the NamedPipeClientStream connect fail](https://github.com/dotnet/runtime/issues/69604) (github.com)
//!
//! Such calls presumably read/write 0 bytes, which you may wish to handle:
//!
//! -   Without cluttering your logs with error messages
//! -   Without performing any expensive processing
//!
//! On the other hand, you should likely avoid such checks anyways, favoring [TOC/TOU](https://en.wikipedia.org/wiki/Time-of-check_to_time-of-use)-avoiding alternatives such as:
//!
//! -   Calling [`pipe::named::create`] with [`file::FLAG_FIRST_PIPE_INSTANCE`], handling [`ERROR::ACCESS_DENIED`] if you wish to avoid fighting with a possibly pre-existing pipe server.
//! -   Calling [`create_file`], handling [`ERROR::PATH_NOT_FOUND`] if you wish to only connect to pre-existing pipe servers.
//!
//!
//!
//! ### Quirks: Serialized I/O
//!
//! Multithreaded access to a single [`pipe::sync::Duplex`] is deadlock-bait in a way that [`TcpStream`] is not.
//! All access to a given non-`OVERLAPPED` pipe object is serialized, even for seemingly "unrelated" operations that seem like they shouldn't be.
//!
//! Suppose a server and client connect to each other, and perform the following operations that happen to get syncronized in the following order:
//! -   Thread #1: Client calls `ReadFile(client_pipe, ...)`
//! -   Thread #2: Client calls `WriteFile(client_pipe, ...)`
//! -   Thread #3: Server calls `ReadFile(server_pipe, ...)`
//! -   Thread #4: Server calls `WriteFile(server_pipe, ...)`
//!
//! Such a program will deadlock, and buffering won't help!
//! -   #2 waits forever for #1 to finish reading `client_pipe` (same object) before writing *anything*.
//! -   #4 waits forever for #3 to finish reading `server_pipe` (same object) before writing *anything*.
//! -   #1 waits forever for a single miserable byte to be read.  None are, despite #4's callstack blocked in `WriteFile`.
//! -   #3 waits forever for a single miserable byte to be read.  None are, despite #2's callstack blocked in `WriteFile`.
//!
//! To combat bugs, <code>&[pipe::sync::Duplex]</code> does not implement [`io::Read`] or [`io::Write`] - only [`pipe::sync::Duplex`] itself does.
//! By contrast, [`io::Read`] *is* implemented for <code>&[pipe::sync::Reader]</code>, and [`io::Write`] for <code>&[pipe::sync::Writer]</code> - these are much less dangerous.
//! `DuplicateHandle` will not help you either: the duplicated handle references the same pipe object.
//!
//! References:
//! -   [Behavior of Win32 named pipe in duplex mode](https://stackoverflow.com/questions/58483293/behavior-of-win32-named-pipe-in-duplex-mode) (stackoverflow.com)
//! -   [IoCreateFile function (wdm.h)](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/wdm/nf-wdm-iocreatefile) (microsoft.com)
//!
//!
//!
//! [`TcpStream`]:              std::net::TcpStream
//! [`TcpListener`]:            std::net::TcpListener
//! [`TcpListener::accept`]:    std::net::TcpListener::accept



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

include!(r"handles\bytes_reader.rs");
include!(r"handles\connected.rs");
include!(r"handles\listener.rs");
include!(r"handles\message_reader.rs");
