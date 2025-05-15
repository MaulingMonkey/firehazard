#![cfg(std)]

use crate::prelude::*;
use crate::os::windows::prelude::*;



/// <code>std::os::windows::io::[FromRawHandle::from_raw_handle](std::os::windows::io::FromRawHandle::from_raw_handle)</code> reads:
/// "The handle passed in must: [...] be an owned handle; in particular, it must be open."
///
/// As such, I believe <code>std::fs::[File](std::fs::File)::[from_raw_handle](std::fs::File::from_raw_handle)(core::ptr::[null_mut](core::ptr::null_mut)())</code> is undefined behavior.
/// This is further evidenced by <code>std::os::windows::io::[HandleOrNull::from_raw_handle](std::os::windows::io::HandleOrNull::from_raw_handle)</code>, which reads:
/// "The passed handle value must either satisfy the safety requirements of FromRawHandle::from_raw_handle, or be null."
///
/// This implies null does *not* meet the baseline requirements of [`FromRawHandle::from_raw_handle`](std::os::windows::io::FromRawHandle::from_raw_handle)
///
const EXPECT_NONNULL_STD_FS_FILE            : &'static str = r#"undefined behavior: `std::fs::File::from_raw_handle(core::ptr::null_mut())` was presumably called earlier - but null is not an open, owned handle, as required per `std::os::windows::io::FromRawHandle::from_raw_handle`'s "Safety" docs."#;
const EXPECT_NONNULL_STD_IO_PIPE_READER     : &'static str = r#"undefined behavior: `std::io::PipeReader::from_raw_handle(core::ptr::null_mut())` was presumably called earlier - but null is not an open, owned handle, as required per `std::os::windows::io::FromRawHandle::from_raw_handle`'s "Safety" docs."#;
const EXPECT_NONNULL_STD_IO_PIPE_WRITER     : &'static str = r#"undefined behavior: `std::io::PipeWriter::from_raw_handle(core::ptr::null_mut())` was presumably called earlier - but null is not an open, owned handle, as required per `std::os::windows::io::FromRawHandle::from_raw_handle`'s "Safety" docs."#;
const EXPECT_NONNULL_STD_NET_TCP_LISTENER   : &'static str = r#"undefined behavior: `std::net::TcpListener::from_raw_socket(0)` was presumably called earlier - but zero is not an open, owned handle, as required per `std::os::windows::io::FromRawSocket::from_raw_socket`'s "Safety" docs."#;
const EXPECT_NONNULL_STD_NET_TCP_STREAM     : &'static str = r#"undefined behavior: `std::net::TcpStream::from_raw_socket(0)` was presumably called earlier - but zero is not an open, owned handle, as required per `std::os::windows::io::FromRawSocket::from_raw_socket`'s "Safety" docs."#;
const EXPECT_NONNULL_STD_NET_UDP_SOCKET     : &'static str = r#"undefined behavior: `std::net::UdpSocket::from_raw_socket(0)` was presumably called earlier - but zero is not an open, owned handle, as required per `std::os::windows::io::FromRawSocket::from_raw_socket`'s "Safety" docs."#;

impl     From<    std::fs::File         > for handle::Owned         { fn from(file:     std::fs::File           ) -> Self { unsafe { Self::from_raw(file.into_raw_handle()  .cast()) }.expect(EXPECT_NONNULL_STD_FS_FILE) } }
impl<'a> From<&'a std::fs::File         > for handle::Borrowed<'a>  { fn from(file: &'a std::fs::File           ) -> Self { unsafe { Self::from_raw(file.as_raw_handle()    .cast()) }.expect(EXPECT_NONNULL_STD_FS_FILE) } }
impl<'a> From<&'a std::fs::File         > for handle::Pseudo<'a>    { fn from(file: &'a std::fs::File           ) -> Self { unsafe { Self::from_raw(file.as_raw_handle()    .cast()) }.expect(EXPECT_NONNULL_STD_FS_FILE) } }
impl     From<    std::io::PipeReader   > for handle::Owned         { fn from(file:     std::io::PipeReader     ) -> Self { unsafe { Self::from_raw(file.into_raw_handle()  .cast()) }.expect(EXPECT_NONNULL_STD_IO_PIPE_READER) } }
impl<'a> From<&'a std::io::PipeReader   > for handle::Borrowed<'a>  { fn from(file: &'a std::io::PipeReader     ) -> Self { unsafe { Self::from_raw(file.as_raw_handle()    .cast()) }.expect(EXPECT_NONNULL_STD_IO_PIPE_READER) } }
impl<'a> From<&'a std::io::PipeReader   > for handle::Pseudo<'a>    { fn from(file: &'a std::io::PipeReader     ) -> Self { unsafe { Self::from_raw(file.as_raw_handle()    .cast()) }.expect(EXPECT_NONNULL_STD_IO_PIPE_READER) } }
impl     From<    std::io::PipeWriter   > for handle::Owned         { fn from(file:     std::io::PipeWriter     ) -> Self { unsafe { Self::from_raw(file.into_raw_handle()  .cast()) }.expect(EXPECT_NONNULL_STD_IO_PIPE_WRITER) } }
impl<'a> From<&'a std::io::PipeWriter   > for handle::Borrowed<'a>  { fn from(file: &'a std::io::PipeWriter     ) -> Self { unsafe { Self::from_raw(file.as_raw_handle()    .cast()) }.expect(EXPECT_NONNULL_STD_IO_PIPE_WRITER) } }
impl<'a> From<&'a std::io::PipeWriter   > for handle::Pseudo<'a>    { fn from(file: &'a std::io::PipeWriter     ) -> Self { unsafe { Self::from_raw(file.as_raw_handle()    .cast()) }.expect(EXPECT_NONNULL_STD_IO_PIPE_WRITER) } }

