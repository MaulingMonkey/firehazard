#![cfg(std)]

use crate::prelude::*;
use crate::io::FileNN;
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
const EXPECT_NONNULL_STD_FILE : &'static str = r#"undefined behavior: `std::fs::File::from_raw_handle(core::ptr::null_mut())` was presumably called earlier - but null is not an open, owned handle, as required per `std::os::windows::io::FromRawHandle::from_raw_handle`'s "Safety" docs."#;

/// XXX: FileNN currently assumes the handle was created without using `FILE_FLAG_OVERLAPPED`.
/// However, std::fs::File may wrap a handle that *was* created using `FILE_FLAG_OVERLAPPED`.
/// See https://github.com/rust-lang/rust/issues/81357 for notes on the required workarounds for soundness.
#[cfg(nope)] // See also disabled unit test bellow
impl     From<    std::fs::File> for FileNN                 { fn from(file:     std::fs::File) -> Self { unsafe { Self::from_raw(file.into_raw_handle()  .cast()) }.expect(EXPECT_NONNULL_STD_FILE) } }
// yep:
impl     From<    std::fs::File> for handle::Owned          { fn from(file:     std::fs::File) -> Self { unsafe { Self::from_raw(file.into_raw_handle()  .cast()) }.expect(EXPECT_NONNULL_STD_FILE) } }
impl<'a> From<&'a std::fs::File> for handle::Borrowed<'a>   { fn from(file: &'a std::fs::File) -> Self { unsafe { Self::from_raw(file.as_raw_handle()    .cast()) }.expect(EXPECT_NONNULL_STD_FILE) } }
impl<'a> From<&'a std::fs::File> for handle::Pseudo<'a>     { fn from(file: &'a std::fs::File) -> Self { unsafe { Self::from_raw(file.as_raw_handle()    .cast()) }.expect(EXPECT_NONNULL_STD_FILE) } }

// XXX: this assumes an NT kernel.  These all identify as `File` handles, but for now I'm being conservative and only converting to basic handles.
impl     From<    std::net::TcpListener> for handle::Owned          { fn from(sock:     std::net::TcpListener   ) -> Self { unsafe { Self::from_raw(sock.into_raw_socket() as _) }.expect(EXPECT_NONNULL_STD_FILE) } }
impl<'a> From<&'a std::net::TcpListener> for handle::Borrowed<'a>   { fn from(sock: &'a std::net::TcpListener   ) -> Self { unsafe { Self::from_raw(sock.as_raw_socket()   as _) }.expect(EXPECT_NONNULL_STD_FILE) } }
impl<'a> From<&'a std::net::TcpListener> for handle::Pseudo<'a>     { fn from(sock: &'a std::net::TcpListener   ) -> Self { unsafe { Self::from_raw(sock.as_raw_socket()   as _) }.expect(EXPECT_NONNULL_STD_FILE) } }

// XXX: this assumes an NT kernel.  These all identify as `File` handles, but for now I'm being conservative and only converting to basic handles.
impl     From<    std::net::TcpStream  > for handle::Owned          { fn from(sock:     std::net::TcpStream     ) -> Self { unsafe { Self::from_raw(sock.into_raw_socket() as _) }.expect(EXPECT_NONNULL_STD_FILE) } }
impl<'a> From<&'a std::net::TcpStream  > for handle::Borrowed<'a>   { fn from(sock: &'a std::net::TcpStream     ) -> Self { unsafe { Self::from_raw(sock.as_raw_socket()   as _) }.expect(EXPECT_NONNULL_STD_FILE) } }
impl<'a> From<&'a std::net::TcpStream  > for handle::Pseudo<'a>     { fn from(sock: &'a std::net::TcpStream     ) -> Self { unsafe { Self::from_raw(sock.as_raw_socket()   as _) }.expect(EXPECT_NONNULL_STD_FILE) } }

// XXX: this assumes an NT kernel.  These all identify as `File` handles, but for now I'm being conservative and only converting to basic handles.
impl     From<    std::net::UdpSocket  > for handle::Owned          { fn from(sock:     std::net::UdpSocket     ) -> Self { unsafe { Self::from_raw(sock.into_raw_socket() as _) }.expect(EXPECT_NONNULL_STD_FILE) } }
impl<'a> From<&'a std::net::UdpSocket  > for handle::Borrowed<'a>   { fn from(sock: &'a std::net::UdpSocket     ) -> Self { unsafe { Self::from_raw(sock.as_raw_socket()   as _) }.expect(EXPECT_NONNULL_STD_FILE) } }
impl<'a> From<&'a std::net::UdpSocket  > for handle::Pseudo<'a>     { fn from(sock: &'a std::net::UdpSocket     ) -> Self { unsafe { Self::from_raw(sock.as_raw_socket()   as _) }.expect(EXPECT_NONNULL_STD_FILE) } }

// FileNN → File should be sound since the former currently forbids `FILE_FLAG_OVERLAPPED`.
impl From<FileNN> for std::fs::File { fn from(file: FileNN) -> Self { unsafe { std::fs::File::from_raw_handle(file.into_handle()) } } }

impl    AsLocalHandle for std::fs::File                 { fn as_handle(&self) -> HANDLE { self.as_raw_handle().cast() } }
impl    AsLocalHandle for std::io::Stderr               { fn as_handle(&self) -> HANDLE { self.as_raw_handle().cast() } }
impl    AsLocalHandle for std::io::Stdin                { fn as_handle(&self) -> HANDLE { self.as_raw_handle().cast() } }
impl    AsLocalHandle for std::io::Stdout               { fn as_handle(&self) -> HANDLE { self.as_raw_handle().cast() } }
impl    AsLocalHandle for std::process::ChildStderr     { fn as_handle(&self) -> HANDLE { self.as_raw_handle().cast() } }
impl    AsLocalHandle for std::process::ChildStdin      { fn as_handle(&self) -> HANDLE { self.as_raw_handle().cast() } }
impl    AsLocalHandle for std::process::ChildStdout     { fn as_handle(&self) -> HANDLE { self.as_raw_handle().cast() } }

#[cfg(test)] mod tests {
    use crate::prelude::*;
    #[cfg(std)] use std::os::windows::io::*;

    #[cfg(nope)] // File → FileNN currently disabled
    #[test] #[should_panic = "undefined behavior"] fn null_std_fs_file_to_filenn() {
        let null = unsafe { std::fs::File::from_raw_handle(null_mut()) }; // arguably u.b.
        let _panic = io::FileNN::from(null); // u.b. detected
    }

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
