use super::*;

use crate::prelude::*;
use crate::alloc::{CBox, CBoxSized};

use winapi::um::winnt::TOKEN_GROUPS;

use core::fmt::{self, Debug, Formatter};



#[doc(alias = "TOKEN_GROUPS")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_groups)\]
/// ≈ `Box<(TOKEN_GROUPS, ..)>`
///
#[repr(transparent)] pub struct BoxTokenGroups(CBox<TOKEN_GROUPS>);

impl BoxTokenGroups {
    pub fn from_raw(cbs: CBoxSized<TOKEN_GROUPS>) -> Self {
        let groups = unsafe { assert_valid_after_header_slice(&cbs, cbs.Groups.as_ptr(), cbs.GroupCount, true) };
        for group in groups.iter() { assert_valid_saa(&cbs, *group) } // REQUIRED FOR SOUNDNESS
        Self(cbs.into())
    }

    pub fn as_winapi(&self) -> *mut TOKEN_GROUPS { self.0.as_ptr() as *mut _ }

    pub fn group_count(&self) -> u32 { self.0.GroupCount }

    pub fn groups    <'s>(&'s     self) -> &'s     [sid::AndAttributes<'s>] { unsafe { slice::from_flexible_array_ref(self.0.as_ptr()    .cast::<TokenGroups>(), |g| usize::from32(g.group_count), |g| &raw const (*g).groups) } }
    pub fn groups_mut<'s>(&'s mut self) -> &'s mut [sid::AndAttributes<'s>] { unsafe { slice::from_flexible_array_mut(self.0.as_mut_ptr().cast::<TokenGroups>(), |g| usize::from32(g.group_count), |g| &raw mut   (*g).groups) } }
}

impl Debug for BoxTokenGroups {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_list()
            .entries(self.groups().iter())
            .finish()
    }
}



#[doc(alias = "TOKEN_GROUPS")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_groups)\]
/// TOKEN_GROUPS
///
#[repr(C)] struct TokenGroups<'s> {
    group_count:    u32,
    groups:         [sid::AndAttributes<'s>; 1],
}

structure!(@assert layout TokenGroups<'_> => TOKEN_GROUPS {
    group_count     == GroupCount,
    groups          == Groups,
});
