use crate::*;

use core::fmt::{self, Debug, Formatter};



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/winstation/window-station-security-and-access-rights)\]
/// DWORD/[u32]: Mask for removing or restricting access rights to Window Station objects
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct AccessRightsMask(u32);

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/winstation/window-station-security-and-access-rights)\]
/// DWORD/[u32]: Access rights for Window Station objects
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

#[doc(alias = "WINSTA_ALL_ACCESS"           )] #[doc = r"\[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/winstation/window-station-security-and-access-rights)\] WINSTA_ALL_ACCESS"          ] pub const ALL_ACCESS            : AccessRights = AccessRights(winapi::um::winuser::WINSTA_ALL_ACCESS          );
#[doc(alias = "WINSTA_ACCESSCLIPBOARD"      )] #[doc = r"\[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/winstation/window-station-security-and-access-rights)\] WINSTA_ACCESSCLIPBOARD"     ] pub const ACCESSCLIPBOARD       : AccessRights = AccessRights(winapi::um::winuser::WINSTA_ACCESSCLIPBOARD     );
#[doc(alias = "WINSTA_ACCESSGLOBALATOMS"    )] #[doc = r"\[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/winstation/window-station-security-and-access-rights)\] WINSTA_ACCESSGLOBALATOMS"   ] pub const ACCESSGLOBALATOMS     : AccessRights = AccessRights(winapi::um::winuser::WINSTA_ACCESSGLOBALATOMS   );
#[doc(alias = "WINSTA_CREATEDESKTOP"        )] #[doc = r"\[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/winstation/window-station-security-and-access-rights)\] WINSTA_CREATEDESKTOP"       ] pub const CREATEDESKTOP         : AccessRights = AccessRights(winapi::um::winuser::WINSTA_CREATEDESKTOP       );
#[doc(alias = "WINSTA_ENUMDESKTOPS"         )] #[doc = r"\[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/winstation/window-station-security-and-access-rights)\] WINSTA_ENUMDESKTOPS"        ] pub const ENUMDESKTOPS          : AccessRights = AccessRights(winapi::um::winuser::WINSTA_ENUMDESKTOPS        );
#[doc(alias = "WINSTA_ENUMERATE"            )] #[doc = r"\[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/winstation/window-station-security-and-access-rights)\] WINSTA_ENUMERATE"           ] pub const ENUMERATE             : AccessRights = AccessRights(winapi::um::winuser::WINSTA_ENUMERATE           );
#[doc(alias = "WINSTA_EXITWINDOWS"          )] #[doc = r"\[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/winstation/window-station-security-and-access-rights)\] WINSTA_EXITWINDOWS"         ] pub const EXITWINDOWS           : AccessRights = AccessRights(winapi::um::winuser::WINSTA_EXITWINDOWS         );
#[doc(alias = "WINSTA_READATTRIBUTES"       )] #[doc = r"\[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/winstation/window-station-security-and-access-rights)\] WINSTA_READATTRIBUTES"      ] pub const READATTRIBUTES        : AccessRights = AccessRights(winapi::um::winuser::WINSTA_READATTRIBUTES      );
#[doc(alias = "WINSTA_READSCREEN"           )] #[doc = r"\[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/winstation/window-station-security-and-access-rights)\] WINSTA_READSCREEN"          ] pub const READSCREEN            : AccessRights = AccessRights(winapi::um::winuser::WINSTA_READSCREEN          );
#[doc(alias = "WINSTA_WRITEATTRIBUTES"      )] #[doc = r"\[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/winstation/window-station-security-and-access-rights)\] WINSTA_WRITEATTRIBUTES"     ] pub const WRITEATTRIBUTES       : AccessRights = AccessRights(winapi::um::winuser::WINSTA_WRITEATTRIBUTES     );
