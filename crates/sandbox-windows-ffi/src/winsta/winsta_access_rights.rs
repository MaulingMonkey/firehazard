use crate::*;

use core::fmt::{self, Debug, Formatter};
use core::ops::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/winstation/window-station-security-and-access-rights)\]
/// DWORD/[u32]: Access rights mask for removing or restricting access rights
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct AccessRightsMask(u32);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/winstation/window-station-security-and-access-rights)\]
/// DWORD/[u32]: Access rights for Access-Token objects
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct AccessRights(u32);

impl AccessRightsMask {
    pub fn as_u32(self) -> u32 { self.0 }
}

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
        use winapi::um::winuser::*;

        v!(WINSTA_ALL_ACCESS);
        v!(WINSTA_ACCESSCLIPBOARD);
        v!(WINSTA_ACCESSGLOBALATOMS);
        v!(WINSTA_CREATEDESKTOP);
        v!(WINSTA_ENUMDESKTOPS);
        v!(WINSTA_ENUMERATE);
        v!(WINSTA_EXITWINDOWS);
        v!(WINSTA_READATTRIBUTES);
        v!(WINSTA_READSCREEN);
        v!(WINSTA_WRITEATTRIBUTES);

        v!(GENERIC_READ);
        v!(GENERIC_WRITE);
        v!(GENERIC_EXECUTE);
        v!(GENERIC_ALL);

        v!(DELETE);
        v!(READ_CONTROL);
        v!(WRITE_DAC);
        v!(WRITE_OWNER);
        v!(SYNCHRONIZE);

        if v != 0 { write!(fmt, "0x{:04x}", v)? }

        Ok(())
    }
}

impl From<()> for AccessRights { fn from(_: ()) -> Self { Self(0) } }
impl From<AccessRights> for u32 { fn from(ar: AccessRights) -> Self { ar.as_u32() } }
impl From<access::Mask> for AccessRights { fn from(am: access::Mask) -> Self { Self(am.into()) } }
impl From<AccessRights> for access::Mask { fn from(am: AccessRights) -> Self { unsafe { access::Mask::from_unchecked(am.as_u32()) } } }

impl BitAnd         for AccessRights { type Output = Self; fn bitand(self, rhs: Self) -> Self::Output { Self(self.as_u32() & rhs.as_u32()) } }
impl BitXor         for AccessRights { type Output = Self; fn bitxor(self, rhs: Self) -> Self::Output { Self(self.as_u32() ^ rhs.as_u32()) } }
impl BitOr          for AccessRights { type Output = Self; fn bitor (self, rhs: Self) -> Self::Output { Self(self.as_u32() | rhs.as_u32()) } }
impl BitAndAssign   for AccessRights { fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.as_u32() } }
impl BitXorAssign   for AccessRights { fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.as_u32() } }
impl BitOrAssign    for AccessRights { fn bitor_assign (&mut self, rhs: Self) { self.0 |= rhs.as_u32() } }

impl BitAnd         <access::Mask> for AccessRights { type Output = Self; fn bitand(self, rhs: access::Mask) -> Self::Output { Self(self.as_u32() & rhs.as_u32()) } }
impl BitXor         <access::Mask> for AccessRights { type Output = Self; fn bitxor(self, rhs: access::Mask) -> Self::Output { Self(self.as_u32() ^ rhs.as_u32()) } }
impl BitOr          <access::Mask> for AccessRights { type Output = Self; fn bitor (self, rhs: access::Mask) -> Self::Output { Self(self.as_u32() | rhs.as_u32()) } }
impl BitAndAssign   <access::Mask> for AccessRights { fn bitand_assign(&mut self, rhs: access::Mask) { self.0 &= rhs.as_u32() } }
impl BitXorAssign   <access::Mask> for AccessRights { fn bitxor_assign(&mut self, rhs: access::Mask) { self.0 ^= rhs.as_u32() } }
impl BitOrAssign    <access::Mask> for AccessRights { fn bitor_assign (&mut self, rhs: access::Mask) { self.0 |= rhs.as_u32() } }

impl Not                            for AccessRights { type Output = AccessRightsMask; fn not(self) -> Self::Output { AccessRightsMask(!self.as_u32()) } }
impl BitAnd<AccessRightsMask>       for AccessRights { type Output = AccessRights; fn bitand(self, rhs: AccessRightsMask) -> AccessRights { AccessRights(self.as_u32() & rhs.as_u32()) } }
impl BitAnd<access::MaskMask>       for AccessRights { type Output = AccessRights; fn bitand(self, rhs: access::MaskMask) -> AccessRights { AccessRights(self.as_u32() & rhs.as_u32()) } }
impl BitAnd<AccessRights>       for AccessRightsMask { type Output = AccessRights; fn bitand(self, rhs: AccessRights    ) -> AccessRights { AccessRights(self.as_u32() & rhs.as_u32()) } }
impl BitAnd<AccessRights>       for access::MaskMask { type Output = AccessRights; fn bitand(self, rhs: AccessRights    ) -> AccessRights { AccessRights(self.as_u32() & rhs.as_u32()) } }
impl BitAndAssign<AccessRightsMask> for AccessRights { fn bitand_assign(&mut self, rhs: AccessRightsMask) { self.0 &= rhs.as_u32() } }
impl BitAndAssign<access::MaskMask> for AccessRights { fn bitand_assign(&mut self, rhs: access::MaskMask) { self.0 &= rhs.as_u32() } }

pub const ALL_ACCESS            : AccessRights = AccessRights(winapi::um::winuser::WINSTA_ALL_ACCESS);
pub const ACCESSCLIPBOARD       : AccessRights = AccessRights(winapi::um::winuser::WINSTA_ACCESSCLIPBOARD);
pub const ACCESSGLOBALATOMS     : AccessRights = AccessRights(winapi::um::winuser::WINSTA_ACCESSGLOBALATOMS);
pub const CREATEDESKTOP         : AccessRights = AccessRights(winapi::um::winuser::WINSTA_CREATEDESKTOP);
pub const ENUMDESKTOPS          : AccessRights = AccessRights(winapi::um::winuser::WINSTA_ENUMDESKTOPS);
pub const ENUMERATE             : AccessRights = AccessRights(winapi::um::winuser::WINSTA_ENUMERATE);
pub const EXITWINDOWS           : AccessRights = AccessRights(winapi::um::winuser::WINSTA_EXITWINDOWS);
pub const READATTRIBUTES        : AccessRights = AccessRights(winapi::um::winuser::WINSTA_READATTRIBUTES);
pub const READSCREEN            : AccessRights = AccessRights(winapi::um::winuser::WINSTA_READSCREEN);
pub const WRITEATTRIBUTES       : AccessRights = AccessRights(winapi::um::winuser::WINSTA_WRITEATTRIBUTES);
