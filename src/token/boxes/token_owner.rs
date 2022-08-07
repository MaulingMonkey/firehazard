use crate::*;

use std::fmt::{self, Debug, Formatter};
use std::mem::{size_of, align_of};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_owner)\] ~ `Box<(TOKEN_OWNER, ..)>`
pub struct BoxTokenOwner(Box<[u8]>);

impl BoxTokenOwner {
    pub unsafe fn from_raw(bytes: Box<[u8]>) -> Self {
        assert!(bytes.len() >= size_of::<sid::Ptr>());
        assert!(bytes.as_ptr() as usize % align_of::<sid::Ptr>() == 0);
        Self(bytes)
    }

    pub fn owner<'s>(&'s self) -> &'s sid::Ptr<'s> {
        unsafe { &*(self.0.as_ptr() as *const sid::Ptr) }
    }
}

impl Debug for BoxTokenOwner {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("BoxTokenOwner").field("owner", self.owner()).finish()
    }
}
