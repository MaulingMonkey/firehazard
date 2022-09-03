use super::*;

use crate::*;
use crate::alloc::*;

use winapi::um::winnt::TOKEN_GROUPS_AND_PRIVILEGES;

use core::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_groups_and_privileges)\] ~ `Box<(TOKEN_GROUPS_AND_PRIVILEGES, ..)>`
#[repr(transparent)] pub struct BoxTokenGroupsAndPrivileges(CBox<TOKEN_GROUPS_AND_PRIVILEGES>);

impl BoxTokenGroupsAndPrivileges {
    pub fn from_raw(cbs: CBoxSized<TOKEN_GROUPS_AND_PRIVILEGES>) -> Self {
        let sids = unsafe { assert_valid_after_header_slice(&cbs, cbs.Sids, cbs.SidCount, false) };
        for sid in sids { assert_valid_saa(&cbs, *sid) }
        let sids = unsafe { assert_valid_after_header_slice(&cbs, cbs.RestrictedSids, cbs.RestrictedSidCount, false) };
        for sid in sids { assert_valid_saa(&cbs, *sid) }
        let privs = unsafe { assert_valid_after_header_slice(&cbs, cbs.Privileges, cbs.PrivilegeCount, false) };
        let _ = privs; // all bit patterns valid
        Self(cbs.into())
    }

    /// Sids[.. SidCount]
    pub fn sids    (&    self) -> &    [sid::AndAttributes] { unsafe { core::slice::from_raw_parts    (self.header().Sids.cast(), usize::from32(self.header().SidCount)) } }
    /// Sids[.. SidCount]
    pub fn sids_mut(&mut self) -> &mut [sid::AndAttributes] { unsafe { core::slice::from_raw_parts_mut(self.header().Sids.cast(), usize::from32(self.header().SidCount)) } }

    /// RestrictedSids[.. RestrictedSidCount]
    pub fn restricted_sids    (&    self) -> &    [sid::AndAttributes] { unsafe { core::slice::from_raw_parts    (self.header().RestrictedSids.cast(), usize::from32(self.header().RestrictedSidCount)) } }
    /// RestrictedSids[.. RestrictedSidCount]
    pub fn restricted_sids_mut(&mut self) -> &mut [sid::AndAttributes] { unsafe { core::slice::from_raw_parts_mut(self.header().RestrictedSids.cast(), usize::from32(self.header().RestrictedSidCount)) } }

    /// Privileges[.. PrivilegeCount]
    pub fn privileges    (&    self) -> &    [privilege::LuidAndAttributes] { unsafe { core::slice::from_raw_parts    (self.header().Privileges.cast(), usize::from32(self.header().PrivilegeCount)) } }
    /// Privileges[.. PrivilegeCount]
    pub fn privileges_mut(&mut self) -> &mut [privilege::LuidAndAttributes] { unsafe { core::slice::from_raw_parts_mut(self.header().Privileges.cast(), usize::from32(self.header().PrivilegeCount)) } }

    /// AuthenticationId
    pub fn authentication_id(&self) -> Luid { self.header().AuthenticationId.into() }
    /// AuthenticationId
    pub fn set_authentication_id(&mut self, luid: impl Into<Luid>) { self.header_mut().AuthenticationId = luid.into().into(); }

    fn header    (&    self) -> &    TOKEN_GROUPS_AND_PRIVILEGES { &    *self.0 }
    fn header_mut(&mut self) -> &mut TOKEN_GROUPS_AND_PRIVILEGES { &mut *self.0 }
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
