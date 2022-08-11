use crate::*;

use winapi::um::winnt::SID;

use std::fmt::{self, Debug, Formatter};
use std::marker::PhantomData;
use std::ops::Deref;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-sid)\] ~ Box<(SID, ???), D>
#[repr(transparent)] pub struct Box<D: alloc::Deallocator>(*mut SID, PhantomData<D>);

impl<D: alloc::Deallocator> Debug for Box<D> { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Debug::fmt(&**self, fmt) } }
impl<D: alloc::Deallocator> Deref for Box<D> { type Target = sid::Value; fn deref(&self) -> &Self::Target { unsafe { std::mem::transmute(self) } } }
impl<D: alloc::Deallocator> Drop  for Box<D> { fn drop(&mut self) { unsafe { D::free(self.0) } } }

impl<'s, D: alloc::Deallocator> From<&'s Box<D>> for sid::Ptr<'s> {
    fn from(sid: &'s Box<D>) -> Self {
        sid.as_sid_ptr()
    }
}

impl<D: alloc::Deallocator> Box<D> {
    /// ### Safety
    /// *   `sid` should be a valid [`LocalAlloc`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localalloc)ed buffer containing a valid [`SID`](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-sid).
    /// *   `sid` should not be [`LocalFree`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localfree)ed by anything else as [`Box::from_raw_unchecked`] takes ownership.
    /// *   As an exception to the above, `sid` may be null if you do nothing but drop the resulting [`Box`]
    pub unsafe fn from_raw_unchecked(sid: *mut SID) -> Self { Self(sid, PhantomData) }

    /// ### Safety
    /// *   `sid` should be a valid [`LocalAlloc`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localalloc)ed buffer containing a valid [`SID`](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-sid).
    /// *   `sid` should not be [`LocalFree`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localfree)ed by anything else as [`Box::from_raw_unchecked`] takes ownership.
    /// *   As an exception to the above, `sid` may be null
    pub unsafe fn from_raw(sid: *mut SID) -> Option<Self> { if sid.is_null() { None } else { Some(Self(sid, PhantomData)) } }

    pub fn as_sid_ptr(&self) -> sid::Ptr { unsafe { sid::Ptr::from_raw_unchecked(self.0) } }
}
