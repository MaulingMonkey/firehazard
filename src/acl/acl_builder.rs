use crate::prelude::*;

use winapi::shared::guiddef::GUID;
use winapi::shared::minwindef::FALSE;
use winapi::shared::winerror::*;
use winapi::um::securitybaseapi::*;
use winapi::um::winnt::ACL;

/// `WORD` sizes etc. limit the size to `0xFFFF`.
/// However, InitializeAcl will fail with `0xFFFF`, since the end isn't DWORD aligned.
/// Instead, specify `0xFFFC` which *is* DWORD aligned.
macro_rules! max_acl_bytes { () => { 0xFFFC } }



pub struct Builder {
    u: U,
}

#[repr(C)] union U {
    bytes:  [u8; max_acl_bytes!()], // max acl is 64 KiB
    acl:    ACL,
}

impl<'a> From<&'a mut Builder> for acl::Ptr<'a> { fn from(b: &'a mut Builder) -> Self { b.as_acl_ptr() } }

impl Builder {
    #[doc(alias = "InitializeAcl")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-initializeacl)\]
    /// InitializeAcl
    ///
    pub fn new(revision: acl::Revision) -> Self {
        let mut b = Self { u: U { bytes: [0; max_acl_bytes!()] } };
        assert!(FALSE != unsafe { InitializeAcl(
            &mut b.u.acl,
            max_acl_bytes!(),
            revision.into(),
        )}, "GetLastError()=={:?}", firehazard::Error::get_last());
        b
    }

    fn as_winapi(&mut self) -> *mut ACL { unsafe { &mut self.u.acl } }
    fn as_winapi_const(&self) -> *mut ACL { unsafe { &self.u.acl as *const _ as *mut _ } }
    pub fn as_acl_ptr(&self) -> acl::Ptr { unsafe { acl::Ptr::from_raw_unchecked(&self.u.acl as *const _ as *mut _) } }
    #[allow(dead_code)] pub(crate) fn as_bytes(&self) -> &[u8] { let len = usize::from32(self.as_acl_ptr().get_acl_size_information().AclBytesInUse); unsafe { &self.u.bytes[..len] } }


    pub fn finish(&mut self) -> firehazard::Result<&mut Self> {
        // TODO: eliminate the need for this by only temporarilly growing AclSize immediately before add_*_ace and then shrinking it afterwards?
        let size = self.as_acl_ptr().get_acl_size_information().AclBytesInUse;
        let size = u16::try_from(size).map_err(|_| ERROR_ALLOTTED_SPACE_EXCEEDED)?;
        self.u.acl.AclSize = size;
        Ok(self)
    }

    // https://learn.microsoft.com/en-us/windows/win32/secauthz/authorization-functions

