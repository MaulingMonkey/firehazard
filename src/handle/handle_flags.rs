#[allow(unused_imports)] use crate::*;
use core::fmt::{self, Debug, Formatter};



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-gethandleinformation)\] DWORD/[u32]: Handle information flags mask
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct FlagsMask(u32);

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-gethandleinformation)\] DWORD/[u32]: Handle information flags
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
        use winapi::um::winbase::*;
        flags!(self.0, fmt, "0x{:X}", [
            HANDLE_FLAG_INHERIT,
            HANDLE_FLAG_PROTECT_FROM_CLOSE,
        ])
    }
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-gethandleinformation)\] HANDLE_FLAG_INHERIT<br>
/// If this flag is set, a child process created with the `inherit_handles` parameter of [`create_process_a`] set to `true` will inherit the object handle.
pub const FLAG_INHERIT : Flags = Flags(winapi::um::winbase::HANDLE_FLAG_INHERIT);

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-gethandleinformation)\] HANDLE_FLAG_PROTECT_FROM_CLOSE<br>
/// If this flag is set, calling the [`close_handle`] function will not close the object handle.
pub const FLAG_PROTECT_FROM_CLOSE : Flags = Flags(winapi::um::winbase::HANDLE_FLAG_PROTECT_FROM_CLOSE);
