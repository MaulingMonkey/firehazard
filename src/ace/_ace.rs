//! \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-ace_header)\]
//! [`ACL`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-acl) /
//! [`ACE_HEADER`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-ace_header)
//! related types and manipulation functions

// https://learn.microsoft.com/en-us/windows/win32/secauthz/ace-strings
// https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-ace_header

mod ace_flags;                      pub use ace_flags::*;
mod ace_header;                     pub use ace_header::*;
mod ace_iter;                       pub use ace_iter::*;
mod ace_ptr;                        pub use ace_ptr::*;
mod ace_type;                       pub use ace_type::*;
