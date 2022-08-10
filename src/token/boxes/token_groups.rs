use crate::*;

use winapi::um::winnt::TOKEN_GROUPS;

use std::fmt::{self, Debug, Formatter};
use std::mem::{size_of, align_of};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_groups)\] ~ `Box<(TOKEN_GROUPS, ..)>`
pub struct BoxTokenGroups(Box<[u8]>);

impl BoxTokenGroups {
    pub unsafe fn from_raw(bytes: Box<[u8]>) -> Self {
        assert!(bytes.len() >= 4);
        assert!(bytes.as_ptr() as usize % Self::GROUPS_ALIGN == 0);
        let btg = Self(bytes);
        assert!(usize::from32(btg.group_count()) <= (btg.0.len()-Self::GROUPS_OFFSET)/size_of::<sid::AndAttributes>());
        btg
    }

    pub fn as_winapi(&self) -> *mut TOKEN_GROUPS { self.0.as_ptr() as *mut _ }
    pub fn size_of_bytes(&self) -> usize { self.0.len() }

    pub fn group_count(&self) -> u32 {
        let b = &*self.0;
        u32::from_ne_bytes([b[0], b[1], b[2], b[3]])
    }

    pub fn groups    <'s>(&'s     self) -> &'s     [sid::AndAttributes<'s>] { let len = self.groups_len(); unsafe { std::slice::from_raw_parts    (self.groups_ptr    (), len) } }
    pub fn groups_mut<'s>(&'s mut self) -> &'s mut [sid::AndAttributes<'s>] { let len = self.groups_len(); unsafe { std::slice::from_raw_parts_mut(self.groups_mut_ptr(), len) } }

    fn groups_len(&self) -> usize { usize::from32(self.group_count()) }

    fn groups_ptr    <'s>(&'s     self) -> *const sid::AndAttributes<'s> { self.0[Self::GROUPS_OFFSET..].as_ptr().cast() }
    fn groups_mut_ptr<'s>(&'s mut self) -> *mut   sid::AndAttributes<'s> { self.0[Self::GROUPS_OFFSET..].as_mut_ptr().cast() }

    const fn max_usize(a: usize, b: usize) -> usize { if a < b { b } else { a } }
    const GROUPS_ALIGN  : usize = Self::max_usize(align_of::<u32>(), align_of::<sid::AndAttributes>());
    const GROUPS_OFFSET : usize = Self::max_usize(size_of ::<u32>(), align_of::<sid::AndAttributes>());
}

impl Debug for BoxTokenGroups {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_list()
            .entries(self.groups().iter())
            .finish()
    }
}
