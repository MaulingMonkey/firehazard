#[allow(unused_imports)] use crate::*;
use core::fmt::{self, Debug, Formatter};



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_privileges)\] DWORD/[u32]: SE_GROUP_*: SID (group) attributes flags mask
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct AttributesMask(u32);

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_privileges)\] DWORD/[u32]: SE_GROUP_*: SID (group) attributes flags
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Attributes(u32);

flags!(impl .. for Attributes(u32) - AttributesMask);

impl Attributes {
    /// ### Safety
    /// *   Some APIs might theoretically assume flags are a valid?
    pub const unsafe fn from_unchecked(flags: u32) -> Self { Self(flags) }
}

impl Debug for Attributes {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        use winapi::um::winnt::*;
        flags!(self.0, fmt, "0x{:X}", [
            SE_PRIVILEGE_ENABLED,
            SE_PRIVILEGE_ENABLED_BY_DEFAULT,
            SE_PRIVILEGE_REMOVED,
            SE_PRIVILEGE_USED_FOR_ACCESS,
        ])
    }
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_privileges)\]
/// The privilege is enabled.
/// Note that disabled privileges can be enabled with just an appropriate token handle - remove them outright if you're restricting privileges for security purpouses.
pub const ENABLED               : Attributes        = Attributes(winapi::um::winnt::SE_PRIVILEGE_ENABLED);

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_privileges)\]
/// The privilege is enabled by default.
pub const ENABLED_BY_DEFAULT    : Attributes        = Attributes(winapi::um::winnt::SE_PRIVILEGE_ENABLED_BY_DEFAULT);

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_privileges)\]
/// Mark a privilege for removal when used with [`adjust_token_privileges`].
pub const REMOVED               : Attributes        = Attributes(winapi::um::winnt::SE_PRIVILEGE_REMOVED);

/// The privilege was used to gain access to an object or service.
/// This flag is used to identify the relevant privileges in a set passed by a client application that may contain unnecessary privileges.
pub const USED_FOR_ACCESS       : Attributes        = Attributes(winapi::um::winnt::SE_PRIVILEGE_USED_FOR_ACCESS);

/// All valid attributes as of the windows SDK `winapi` and/or this crate was built in reference to?
pub const VALID_ATTRIBUTES      : AttributesMask    = AttributesMask(winapi::um::winnt::SE_PRIVILEGE_VALID_ATTRIBUTES);
