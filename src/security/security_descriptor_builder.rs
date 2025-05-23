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
    /// ### Arguments
    ///
    /// | `dacl`            | Present   | Behavior  |
    /// | ------------------| ----------| ----------|
    /// | [`acl::Default`]  | false     | Default DACL for object type
    /// | [`acl::Null`]     | true      | ❌ All access for everyone, no security! ❌
    /// | [`acl::Ref`]      | true      | A custom DACL
    ///
    pub fn dacl<'acl>(
        mut self,
        dacl:               impl acl::InDefaultOrNullOrRef<'acl>,
        dacl_defaulted:     bool,
    ) -> firehazard::Result<DescriptorBuilder<'acl>> where 'builder : 'acl {
        let (present, dacl) = dacl.into_present_ptr();
        firehazard::Error::get_last_if(FALSE == unsafe { SetSecurityDescriptorDacl(
            self.as_pdesc(),
            present as _,
            dacl.map_or(null_mut(), |p| p.as_pacl().as_ptr()),
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
    /// ### Arguments
    ///
    /// | `sacl`            | Present   | Behavior  |
    /// | ------------------| ----------| ----------|
    /// | [`acl::Default`]  | false     | Default SACL for object type
    /// | [`acl::Null`]     | true      | ❌ All access for everyone? No security? ❌
    /// | [`acl::Ref`]      | true      | A custom SACL
    ///
    pub fn sacl<'acl>(
        mut self,
        sacl:               impl acl::InDefaultOrNullOrRef<'acl>,
        sacl_defaulted:     bool,
    ) -> firehazard::Result<DescriptorBuilder<'acl>> where 'builder : 'acl {
        let (present, sacl) = sacl.into_present_ptr();
        firehazard::Error::get_last_if(FALSE == unsafe { SetSecurityDescriptorSacl(
            self.as_pdesc(),
            present as _,
            sacl.map_or(null_mut(), |p| p.as_pacl().as_ptr()),
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
