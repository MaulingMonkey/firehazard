use core::convert::Infallible;
use core::fmt::{self, Debug, Formatter};
use core::ops::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-createrestrictedtoken)\] DWORD/[u32]: CreateRestrictedToken flags mask
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct RestrictedFlagsMask(u32);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-createrestrictedtoken)\] DWORD/[u32]: CreateRestrictedToken flags
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct RestrictedFlags(u32);

impl RestrictedFlagsMask {
    pub fn as_u32(self) -> u32 { self.0 }
}

impl RestrictedFlags {
    /// ### Safety
    /// *   Some APIs might theoretically assume access rights are a valid?
    pub const unsafe fn from_unchecked(rights: u32) -> Self { Self(rights) }

    pub fn as_u32(self) -> u32 { self.0 }
}

impl Debug for RestrictedFlags {
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

        v!(DISABLE_MAX_PRIVILEGE);
        v!(SANDBOX_INERT);
        v!(LUA_TOKEN);
        v!(WRITE_RESTRICTED);

        if v != 0 { write!(fmt, "0x{:X}", v)? }

        Ok(())
    }
}

impl From<()> for RestrictedFlags { fn from(_: ()) -> Self { Self(0) } }
impl From<Option<Infallible>> for RestrictedFlags { fn from(_: Option<Infallible>) -> Self { Self(0) } }
impl From<RestrictedFlags> for u32 { fn from(ar: RestrictedFlags) -> Self { ar.as_u32() } }

impl BitAnd         for RestrictedFlags { type Output = Self; fn bitand(self, rhs: Self) -> Self::Output { Self(self.as_u32() & rhs.as_u32()) } }
impl BitXor         for RestrictedFlags { type Output = Self; fn bitxor(self, rhs: Self) -> Self::Output { Self(self.as_u32() ^ rhs.as_u32()) } }
impl BitOr          for RestrictedFlags { type Output = Self; fn bitor (self, rhs: Self) -> Self::Output { Self(self.as_u32() | rhs.as_u32()) } }
impl BitAndAssign   for RestrictedFlags { fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.as_u32() } }
impl BitXorAssign   for RestrictedFlags { fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.as_u32() } }
impl BitOrAssign    for RestrictedFlags { fn bitor_assign (&mut self, rhs: Self) { self.0 |= rhs.as_u32() } }

impl Not                                for RestrictedFlags     { type Output = RestrictedFlagsMask; fn not(self) -> Self::Output { RestrictedFlagsMask(!self.as_u32()) } }
impl BitAnd<RestrictedFlagsMask>        for RestrictedFlags     { type Output = RestrictedFlags; fn bitand(self, rhs: RestrictedFlagsMask) -> RestrictedFlags { RestrictedFlags(self.as_u32() & rhs.as_u32()) } }
impl BitAnd<RestrictedFlags>            for RestrictedFlagsMask { type Output = RestrictedFlags; fn bitand(self, rhs: RestrictedFlags    ) -> RestrictedFlags { RestrictedFlags(self.as_u32() & rhs.as_u32()) } }
impl BitAndAssign<RestrictedFlagsMask>  for RestrictedFlags     { fn bitand_assign(&mut self, rhs: RestrictedFlagsMask) { self.0 &= rhs.as_u32() } }

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
