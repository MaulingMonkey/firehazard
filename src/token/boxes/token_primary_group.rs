use crate::*;
use crate::alloc::*;

use winapi::um::winnt::TOKEN_PRIMARY_GROUP;

use core::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_primary_group)\] ~ `Box<(TOKEN_PRIMARY_GROUP, ..)>`
pub struct BoxTokenPrimaryGroup(CBox<TOKEN_PRIMARY_GROUP>);

impl BoxTokenPrimaryGroup {
    pub unsafe fn from_raw(cbs: CBoxSized<TOKEN_PRIMARY_GROUP>) -> Self {
        // TODO: validate
        Self(cbs.into())
    }

    pub fn primary_group<'s>(&'s self) -> &'s sid::Ptr<'s> {
        unsafe { &*(self.0.as_ptr() as *const sid::Ptr) }
    }
}

impl Debug for BoxTokenPrimaryGroup {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("BoxTokenPrimaryGroup").field("primary_group", self.primary_group()).finish()
    }
}
