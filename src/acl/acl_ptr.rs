use crate::prelude::*;

use winapi::shared::winerror::*;
use winapi::um::securitybaseapi::GetAclInformation;
use winapi::um::winnt::*;

use core::fmt::{self, Debug, Formatter};



#[doc(alias = "PACL")]
#[doc(alias = "ACL")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-acl)\]
/// ≈ Non-null PACL
///
#[derive(Clone, Copy)] #[repr(transparent)] pub struct Ref<'a>(NonNull<ACL>, PhantomData<&'a ACL>);

impl<'a> Ref<'a> {
    /// ### Safety
    /// `acl` should point to a valid [`ACL`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-acl) for the lifetime `'a` given [`acl::Ref<'a>`].
    pub const unsafe fn from_raw_unchecked(acl: NonNull<ACL>) -> Self { Self(acl, PhantomData) }

    /// ### Safety
    /// `acl` should point to a valid [`ACL`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-acl) for the lifetime `'a` given [`acl::Ref<'a>`].
    pub unsafe fn from_raw(acl: NonNull<ACL>, bytes: usize) -> firehazard::Result<Self> {
        if !acl.is_aligned()                        { return Err(Error(ERROR_INVALID_ACL)) }
        if bytes < size_of::<ACL>()                 { return Err(Error(ERROR_INVALID_ACL)) }
        let header : ACL = unsafe { acl.read() };
        if usize::from(header.AclSize) > bytes      { return Err(Error(ERROR_INVALID_ACL)) }
        // TODO: enumerate and validate ACE pointers, sizes, make fn safe, make callers safe
        Ok(Self(acl, PhantomData))
    }

    pub fn acl_revision(self) -> u8 { unsafe { self.0.as_ref().AclRevision } }
    // Sbz1: padding
    pub fn acl_size(self) -> usize { unsafe { self.0.as_ref().AclSize as _ } }
    pub fn ace_count(self) -> usize { unsafe { self.0.as_ref().AceCount as _ } }
    // Sbz2: padding

    pub fn aces(self) -> ace::Iter<'a> { ace::Iter::new(self) }

    pub fn as_pacl(self) -> NonNull<ACL> { self.0 }

    #[doc(alias = "GetAclInformation")]
    #[doc(alias = "AclRevisionInformation")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-getaclinformation)\]
    /// GetAclInformation(..., AclRevisionInformation)
    ///
    #[allow(dead_code)]
    pub(crate) fn get_acl_revision_information(self) -> ACL_REVISION_INFORMATION { unsafe { self.get_acl_information(AclRevisionInformation) } }

    #[doc(alias = "GetAclInformation")]
    #[doc(alias = "AclRevisionInformation")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-getaclinformation)\]
    /// GetAclInformation(..., AclRevisionInformation)
    ///
    #[allow(dead_code)]
    pub(crate) fn get_acl_revision(self) -> acl::Revision { unsafe { acl::Revision::from_unchecked(self.get_acl_revision_information().AclRevision as _) } }

    #[doc(alias = "GetAclInformation")]
    #[doc(alias = "AclSizeInformation")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-getaclinformation)\]
    /// GetAclInformation(..., AclSizeInformation)
    ///
    pub(crate) fn get_acl_size_information(self) -> ACL_SIZE_INFORMATION { unsafe { self.get_acl_information(AclSizeInformation) } }

    #[doc(alias = "GetAclInformation")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-getaclinformation)\]
    /// GetAclInformation(...)
    ///
    /// ### Safety
    /// *   `class` must be valid and must match `T`
    unsafe fn get_acl_information<T: Default>(self, class: u32) -> T {
        let mut info : T = Default::default();
        let success = 0 != unsafe { GetAclInformation(self.0.as_ptr(), &mut info as *mut _ as *mut _, core::mem::size_of::<T>() as _, class) };
        assert!(success, "GetAclInformation failed with {:?}", firehazard::Error::get_last());
        info
    }
}

impl<'a> Debug for Ref<'a> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "acl::Ref(&")?;
        fmt.debug_list().entries(self.aces()).finish()?;
        write!(fmt, ")")
    }
}
