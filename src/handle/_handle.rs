//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/sysinfo/kernel-objects)\]
//! Generic Kernel Object `HANDLE` wrapping types and functions.
//!
//! Most crate-local handle types are [`core::ptr::NonNull`] wrappers that, if wrapped in an [`Option`], are ABI-compatible with [`HANDLE`].
//! The same cannot be said for [`std`] or third party crate types.
//!
//! | Kernel Type                                   | Owned                                                                     | Borrowed                                                      | Psuedo                    | Relevance<br>(if any) |
//! | --------------------------------------------- | ------------------------------------------------------------------------- | ---------------------------------------------------------     | ------------------------- | --------------------- |
//! | \*                                            | [handle::Owned]                                                           | [handle::Borrowed]                                            | [handle::Psuedo]          |
//! | [Access Token]                                | [token::OwnedHandle]                                                      | [token::Handle]                                               | [token::PsuedoHandle]     | Access restriction
//! | ~~[Change Notification]~~                     |                                                                           |                                                               |                           |
//! | ~~[Communications Device]~~                   | [io::File]<br>[std::fs::File]                                             | [io::FileHandle]<br>[io::ReadHandle]<br>[io::WriteHandle]     |                           |
//! | [Console Input Buffer]                        | [io::File]<br>[io::ReadPipe]<br>[std::fs::File]                           | [io::ReadHandle]<br>[std::io::Stdin]<br>[std::io::StdinLock]  |                           | IPC
//! | [Console Screen Buffer]                       | [io::File]<br>[io::WritePipe]<br>[std::fs::File]                          | [io::WriteHandle]<br>[std::io::Stdout]<br>[std::io::StdoutLock]<br>[std::io::Stderr]<br>[std::io::StderrLock] | | IPC
//! | [Desktop]                                     | [desktop::OwnedHandle]                                                    |                                                               |                           | Access restriction
//! | [Event]                                       |                                                                           |                                                               |                           | IPC
//! | ~~[Event Log]~~                               |                                                                           |                                                               |                           |
//! | [File]                                        | [io::File]<br>[std::fs::File]                                             | [io::FileHandle]<br>[io::ReadHandle]<br>[io::WriteHandle]     |                           | IPC
//! | [File Mapping]                                |                                                                           |                                                               |                           | IPC
//! | ~~[Find File]~~                               | [std::fs::read_dir]                                                       |                                                               |                           |
//! | [Heap]                                        |                                                                           |                                                               |                           | no_std
//! | [I/O Completion Port]                         |                                                                           |                                                               |                           | IPC
//! | [Job]                                         | [job::OwnedHandle]                                                        | [job::Handle]                                                 |                           | Access restriction
//! | ~~[Mailslot]~~                                |                                                                           |                                                               |                           | IPC
//! | [Memory Resource Notification]                |                                                                           |                                                               |                           | Resource limits
//! | [Module]                                      | *Other Crates:*<br>[minidl::Library]<br>[dlopen::raw::Library]            |                                                               |                           | Code patching, debug
//! | [Mutex]                                       | *Local, not win32:*<br>[std::sync::Mutex]                                 | *Local, not win32:*<br>[std::sync::MutexGuard]                |                           | IPC
//! | [Pipe] (Bytewise)<br>(Anonymous or Named)     | [io::ReadPipe]<br>[io::WritePipe]                                         | [io::ReadHandle]<br>[io::WriteHandle]                         |                           | IPC
//! | [Pipe] (Message-based)<br>(Named Only)        |                                                                           |                                                               |                           | IPC
//! | [Process]                                     | [process::OwnedHandle]<br>[std::process::Child]                           | [process::Handle]                                             | [process::PsuedoHandle]   | Access restriction <br> IPC
//! | [Semaphore]                                   |                                                                           |                                                               |                           | IPC
//! | ~~[Socket]~~                                  | [std::net::TcpListener]<br>[std::net::TcpStream]<br>[std::net::UdpSocket] |                                                               |                           | IPC
//! | [Thread]                                      | [thread::OwnedHandle]<br>[std::thread::JoinHandle]                        | [thread::Handle]                                              | [thread::PsuedoHandle]    | Access restriction
//! | ~~[Timer]~~                                   |                                                                           |                                                               |                           |
//! | [Update Resource]                             |                                                                           |                                                               |                           | Code patching
//! | [Window Station]                              | [winsta::OwnedHandle]                                                     |                                                               |                           | Access restriction
//! | ~~[GDI Objects](https://docs.microsoft.com/en-us/windows/win32/sysinfo/gdi-objects)~~
//! | ~~[User Objects](https://docs.microsoft.com/en-us/windows/win32/sysinfo/user-objects)~~
//!
//! Crossed out kernel types are unlikely to ever be supported.
//! Empty owned columns might gain support in the future.
//! Some third party crates are listed where this crate has no natural support.
//! Some std types are listed as well.
//!
//!
//!
//! [minidl::Library]:                  https://docs.rs/minidl/latest/minidl/struct.Library.html
//! [dlopen::raw::Library]:             https://docs.rs/dlopen/latest/dlopen/raw/struct.Library.html
//!
//! [Access Token]:                     https://docs.microsoft.com/en-us/windows/win32/secauthz/access-tokens
//! [Change Notification]:              https://docs.microsoft.com/en-us/windows/win32/fileio/obtaining-directory-change-notifications
//! [Communications Device]:            https://docs.microsoft.com/en-us/windows/win32/devio/communications-resources
//! [Console Input Buffer]:             https://docs.microsoft.com/en-us/windows/console/console-input-buffer
//! [Console Screen Buffer]:            https://docs.microsoft.com/en-us/windows/console/console-screen-buffers
//! [Desktop]:                          https://docs.microsoft.com/en-us/windows/win32/winstation/desktops
//! [Event]:                            https://docs.microsoft.com/en-us/windows/win32/sync/event-objects
//! [Event Log]:                        https://docs.microsoft.com/en-us/windows/win32/eventlog/event-logging
//! [File]:                             https://docs.microsoft.com/en-us/windows/win32/fileio/file-objects
//! [File Mapping]:                     https://docs.microsoft.com/en-us/windows/win32/memory/file-mapping
//! [Find File]:                        https://docs.microsoft.com/en-us/windows/win32/fileio/listing-the-files-in-a-directory
//! [Heap]:                             https://docs.microsoft.com/en-us/windows/win32/memory/heap-functions
//! [I/O Completion Port]:              https://docs.microsoft.com/en-us/windows/win32/fileio/i-o-completion-ports
//! [Job]:                              https://docs.microsoft.com/en-us/windows/win32/procthread/job-objects
//! [Mailslot]:                         https://docs.microsoft.com/en-us/windows/win32/ipc/mailslots
//! [Memory Resource Notification]:     https://docs.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-creatememoryresourcenotification
//! [Module]:                           https://docs.microsoft.com/en-us/windows/win32/psapi/module-information
//! [Mutex]:                            https://docs.microsoft.com/en-us/windows/win32/sync/mutex-objects
//! [Pipe]:                             https://docs.microsoft.com/en-us/windows/win32/ipc/about-pipes
//! [Process]:                          https://docs.microsoft.com/en-us/windows/win32/procthread/about-processes-and-threads
//! [Semaphore]:                        https://docs.microsoft.com/en-us/windows/win32/sync/semaphore-objects
//! [Socket]:                           https://docs.microsoft.com/en-us/windows/win32/winsock/getting-started-with-winsock
//! [Thread]:                           https://docs.microsoft.com/en-us/windows/win32/procthread/about-processes-and-threads
//! [Timer]:                            https://docs.microsoft.com/en-us/windows/win32/sync/waitable-timer-objects
//! [Update Resource]:                  https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-updateresourcea
//! [Window Station]:                   https://docs.microsoft.com/en-us/windows/win32/winstation/window-stations

#[allow(unused_imports)] use crate::*;
#[allow(unused_imports)] use winapi::shared::ntdef::HANDLE;



#[path = "handle_handles.rs"]   mod handles;            pub use handles::*;
#[path = "handle_flags.rs"]     mod flags;              pub use flags::*;
#[path = "handle_funcs.rs"]     pub(crate) mod funcs;   pub use funcs::*;
#[path = "handle_traits.rs"]    pub(crate) mod traits;  pub use traits::*;
