use crate::*;
use crate::security::Descriptor;

use winapi::shared::minwindef::FALSE;
use winapi::um::securitybaseapi::*;
use winapi::um::winnt::*;

use core::marker::PhantomData;
use core::ptr::null_mut;



#[doc(alias = "SECURITY_DESCRIPTOR")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-security_descriptor)\]
/// SECURITY_DESCRIPTOR Builder
///
#[repr(transparent)] pub struct DescriptorBuilder<'b>(Descriptor<'b>);

impl DescriptorBuilder<'static> {
    #[doc(alias = "InitializeSecurityDescriptor")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-initializesecuritydescriptor)\]
    /// InitializeSecurityDescriptor(..., SECURITY_DESCRIPTOR_REVISION)
    ///
    pub fn new() -> Self { Self::new_revision(SECURITY_DESCRIPTOR_REVISION).unwrap() }

    #[doc(alias = "InitializeSecurityDescriptor")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-initializesecuritydescriptor)\]
    /// InitializeSecurityDescriptor
    ///
    fn new_revision(revision: u32) -> Result<Self, Error> {
        let mut b = Self(Descriptor { desc: Default::default(), phantom: PhantomData });
        Error::get_last_if(FALSE == unsafe { InitializeSecurityDescriptor(b.as_pdesc(), revision) })?;
        Ok(b)
    }
}

impl<'builder> DescriptorBuilder<'builder> {
    // https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-setsecuritydescriptorcontrol
    // TODO: SetSecurityDescriptorControl

    #[doc(alias = "SetSecurityDescriptorDacl")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-setsecuritydescriptordacl)\]
    /// SetSecurityDescriptorDacl
    ///
    pub fn dacl<'acl>(mut self, dacl_present: bool, dacl: impl Into<Option<acl::Ptr<'acl>>>, dacl_defaulted: bool) -> Result<DescriptorBuilder<'acl>, Error> where 'builder : 'acl {
        let dacl = dacl.into();
        let dacl = dacl.map_or(null_mut(), |o| o.as_pacl());
        Error::get_last_if(FALSE == unsafe { SetSecurityDescriptorDacl(self.as_pdesc(), dacl_present as _, dacl, dacl_defaulted as _) })?;
        Ok(self)
    }

    #[doc(alias = "SetSecurityDescriptorGroup")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-setsecuritydescriptorgroup)\]
    /// SetSecurityDescriptorGroup
    ///
    pub fn group<'sid>(mut self, group: impl Into<Option<&'sid sid::Value>>, group_defaulted: bool) -> Result<DescriptorBuilder<'sid>, Error> where 'builder : 'sid {
        let group = group.into();
        let group = group.map_or(null_mut(), |o| o.as_psid());
        Error::get_last_if(FALSE == unsafe { SetSecurityDescriptorGroup(self.as_pdesc(), group, group_defaulted as _) })?;
        Ok(self)
    }

    #[doc(alias = "SetSecurityDescriptorOwner")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-setsecuritydescriptorowner)\]
    /// SetSecurityDescriptorOwner
    ///
    pub fn owner<'sid>(mut self, owner: impl Into<Option<&'sid sid::Value>>, owner_defaulted: bool) -> Result<DescriptorBuilder<'sid>, Error> where 'builder : 'sid {
        let owner = owner.into();
        let owner = owner.map_or(null_mut(), |o| o.as_psid());
        Error::get_last_if(FALSE == unsafe { SetSecurityDescriptorOwner(self.as_pdesc(), owner, owner_defaulted as _) })?;
        Ok(self)
    }

    // https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-setsecuritydescriptorrmcontrol
    // TODO: SetSecurityDescriptorRMControl

    #[doc(alias = "SetSecurityDescriptorSacl")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-setsecuritydescriptorsacl)\]
    /// SetSecurityDescriptorSacl
    ///
    pub fn sacl<'acl>(mut self, sacl_present: bool, sacl: impl Into<Option<acl::Ptr<'acl>>>, sacl_defaulted: bool) -> Result<DescriptorBuilder<'acl>, Error> where 'builder : 'acl {
        let sacl = sacl.into();
        let sacl = sacl.map_or(null_mut(), |o| o.as_pacl());
        Error::get_last_if(FALSE == unsafe { SetSecurityDescriptorSacl(self.as_pdesc(), sacl_present as _, sacl, sacl_defaulted as _) })?;
        Ok(self)
    }

    pub fn finish(self) -> Descriptor<'builder> { self.0 }

    fn as_pdesc(&mut self) -> PSECURITY_DESCRIPTOR {
        let desc : *mut SECURITY_DESCRIPTOR = &mut self.0.desc;
        desc.cast()
    }
}
