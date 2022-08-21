use winapi::um::winnt::TOKEN_APPCONTAINER_INFORMATION;

use crate::*;
use crate::alloc::*;

use core::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_appcontainer_information)\] ~ `Box<(TOKEN_APPCONTAINER_INFORMATION, ..)>`
pub struct BoxTokenAppcontainerInformation(CBox<TOKEN_APPCONTAINER_INFORMATION>);

impl BoxTokenAppcontainerInformation {
    pub unsafe fn from_raw(cbs: CBoxSized<TOKEN_APPCONTAINER_INFORMATION>) -> Self {
        // TODO: validate sid length
        Self(cbs.into())
    }

    pub fn token_app_container<'s>(&'s self) -> &'s sid::Ptr<'s> {
        unsafe { &*(self.0.as_ptr() as *const sid::Ptr) }
    }
}

impl Debug for BoxTokenAppcontainerInformation {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("BoxTokenAppcontainerInformation").field("token_app_container", self.token_app_container()).finish()
    }
}
