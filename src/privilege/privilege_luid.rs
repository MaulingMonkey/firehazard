use crate::*;
use winapi::um::winnt::LUID;
use core::fmt::{self, Debug, Formatter};



#[doc(alias = "LUID")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/secauthz/privilege-constants)\]
/// LUID, referencing a [privilege](https://learn.microsoft.com/en-us/windows/win32/secauthz/privilege-constants#constants) such as `"SeShutdownPrivilege"`
///
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)] #[repr(transparent)] pub struct Luid(pub crate::Luid);

impl From<u64>  for Luid { fn from(value: u64 ) -> Self { Self(crate::Luid::from(value)) } }
impl From<LUID> for Luid { fn from(value: LUID) -> Self { Self(crate::Luid::from(value)) } }
impl From<Luid> for u64  { fn from(value: Luid) -> Self { Self::from(value.0) } }
impl From<Luid> for LUID { fn from(value: Luid) -> Self { Self::from(value.0) } }

impl Debug for privilege::Luid {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        let luid = u64::from(self.0);
        if let Ok(name) = lookup_privilege_name_a(*self) {
            write!(fmt, "privilege::Luid(0x{:x} {:?})", luid, name)
        } else {
            write!(fmt, "privilege::Luid(0x{:x} ???)", luid)
        }
    }
}



#[doc(alias = "LUID_AND_ATTRIBUTES")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_privileges)\]
/// LUID_AND_ATTRIBUTES, in the context of TOKEN_PRIVILEGES specifically
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)] pub struct LuidAndAttributes {
    pub luid:       privilege::Luid,
    pub attributes: privilege::Attributes,
}

structure!(@assert layout LuidAndAttributes => winapi::um::winnt::LUID_AND_ATTRIBUTES {
    luid        == Luid,
    attributes  == Attributes,
});

impl LuidAndAttributes {
    pub fn new(luid: impl Into<privilege::Luid>, attributes: impl Into<privilege::Attributes>) -> Self {
        Self { luid: luid.into(), attributes: attributes.into() }
    }
}
