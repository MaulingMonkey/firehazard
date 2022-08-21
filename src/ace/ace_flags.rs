use core::fmt::{self, Debug, Display, Formatter};
use core::ops::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-ace_header#members)\]
/// ACE_HEADER::AceFlags Mask
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct FlagsMask(u8);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-ace_header#members)\]
/// ACE_HEADER::AceFlags
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Flags(u8);

impl Flags {
    /// ### Safety
    /// *   Some APIs might theoretically assume access rights are a valid?
    pub const unsafe fn from_unchecked(rights: u8) -> Self { Self(rights) }

    pub fn as_u8(self) -> u8 { self.0 }

    pub fn short_flags(self) -> Option<impl Display + Copy> {
        return Some(Short(self.0));
        #[derive(Clone, Copy)] struct Short(u8);
        impl Display for Short {
            fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
                let mut v = self.0;
                for (short, flag, _long) in SHORT_FLAG_LONG {
                    if 0 == v & flag { continue }
                    write!(fmt, "{short}")?;
                    v =! flag;
                }
                if v != 0 { write!(fmt, "??")?; } // 0x20
                Ok(())
            }
        }
    }
}

impl Debug for Flags {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        let mut v = self.0;
        if v == 0 { return write!(fmt, "0") }
        for (_short, flag, long) in SHORT_FLAG_LONG {
            if 0 == v & flag { continue }
            write!(fmt, "{long}")?;
            v =! flag;
            if v != 0 { write!(fmt, " | ")?; }
        }
        if v != 0 { write!(fmt, "0x{:02x}", v)? }
        Ok(())
    }
}

impl From<()> for Flags { fn from(_: ()) -> Self { Self(0) } }
impl From<Flags> for u8  { fn from(ar: Flags) -> Self { ar.0 } }
impl From<Flags> for u32 { fn from(ar: Flags) -> Self { ar.0 as _ } }

impl BitAnd         for Flags { type Output = Self; fn bitand(self, rhs: Self) -> Self::Output { Self(self.0 & rhs.0) } }
impl BitXor         for Flags { type Output = Self; fn bitxor(self, rhs: Self) -> Self::Output { Self(self.0 ^ rhs.0) } }
impl BitOr          for Flags { type Output = Self; fn bitor (self, rhs: Self) -> Self::Output { Self(self.0 | rhs.0) } }
impl BitAndAssign   for Flags { fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0 } }
impl BitXorAssign   for Flags { fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0 } }
impl BitOrAssign    for Flags { fn bitor_assign (&mut self, rhs: Self) { self.0 |= rhs.0 } }

impl Not                        for Flags       { type Output = FlagsMask;  fn not(self) -> Self::Output { FlagsMask(!self.0) } }
impl BitAnd<FlagsMask>          for Flags       { type Output = Flags;      fn bitand(self, rhs: FlagsMask) -> Flags { Flags(self.0 & rhs.0) } }
impl BitAnd<Flags>              for FlagsMask   { type Output = Flags;      fn bitand(self, rhs: Flags    ) -> Flags { Flags(self.0 & rhs.0) } }
impl BitAndAssign<FlagsMask>    for Flags       { fn bitand_assign(&mut self, rhs: FlagsMask) { self.0 &= rhs.0 } }

pub const OBJECT_INHERIT_ACE            : Flags     = Flags     (winapi::um::winnt::OBJECT_INHERIT_ACE          ); // 0x01
pub const CONTAINER_INHERIT_ACE         : Flags     = Flags     (winapi::um::winnt::CONTAINER_INHERIT_ACE       ); // 0x02
pub const NO_PROPAGATE_INHERIT_ACE      : Flags     = Flags     (winapi::um::winnt::NO_PROPAGATE_INHERIT_ACE    ); // 0x04
pub const INHERIT_ONLY_ACE              : Flags     = Flags     (winapi::um::winnt::INHERIT_ONLY_ACE            ); // 0x08
pub const INHERITED_ACE                 : Flags     = Flags     (winapi::um::winnt::INHERITED_ACE               ); // 0x10
pub const VALID_INHERIT_FLAGS           : FlagsMask = FlagsMask (winapi::um::winnt::VALID_INHERIT_FLAGS         ); // 0x1F
// 0x20
pub const SUCCESSFUL_ACCESS_ACE_FLAG    : Flags     = Flags     (winapi::um::winnt::SUCCESSFUL_ACCESS_ACE_FLAG  ); // 0x40 - also previously TRUST_PROTECTED_FILTER_ACE_FLAG
pub const FAILED_ACCESS_ACE_FLAG        : Flags     = Flags     (winapi::um::winnt::FAILED_ACCESS_ACE_FLAG      ); // 0x80

macro_rules! table { ( $( $short:literal $sddl:ident $flag:tt )* ) => {
    &[$(
        (winapi::shared::sddl::$sddl, winapi::um::winnt::$flag, stringify!($flag))
    ),*]
}}

/// https://docs.microsoft.com/en-us/windows/win32/secauthz/ace-strings
const SHORT_FLAG_LONG : &'static [(&'static str, u8, &'static str)] = table! [
    // str  sddl.h                      aceflag
    "CI"    SDDL_CONTAINER_INHERIT      CONTAINER_INHERIT_ACE
    "OI"    SDDL_OBJECT_INHERIT         OBJECT_INHERIT_ACE
    "NP"    SDDL_NO_PROPAGATE           NO_PROPAGATE_INHERIT_ACE
    "IO"    SDDL_INHERIT_ONLY           INHERIT_ONLY_ACE
    "ID"    SDDL_INHERITED              INHERITED_ACE
    "SA"    SDDL_AUDIT_SUCCESS          SUCCESSFUL_ACCESS_ACE_FLAG
    "FA"    SDDL_AUDIT_FAILURE          FAILED_ACCESS_ACE_FLAG
    // "TP" SDDL_TRUST_PROTECTED_FILTER TRUST_PROTECTED_FILTER_ACE_FLAG     N/A: Windows Server 2016, Windows 10 Version 1607, Windows 10 Version 1511, Windows 10 Version 1507, Windows Server 2012 R2, Windows 8.1, Windows Server 2012, Windows 8, Windows Server 2008 R2, Windows 7, Windows Server 2008, Windows Vista and Windows Server 2003
    // "CR" SDDL_CRITICAL               CRITICAL_ACE_FLAG                   N/A: Windows Server Version 1803, Windows 10 Version 1803, Windows Server Version 1709, Windows 10 Version 1709, Windows 10 Version 1703, Windows Server 2016, Windows 10 Version 1607, Windows 10 Version 1511, Windows 10 Version 1507, Windows Server 2012 R2, Windows 8.1, Windows Server 2012, Windows 8, Windows Server 2008 R2, Windows 7, Windows Server 2008, Windows Vista and Windows Server 2003
];
