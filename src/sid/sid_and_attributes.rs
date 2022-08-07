use crate::*;

use winapi::um::winnt::SID_AND_ATTRIBUTES;

use std::fmt::{self, Debug, Formatter};
use std::mem::{align_of, size_of};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-sid_and_attributes)\] ~ SID_AND_ATTRIBUTES ~ (sid::Ptr, u32)
#[derive(Clone, Copy)] #[repr(C)] pub struct AndAttributes<'a> {
    pub sid:        sid::Ptr<'a>,
    pub attributes: u32,
}

impl<'a> sid::AndAttributes<'a> {
    const _ALIGN    : () = assert!(align_of::<SID_AND_ATTRIBUTES>() == align_of::<sid::AndAttributes>());
    const _SIZE     : () = assert!(size_of ::<SID_AND_ATTRIBUTES>() == size_of ::<sid::AndAttributes>());

    pub fn new(sid: impl Into<sid::Ptr<'a>>, attributes: u32) -> Self {
        Self { sid: sid.into(), attributes }
    }
}

impl Debug for sid::AndAttributes<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        struct AsHex(u32);
        impl Debug for AsHex { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "0x{:08x}", self.0) } }
        fmt.debug_struct("sid::AndAttributes")
            .field("sid", &self.sid)
            .field("attributes", &AsHex(self.attributes))
            .finish()
    }
}