// XXX: this assumes an NT kernel.  These all identify as `File` handles, but for now I'm being conservative and only converting to basic handles.
impl     From<    std::net::TcpListener > for handle::Owned         { fn from(sock:     std::net::TcpListener   ) -> Self { unsafe { Self::from_raw(sock.into_raw_socket()      as _) }.expect(EXPECT_NONNULL_STD_NET_TCP_LISTENER) } }
impl<'a> From<&'a std::net::TcpListener > for handle::Borrowed<'a>  { fn from(sock: &'a std::net::TcpListener   ) -> Self { unsafe { Self::from_raw(sock.as_raw_socket()        as _) }.expect(EXPECT_NONNULL_STD_NET_TCP_LISTENER) } }
impl<'a> From<&'a std::net::TcpListener > for handle::Pseudo<'a>    { fn from(sock: &'a std::net::TcpListener   ) -> Self { unsafe { Self::from_raw(sock.as_raw_socket()        as _) }.expect(EXPECT_NONNULL_STD_NET_TCP_LISTENER) } }

// XXX: this assumes an NT kernel.  These all identify as `File` handles, but for now I'm being conservative and only converting to basic handles.
impl     From<    std::net::TcpStream   > for handle::Owned         { fn from(sock:     std::net::TcpStream     ) -> Self { unsafe { Self::from_raw(sock.into_raw_socket()      as _) }.expect(EXPECT_NONNULL_STD_NET_TCP_STREAM) } }
impl<'a> From<&'a std::net::TcpStream   > for handle::Borrowed<'a>  { fn from(sock: &'a std::net::TcpStream     ) -> Self { unsafe { Self::from_raw(sock.as_raw_socket()        as _) }.expect(EXPECT_NONNULL_STD_NET_TCP_STREAM) } }
impl<'a> From<&'a std::net::TcpStream   > for handle::Pseudo<'a>    { fn from(sock: &'a std::net::TcpStream     ) -> Self { unsafe { Self::from_raw(sock.as_raw_socket()        as _) }.expect(EXPECT_NONNULL_STD_NET_TCP_STREAM) } }

// XXX: this assumes an NT kernel.  These all identify as `File` handles, but for now I'm being conservative and only converting to basic handles.
impl     From<    std::net::UdpSocket   > for handle::Owned         { fn from(sock:     std::net::UdpSocket     ) -> Self { unsafe { Self::from_raw(sock.into_raw_socket()      as _) }.expect(EXPECT_NONNULL_STD_NET_UDP_SOCKET) } }
impl<'a> From<&'a std::net::UdpSocket   > for handle::Borrowed<'a>  { fn from(sock: &'a std::net::UdpSocket     ) -> Self { unsafe { Self::from_raw(sock.as_raw_socket()        as _) }.expect(EXPECT_NONNULL_STD_NET_UDP_SOCKET) } }
impl<'a> From<&'a std::net::UdpSocket   > for handle::Pseudo<'a>    { fn from(sock: &'a std::net::UdpSocket     ) -> Self { unsafe { Self::from_raw(sock.as_raw_socket()        as _) }.expect(EXPECT_NONNULL_STD_NET_UDP_SOCKET) } }

