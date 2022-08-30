use crate::*;

use core::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/secauthz/access-rights-for-access-token-objects)\]
/// DWORD/[u32]: Access rights mask for removing or restricting access rights to Access-Token objects
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct AccessRightsMask(u32);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/secauthz/access-rights-for-access-token-objects)\]
/// DWORD/[u32]: Access rights for Access-Token objects
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct AccessRights(u32);

flags!(impl .. for AccessRights(u32) - AccessRightsMask);

impl AccessRights {
    /// ### Safety
    /// *   Some APIs might theoretically assume access rights are a valid?
    pub const unsafe fn from_unchecked(rights: u32) -> Self { Self(rights) }
}

impl Debug for AccessRights {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        use winapi::um::winnt::*;
        flags!(self.0, fmt, "0x{:04X}", [
            TOKEN_ALL_ACCESS,
            TOKEN_ALL_ACCESS_P,
            TOKEN_READ,
            TOKEN_WRITE,
            TOKEN_EXECUTE,
            TOKEN_ACCESS_PSEUDO_HANDLE,

            TOKEN_ASSIGN_PRIMARY,
            TOKEN_DUPLICATE,
            TOKEN_IMPERSONATE,
            TOKEN_QUERY,
            TOKEN_QUERY_SOURCE,
            TOKEN_ADJUST_PRIVILEGES,
            TOKEN_ADJUST_GROUPS,
            TOKEN_ADJUST_DEFAULT,
            TOKEN_ADJUST_SESSIONID,

            DELETE,
            READ_CONTROL,
            WRITE_DAC,
            WRITE_OWNER,
            SYNCHRONIZE, // XXX: not supported: https://docs.microsoft.com/en-us/windows/win32/secauthz/access-rights-for-access-token-objects
        ])
    }
}

pub const ALL_ACCESS            : AccessRights = AccessRights(winapi::um::winnt::TOKEN_ALL_ACCESS);
pub const ALL_ACCESS_P          : AccessRights = AccessRights(winapi::um::winnt::TOKEN_ALL_ACCESS_P);
pub const READ                  : AccessRights = AccessRights(winapi::um::winnt::TOKEN_READ);
pub const WRITE                 : AccessRights = AccessRights(winapi::um::winnt::TOKEN_WRITE);
pub const EXECUTE               : AccessRights = AccessRights(winapi::um::winnt::TOKEN_EXECUTE);
pub const TRUST_CONSTRAINT_MASK : AccessRightsMask = AccessRightsMask(winapi::um::winnt::TOKEN_TRUST_CONSTRAINT_MASK);
pub const ACCESS_PSEUDO_HANDLE  : AccessRights = AccessRights(winapi::um::winnt::TOKEN_ACCESS_PSEUDO_HANDLE);

pub const ASSIGN_PRIMARY        : AccessRights = AccessRights(winapi::um::winnt::TOKEN_ASSIGN_PRIMARY);
pub const DUPLICATE             : AccessRights = AccessRights(winapi::um::winnt::TOKEN_DUPLICATE);
pub const IMPERSONATE           : AccessRights = AccessRights(winapi::um::winnt::TOKEN_IMPERSONATE);
pub const QUERY                 : AccessRights = AccessRights(winapi::um::winnt::TOKEN_QUERY);
pub const QUERY_SOURCE          : AccessRights = AccessRights(winapi::um::winnt::TOKEN_QUERY_SOURCE);
pub const ADJUST_PRIVILEGES     : AccessRights = AccessRights(winapi::um::winnt::TOKEN_ADJUST_PRIVILEGES);
pub const ADJUST_GROUPS         : AccessRights = AccessRights(winapi::um::winnt::TOKEN_ADJUST_GROUPS);
pub const ADJUST_DEFAULT        : AccessRights = AccessRights(winapi::um::winnt::TOKEN_ADJUST_DEFAULT);
pub const ADJUST_SESSIONID      : AccessRights = AccessRights(winapi::um::winnt::TOKEN_ADJUST_SESSIONID);
