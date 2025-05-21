use crate::prelude::*;

use winapi::shared::minwindef::{BYTE, DWORD};
use winapi::um::winnt::{SID, SID_IDENTIFIER_AUTHORITY};

use core::fmt::{self, Debug, Formatter};
use core::ops::Deref;



#[doc(alias = "SID")]
#[doc(alias = "PSID")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-sid)\]
/// ≈ PSID
///
#[derive(Clone, Copy)] #[repr(transparent)] pub struct Ptr<'a>(*mut SID, PhantomData<&'a SID>);

unsafe impl Send for Ptr<'_> {}
unsafe impl Sync for Ptr<'_> {}

impl Debug for Ptr<'_> { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Debug::fmt(&**self, fmt) } }
impl Deref for Ptr<'_> { type Target = sid::Value; fn deref(&self) -> &Self::Target { unsafe { core::mem::transmute(self) } } }

impl Ptr<'_> {
    /// ### Safety
    /// `sid` should be null, or point to a valid [`SID`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-sid)
    /// for the lifetime of the [`sid::Ptr`].
    pub const unsafe fn from_raw_unchecked(sid: *mut SID) -> Self { Self(sid, PhantomData) }

    /// ### Safety
    /// `sid` should be null, or point to a valid [`SID`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-sid)
    /// for the lifetime of the [`sid::Ptr`].
    pub unsafe fn from_raw(sid: *mut SID, bytes: usize) -> firehazard::Result<Self> {
        if sid.is_null() { return Ok(Self(sid, PhantomData)) }
        if bytes < size_of::<Header>() { return Err(Error(ERROR_INVALID_SID)) }
        let header : Header = unsafe { core::ptr::read(sid.cast()) };
        if header.SubAuthorityCount > 15 { return Err(Error(ERROR_INVALID_SUB_AUTHORITY)) }
        let expected_size = size_of::<Header>() + size_of::<DWORD>() * usize::from(header.SubAuthorityCount);
        if bytes < expected_size { return Err(Error(ERROR_INVALID_SID)) }
        Ok(Self(sid, PhantomData))
    }
}



#[doc(alias = "SID")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-sid)\]
/// SID, but without the trailing variable length SubAuthority field
///
#[allow(non_snake_case)]
#[derive(Clone, Copy)]
#[repr(C)] struct Header {
    Revision:               BYTE,
    SubAuthorityCount:      BYTE,
    IdentifierAuthority:    SID_IDENTIFIER_AUTHORITY,
    //SubAuthority:         [DWORD; 0 ..= 15], // varlen based on SubAuthorityCount
}
