use crate::*;

use winapi::um::winnt::TOKEN_PRIVILEGES;

use std::fmt::{self, Debug, Formatter};
use std::mem::{size_of, align_of, size_of_val};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_privileges)\] ~ `Box<(TOKEN_PRIVILEGES, ..)>`
pub struct BoxTokenPrivileges(Box<[u8]>);

impl BoxTokenPrivileges {
    pub unsafe fn from_raw(bytes: Box<[u8]>) -> Self {
        assert!(bytes.len() >= 4);
        assert!(bytes.as_ptr() as usize % Self::PRIVILEGES_ALIGN == 0);
        let btg = Self(bytes);
        assert!(usize::from32(btg.privilege_count()) <= (btg.0.len()-Self::PRIVILEGES_OFFSET)/size_of::<privilege::LuidAndAttributes>());
        btg
    }

    pub fn new(v: impl Into<Self>) -> Self { v.into() }

    pub fn privilege_count(&self) -> u32 {
        let b = &*self.0;
        u32::from_ne_bytes([b[0], b[1], b[2], b[3]])
    }

    pub fn privileges    (&    self) -> &    [privilege::LuidAndAttributes] { unsafe { std::slice::from_raw_parts    (self.privileges_ptr    (), self.privileges_len()) } }
    pub fn privileges_mut(&mut self) -> &mut [privilege::LuidAndAttributes] { unsafe { std::slice::from_raw_parts_mut(self.privileges_mut_ptr(), self.privileges_len()) } }

    pub fn as_token_privileges_mut_ptr(&mut self) -> *mut TOKEN_PRIVILEGES { self.0.as_mut_ptr().cast() }

    fn privileges_len(&self) -> usize { usize::from32(self.privilege_count()) }

    fn privileges_ptr    (&    self) -> *const privilege::LuidAndAttributes { self.0[Self::PRIVILEGES_OFFSET..].as_ptr    ().cast() }
    fn privileges_mut_ptr(&mut self) -> *mut   privilege::LuidAndAttributes { self.0[Self::PRIVILEGES_OFFSET..].as_mut_ptr().cast() }

    const fn max_usize(a: usize, b: usize) -> usize { if a < b { b } else { a } }
    const PRIVILEGES_ALIGN  : usize = Self::max_usize(align_of::<u32>(), align_of::<privilege::LuidAndAttributes>());
    const PRIVILEGES_OFFSET : usize = Self::max_usize(size_of ::<u32>(), align_of::<privilege::LuidAndAttributes>());
}

impl Debug for BoxTokenPrivileges {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_list()
            .entries(self.privileges().iter())
            .finish()
    }
}

impl From<&'_ [privilege::LuidAndAttributes]> for BoxTokenPrivileges {
    fn from(laa: &'_ [privilege::LuidAndAttributes]) -> Self {
        let len32 = u32::try_from(laa.len()).unwrap();
        let n_bytes = BoxTokenPrivileges::PRIVILEGES_OFFSET + size_of_val(laa);
        let mut data = Box::<[u8]>::from(vec![0u8; n_bytes]);
        data[0..4].copy_from_slice(&len32.to_ne_bytes());
        let mut data = unsafe { BoxTokenPrivileges::from_raw(data) };
        data.privileges_mut().copy_from_slice(laa);
        data
    }
}
