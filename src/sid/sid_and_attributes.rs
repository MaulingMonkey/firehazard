use crate::*;

use winapi::um::winnt::SID_AND_ATTRIBUTES;

use core::fmt::{self, Debug, Formatter};
use core::mem::{align_of, size_of};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-sid_and_attributes)\] ~ SID_AND_ATTRIBUTES ~ (sid::Ptr, u32)
#[derive(Clone, Copy)]
#[repr(C)] pub struct AndAttributes<'a> {
    pub sid:        sid::Ptr<'a>,
    pub attributes: u32,
}
const _ : () = assert!(align_of::<SID_AND_ATTRIBUTES>() == align_of::<sid::AndAttributes>());
const _ : () = assert!(size_of ::<SID_AND_ATTRIBUTES>() == size_of ::<sid::AndAttributes>());

impl<'a> sid::AndAttributes<'a> {
    pub fn new(sid: impl Into<sid::Ptr<'a>>, attributes: u32) -> Self {
        Self { sid: sid.into(), attributes }
    }
}

// safe wrapper type -> unsafe raw winapi type (1-way)
impl<'a> AsRef<SID_AND_ATTRIBUTES> for sid::AndAttributes<'a> { fn as_ref(&self) -> &SID_AND_ATTRIBUTES { unsafe { core::mem::transmute(self) } } }
impl<'a> From<sid::AndAttributes<'a>> for SID_AND_ATTRIBUTES { fn from(ts: sid::AndAttributes<'a>) -> Self { *ts.as_ref() } }

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