    #[doc(alias = "AddAccessAllowedAce")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-addaccessallowedace)\]
    /// AddAccessAllowedAce
    ///
    pub fn add_access_allowed_ace<'acl, 'sid>(&'acl mut self,
        ace_revision:   acl::Revision,
        access_mask:    impl Into<access::Mask>,
        sid:            impl Into<sid::Ptr<'sid>>,
    ) -> firehazard::Result<&'acl mut Self> {
        firehazard::Error::get_last_if(FALSE == unsafe { AddAccessAllowedAce(
            self.as_winapi(),
            ace_revision.into(),
            access_mask.into().into(),
            sid.into().as_psid(),
        )})?;
        Ok(self)
    }

    #[doc(alias = "AddAccessAllowedAceEx")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-addaccessallowedaceex)\]
    /// AddAccessAllowedAceEx
    ///
    pub fn add_access_allowed_ace_ex<'acl, 'sid>(&'acl mut self,
        ace_revision:   acl::Revision,
        ace_flags:      ace::Flags,
        access_mask:    impl Into<access::Mask>,
        sid:            impl Into<sid::Ptr<'sid>>,
    ) -> firehazard::Result<&'acl mut Self> {
        firehazard::Error::get_last_if(FALSE == unsafe { AddAccessAllowedAceEx(
            self.as_winapi(),
            ace_revision.into(),
            ace_flags.into(),
            access_mask.into().into(),
            sid.into().as_psid(),
        )})?;
        Ok(self)
    }

    #[doc(alias = "AddAccessAllowedObjectAce")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-addaccessallowedobjectace)\]
    /// AddAccessAllowedObjectAce
    ///
    pub fn add_access_allowed_object_ace<'acl, 'sid>(&'acl mut self,
        ace_revision:               acl::Revision,
        ace_flags:                  ace::Flags,
        access_mask:                impl Into<access::Mask>,
        object_type_guid:           impl Into<Option<GUID>>,
        inherited_object_type_guid: impl Into<Option<GUID>>,
        sid:                        impl Into<sid::Ptr<'sid>>,
    ) -> firehazard::Result<&'acl mut Self> {
        firehazard::Error::get_last_if(FALSE == unsafe { AddAccessAllowedObjectAce(
            self.as_winapi(),
            ace_revision.into(),
            ace_flags.into(),
            access_mask.into().into(),
            object_type_guid.into().as_mut().map_or(null_mut(), |g| g),
            inherited_object_type_guid.into().as_mut().map_or(null_mut(), |g| g),
            sid.into().as_psid()
        )})?;
        Ok(self)
    }

    #[doc(alias = "AddAccessDeniedAce")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-addaccessdeniedace)\]
    /// AddAccessDeniedAce
    ///
    pub fn add_access_denied_ace<'acl, 'sid>(&'acl mut self,
        ace_revision:   acl::Revision,
        access_mask:    impl Into<access::Mask>,
        sid:            impl Into<sid::Ptr<'sid>>,
    ) -> firehazard::Result<&'acl mut Self> {
        firehazard::Error::get_last_if(FALSE == unsafe { AddAccessDeniedAce(
            self.as_winapi(),
            ace_revision.into(),
            access_mask.into().into(),
            sid.into().as_psid(),
        )})?;
        Ok(self)
    }

    #[doc(alias = "AddAccessDeniedAceEx")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-addaccessdeniedaceex)\]
    /// AddAccessDeniedAceEx
    ///
    pub fn add_access_denied_ace_ex<'acl, 'sid>(&'acl mut self,
        ace_revision:   acl::Revision,
        ace_flags:      ace::Flags,
        access_mask:    impl Into<access::Mask>,
        sid:            impl Into<sid::Ptr<'sid>>,
    ) -> firehazard::Result<&'acl mut Self> {
        firehazard::Error::get_last_if(FALSE == unsafe { AddAccessDeniedAceEx(
            self.as_winapi(),
            ace_revision.into(),
            ace_flags.into(),
            access_mask.into().into(),
            sid.into().as_psid(),
        )})?;
        Ok(self)
    }

    #[doc(alias = "AddAccessDeniedObjectAce")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-addaccessdeniedobjectace)\]
    /// AddAccessDeniedObjectAce
    ///
    pub fn add_access_denied_object_ace<'acl, 'sid>(&'acl mut self,
        ace_revision:               acl::Revision,
        ace_flags:                  ace::Flags,
        access_mask:                impl Into<access::Mask>,
        object_type_guid:           impl Into<Option<GUID>>,
        inherited_object_type_guid: impl Into<Option<GUID>>,
        sid:                        impl Into<sid::Ptr<'sid>>,
    ) -> firehazard::Result<&'acl mut Self> {
        firehazard::Error::get_last_if(FALSE == unsafe { AddAccessDeniedObjectAce(
            self.as_winapi(),
            ace_revision.into(),
            ace_flags.into(),
            access_mask.into().into(),
            object_type_guid.into().as_mut().map_or(null_mut(), |g| g),
            inherited_object_type_guid.into().as_mut().map_or(null_mut(), |g| g),
            sid.into().as_psid()
        )})?;
        Ok(self)
    }

    #[doc(alias = "AddAce")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-addace)\]
    /// AddAce &mdash; add an entire Acl
    ///
    pub fn add_acl(&mut self,
        ace_revision:           acl::Revision,
        starting_ace_index:     u32,
        acl:                    acl::Ptr
    ) -> firehazard::Result<&mut Self> {
        if acl.as_pacl().is_null() { return Err(Error(ERROR_INVALID_PARAMETER)); }
        let size : u16 = acl.aces().map(|a| a.header().size).sum();
        let ptr = acl.aces().as_ptr().cast();
        firehazard::Error::get_last_if(FALSE == unsafe { AddAce(
            self.as_winapi(),
            ace_revision.into(),
            starting_ace_index,
            ptr,
            size.into(),
        )})?;
        Ok(self)
    }

    #[doc(alias = "AddAuditAccessObjectAce")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-addauditaccessobjectace)\]
    /// AddAuditAccessObjectAce
    ///
    pub fn add_audit_access_object_ace<'acl, 'sid>(&'acl mut self,
        ace_revision:               acl::Revision,
        ace_flags:                  ace::Flags,
        access_mask:                impl Into<access::Mask>,
        object_type_guid:           impl Into<Option<GUID>>,
        inherited_object_type_guid: impl Into<Option<GUID>>,
        sid:                        impl Into<sid::Ptr<'sid>>,
        audit_success:              bool,
        audit_failure:              bool,
    ) -> firehazard::Result<&'acl mut Self> {
        firehazard::Error::get_last_if(FALSE == unsafe { AddAuditAccessObjectAce(
            self.as_winapi(),
            ace_revision.into(),
            ace_flags.into(),
            access_mask.into().into(),
            object_type_guid.into().as_mut().map_or(null_mut(), |g| g),
            inherited_object_type_guid.into().as_mut().map_or(null_mut(), |g| g),
            sid.into().as_psid(),
            audit_success.into(),
            audit_failure.into(),
        )})?;
        Ok(self)
    }

    #[doc(alias="AddMandatoryAce")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-addmandatoryace)\]
    /// AddMandatoryAce
    ///
    pub fn add_mandatory_ace<'acl, 'sid>(&'acl mut self,
        ace_revision:       acl::Revision,
        ace_flags:          ace::Flags,
        mandatory_policy:   u32, // TODO: strengthen type to limit to SYSTEM_MANDATORY_LABEL_*
        sid:                impl Into<sid::Ptr<'sid>>,
    ) -> firehazard::Result<&'acl mut Self> {
        firehazard::Error::get_last_if(FALSE == unsafe { AddMandatoryAce(
            self.as_winapi(),
            ace_revision.into(),
            ace_flags.into(),
            mandatory_policy.into(),
            sid.into().as_psid(),
        )})?;
        Ok(self)
    }

    // TODO: https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-addresourceattributeace
    // TODO: https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-addscopedpolicyidace

    #[doc(alias = "DeleteAce")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-deleteace)\]
    /// DeleteAce
    ///
    pub fn delete_ace(&mut self, ace_index: u32) -> firehazard::Result<&mut Self> {
        firehazard::Error::get_last_if(FALSE == unsafe { DeleteAce(
            self.as_winapi(),
            ace_index,
        )})?;
        Ok(self)
    }

    // https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-findfirstfreeace

    #[doc(alias = "GetAce")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-getace)\]
    /// GetAce
    ///
    pub fn get_ace(&self, ace_index: u32) -> firehazard::Result<ace::Ptr> {
        let mut ace = null_mut();
        firehazard::Error::get_last_if(FALSE == unsafe { GetAce(
            self.as_winapi_const(),
            ace_index,
            &mut ace,
        )})?;
        unsafe { ace::Ptr::from_raw(ace.cast()) }.ok_or(Error(ERROR_INVALID_HANDLE))
    }

    // https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-setaclinformation
}
