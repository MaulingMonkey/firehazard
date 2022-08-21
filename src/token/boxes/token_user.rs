use crate::*;
use crate::alloc::*;

use winapi::um::winnt::TOKEN_USER;

use core::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_user)\] ~ `Box<(TOKEN_USER, ..)>`
pub struct BoxTokenUser(CBox<TOKEN_USER>);

impl BoxTokenUser {
    pub unsafe fn from_raw(cbs: CBoxSized<TOKEN_USER>) -> Self {
        // TODO: validate
        Self(cbs.into())
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
