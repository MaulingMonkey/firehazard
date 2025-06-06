//! \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-acl)\]
//! [`ACL`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-acl)
//! related types and manipulation functions

mod acl_builder;                        pub use acl_builder::*;
mod acl_in_default_or_null_or_ref;      pub use acl_in_default_or_null_or_ref::*;
mod acl_ptr;                            pub use acl_ptr::*;
mod acl_revision;                       pub use acl_revision::*;
