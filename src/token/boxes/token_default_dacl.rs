use crate::*;

use winapi::um::winnt::TOKEN_DEFAULT_DACL;

use std::fmt::{self, Debug, Formatter};
use std::mem::{size_of, align_of};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_default_dacl)\]
/// ~ `Box<(TOKEN_DEFAULT_DACL, ..)>`
pub struct BoxTokenDefaultDacl(Box<[u8]>);

impl BoxTokenDefaultDacl {
    pub unsafe fn from_raw(bytes: Box<[u8]>) -> Self {
        assert!(bytes.len() >= size_of::<TOKEN_DEFAULT_DACL>());
        assert!(bytes.as_ptr() as usize % align_of::<TOKEN_DEFAULT_DACL>() == 0);
        Self(bytes)
    }

    pub fn default_dacl<'s>(&'s self) -> acl::Ptr<'s> {
        unsafe { acl::Ptr::from_raw_unchecked((*(self.0.as_ptr() as *const TOKEN_DEFAULT_DACL)).DefaultDacl) }
    }
}

impl Debug for BoxTokenDefaultDacl {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("BoxTokenDefaultDacl").field("default_dacl", &self.default_dacl()).finish()
    }
}
