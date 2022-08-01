use crate::{From32, PrivilegeLuidAndAttributes};

use std::fmt::{self, Debug, Formatter};
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
