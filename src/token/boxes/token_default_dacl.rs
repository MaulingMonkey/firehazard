use crate::*;
use crate::alloc::*;

use winapi::um::winnt::TOKEN_DEFAULT_DACL;

use core::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_default_dacl)\]
/// ~ `Box<(TOKEN_DEFAULT_DACL, ..)>`
pub struct BoxTokenDefaultDacl(CBox<TOKEN_DEFAULT_DACL>);

impl BoxTokenDefaultDacl {
    pub unsafe fn from_raw(bytes: CBoxSized<TOKEN_DEFAULT_DACL>) -> Self {
        // TODO: validate acl length
        Self(bytes.into())
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
