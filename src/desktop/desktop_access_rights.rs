use crate::*;

use core::fmt::{self, Debug, Formatter};



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/winstation/desktop-security-and-access-rights)\]
/// DWORD/[u32]: Mask for removing or restricting access rights to Desktop objects
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct AccessRightsMask(u32);

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/winstation/desktop-security-and-access-rights)\]
/// DWORD/[u32]: Access rights for Desktop objects
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
            DESKTOP_CREATEMENU,
            DESKTOP_CREATEWINDOW,
            DESKTOP_ENUMERATE,
            DESKTOP_HOOKCONTROL,
            DESKTOP_JOURNALPLAYBACK,
            DESKTOP_JOURNALRECORD,
            DESKTOP_READOBJECTS,
            DESKTOP_SWITCHDESKTOP,
            DESKTOP_WRITEOBJECTS,

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

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/winstation/desktop-security-and-access-rights)\]
/// DESKTOP_CREATEMENU
pub const CREATEMENU            : AccessRights = AccessRights(winapi::um::winuser::DESKTOP_CREATEMENU       );

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/winstation/desktop-security-and-access-rights)\]
/// DESKTOP_CREATEWINDOW
pub const CREATEWINDOW          : AccessRights = AccessRights(winapi::um::winuser::DESKTOP_CREATEWINDOW     );

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/winstation/desktop-security-and-access-rights)\]
/// DESKTOP_ENUMERATE
pub const ENUMERATE             : AccessRights = AccessRights(winapi::um::winuser::DESKTOP_ENUMERATE        );

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/winstation/desktop-security-and-access-rights)\]
/// DESKTOP_HOOKCONTROL
pub const HOOKCONTROL           : AccessRights = AccessRights(winapi::um::winuser::DESKTOP_HOOKCONTROL      );

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/winstation/desktop-security-and-access-rights)\]
/// DESKTOP_JOURNALPLAYBACK
pub const JOURNALPLAYBACK       : AccessRights = AccessRights(winapi::um::winuser::DESKTOP_JOURNALPLAYBACK  );

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/winstation/desktop-security-and-access-rights)\]
/// DESKTOP_JOURNALRECORD
pub const JOURNALRECORD         : AccessRights = AccessRights(winapi::um::winuser::DESKTOP_JOURNALRECORD    );

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/winstation/desktop-security-and-access-rights)\]
/// DESKTOP_READOBJECTS
pub const READOBJECTS           : AccessRights = AccessRights(winapi::um::winuser::DESKTOP_READOBJECTS      );

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/winstation/desktop-security-and-access-rights)\]
/// DESKTOP_SWITCHDESKTOP
pub const SWITCHDESKTOP         : AccessRights = AccessRights(winapi::um::winuser::DESKTOP_SWITCHDESKTOP    );

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/winstation/desktop-security-and-access-rights)\]
/// DESKTOP_WRITEOBJECTS
pub const WRITEOBJECTS          : AccessRights = AccessRights(winapi::um::winuser::DESKTOP_WRITEOBJECTS     );
