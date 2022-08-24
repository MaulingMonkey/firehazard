use crate::*;

use core::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/winstation/window-station-security-and-access-rights)\]
/// DWORD/[u32]: Access rights mask for removing or restricting access rights
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct AccessRightsMask(u32);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/winstation/window-station-security-and-access-rights)\]
/// DWORD/[u32]: Access rights for Access-Token objects
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct AccessRights(u32);

flags!(impl .. for AccessRights(u32) - AccessRightsMask { });

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
        use winapi::um::winnt::*;
        use winapi::um::winuser::*;
        flags!(self.0, fmt, "0x{:04X}", [
            WINSTA_ALL_ACCESS,
            WINSTA_ACCESSCLIPBOARD,
            WINSTA_ACCESSGLOBALATOMS,
            WINSTA_CREATEDESKTOP,
            WINSTA_ENUMDESKTOPS,
            WINSTA_ENUMERATE,
            WINSTA_EXITWINDOWS,
            WINSTA_READATTRIBUTES,
            WINSTA_READSCREEN,
            WINSTA_WRITEATTRIBUTES,

            GENERIC_READ,
            GENERIC_WRITE,
            GENERIC_EXECUTE,
            GENERIC_ALL,

            DELETE,
            READ_CONTROL,
            WRITE_DAC,
            WRITE_OWNER,
            SYNCHRONIZE,
        ])
    }
}

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
