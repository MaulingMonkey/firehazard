use super::assert_valid_saa;

use crate::*;
use crate::alloc::*;

use winapi::um::winnt::TOKEN_USER;

use core::fmt::{self, Debug, Formatter};



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_user)\]
/// ≈ `Box<(TOKEN_USER, ..)>`
///
#[repr(transparent)] pub struct BoxTokenUser(CBox<TOKEN_USER>);

impl BoxTokenUser {
    pub fn from_raw(cbs: CBoxSized<TOKEN_USER>) -> Self {
        assert_valid_saa(&cbs, cbs.User); // REQUIRED FOR SOUNDNESS
        Self(cbs.into())
    }

    /// User
    pub fn user<'s>(&'s self) -> &'s sid::AndAttributes<'s> {
        unsafe { &*(&self.0.User as *const _ as *const sid::AndAttributes) }
    }
}

impl Debug for BoxTokenUser {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("BoxTokenUser").field("user", self.user()).finish()
    }
}
