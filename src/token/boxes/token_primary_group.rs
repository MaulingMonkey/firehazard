use super::assert_valid_sid;

use crate::prelude::*;
use crate::alloc::*;

use winapi::um::winnt::TOKEN_PRIMARY_GROUP;

use core::fmt::{self, Debug, Formatter};



#[doc(alias = "TOKEN_PRIMARY_GROUP")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_primary_group)\]
/// ≈ `Box<(TOKEN_PRIMARY_GROUP, ..)>`
///
#[repr(transparent)] pub struct BoxTokenPrimaryGroup(CBox<TOKEN_PRIMARY_GROUP>);

impl BoxTokenPrimaryGroup {
    pub fn from_raw(cbs: CBoxSized<TOKEN_PRIMARY_GROUP>) -> Self {
        assert_valid_sid(&cbs, cbs.PrimaryGroup); // REQUIRED FOR SOUNDNESS
        Self(cbs.into())
    }

    /// PrimaryGroup
    pub fn primary_group<'s>(&'s self) -> sid::Ptr<'s> {
        unsafe { sid::Ptr::from_raw_unchecked(self.0.PrimaryGroup.cast()) }
    }
}

impl Debug for BoxTokenPrimaryGroup {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("BoxTokenPrimaryGroup").field("primary_group", &self.primary_group()).finish()
    }
}
