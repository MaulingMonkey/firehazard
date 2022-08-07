use std::fmt::{self, Debug, Formatter};
use std::ops::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/secauthz/access-rights-for-access-token-objects)\] Access rights mask for removing access rights
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct AccessRightsMask(u32);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/secauthz/access-rights-for-access-token-objects)\] Access rights for Access-Token objects
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct AccessRights(u32);

impl AccessRights {
    /// ### Safety
    /// *   Some APIs might theoretically assume access rights are a valid?
    pub const unsafe fn from_unchecked(rights: u32) -> Self { Self(rights) }

    pub fn as_u32(self) -> u32 { self.0 }
}

impl Debug for AccessRights {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        let mut v = self.0;
        if v == 0 { return write!(fmt, "0") }

        macro_rules! v { ($e:expr) => {{
            const E : u32 = $e;
            if v & E != 0 {
                write!(fmt, "{}", stringify!($e))?;
                v &= !E;
                if v != 0 { write!(fmt, " | ")?; }
            }
        }}}

        use winapi::um::winnt::*;

        v!(TOKEN_ALL_ACCESS);
        v!(TOKEN_ALL_ACCESS_P);
        v!(TOKEN_READ);
        v!(TOKEN_WRITE);
        v!(TOKEN_EXECUTE);
        v!(TOKEN_ACCESS_PSEUDO_HANDLE);

        v!(TOKEN_ASSIGN_PRIMARY);
        v!(TOKEN_DUPLICATE);
        v!(TOKEN_IMPERSONATE);
        v!(TOKEN_QUERY);
        v!(TOKEN_QUERY_SOURCE);
        v!(TOKEN_ADJUST_PRIVILEGES);
        v!(TOKEN_ADJUST_GROUPS);
        v!(TOKEN_ADJUST_DEFAULT);
        v!(TOKEN_ADJUST_SESSIONID);

        if v != 0 { write!(fmt, "0x{:04x}", v)? }

        Ok(())
    }
}

impl From<()> for AccessRights { fn from(_: ()) -> Self { Self(0) } }
impl From<AccessRights> for u32 { fn from(ar: AccessRights) -> Self { ar.0 } }

impl BitAnd         for AccessRights { type Output = Self; fn bitand(self, rhs: Self) -> Self::Output { Self(self.0 & rhs.0) } }
impl BitXor         for AccessRights { type Output = Self; fn bitxor(self, rhs: Self) -> Self::Output { Self(self.0 ^ rhs.0) } }
impl BitOr          for AccessRights { type Output = Self; fn bitor (self, rhs: Self) -> Self::Output { Self(self.0 | rhs.0) } }
impl BitAndAssign   for AccessRights { fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0 } }
impl BitXorAssign   for AccessRights { fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0 } }
impl BitOrAssign    for AccessRights { fn bitor_assign (&mut self, rhs: Self) { self.0 |= rhs.0 } }

impl Not                            for AccessRights { type Output = AccessRightsMask; fn not(self) -> Self::Output { AccessRightsMask(!self.0) } }
impl BitAnd<AccessRightsMask>       for AccessRights { type Output = AccessRights; fn bitand(self, rhs: AccessRightsMask) -> Self::Output { Self(self.0 & rhs.0) } }
impl BitAndAssign<AccessRightsMask> for AccessRights { fn bitand_assign(&mut self, rhs: AccessRightsMask) { self.0 &= rhs.0 } }

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
