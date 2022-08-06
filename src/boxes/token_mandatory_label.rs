use crate::refs::SidAndAttributes;

use winapi::um::winnt::TOKEN_MANDATORY_LABEL;

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

    pub fn as_winapi(&self) -> *mut TOKEN_MANDATORY_LABEL { self.0.as_ptr() as *mut _ }

    pub fn label<'s>(&'s self) -> &'s SidAndAttributes<'s> {
        unsafe { &*(self.0.as_ptr() as *const SidAndAttributes) }
    }

    pub fn label_mut<'s>(&'s mut self) -> &'s mut SidAndAttributes<'s> {
        unsafe { &mut *(self.0.as_mut_ptr() as *mut SidAndAttributes) }
    }
}

impl Debug for BoxTokenMandatoryLabel {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("BoxTokenMandatoryLabel").field("label", self.label()).finish()
    }
}
