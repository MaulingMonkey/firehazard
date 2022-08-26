use core::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-createrestrictedtoken)\] DWORD/[u32]: CreateRestrictedToken flags mask
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct RestrictedFlagsMask(u32);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-createrestrictedtoken)\] DWORD/[u32]: CreateRestrictedToken flags
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct RestrictedFlags(u32);

flags!(impl .. for RestrictedFlags(u32) - RestrictedFlagsMask);

impl RestrictedFlags {
    /// ### Safety
    /// *   Some APIs might theoretically assume access rights are a valid?
    pub const unsafe fn from_unchecked(rights: u32) -> Self { Self(rights) }
}

impl Debug for RestrictedFlags {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        use winapi::um::winnt::*;
        flags!(self.0, fmt, "0x{:X}", [
            DISABLE_MAX_PRIVILEGE,
            SANDBOX_INERT,
            LUA_TOKEN,
            WRITE_RESTRICTED,
        ])
    }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-createrestrictedtoken)\]
/// Remove all privileges except `SeChangeNotifyPrivilege`.
///
/// ⚠️ Causes `privileges_to_delete` to be ignored!
pub const DISABLE_MAX_PRIVILEGE : RestrictedFlags = RestrictedFlags(winapi::um::winnt::DISABLE_MAX_PRIVILEGE);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-createrestrictedtoken)\]
/// This flag appears to be meant for installers - see Microsoft docs for more details.
///
/// > If this value is used, the system does not check AppLocker rules or apply Software Restriction Policies.
/// > For AppLocker, this flag disables checks for all four rule collections: Executable, Windows Installer, Script, and DLL.
pub const SANDBOX_INERT : RestrictedFlags = RestrictedFlags(winapi::um::winnt::SANDBOX_INERT);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-createrestrictedtoken)\]
/// Create a **L**east-privileged **U**ser **A**ccount token (e.g. non-elevated).
///
/// ⚠️ Might also cause `privileges_to_delete` to be ignored - see [chromium#551665 comment#2](https://bugs.chromium.org/p/chromium/issues/detail?id=551665#c2)
pub const LUA_TOKEN : RestrictedFlags = RestrictedFlags(winapi::um::winnt::LUA_TOKEN);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-createrestrictedtoken)\]
/// Does this mean restricting SIDs also restrict reads if this flag isn't specified?
///
/// > The new token contains restricting SIDs that are considered only when evaluating write access.
pub const WRITE_RESTRICTED : RestrictedFlags = RestrictedFlags(winapi::um::winnt::WRITE_RESTRICTED);
