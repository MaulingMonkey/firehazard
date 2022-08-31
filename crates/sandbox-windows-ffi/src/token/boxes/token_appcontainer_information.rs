use super::assert_valid_sid;
use crate::*;
use crate::alloc::*;

use winapi::um::winnt::TOKEN_APPCONTAINER_INFORMATION;

use core::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_appcontainer_information)\] ~ `Box<(TOKEN_APPCONTAINER_INFORMATION, ..)>`
pub struct BoxTokenAppcontainerInformation(CBox<TOKEN_APPCONTAINER_INFORMATION>);

impl BoxTokenAppcontainerInformation {
    pub fn from_raw(cbs: CBoxSized<TOKEN_APPCONTAINER_INFORMATION>) -> Self {
        assert_valid_sid(&cbs, cbs.TokenAppContainer); // REQUIRED FOR SOUNDNESS
        Self(cbs.into())
    }

    /// TokenAppContainer
    pub fn token_app_container<'s>(&'s self) -> sid::Ptr<'s> {
        unsafe { sid::Ptr::from_raw_unchecked(self.0.TokenAppContainer.cast()) }
    }
}

impl Debug for BoxTokenAppcontainerInformation {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("BoxTokenAppcontainerInformation").field("token_app_container", &self.token_app_container()).finish()
    }
}
