use crate::*;
use crate::alloc::*;

use winapi::um::winnt::TOKEN_MANDATORY_LABEL;

use core::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_mandatory_label)\] ~ `Box<(TOKEN_MANDATORY_LABEL, ..)>`
pub struct BoxTokenMandatoryLabel(CBox<TOKEN_MANDATORY_LABEL>);

impl BoxTokenMandatoryLabel {
    pub unsafe fn from_raw(cbs: CBoxSized<TOKEN_MANDATORY_LABEL>) -> Self {
        // TODO: validate
        Self(cbs.into())
    }

    pub fn as_winapi(&self) -> *mut TOKEN_MANDATORY_LABEL { self.0.as_ptr() as *mut _ }

    pub fn label<'s>(&'s self) -> &'s sid::AndAttributes<'s> {
        unsafe { &*(self.0.as_ptr() as *const sid::AndAttributes) }
    }

    pub fn label_mut<'s>(&'s mut self) -> &'s mut sid::AndAttributes<'s> {
        unsafe { &mut *(self.0.as_mut_ptr() as *mut sid::AndAttributes) }
    }
}

impl Debug for BoxTokenMandatoryLabel {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("BoxTokenMandatoryLabel").field("label", self.label()).finish()
    }
}
