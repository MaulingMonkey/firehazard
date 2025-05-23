use crate::prelude::*;
use crate::alloc::*;
use crate::token::boxes::boxes_util::assert_valid_ptr_bytes;

use winapi::um::winnt::TOKEN_DEFAULT_DACL;

use core::fmt::{self, Debug, Formatter};



#[doc(alias = "TOKEN_DEFAULT_DACL")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_default_dacl)\]
/// ≈ `Box<(TOKEN_DEFAULT_DACL, ..)>`
///
#[repr(transparent)] pub struct BoxTokenDefaultDacl(CBox<TOKEN_DEFAULT_DACL>);

impl BoxTokenDefaultDacl {
    pub unsafe fn from_raw(cbs: CBoxSized<TOKEN_DEFAULT_DACL>) -> Self {
        NonNull::new(cbs.DefaultDacl).map(|dacl| {
            let acl_bytes = assert_valid_ptr_bytes(&cbs, cbs.DefaultDacl, 1);
            unsafe { acl::Ref::from_raw(dacl, acl_bytes) }.unwrap(); // this isn't 100% safe yet
        });
        Self(cbs.into())
    }

    /// DefaultDacl
    pub fn default_dacl<'s>(&'s self) -> Option<acl::Ref<'s>> {
        NonNull::new(self.0.DefaultDacl).map(|dacl| unsafe { acl::Ref::from_raw_unchecked(dacl) })
    }
}

impl Debug for BoxTokenDefaultDacl {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("BoxTokenDefaultDacl").field("default_dacl", &self.default_dacl()).finish()
    }
}



#[test] fn null_default_dacl() {
    let t = open_process_token(get_current_process(), token::ALL_ACCESS).unwrap();
    let t = create_restricted_token(&t, None, None, None, None).unwrap();
    assert!(t.default_dacl().unwrap().default_dacl().is_some());
    t.set_default_dacl(acl::Null).unwrap();
    assert!(t.default_dacl().unwrap().default_dacl().is_none());
}
