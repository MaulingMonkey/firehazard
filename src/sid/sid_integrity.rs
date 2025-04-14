#![doc = include_str!("sid_integrity.md")]
#![allow(non_upper_case_globals)] // enum-like values

use crate::*;

use core::fmt::{self, Debug, Formatter};
use core::hash::Hash;



#[doc = include_str!("sid_integrity.md")]
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)] pub struct Level { sid: sid::Static<1> }

impl Default    for Level { fn default() -> Self { Level::Untrusted } }
impl PartialOrd for Level { fn partial_cmp  (&self, other: &Self) -> Option<core::cmp::Ordering>    { (*self.sid.as_sid_ptr()).partial_cmp   (&*other.sid.as_sid_ptr()) } }
impl Ord        for Level { fn cmp          (&self, other: &Self) -> core::cmp::Ordering            { (*self.sid.as_sid_ptr()).cmp           (&*other.sid.as_sid_ptr()) } }
impl Hash       for Level { fn hash<H: core::hash::Hasher>(&self, state: &mut H)                    { (*self.sid.as_sid_ptr()).hash(state) } }
impl<'a> From<&'a Level> for sid::Ptr<'a> { fn from(level: &'a Level) -> Self { level.sid.as_sid_ptr() } }

impl Debug for Level {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        let friendly = match *self {
            Untrusted           => "sid::integrity::Untrusted",
            Low                 => "sid::integrity::Low",
            Medium              => "sid::integrity::Medium",
            MediumPlus          => "sid::integrity::MediumPlus",
            High                => "sid::integrity::High",
            System              => "sid::integrity::System",
            ProtectedProcess    => "sid::integrity::ProtectedProcess",
            _other => {
                let sid  = self.sid.as_sid_ptr();
                let rev  = sid.revision();
                let auth = sid.authority_u64();
                let sa0  = sid.subauthorities()[0];
                return write!(fmt, "sid::integrity::Level(S-{rev}-{auth}-0x{sa0:04X})")
            },
        };
        fmt.write_str(friendly)
    }
}

impl Level {
    /// Create a new integrity level that doesn't exactly match any of the existing integrity levels.
    pub const fn new(sa0: u32) -> Self { Self { sid: sid::Static::new(1, 16, [sa0]) } } // 16 = SECURITY_MANDATORY_LABEL_AUTHORITY
    #[doc(alias = "SECURITY_MANDATORY_UNTRUSTED_RID"            )] #[doc = "SECURITY_MANDATORY_UNTRUSTED_RID"           ] pub const Untrusted         : Level = Level::new(0x0000);
    #[doc(alias = "SECURITY_MANDATORY_LOW_RID"                  )] #[doc = "SECURITY_MANDATORY_LOW_RID"                 ] pub const Low               : Level = Level::new(0x1000);
    #[doc(alias = "SECURITY_MANDATORY_MEDIUM_RID"               )] #[doc = "SECURITY_MANDATORY_MEDIUM_RID"              ] pub const Medium            : Level = Level::new(0x2000);
    #[doc(alias = "SECURITY_MANDATORY_MEDIUM_PLUS_RID"          )] #[doc = "SECURITY_MANDATORY_MEDIUM_PLUS_RID"         ] pub const MediumPlus        : Level = Level::new(0x2100);
    #[doc(alias = "SECURITY_MANDATORY_HIGH_RID"                 )] #[doc = "SECURITY_MANDATORY_HIGH_RID"                ] pub const High              : Level = Level::new(0x3000);
    #[doc(alias = "SECURITY_MANDATORY_SYSTEM_RID"               )] #[doc = "SECURITY_MANDATORY_SYSTEM_RID"              ] pub const System            : Level = Level::new(0x4000);
    #[doc(alias = "SECURITY_MANDATORY_PROTECTED_PROCESS_RID"    )] #[doc = "SECURITY_MANDATORY_PROTECTED_PROCESS_RID"   ] pub const ProtectedProcess  : Level = Level::new(0x5000);
}

#[doc(alias = "SECURITY_MANDATORY_UNTRUSTED_RID"            )] #[doc = "SECURITY_MANDATORY_UNTRUSTED_RID"           ] pub const Untrusted         : Level = Level::new(0x0000);
#[doc(alias = "SECURITY_MANDATORY_LOW_RID"                  )] #[doc = "SECURITY_MANDATORY_LOW_RID"                 ] pub const Low               : Level = Level::new(0x1000);
#[doc(alias = "SECURITY_MANDATORY_MEDIUM_RID"               )] #[doc = "SECURITY_MANDATORY_MEDIUM_RID"              ] pub const Medium            : Level = Level::new(0x2000);
#[doc(alias = "SECURITY_MANDATORY_MEDIUM_PLUS_RID"          )] #[doc = "SECURITY_MANDATORY_MEDIUM_PLUS_RID"         ] pub const MediumPlus        : Level = Level::new(0x2100);
#[doc(alias = "SECURITY_MANDATORY_HIGH_RID"                 )] #[doc = "SECURITY_MANDATORY_HIGH_RID"                ] pub const High              : Level = Level::new(0x3000);
#[doc(alias = "SECURITY_MANDATORY_SYSTEM_RID"               )] #[doc = "SECURITY_MANDATORY_SYSTEM_RID"              ] pub const System            : Level = Level::new(0x4000);
#[doc(alias = "SECURITY_MANDATORY_PROTECTED_PROCESS_RID"    )] #[doc = "SECURITY_MANDATORY_PROTECTED_PROCESS_RID"   ] pub const ProtectedProcess  : Level = Level::new(0x5000);
