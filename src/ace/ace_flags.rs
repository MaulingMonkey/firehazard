use core::fmt::{self, Debug, Display, Formatter};



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-ace_header#members)\]
/// ACE_HEADER::AceFlags Mask
///
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct FlagsMask(u8);

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-ace_header#members)\]
/// ACE_HEADER::AceFlags
///
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Flags(u8);

flags!(impl .. for Flags(u32, u8) - FlagsMask);

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
        if v != 0 { write!(fmt, "0x{:02X}", v)? }
        Ok(())
    }
}



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-ace_header#members)\]
/// OBJECT_INHERIT_ACE
///
pub const OBJECT_INHERIT_ACE            : Flags     = Flags     (winapi::um::winnt::OBJECT_INHERIT_ACE          ); // 0x01

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-ace_header#members)\]
/// CONTAINER_INHERIT_ACE
///
pub const CONTAINER_INHERIT_ACE         : Flags     = Flags     (winapi::um::winnt::CONTAINER_INHERIT_ACE       ); // 0x02

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-ace_header#members)\]
/// NO_PROPAGATE_INHERIT_ACE
///
pub const NO_PROPAGATE_INHERIT_ACE      : Flags     = Flags     (winapi::um::winnt::NO_PROPAGATE_INHERIT_ACE    ); // 0x04



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-ace_header#members)\]
/// INHERIT_ONLY_ACE
///
pub const INHERIT_ONLY_ACE              : Flags     = Flags     (winapi::um::winnt::INHERIT_ONLY_ACE            ); // 0x08

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-ace_header#members)\]
/// INHERITED_ACE
///
pub const INHERITED_ACE                 : Flags     = Flags     (winapi::um::winnt::INHERITED_ACE               ); // 0x10

/// \[<strike>microsoft.com</strike>\]
/// VALID_INHERIT_FLAGS
///
pub const VALID_INHERIT_FLAGS           : FlagsMask = FlagsMask (winapi::um::winnt::VALID_INHERIT_FLAGS         ); // 0x1F

// 0x20

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-ace_header#members)\]
/// SUCCESSFUL_ACCESS_ACE_FLAG
///
pub const SUCCESSFUL_ACCESS_ACE_FLAG    : Flags     = Flags     (winapi::um::winnt::SUCCESSFUL_ACCESS_ACE_FLAG  ); // 0x40 - also previously TRUST_PROTECTED_FILTER_ACE_FLAG

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-ace_header#members)\]
/// FAILED_ACCESS_ACE_FLAG
///
pub const FAILED_ACCESS_ACE_FLAG        : Flags     = Flags     (winapi::um::winnt::FAILED_ACCESS_ACE_FLAG      ); // 0x80



macro_rules! table { ( $( $short:literal $sddl:ident $flag:tt )* ) => {
    &[$(
        (winapi::shared::sddl::$sddl, winapi::um::winnt::$flag, stringify!($flag))
    ),*]
}}

/// https://learn.microsoft.com/en-us/windows/win32/secauthz/ace-strings
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
