use crate::*;
use crate::alloc::*;

use winapi::um::winnt::TOKEN_GROUPS;

use core::fmt::{self, Debug, Formatter};
use core::mem::{size_of, align_of};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_groups)\] ~ `Box<(TOKEN_GROUPS, ..)>`
pub struct BoxTokenGroups(CBox<TOKEN_GROUPS>);

impl BoxTokenGroups {
    pub unsafe fn from_raw(cbs: CBoxSized<TOKEN_GROUPS>) -> Self {
        let bytes = cbs.bytes();
        assert!(bytes >= 4);
        assert!(cbs.as_ptr() as usize % Self::GROUPS_ALIGN == 0); // TODO: static assert alignment instead
        let btg = Self(cbs.into());
        assert!(usize::from32(btg.group_count()) <= (bytes-Self::GROUPS_OFFSET)/size_of::<sid::AndAttributes>());
        btg
    }

    pub fn as_winapi(&self) -> *mut TOKEN_GROUPS { self.0.as_ptr() as *mut _ }

    pub fn group_count(&self) -> u32 { self.0.GroupCount }

    pub fn groups    <'s>(&'s     self) -> &'s     [sid::AndAttributes<'s>] { let len = self.groups_len(); unsafe { core::slice::from_raw_parts    (self.groups_ptr    (), len) } }
    pub fn groups_mut<'s>(&'s mut self) -> &'s mut [sid::AndAttributes<'s>] { let len = self.groups_len(); unsafe { core::slice::from_raw_parts_mut(self.groups_mut_ptr(), len) } }

    fn groups_len(&self) -> usize { usize::from32(self.group_count()) }

    // XXX: Not 100% sure this avoids [Strict Providence](https://doc.rust-lang.org/std/ptr/index.html#strict-provenance) spatial narrowing to Groups[0]
    fn groups_ptr    <'s>(&'s     self) -> *const sid::AndAttributes<'s> { unsafe { core::ptr::addr_of!    ((*self.0.as_ptr    ()).Groups).cast() } }
    fn groups_mut_ptr<'s>(&'s mut self) -> *mut   sid::AndAttributes<'s> { unsafe { core::ptr::addr_of_mut!((*self.0.as_mut_ptr()).Groups).cast() } }

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
