use crate::*;
use winapi::um::winnt::ACE_HEADER;
use core::mem::{align_of, size_of};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-ace_header)\]
/// ~ ACE_HEADER
///
/// ### ABI Differences
///
/// A raw `ACE_HEADER` has only 2-byte alignment - however, actual ACEs (and this [`ace::Header`] type) have 4-byte alignment:
/// > Each ACL and ACE structure begins on a DWORD boundary.
/// >
/// > <https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-acl>
#[derive(Clone, Copy, Debug)] #[repr(C, align(4))] pub struct Header {
    pub ty:     ace::Type,
    pub flags:  ace::Flags,
    pub size:   u16,
}
const _ : () = assert!(align_of::<ACE_HEADER>() <= align_of::<ace::Header>());
const _ : () = assert!(size_of ::<ACE_HEADER>() == size_of ::<ace::Header>());
