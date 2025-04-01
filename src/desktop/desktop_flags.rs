use core::fmt::{self, Debug, Formatter};



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createdesktopa)\] DWORD/[u32]: CreateDesktop/OpenDesktop flags mask
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct FlagsMask(u32);

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createdesktopa)\] DWORD/[u32]: CreateDesktop/OpenDesktop flags
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Flags(u32);

flags!(impl .. for Flags(u32) - FlagsMask);

impl Flags {
    /// ### Safety
    /// *   Some APIs might theoretically assume flags are a valid?
    pub const unsafe fn from_unchecked(flags: u32) -> Self { Self(flags) }
}

impl Debug for Flags {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        use winapi::um::winuser::*;
        flags!(self.0, fmt, "0x{:X}", [
            DF_ALLOWOTHERACCOUNTHOOK,
        ])
    }
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-opendesktopa)\]
/// DF_ALLOWOTHERACCOUNTHOOK
/// <br>
/// Allows processes running in other accounts on the desktop to set hooks in this process.
pub const ALLOWOTHERACCOUNTHOOK : Flags = Flags(winapi::um::winuser::DF_ALLOWOTHERACCOUNTHOOK);
