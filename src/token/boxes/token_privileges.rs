use super::*;

use crate::prelude::*;
use crate::alloc::{CBox, CBoxSized};

use winapi::um::winnt::TOKEN_PRIVILEGES;

use core::fmt::{self, Debug, Formatter};
use core::mem::offset_of;



#[doc(alias = "TOKEN_PRIVILEGES")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_privileges)\]
/// ≈ `Box<(TOKEN_PRIVILEGES, ..)>`
///
#[repr(transparent)] pub struct BoxTokenPrivileges(CBox<TOKEN_PRIVILEGES>);

impl BoxTokenPrivileges {
    pub fn from_raw(cbs: CBoxSized<TOKEN_PRIVILEGES>) -> Self {
        let _privs = unsafe { assert_valid_after_header_slice(&cbs, cbs.Privileges.as_ptr(), cbs.PrivilegeCount, true) };
        Self(cbs.into())
    }

    pub fn new(v: impl Into<Self>) -> Self { v.into() }

    pub fn as_winapi(&mut self) -> *mut TOKEN_PRIVILEGES { self.0.as_mut_ptr().cast() }

    pub fn privilege_count(&self) -> u32 { self.0.PrivilegeCount }

    pub fn privileges    (&    self) -> &    [privilege::LuidAndAttributes] { unsafe { slice::from_flexible_array_ref(self.0.as_ptr()    .cast::<TokenPrivileges>(), |p| usize::from32(p.privilege_count), |p| &raw const (*p).privileges) } }
    pub fn privileges_mut(&mut self) -> &mut [privilege::LuidAndAttributes] { unsafe { slice::from_flexible_array_mut(self.0.as_mut_ptr().cast::<TokenPrivileges>(), |p| usize::from32(p.privilege_count), |p| &raw mut   (*p).privileges) } }

    #[doc(hidden)] #[deprecated = "renamed to `as_winapi`"]
    pub fn as_token_privileges_mut_ptr(&mut self) -> *mut TOKEN_PRIVILEGES { self.as_winapi() }
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
        let n_bytes = offset_of!(TOKEN_PRIVILEGES, Privileges) + size_of_val(laa).max(size_of::<privilege::LuidAndAttributes>());
        let mut data = CBoxSized::<TOKEN_PRIVILEGES>::new_oversized(Default::default(), n_bytes);
        data.PrivilegeCount = len32;
        let mut data = BoxTokenPrivileges::from_raw(data);
        data.privileges_mut().copy_from_slice(laa);
        data
    }
}



#[doc(alias = "TOKEN_PRIVILEGES")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_privileges)\]
/// TOKEN_PRIVILEGES
///
#[repr(C)] struct TokenPrivileges {
    privilege_count:    u32,
    privileges:         [privilege::LuidAndAttributes; 1],
}

structure!(@assert layout TokenPrivileges => TOKEN_PRIVILEGES {
    privilege_count     == PrivilegeCount,
    privileges          == Privileges,
});
