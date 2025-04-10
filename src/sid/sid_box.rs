use crate::*;

use ialloc::meta::Stateless;
use ialloc::thin::Free;

use winapi::um::winnt::SID;

use core::fmt::{self, Debug, Formatter};
use core::marker::PhantomData;
use core::ops::Deref;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-sid)\]
/// ≈ Box<(SID, ???), A>
///
#[repr(transparent)] pub struct Box<A: Free + Stateless>(*mut SID, PhantomData<A>);

impl<A: Free + Stateless> Debug for Box<A> { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Debug::fmt(&**self, fmt) } }
impl<A: Free + Stateless> Deref for Box<A> { type Target = sid::Value; fn deref(&self) -> &Self::Target { unsafe { core::mem::transmute(self) } } }
impl<A: Free + Stateless> Drop  for Box<A> { fn drop(&mut self) { unsafe { A::default().free_nullable(self.0.cast()) } } }

impl<'s, A: Free + Stateless> From<&'s Box<A>> for sid::Ptr<'s> {
    fn from(sid: &'s Box<A>) -> Self {
        sid.as_sid_ptr()
    }
}

impl<A: Free + Stateless> Box<A> {
    /// ### Safety
    /// *   `sid` should be a valid [`LocalAlloc`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localalloc)ed buffer containing a valid [`SID`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-sid).
    /// *   `sid` should not be [`LocalFree`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localfree)ed by anything else as [`Box::from_raw_unchecked`] takes ownership.
    /// *   As an exception to the above, `sid` may be null if you do nothing but drop the resulting [`Box`]
    pub unsafe fn from_raw_unchecked(sid: *mut SID) -> Self { Self(sid, PhantomData) }

    /// ### Safety
    /// *   `sid` should be a valid [`LocalAlloc`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localalloc)ed buffer containing a valid [`SID`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-sid).
    /// *   `sid` should not be [`LocalFree`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localfree)ed by anything else as [`Box::from_raw_unchecked`] takes ownership.
    /// *   As an exception to the above, `sid` may be null
    pub unsafe fn from_raw(sid: *mut SID) -> Option<Self> { if sid.is_null() { None } else { Some(Self(sid, PhantomData)) } }

    pub fn as_sid_ptr(&self) -> sid::Ptr { unsafe { sid::Ptr::from_raw_unchecked(self.0) } }
}
