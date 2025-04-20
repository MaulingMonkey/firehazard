use crate::prelude::*;
use crate::security::Descriptor;

use winapi::um::securitybaseapi::*;
use winapi::um::winnt::*;



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
    fn new_revision(revision: u32) -> firehazard::Result<Self> {
        let mut b = Self(Descriptor { desc: Default::default(), phantom: PhantomData });
        firehazard::Error::get_last_if(FALSE == unsafe { InitializeSecurityDescriptor(
            b.as_pdesc(),
            revision,
        )})?;
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
    pub fn dacl<'acl>(
        mut self,
        dacl_present:       bool,
        dacl:               impl Into<Option<acl::Ptr<'acl>>>,
        dacl_defaulted:     bool,
    ) -> firehazard::Result<DescriptorBuilder<'acl>> where 'builder : 'acl {
        firehazard::Error::get_last_if(FALSE == unsafe { SetSecurityDescriptorDacl(
            self.as_pdesc(),
            dacl_present as _,
            dacl.into().map_or(null_mut(), |o| o.as_pacl()),
            dacl_defaulted as _,
        )})?;
        Ok(self)
    }

    #[doc(alias = "SetSecurityDescriptorGroup")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-setsecuritydescriptorgroup)\]
    /// SetSecurityDescriptorGroup
    ///
    pub fn group<'sid>(
        mut self,
        group:              impl Into<Option<&'sid sid::Value>>,
        group_defaulted:    bool,
    ) -> firehazard::Result<DescriptorBuilder<'sid>> where 'builder : 'sid {
        firehazard::Error::get_last_if(FALSE == unsafe { SetSecurityDescriptorGroup(
            self.as_pdesc(),
            group.into().map_or(null_mut(), |o| o.as_psid()),
            group_defaulted as _,
        )})?;
        Ok(self)
    }

    #[doc(alias = "SetSecurityDescriptorOwner")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-setsecuritydescriptorowner)\]
    /// SetSecurityDescriptorOwner
    ///
    pub fn owner<'sid>(
        mut self,
        owner:              impl Into<Option<&'sid sid::Value>>,
        owner_defaulted:    bool,
    ) -> firehazard::Result<DescriptorBuilder<'sid>> where 'builder : 'sid {
        firehazard::Error::get_last_if(FALSE == unsafe { SetSecurityDescriptorOwner(
            self.as_pdesc(),
            owner.into().map_or(null_mut(), |o| o.as_psid()),
            owner_defaulted as _,
        )})?;
        Ok(self)
    }

    // https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-setsecuritydescriptorrmcontrol
    // TODO: SetSecurityDescriptorRMControl

    #[doc(alias = "SetSecurityDescriptorSacl")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-setsecuritydescriptorsacl)\]
    /// SetSecurityDescriptorSacl
    ///
    pub fn sacl<'acl>(
        mut self,
        sacl_present:       bool,
        sacl:               impl Into<Option<acl::Ptr<'acl>>>,
        sacl_defaulted:     bool,
    ) -> firehazard::Result<DescriptorBuilder<'acl>> where 'builder : 'acl {
        firehazard::Error::get_last_if(FALSE == unsafe { SetSecurityDescriptorSacl(
            self.as_pdesc(),
            sacl_present as _,
            sacl.into().map_or(null_mut(), |o| o.as_pacl()),
            sacl_defaulted as _,
        )})?;
        Ok(self)
    }

    pub fn finish(self) -> Descriptor<'builder> { self.0 }

    fn as_pdesc(&mut self) -> PSECURITY_DESCRIPTOR {
        let desc : *mut SECURITY_DESCRIPTOR = &mut self.0.desc;
        desc.cast()
    }
}
