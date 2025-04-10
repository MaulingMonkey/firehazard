use crate::*;
use crate::alloc::*;
use crate::token::boxes::boxes_util::assert_valid_ptr_bytes;

use winapi::um::winnt::TOKEN_DEFAULT_DACL;

use core::fmt::{self, Debug, Formatter};



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_default_dacl)\]
/// ≈ `Box<(TOKEN_DEFAULT_DACL, ..)>`
///
#[repr(transparent)] pub struct BoxTokenDefaultDacl(CBox<TOKEN_DEFAULT_DACL>);

impl BoxTokenDefaultDacl {
    pub unsafe fn from_raw(cbs: CBoxSized<TOKEN_DEFAULT_DACL>) -> Self {
        let acl_bytes = assert_valid_ptr_bytes(&cbs, cbs.DefaultDacl, 1);
        unsafe { acl::Ptr::from_raw(cbs.DefaultDacl, acl_bytes) }.unwrap(); // this isn't 100% safe yet
        Self(cbs.into())
    }

    /// DefaultDacl
    pub fn default_dacl<'s>(&'s self) -> acl::Ptr<'s> {
        unsafe { acl::Ptr::from_raw_unchecked(self.0.DefaultDacl) }
    }
}

impl Debug for BoxTokenDefaultDacl {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("BoxTokenDefaultDacl").field("default_dacl", &self.default_dacl()).finish()
    }
}
