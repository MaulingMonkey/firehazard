use crate::refs::SidAndAttributes;

use std::fmt::{self, Debug, Formatter};
use std::mem::{size_of, align_of};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_mandatory_label)\] ~ `Box<(TOKEN_MANDATORY_LABEL, ..)>`
pub struct BoxTokenMandatoryLabel(Box<[u8]>);

impl BoxTokenMandatoryLabel {
    pub unsafe fn from_raw(bytes: Box<[u8]>) -> Self {
        assert!(bytes.len() >= size_of::<SidAndAttributes>());
        assert!(bytes.as_ptr() as usize % align_of::<SidAndAttributes>() == 0);
        Self(bytes)
    }

    pub fn label<'s>(&'s self) -> &'s SidAndAttributes<'s> {
        unsafe { &*(self.0.as_ptr() as *const SidAndAttributes) }
    }
}

impl Debug for BoxTokenMandatoryLabel {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("BoxTokenMandatoryLabel").field("label", self.label()).finish()
    }
}
