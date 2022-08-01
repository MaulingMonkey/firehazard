use winapi::um::winnt::LUID_AND_ATTRIBUTES;

use std::fmt::{self, Debug, Formatter};
use std::mem::{align_of, size_of};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_privileges)\] LUID_AND_ATTRIBUTES, in the context of TOKEN_PRIVILEGES specifically
pub type PrivilegeLuidAndAttributes = LuidAndAttributes<crate::PrivilegeLuid>;

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-luid_and_attributes)\] LUID_AND_ATTRIBUTES
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)] #[repr(C)] pub struct LuidAndAttributes<Luid> {
    pub luid:       Luid,
    pub attributes: u32,
}

impl<Luid: Debug> Debug for LuidAndAttributes<Luid> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        // TODO: name attributes via https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_privileges
        write!(fmt, "LuidAndAttributes {{ luid: {:?}, attributes: 0x{:08x} }}", self.luid, self.attributes)
    }
}

const _LUID_AND_ATTRIBUTES_SIZE  : () = assert!(align_of::<LUID_AND_ATTRIBUTES>() == align_of::<LuidAndAttributes<crate::Luid>>());
const _LUID_AND_ATTRIBUTES_ALIGN : () = assert!(size_of ::<LUID_AND_ATTRIBUTES>() == size_of ::<LuidAndAttributes<crate::Luid>>());
