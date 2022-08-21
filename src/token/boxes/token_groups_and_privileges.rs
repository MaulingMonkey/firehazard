use crate::*;
use crate::alloc::*;

use winapi::um::winnt::TOKEN_GROUPS_AND_PRIVILEGES;

use core::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_groups_and_privileges)\] ~ `Box<(TOKEN_GROUPS_AND_PRIVILEGES, ..)>`
pub struct BoxTokenGroupsAndPrivileges(CBox<TOKEN_GROUPS_AND_PRIVILEGES>);

impl BoxTokenGroupsAndPrivileges {
    pub unsafe fn from_raw(cbs: CBoxSized<TOKEN_GROUPS_AND_PRIVILEGES>) -> Self {
        // TODO: validate fields
        Self(cbs.into())
    }

    /// Sids+0 .. Sids+SidCount
    pub fn sids(&self) -> &[sid::AndAttributes] { unsafe { core::slice::from_raw_parts(self.header().Sids.cast(), usize::from32(self.header().SidCount)) } }

    /// RestrictedSids+0 .. RestrictedSids+RestrictedSidCount
    pub fn restricted_sids(&self) -> &[sid::AndAttributes] { unsafe { core::slice::from_raw_parts(self.header().RestrictedSids.cast(), usize::from32(self.header().RestrictedSidCount)) } }

    /// Privileges+0 .. Privileges+PrivilegeCount
    pub fn privileges(&self) -> &[privilege::LuidAndAttributes] { unsafe { core::slice::from_raw_parts(self.header().Privileges.cast(), usize::from32(self.header().PrivilegeCount)) } }

    /// AuthenticationIds
    pub fn authentication_id(&self) -> Luid { self.header().AuthenticationId.into() }

    fn header(&self) -> &TOKEN_GROUPS_AND_PRIVILEGES { unsafe { &*self.0.as_ptr().cast() } }
}

impl Debug for BoxTokenGroupsAndPrivileges {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("BoxTokenGroupsAndPrivileges")
            .field("sids",              &self.sids()                )
            .field("restricted_sids",   &self.restricted_sids()     )
            .field("privileges",        &self.privileges()          )
            .field("authentication_id", &self.authentication_id()   )
            .finish()
    }
}
