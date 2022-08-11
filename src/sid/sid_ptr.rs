use crate::*;

use winapi::um::winnt::SID;

use std::fmt::{self, Debug, Formatter};
use std::marker::PhantomData;
use std::ops::Deref;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-sid)\] ~ PSID
#[derive(Clone, Copy)] #[repr(transparent)] pub struct Ptr<'a>(*mut SID, PhantomData<&'a SID>);
// TODO: consider merging Ptr<'a> into Sid by introducing Borrower<'a> ?

impl Debug for Ptr<'_> { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Debug::fmt(&**self, fmt) } }
impl Deref for Ptr<'_> { type Target = sid::Value; fn deref(&self) -> &Self::Target { unsafe { std::mem::transmute(self) } } }

impl Ptr<'_> {
    /// ### Safety
    /// `sid` should be null, or point to a valid [`SID`](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-sid)
    /// for the lifetime of the [`sid::Ptr`].
    pub const unsafe fn from_raw_unchecked(sid: *mut SID) -> Self { Self(sid, PhantomData) }
}
