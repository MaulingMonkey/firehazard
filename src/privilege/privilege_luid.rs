use crate::*;
use winapi::um::winnt::LUID;
use core::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/secauthz/privilege-constants)\]
/// LUID, referencing a [privilege](https://docs.microsoft.com/en-us/windows/win32/secauthz/privilege-constants#constants) such as `"SeShutdownPrivilege"`
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
