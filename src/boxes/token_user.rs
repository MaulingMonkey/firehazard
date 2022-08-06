use crate::*;

use std::fmt::{self, Debug, Formatter};
use std::mem::{size_of, align_of};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_user)\] ~ `Box<(TOKEN_USER, ..)>`
pub struct BoxTokenUser(Box<[u8]>);

impl BoxTokenUser {
    pub unsafe fn from_raw(bytes: Box<[u8]>) -> Self {
        assert!(bytes.len() >= size_of::<sid::AndAttributes>());
        assert!(bytes.as_ptr() as usize % align_of::<sid::AndAttributes>() == 0);
        Self(bytes)
    }

    pub fn user<'s>(&'s self) -> &'s sid::AndAttributes<'s> {
        unsafe { &*(self.0.as_ptr() as *const sid::AndAttributes) }
    }
}

impl Debug for BoxTokenUser {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("BoxTokenUser").field("user", self.user()).finish()
    }
}
