use core::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createdesktopa)\] DWORD/[u32]: CreateDesktop/OpenDesktop flags mask
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct FlagsMask(u32);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createdesktopa)\] DWORD/[u32]: CreateDesktop/OpenDesktop flags
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Flags(u32);

flags!(impl .. for Flags(u32) - FlagsMask);

impl Flags {
    /// ### Safety
    /// *   Some APIs might theoretically assume access rights are a valid?
    pub const unsafe fn from_unchecked(rights: u32) -> Self { Self(rights) }
}

impl Debug for Flags {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        use winapi::um::winuser::*;
        flags!(self.0, fmt, "0x{:X}", [
            DF_ALLOWOTHERACCOUNTHOOK,
        ])
    }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-opendesktopa)\]
/// Allows processes running in other accounts on the desktop to set hooks in this process.
pub const ALLOWOTHERACCOUNTHOOK : Flags = Flags(winapi::um::winuser::DF_ALLOWOTHERACCOUNTHOOK);
