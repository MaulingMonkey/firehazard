use crate::*;

use winapi::shared::winerror::*;
use winapi::um::securitybaseapi::GetAclInformation;
use winapi::um::winnt::*;

use core::fmt::{self, Debug, Formatter};
use core::marker::PhantomData;
use core::mem::{align_of, size_of};



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-acl)\] ~ PACL
#[derive(Clone, Copy)] #[repr(transparent)] pub struct Ptr<'a>(*mut ACL, PhantomData<&'a ACL>);

impl Ptr<'_> {
    /// ### Safety
    /// `acl` should point to a valid [`ACL`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-acl) for the lifetime `'a` given [`acl::Ptr<'a>`].
    pub const unsafe fn from_raw_unchecked(acl: *mut ACL) -> Self { Self(acl, PhantomData) }

    /// ### Safety
    /// `acl` should be null, or point to a valid [`ACL`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-acl) for the lifetime `'a` given [`acl::Ptr<'a>`].
    pub unsafe fn from_raw(acl: *mut ACL, bytes: usize) -> Result<Self, Error> {
        if acl.is_null()                            { return Ok(Self(acl, PhantomData)) }
        if acl as usize % align_of::<ACL>() != 0    { return Err(Error(ERROR_INVALID_ACL)) }
        if bytes < size_of::<ACL>()                 { return Err(Error(ERROR_INVALID_ACL)) }
        let header : ACL = unsafe { *acl };
        if usize::from(header.AclSize) > bytes      { return Err(Error(ERROR_INVALID_ACL)) }
        // TODO: enumerate and validate ACE pointers, sizes, make fn safe, make callers safe
        Ok(Self(acl, PhantomData))
    }

    pub fn acl_revision(&self) -> u8 { unsafe { (*self.0).AclRevision } }
    // Sbz1: padding
    pub fn acl_size(&self) -> usize { unsafe { (*self.0).AclSize as _ } }
    pub fn ace_count(&self) -> usize { unsafe { (*self.0).AceCount as _ } }
    // Sbz2: padding

    pub fn aces(&self) -> ace::Iter { ace::Iter::new(*self) }

    pub fn as_pacl(self) -> PACL { self.0.cast() }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-getaclinformation)\]
    /// GetAclInformation(..., AclRevisionInformation)
    #[allow(dead_code)]
    pub(crate) fn get_acl_revision_information(&self) -> ACL_REVISION_INFORMATION { unsafe { self.get_acl_information(AclRevisionInformation) } }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-getaclinformation)\]
    /// GetAclInformation(..., AclRevisionInformation)
    #[allow(dead_code)]
    pub(crate) fn get_acl_revision(&self) -> acl::Revision { unsafe { acl::Revision::from_unchecked(self.get_acl_revision_information().AclRevision as _) } }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-getaclinformation)\]
    /// GetAclInformation(..., AclSizeInformation)
    pub(crate) fn get_acl_size_information(&self) -> ACL_SIZE_INFORMATION { unsafe { self.get_acl_information(AclSizeInformation) } }

    /// ### Safety
    /// *   `class` must be valid and must match `T`
    unsafe fn get_acl_information<T: Default>(&self, class: u32) -> T {
        let mut info : T = Default::default();
        let success = 0 != unsafe { GetAclInformation(self.0, &mut info as *mut _ as *mut _, core::mem::size_of::<T>() as _, class) };
        assert!(success, "GetAclInformation failed with {:?}", Error::get_last());
        info
    }
}

impl<'a> Debug for Ptr<'a> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        if self.0.is_null() { return write!(fmt, "NULL") }
        write!(fmt, "acl::Ptr( &ACL {{ AceCount: {}, ", self.ace_count())?;
        fmt.debug_list().entries(self.aces()).finish()?;
        write!(fmt, " }} )")
    }
}
