use crate::*;
use winapi::um::winnt::LUID_AND_ATTRIBUTES;
use core::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-luid_and_attributes)\] LUID_AND_ATTRIBUTES
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)] pub struct LuidAndAttributes<Luid> {
    pub luid:       Luid,
    pub attributes: u32,
}

structure!(@assert layout LuidAndAttributes<Luid> => LUID_AND_ATTRIBUTES {
    luid        == Luid,
    attributes  == Attributes,
});

impl<Luid> LuidAndAttributes<Luid> {
    pub fn new(luid: impl Into<Luid>, attributes: u32) -> Self {
        Self { luid: luid.into(), attributes }
    }
}

impl<Luid: Debug> Debug for LuidAndAttributes<Luid> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        // TODO: name attributes via https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_privileges
        write!(fmt, "LuidAndAttributes {{ luid: {:?}, attributes: 0x{:08x} }}", self.luid, self.attributes)
    }
}
