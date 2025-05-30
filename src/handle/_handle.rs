//! \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/sysinfo/kernel-objects)\]
//! Generic Kernel Object `HANDLE` wrapping types and functions.
//!
//! Most crate-local handle types are [`core::ptr::NonNull`] wrappers that, if wrapped in an [`Option`], are ABI-compatible with [`HANDLE`].
//! The same cannot be said for [`std`] or third party crate types.
//!
//! | Kernel Type                                   | Owned                                                                     | Borrowed                                                      | Borrowed or Pseudo        | Relevance<br>(if any) |
//! | --------------------------------------------- | ------------------------------------------------------------------------- | ---------------------------------------------------------     | ------------------------- | --------------------- |
//! | \*                                            | [handle::Owned] <br><br> *transparent `HANDLE`:* <br> [std::...::OwnedHandle] | [handle::Borrowed] | [handle::Pseudo]  <br><br> *transparent `HANDLE`:* <br> [std::...::BorrowedHandle] |
//! | [Access Token]                                | [token::OwnedHandle]                                                      | [token::Handle]                                               | [token::PseudoHandle]     | Access&nbsp;restriction
//! | ~~[Change Notification]~~                     |                                                                           |                                                               |                           |
//! | ~~[Communications Device]~~                   | [io::sync::OwnedFile]<br>[std::fs::File]                                  | [io::sync::BorrowedFile]<br>[io::sync::BorrowedReader]<br>[io::sync::BorrowedWriter] |    |
//! | [Console Input Buffer]                        | [io::sync::OwnedFile]<br>[pipe::sync::OwnedReader]<br>[std::fs::File]     | [io::sync::BorrowedReader]<br>[std::io::Stdin]<br>[std::io::StdinLock] |                  | IPC
//! | [Console Screen Buffer]                       | [io::sync::OwnedFile]<br>[pipe::sync::OwnedWriter]<br>[std::fs::File]     | [io::sync::BorrowedWriter]<br>[std::io::Stdout]<br>[std::io::StdoutLock]<br>[std::io::Stderr]<br>[std::io::StderrLock] | | IPC
//! | [Desktop]                                     | [desktop::OwnedHandle]                                                    | [desktop::Handle]                                             |                           | Access restriction
//! | [Event]                                       |                                                                           |                                                               |                           | IPC
//! | ~~[Event Log]~~                               |                                                                           |                                                               |                           |
//! | [File]                                        | [io::sync::OwnedFile]<br>[io::sync::OwnedReader]<br>[io::sync::OwnedWriter]<br>[std::fs::File] | [io::sync::BorrowedFile]<br>[io::sync::BorrowedReader]<br>[io::sync::BorrowedWriter] | | IPC
//! | [File Mapping]                                |                                                                           |                                                               |                           | IPC
//! | ~~[Find File]~~                               | [std::fs::ReadDir]                                                        |                                                               |                           |
//! | [Heap]                                        | *Other Crates:* <br> [ialloc::...::Heap]                                  |                                                               |                           | no_std
//! | [I/O Completion Port]                         |                                                                           |                                                               |                           | IPC
//! | [Job]                                         | [job::OwnedHandle]                                                        | [job::Handle]                                                 |                           | Access restriction
//! | ~~[Mailslot]~~                                |                                                                           |                                                               |                           | IPC
//! | [Memory Resource Notification]                |                                                                           |                                                               |                           | Resource limits
//! | [Module] (not a `HANDLE`)                     | *Other Crates:*<br>[minidl::Library]<br>[dlopen::raw::Library]            |                                                               |                           | Code patching <br> Debug
//! | [Mutex]                                       | *Local, not win32:*<br>[std::sync::Mutex]                                 | *Local, not win32:*<br>[std::sync::MutexGuard]                |                           | IPC
//! | [Pipe] (Bytewise)<br>(Anonymous or Named)     | [pipe::sync::OwnedDuplex]<br>[pipe::sync::OwnedReader]<br>[pipe::sync::OwnedWriter]<br>[std::io::PipeReader]<br>[std::io::PipeWriter] | [pipe::sync::BorrowedDuplex]<br>[pipe::sync::BorrowedReader]<br>[pipe::sync::BorrowedWriter] | | IPC
//! | [Pipe] (Message-based)<br>(Named Only)        |                                                                           |                                                               |                           | IPC
//! | [Process]                                     | [process::OwnedHandle]<br>[std::process::Child]                           | [process::Handle]                                             | [process::PseudoHandle]   | Access restriction <br> IPC
//! | [Pseudo Console]                              | [pseudoconsole::Owned]                                                    |                                                               |                           | IPC?
//! | [Semaphore]                                   |                                                                           |                                                               |                           | IPC
//! | [Socket]                                      | [std::net::TcpListener] <br> [std::net::TcpStream] <br> [std::net::UdpSocket] <br><br> *transparent `SOCKET`:* <br> [std::...::OwnedSocket]   | *transparent `SOCKET`:* <br> [std::...::BorrowedSocket] | | IPC
//! | [Thread]                                      | [thread::OwnedHandle]<br>[std::thread::JoinHandle]                        | [thread::Handle]                                              | [thread::PseudoHandle]    | Access restriction
//! | ~~[Timer]~~                                   |                                                                           |                                                               |                           |
//! | [Update Resource]                             |                                                                           |                                                               |                           | Code patching
//! | [Window Station]                              | [winsta::OwnedHandle]                                                     | [winsta::Handle]                                              |                           | Access restriction
//! | ~~[GDI Objects](https://learn.microsoft.com/en-us/windows/win32/sysinfo/gdi-objects)~~
//! | ~~[User Objects](https://learn.microsoft.com/en-us/windows/win32/sysinfo/user-objects)~~
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
//! [ialloc::...::Heap]:                https://docs.rs/ialloc/latest/ialloc/allocator/win32/struct.Heap.html
//!
//! [std::io::PipeReader]:              https://doc.rust-lang.org/std/io/struct.PipeReader.html
//! [std::io::PipeWriter]:              https://doc.rust-lang.org/std/io/struct.PipeWriter.html
//! [std::...::BorrowedHandle]:         std::os::windows::io::BorrowedHandle
//! [std::...::BorrowedSocket]:         std::os::windows::io::BorrowedSocket
//! [std::...::OwnedHandle]:            std::os::windows::io::OwnedHandle
//! [std::...::OwnedSocket]:            std::os::windows::io::OwnedSocket
//!
//! [Access Token]:                     https://learn.microsoft.com/en-us/windows/win32/secauthz/access-tokens
//! [Change Notification]:              https://learn.microsoft.com/en-us/windows/win32/fileio/obtaining-directory-change-notifications
//! [Communications Device]:            https://learn.microsoft.com/en-us/windows/win32/devio/communications-resources
//! [Console Input Buffer]:             https://learn.microsoft.com/en-us/windows/console/console-input-buffer
//! [Console Screen Buffer]:            https://learn.microsoft.com/en-us/windows/console/console-screen-buffers
//! [Desktop]:                          https://learn.microsoft.com/en-us/windows/win32/winstation/desktops
//! [Event]:                            https://learn.microsoft.com/en-us/windows/win32/sync/event-objects
//! [Event Log]:                        https://learn.microsoft.com/en-us/windows/win32/eventlog/event-logging
//! [File]:                             https://learn.microsoft.com/en-us/windows/win32/fileio/file-objects
//! [File Mapping]:                     https://learn.microsoft.com/en-us/windows/win32/memory/file-mapping
//! [Find File]:                        https://learn.microsoft.com/en-us/windows/win32/fileio/listing-the-files-in-a-directory
//! [Heap]:                             https://learn.microsoft.com/en-us/windows/win32/memory/heap-functions
//! [I/O Completion Port]:              https://learn.microsoft.com/en-us/windows/win32/fileio/i-o-completion-ports
//! [Job]:                              https://learn.microsoft.com/en-us/windows/win32/procthread/job-objects
//! [Mailslot]:                         https://learn.microsoft.com/en-us/windows/win32/ipc/mailslots
//! [Memory Resource Notification]:     https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-creatememoryresourcenotification
//! [Module]:                           https://learn.microsoft.com/en-us/windows/win32/psapi/module-information
//! [Mutex]:                            https://learn.microsoft.com/en-us/windows/win32/sync/mutex-objects
//! [Pipe]:                             https://learn.microsoft.com/en-us/windows/win32/ipc/about-pipes
//! [Process]:                          https://learn.microsoft.com/en-us/windows/win32/procthread/about-processes-and-threads
//! [Pseudo Console]:                   https://learn.microsoft.com/en-us/windows/console/pseudoconsoles
//! [Semaphore]:                        https://learn.microsoft.com/en-us/windows/win32/sync/semaphore-objects
//! [Socket]:                           https://learn.microsoft.com/en-us/windows/win32/winsock/getting-started-with-winsock
//! [Thread]:                           https://learn.microsoft.com/en-us/windows/win32/procthread/about-processes-and-threads
//! [Timer]:                            https://learn.microsoft.com/en-us/windows/win32/sync/waitable-timer-objects
//! [Update Resource]:                  https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-updateresourcea
//! [Window Station]:                   https://learn.microsoft.com/en-us/windows/win32/winstation/window-stations

#[allow(unused_imports)] use crate::prelude::*;
#[allow(unused_imports)] use winapi::shared::ntdef::HANDLE;



#[path = "handle_handles.rs"]   mod handles;            pub use handles::*;
#[path = "handle_flags.rs"]     mod flags;              pub use flags::*;
#[path = "handle_traits.rs"]    pub(crate) mod traits;  pub use traits::*;

pub use funcs::*;
pub(crate) mod funcs {
    use crate::prelude::*;
    include!(r"funcs\close_handle.rs");
    include!(r"funcs\compare_object_handles.rs");
    include!(r"funcs\duplicate_handle.rs");
    include!(r"funcs\get_handle_information.rs");
    include!(r"funcs\nt_query_object.rs");
    include!(r"funcs\set_handle_information.rs");
}
include!(r"funcs\debug.rs"); // XXX: don't re-export at crate root
