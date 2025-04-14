use super::assert_valid_saa;

use crate::*;
use crate::alloc::*;

use winapi::um::winnt::TOKEN_MANDATORY_LABEL;

use core::fmt::{self, Debug, Formatter};



#[doc(alias = "TOKEN_MANDATORY_LABEL")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_mandatory_label)\]
/// ≈ `Box<(TOKEN_MANDATORY_LABEL, ..)>`
///
#[repr(transparent)] pub struct BoxTokenMandatoryLabel(CBox<TOKEN_MANDATORY_LABEL>);

impl BoxTokenMandatoryLabel {
    pub fn from_raw(cbs: CBoxSized<TOKEN_MANDATORY_LABEL>) -> Self {
        assert_valid_saa(&cbs, cbs.Label); // REQUIRED FOR SOUNDNESS
        Self(cbs.into())
    }

    pub fn as_winapi(&self) -> *mut TOKEN_MANDATORY_LABEL { self.0.as_ptr() as *mut _ }

    pub fn label<'s>(&'s self) -> &'s sid::AndAttributes<'s> {
        unsafe { &*(&self.0.Label as *const _ as *const sid::AndAttributes) }
    }

    pub fn label_mut<'s>(&'s mut self) -> &'s mut sid::AndAttributes<'s> {
        unsafe { &mut *(&mut self.0.Label as *mut _ as *mut sid::AndAttributes) }
    }
}

impl Debug for BoxTokenMandatoryLabel {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("BoxTokenMandatoryLabel").field("label", self.label()).finish()
    }
}