// {io,pipe}::sync::* → std::* should be sound since the former currently forbids `FILE_FLAG_OVERLAPPED`.
impl From<io::sync::OwnedFile       > for std::fs::File         { fn from(h: io::sync::OwnedFile    ) -> Self { unsafe { std::fs::File       ::from_raw_handle(h.into_handle()) } } }
impl From<io::sync::OwnedFile       > for std::io::PipeReader   { fn from(h: io::sync::OwnedFile    ) -> Self { unsafe { std::io::PipeReader ::from_raw_handle(h.into_handle()) } } }
impl From<io::sync::OwnedFile       > for std::io::PipeWriter   { fn from(h: io::sync::OwnedFile    ) -> Self { unsafe { std::io::PipeWriter ::from_raw_handle(h.into_handle()) } } }
//pl From<io::sync::OwnedDuplex     > for std::fs::File         { fn from(h: io::sync::OwnedDuplex  ) -> Self { unsafe { std::fs::File       ::from_raw_handle(h.into_handle()) } } } // XXX: kinda sketch?
impl From<io::sync::OwnedDuplex     > for std::io::PipeReader   { fn from(h: io::sync::OwnedDuplex  ) -> Self { unsafe { std::io::PipeReader ::from_raw_handle(h.into_handle()) } } }
impl From<io::sync::OwnedDuplex     > for std::io::PipeWriter   { fn from(h: io::sync::OwnedDuplex  ) -> Self { unsafe { std::io::PipeWriter ::from_raw_handle(h.into_handle()) } } }
impl From<io::sync::OwnedReader     > for std::io::PipeReader   { fn from(h: io::sync::OwnedReader  ) -> Self { unsafe { std::io::PipeReader ::from_raw_handle(h.into_handle()) } } }
impl From<io::sync::OwnedWriter     > for std::io::PipeWriter   { fn from(h: io::sync::OwnedWriter  ) -> Self { unsafe { std::io::PipeWriter ::from_raw_handle(h.into_handle()) } } }
//pl From<pipe::sync::OwnedDuplex   > for std::fs::File         { fn from(h: pipe::sync::OwnedDuplex) -> Self { unsafe { std::fs::File       ::from_raw_handle(h.into_handle()) } } } // XXX: kinda sketch?
impl From<pipe::sync::OwnedDuplex   > for std::io::PipeReader   { fn from(h: pipe::sync::OwnedDuplex) -> Self { unsafe { std::io::PipeReader ::from_raw_handle(h.into_handle()) } } }
impl From<pipe::sync::OwnedDuplex   > for std::io::PipeWriter   { fn from(h: pipe::sync::OwnedDuplex) -> Self { unsafe { std::io::PipeWriter ::from_raw_handle(h.into_handle()) } } }
impl From<pipe::sync::OwnedReader   > for std::io::PipeReader   { fn from(h: pipe::sync::OwnedReader) -> Self { unsafe { std::io::PipeReader ::from_raw_handle(h.into_handle()) } } }
impl From<pipe::sync::OwnedWriter   > for std::io::PipeWriter   { fn from(h: pipe::sync::OwnedWriter) -> Self { unsafe { std::io::PipeWriter ::from_raw_handle(h.into_handle()) } } }

impl    AsLocalHandle for std::fs::File                 { fn as_handle(&self) -> HANDLE { self.as_raw_handle().cast() } }
impl    AsLocalHandle for std::io::PipeReader           { fn as_handle(&self) -> HANDLE { self.as_raw_handle().cast() } }
impl    AsLocalHandle for std::io::PipeWriter           { fn as_handle(&self) -> HANDLE { self.as_raw_handle().cast() } }
impl    AsLocalHandle for std::io::Stderr               { fn as_handle(&self) -> HANDLE { self.as_raw_handle().cast() } }
impl    AsLocalHandle for std::io::Stdin                { fn as_handle(&self) -> HANDLE { self.as_raw_handle().cast() } }
impl    AsLocalHandle for std::io::Stdout               { fn as_handle(&self) -> HANDLE { self.as_raw_handle().cast() } }
impl    AsLocalHandle for std::net::TcpListener         { fn as_handle(&self) -> HANDLE { self.as_raw_socket() as _ } }// XXX: this assumes an NT kernel.
impl    AsLocalHandle for std::net::TcpStream           { fn as_handle(&self) -> HANDLE { self.as_raw_socket() as _ } }// XXX: this assumes an NT kernel.
impl    AsLocalHandle for std::net::UdpSocket           { fn as_handle(&self) -> HANDLE { self.as_raw_socket() as _ } }// XXX: this assumes an NT kernel.
impl    AsLocalHandle for std::process::ChildStderr     { fn as_handle(&self) -> HANDLE { self.as_raw_handle().cast() } }
impl    AsLocalHandle for std::process::ChildStdin      { fn as_handle(&self) -> HANDLE { self.as_raw_handle().cast() } }
impl    AsLocalHandle for std::process::ChildStdout     { fn as_handle(&self) -> HANDLE { self.as_raw_handle().cast() } }

#[cfg(test)] mod tests {
    use crate::prelude::*;
    #[cfg(std)] use std::os::windows::io::*;

    #[cfg(std)]
    #[test] #[should_panic = "undefined behavior"] fn null_std_fs_file_to_owned() {
        let null = unsafe { std::fs::File::from_raw_handle(null_mut()) }; // arguably u.b.
        let _panic = handle::Owned::from(null); // u.b. detected
    }

    #[cfg(std)]
    #[test] #[should_panic = "undefined behavior"] fn null_std_fs_file_to_borrowed() {
        let null = unsafe { std::fs::File::from_raw_handle(null_mut()) }; // arguably u.b.
        let _panic = handle::Borrowed::from(&null); // u.b. detected
    }

    #[cfg(std)]
    #[test] #[should_panic = "undefined behavior"] fn null_std_fs_file_to_pseudo() {
        let null = unsafe { std::fs::File::from_raw_handle(null_mut()) }; // arguably u.b.
        let _panic = handle::Pseudo::from(&null); // u.b. detected
    }
}
