use crate::*;
use crate::alloc::*;

use winapi::um::winnt::TOKEN_PRIVILEGES;

use core::fmt::{self, Debug, Formatter};
use core::mem::{size_of, align_of, size_of_val, zeroed};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_privileges)\] ~ `Box<(TOKEN_PRIVILEGES, ..)>`
pub struct BoxTokenPrivileges(CBox<TOKEN_PRIVILEGES>);

impl BoxTokenPrivileges {
    pub unsafe fn from_raw(cbs: CBoxSized<TOKEN_PRIVILEGES>) -> Self {
        let bytes = cbs.bytes();
        assert!(bytes >= 4);
        assert!(cbs.as_ptr() as usize % Self::PRIVILEGES_ALIGN == 0); // TODO: static assert size instead
        let btg = Self(cbs.into());
        assert!(usize::from32(btg.privilege_count()) <= (bytes-Self::PRIVILEGES_OFFSET)/size_of::<privilege::LuidAndAttributes>());
        btg
    }

    pub fn new(v: impl Into<Self>) -> Self { v.into() }

    pub fn privilege_count(&self) -> u32 { self.0.PrivilegeCount }

    pub fn privileges    (&    self) -> &    [privilege::LuidAndAttributes] { unsafe { core::slice::from_raw_parts    (self.privileges_ptr    (), self.privileges_len()) } }
    pub fn privileges_mut(&mut self) -> &mut [privilege::LuidAndAttributes] { unsafe { core::slice::from_raw_parts_mut(self.privileges_mut_ptr(), self.privileges_len()) } }

    pub fn as_token_privileges_mut_ptr(&mut self) -> *mut TOKEN_PRIVILEGES { self.0.as_mut_ptr().cast() }

    fn privileges_len(&self) -> usize { usize::from32(self.privilege_count()) }

    // XXX: Not 100% sure this avoids [Strict Providence](https://doc.rust-lang.org/std/ptr/index.html#strict-provenance) spatial narrowing to Groups[0]
    fn privileges_ptr    (&    self) -> *const privilege::LuidAndAttributes { unsafe { core::ptr::addr_of!    ((*self.0.as_ptr    ()).Privileges).cast() } }
    fn privileges_mut_ptr(&mut self) -> *mut   privilege::LuidAndAttributes { unsafe { core::ptr::addr_of_mut!((*self.0.as_mut_ptr()).Privileges).cast() } }

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
        let mut data = CBoxSized::<TOKEN_PRIVILEGES>::new_oversized(unsafe{zeroed()}, n_bytes);
        data.PrivilegeCount = len32;
        let mut data = unsafe { BoxTokenPrivileges::from_raw(data) };
        data.privileges_mut().copy_from_slice(laa);
        data
    }
}
