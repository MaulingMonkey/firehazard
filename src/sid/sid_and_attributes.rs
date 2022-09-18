use crate::*;

use winapi::um::winnt::SID_AND_ATTRIBUTES;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-sid_and_attributes)\] ~ SID_AND_ATTRIBUTES ~ (sid::Ptr, u32)
#[derive(Clone, Copy, Debug)]
#[repr(C)] pub struct AndAttributes<'a> {
    pub sid:        sid::Ptr<'a>,
    pub attributes: sid::Attributes,
}

structure!(@assert layout sid::AndAttributes => SID_AND_ATTRIBUTES {
    sid             == Sid,
    attributes      == Attributes,
});

impl<'a> sid::AndAttributes<'a> {
    pub fn new(sid: impl Into<sid::Ptr<'a>>, attributes: impl Into<sid::Attributes>) -> Self {
        Self { sid: sid.into(), attributes: attributes.into() }
    }
}

// safe wrapper type -> unsafe raw winapi type (1-way)
impl<'a> AsRef<SID_AND_ATTRIBUTES> for sid::AndAttributes<'a> { fn as_ref(&self) -> &SID_AND_ATTRIBUTES { unsafe { core::mem::transmute(self) } } }
impl<'a> From<sid::AndAttributes<'a>> for SID_AND_ATTRIBUTES { fn from(ts: sid::AndAttributes<'a>) -> Self { *ts.as_ref() } }
