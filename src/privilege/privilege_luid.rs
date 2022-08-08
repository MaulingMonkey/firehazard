use crate::*;
use crate::error::{LastError, get_last_error};

use winapi::shared::winerror::*;
use winapi::um::winbase::{LookupPrivilegeNameA, LookupPrivilegeValueA};
use winapi::um::winnt::LUID;

use std::fmt::{self, Debug, Formatter};
use std::ptr::null_mut;



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
        if let Ok(name) = self.lookup_privilege_name_a() {
            write!(fmt, "privilege::Luid(0x{:x} {:?})", luid, name)
        } else {
            write!(fmt, "privilege::Luid(0x{:x} ???)", luid)
        }
    }
}

impl privilege::Luid {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-lookupprivilegevaluea)\] LookupPrivilegeValueA
    pub fn lookup_privilege_value_a(name: impl abistr::AsCStr) -> Result<Self, LastError> {
        let name = name.as_cstr();
        let mut luid = crate::Luid::from(0u64);
        let succeeded = 0 != unsafe { LookupPrivilegeValueA(null_mut(), name, &mut luid.0) };
        if succeeded { Ok(Self(luid)) } else { Err(LastError::get()) }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-lookupprivilegenamea)\] LookupPrivilegeNameA
    pub fn lookup_privilege_name_a(mut self) -> Result<String, LastError> {
        let system_name = null_mut();
        let luid = &mut self.0.0;
        let mut len = 0;
        if 0 == unsafe { LookupPrivilegeNameA(system_name, luid, null_mut(), &mut len) } {
            match get_last_error() {
                ERROR_INSUFFICIENT_BUFFER   => {},
                other                       => return Err(LastError(other)),
            }
        }
        let mut buf = vec![0u8; usize::from32(len)];
        let succeeded = 0 != unsafe { LookupPrivilegeNameA(system_name, luid, buf.as_mut_ptr().cast(), &mut len) };
        if !succeeded { return Err(LastError::get()) }
        buf.shrink_to(usize::from32(len)); // on the off chance that len shrunk (if it grew, we would've already returned `LastError(ERROR_INSUFFICIENT_BUFFER)`)
        assert!(buf.pop() == Some(b'\0'), "BUG: privilege name was expected to be null terminated");
        String::from_utf8(buf).map_err(|_| LastError(ERROR_INVALID_DATA))
    }
}