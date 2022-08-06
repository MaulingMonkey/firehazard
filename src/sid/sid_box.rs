use crate::*;

use winapi::um::winnt::SID;

use std::fmt::{self, Debug, Formatter};
use std::marker::PhantomData;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-sid)\] ~ Box<(SID, ???), D>
#[repr(transparent)] pub struct Box<D: alloc::SidDeallocator>(*mut SID, PhantomData<D>);

impl<D: alloc::SidDeallocator> Drop for Box<D> {
    fn drop(&mut self) {
        unsafe { D::free(self.0) }
    }
}

impl<D: alloc::SidDeallocator> Debug for Box<D> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.as_sid_ptr(), fmt)
    }
}

impl<'s, D: alloc::SidDeallocator> From<&'s Box<D>> for sid::Ptr<'s> {
    fn from(sid: &'s Box<D>) -> Self {
        sid.as_sid_ptr()
    }
}

impl<D: alloc::SidDeallocator> Box<D> {
    /// ### Safety
    /// *   `sid` should be a valid [`LocalAlloc`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localalloc)ed buffer containing a valid [`SID`](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-sid).
    /// *   `sid` should not be [`LocalFree`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localfree)ed by anything else as [`Box::from_raw_unchecked`] takes ownership.
    /// *   As an exception to the above, `sid` may be null if you do nothing but drop the resulting [`Box`]
    pub unsafe fn from_raw_unchecked(sid: *mut SID) -> Self { Self(sid, PhantomData) }

    /// ### Safety
    /// *   `sid` should be a valid [`LocalAlloc`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localalloc)ed buffer containing a valid [`SID`](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-sid).
    /// *   `sid` should not be [`LocalFree`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localfree)ed by anything else as [`Box::from_raw_unchecked`] takes ownership.
    /// *   As an exception to the above, `sid` may be null
    pub unsafe fn from_raw(sid: *mut SID) -> Option<Self> {
        if sid.is_null() { None } else { Some(Self(sid, PhantomData)) }
    }

    pub fn as_sid_ptr(&self) -> sid::Ptr { unsafe { sid::Ptr::from_raw_unchecked(self.0) } }
}
