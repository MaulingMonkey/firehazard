//! [`std::os::windows::prelude`] re-export or [`no_std`](https://docs.rust-embedded.org/book/intro/no-std.html) placeholders

//b use super::ffi::OsStrExt;
//b use super::ffi::OsStringExt;
//b use super::fs::FileExt;
//b use super::fs::MetadataExt;
//b use super::fs::OpenOptionsExt;
pub use super::io::AsHandle;
//b use super::io::AsSocket;
pub use super::io::BorrowedHandle;
//b use super::io::BorrowedSocket;
pub use super::io::FromRawHandle;
//b use super::io::FromRawSocket;
pub use super::io::HandleOrInvalid;
pub use super::io::IntoRawHandle;
//b use super::io::IntoRawSocket;
pub use super::io::OwnedHandle;
//b use super::io::OwnedSocket;
pub use super::io::AsRawHandle;
//b use super::io::AsRawSocket;
pub use super::io::RawHandle;
//b use super::io::RawSocket;
