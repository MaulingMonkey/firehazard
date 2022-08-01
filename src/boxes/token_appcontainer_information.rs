use crate::refs::SidPtr;

use std::fmt::{self, Debug, Formatter};
use std::mem::{size_of, align_of};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_appcontainer_information)\] ~ `Box<(TOKEN_APPCONTAINER_INFORMATION, ..)>`
pub struct BoxTokenAppcontainerInformation(Box<[u8]>);

impl BoxTokenAppcontainerInformation {
    pub unsafe fn from_raw(bytes: Box<[u8]>) -> Self {
        assert!(bytes.len() >= size_of::<SidPtr>());
        assert!(bytes.as_ptr() as usize % align_of::<SidPtr>() == 0);
        Self(bytes)
    }

    pub fn token_app_container<'s>(&'s self) -> &'s SidPtr<'s> {
        unsafe { &*(self.0.as_ptr() as *const SidPtr) }
    }
}

impl Debug for BoxTokenAppcontainerInformation {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("BoxTokenAppcontainerInformation").field("token_app_container", self.token_app_container()).finish()
    }
}
