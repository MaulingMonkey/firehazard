use super::assert_valid_saa;

use crate::*;
use crate::alloc::*;

use winapi::um::winnt::{TOKEN_GROUPS, SID_AND_ATTRIBUTES};

use core::fmt::{self, Debug, Formatter};
use core::mem::{size_of, align_of};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_groups)\] ~ `Box<(TOKEN_GROUPS, ..)>`
pub struct BoxTokenGroups(CBox<TOKEN_GROUPS>);

impl BoxTokenGroups {
    pub fn from_raw(cbs: CBoxSized<TOKEN_GROUPS>) -> Self {
        let group_count = usize::from32(cbs.GroupCount);
        assert!(group_count <= (cbs.bytes()-Self::GROUPS_OFFSET)/size_of::<sid::AndAttributes>());

        let groups = provenance_addr(cbs.as_ptr(), cbs.Groups.as_ptr());
        let groups = unsafe { core::slice::from_raw_parts::<SID_AND_ATTRIBUTES>(groups, group_count) };
        for group in groups.iter() { assert_valid_saa(&cbs, *group) } // REQUIRED FOR SOUNDNESS

        Self(cbs.into())
    }

    pub fn as_winapi(&self) -> *mut TOKEN_GROUPS { self.0.as_ptr() as *mut _ }

    pub fn group_count(&self) -> u32 { self.0.GroupCount }

    pub fn groups    <'s>(&'s     self) -> &'s     [sid::AndAttributes<'s>] { let len = self.groups_len(); unsafe { core::slice::from_raw_parts    (self.groups_ptr    (), len) } }
    pub fn groups_mut<'s>(&'s mut self) -> &'s mut [sid::AndAttributes<'s>] { let len = self.groups_len(); unsafe { core::slice::from_raw_parts_mut(self.groups_mut_ptr(), len) } }

    fn groups_len(&self) -> usize { usize::from32(self.group_count()) }

    fn groups_ptr    <'s>(&'s     self) -> *const sid::AndAttributes<'s> { provenance_addr    (self.0.as_ptr(),     self.0.Groups.as_ptr().cast()    ) }
    fn groups_mut_ptr<'s>(&'s mut self) -> *mut   sid::AndAttributes<'s> { provenance_addr_mut(self.0.as_mut_ptr(), self.0.Groups.as_mut_ptr().cast()) }

    const fn max_usize(a: usize, b: usize) -> usize { if a < b { b } else { a } }
    const GROUPS_OFFSET : usize = Self::max_usize(size_of ::<u32>(), align_of::<sid::AndAttributes>());
}

impl Debug for BoxTokenGroups {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_list()
            .entries(self.groups().iter())
            .finish()
    }
}
