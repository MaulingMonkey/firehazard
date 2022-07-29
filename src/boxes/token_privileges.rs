use winapi::shared::ntdef::LUID;
use winapi::um::winnt::LUID_AND_ATTRIBUTES;

use crate::From32;

use std::fmt::{self, Debug, Formatter};
use std::hash::Hash;
use std::mem::{size_of, align_of};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_privileges)\] ~ `Box<(TOKEN_PRIVILEGES, ..)>`
pub struct BoxTokenPrivileges(Box<[u8]>);

impl BoxTokenPrivileges {
    pub unsafe fn from_raw(bytes: Box<[u8]>) -> Self {
        assert!(bytes.len() >= 4);
        let btg = Self(bytes);
        assert!(usize::from32(btg.privilege_count()) <= (btg.0.len()-Self::PRIVILEGES_OFFSET)/size_of::<PrivilegeLuidAndAttributes>());
        assert!(btg.privileges_ptr() as usize % Self::PRIVILEGES_ALIGN == 0);
        btg
    }

    pub fn privilege_count(&self) -> u32 {
        let b = &*self.0;
        u32::from_ne_bytes([b[0], b[1], b[2], b[3]])
    }

    pub fn privileges(&self) -> &[PrivilegeLuidAndAttributes] {
        unsafe { std::slice::from_raw_parts(self.privileges_ptr(), usize::from32(self.privilege_count())) }
    }

    fn privileges_ptr(&self) -> *const PrivilegeLuidAndAttributes {
        self.0[Self::PRIVILEGES_OFFSET..].as_ptr().cast()
    }

    const fn max_usize(a: usize, b: usize) -> usize { if a < b { b } else { a } }
    const PRIVILEGES_ALIGN  : usize = Self::max_usize(align_of::<u32>(), align_of::<PrivilegeLuidAndAttributes>());
    const PRIVILEGES_OFFSET : usize = Self::max_usize(size_of ::<u32>(), align_of::<PrivilegeLuidAndAttributes>());
}

impl Debug for BoxTokenPrivileges {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_list()
            .entries(self.privileges().iter())
            .finish()
    }
}



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_privileges)\] LUID_AND_ATTRIBUTES, in the context of TOKEN_PRIVILEGES specifically
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)] #[repr(C)] pub struct PrivilegeLuidAndAttributes {
    pub luid:       Luid,
    pub attributes: u32,
}

impl Debug for PrivilegeLuidAndAttributes {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        // TODO: name luid via https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-lookupprivilegenamea
        // TODO: name attributes via https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_privileges
        write!(fmt, "PrivilegeLuidAndAttributes {{ luid: 0x{:016x}, attributes: 0x{:08x} }}", u64::from(self.luid), self.attributes)
    }
}

const _LUID_AND_ATTRIBUTES_SIZE  : () = assert!(align_of::<LUID_AND_ATTRIBUTES>() == align_of::<PrivilegeLuidAndAttributes>());
const _LUID_AND_ATTRIBUTES_ALIGN : () = assert!(size_of ::<LUID_AND_ATTRIBUTES>() == size_of ::<PrivilegeLuidAndAttributes>());



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-luid)\] LUID (~ a 32-bit aligned `u64` / "Locally Unique IDentifier")
#[derive(Clone, Copy)] #[repr(transparent)] pub struct Luid(LUID);
impl Debug      for Luid { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "Luid(0x{:08x})", u64::from(*self)) } }
impl From<u64>  for Luid { fn from(value: u64) -> Self { Self(LUID { HighPart: (value>>32) as _, LowPart: value as _ }) } }
impl From<Luid> for u64  { fn from(value: Luid) -> Self { (value.0.HighPart as u64) << 32 | value.0.LowPart as u64 } }
impl PartialEq  for Luid { fn eq(&self, other: &Self) -> bool { u64::from(*self) == u64::from(*other) } }
impl Eq         for Luid {}
impl PartialOrd for Luid { fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { u64::from(*self).partial_cmp(&u64::from(*other)) } }
impl Ord        for Luid { fn cmp(&self, other: &Self) -> std::cmp::Ordering { u64::from(*self).cmp(&u64::from(*other)) } }
impl Hash       for Luid { fn hash<H: std::hash::Hasher>(&self, state: &mut H) { u64::from(*self).hash(state) } }
const _LUID_SIZE  : () = assert!(size_of ::<LUID>() == size_of ::<Luid>());
const _LUID_ALIGN : () = assert!(align_of::<LUID>() == align_of::<Luid>());
