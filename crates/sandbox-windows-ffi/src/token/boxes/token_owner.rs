use crate::*;
use crate::alloc::*;

use winapi::um::winnt::TOKEN_OWNER;

use core::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_owner)\] ~ `Box<(TOKEN_OWNER, ..)>`
pub struct BoxTokenOwner(CBox<TOKEN_OWNER>);

impl BoxTokenOwner {
    pub unsafe fn from_raw(cbs: CBoxSized<TOKEN_OWNER>) -> Self {
        // TODO: validate
        Self(cbs.into())
    }

    /// Owner
    pub fn owner<'s>(&'s self) -> sid::Ptr<'s> {
        unsafe { sid::Ptr::from_raw_unchecked(self.0.Owner.cast()) }
    }
}

impl Debug for BoxTokenOwner {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("BoxTokenOwner").field("owner", &self.owner()).finish()
    }
}