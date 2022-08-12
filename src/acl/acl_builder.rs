use crate::*;
use crate::error::LastError;

use winapi::shared::guiddef::GUID;
use winapi::shared::winerror::{ERROR_INVALID_HANDLE, ERROR_ALLOTTED_SPACE_EXCEEDED, ERROR_INVALID_PARAMETER};
use winapi::um::securitybaseapi::*;
use winapi::um::winnt::ACL;

use std::mem::size_of;
use std::ptr::null_mut;

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
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-initializeacl)\]
    /// InitializeAcl
    pub fn new(revision: acl::Revision) -> Self {
        let mut b = Self { u: U { bytes: [0; max_acl_bytes!()] } };
        assert!(0 != unsafe { InitializeAcl(&mut b.u.acl, max_acl_bytes!(), revision.into()) }, "GetLastError()=={:?}", error::LastError::get());
        b
    }

    fn as_winapi(&mut self) -> *mut ACL { unsafe { &mut self.u.acl } }
    fn as_winapi_const(&self) -> *mut ACL { unsafe { &self.u.acl as *const _ as *mut _ } }
    pub fn as_acl_ptr(&self) -> acl::Ptr { unsafe { acl::Ptr::from_raw_unchecked(&self.u.acl as *const _ as *mut _) } }
    #[allow(dead_code)] pub(crate) fn as_bytes(&self) -> &[u8] { let len = usize::from32(self.as_acl_ptr().get_acl_size_information().AclBytesInUse); unsafe { &self.u.bytes[..len] } }


    pub fn finish(&mut self) -> Result<&mut Self, LastError> {
        // TODO: eliminate the need for this by only temporarilly growing AclSize immediately before add_*_ace and then shrinking it afterwards?
        let size = self.as_acl_ptr().get_acl_size_information().AclBytesInUse;
        let size = u16::try_from(size).map_err(|_| LastError(ERROR_ALLOTTED_SPACE_EXCEEDED))?;
        self.u.acl.AclSize = size;
        Ok(self)
    }

    // https://docs.microsoft.com/en-us/windows/win32/secauthz/authorization-functions

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-addaccessallowedace)\]
    /// `AddAccessAllowedAce`
    pub fn add_access_allowed_ace<'acl, 'sid>(&'acl mut self,
        ace_revision:   acl::Revision,
        access_mask:    token::AccessRights,
        sid:            impl Into<sid::Ptr<'sid>>,
    ) -> Result<&'acl mut Self, LastError> {
        let success = 0 != unsafe { AddAccessAllowedAce(self.as_winapi(), ace_revision.into(), access_mask.into(), sid.into().as_psid()) };
        if success { Ok(self) } else { Err(LastError::get()) }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-addaccessallowedaceex)\]
    /// `AddAccessAllowedAceEx`
    pub fn add_access_allowed_ace_ex<'acl, 'sid>(&'acl mut self,
        ace_revision:   acl::Revision,
        ace_flags:      ace::Flags,
        access_mask:    token::AccessRights,
        sid:            impl Into<sid::Ptr<'sid>>,
    ) -> Result<&'acl mut Self, LastError> {
        let success = 0 != unsafe { AddAccessAllowedAceEx(self.as_winapi(), ace_revision.into(), ace_flags.into(), access_mask.into(), sid.into().as_psid()) };
        if success { Ok(self) } else { Err(LastError::get()) }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-addaccessallowedobjectace)\]
    /// `AddAccessAllowedObjectAce`
    pub fn add_access_allowed_object_ace<'acl, 'sid>(&'acl mut self,
        ace_revision:               acl::Revision,
        ace_flags:                  ace::Flags,
        access_mask:                token::AccessRights,
        object_type_guid:           impl Into<Option<GUID>>,
        inherited_object_type_guid: impl Into<Option<GUID>>,
        sid:                        impl Into<sid::Ptr<'sid>>,
    ) -> Result<&'acl mut Self, LastError> {
        let success = 0 != unsafe { AddAccessAllowedObjectAce(
            self.as_winapi(),
            ace_revision.into(),
            ace_flags.into(),
            access_mask.into(),
            object_type_guid.into().as_mut().map_or(null_mut(), |g| g),
            inherited_object_type_guid.into().as_mut().map_or(null_mut(), |g| g),
            sid.into().as_psid()
        )};
        if success { Ok(self) } else { Err(LastError::get()) }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-addaccessdeniedace)\]
    /// `AddAccessDeniedAce`
    pub fn add_access_denied_ace<'acl, 'sid>(&'acl mut self,
        ace_revision:   acl::Revision,
        access_mask:    token::AccessRights,
        sid:            impl Into<sid::Ptr<'sid>>,
    ) -> Result<&'acl mut Self, LastError> {
        let success = 0 != unsafe { AddAccessDeniedAce(self.as_winapi(), ace_revision.into(), access_mask.into(), sid.into().as_psid()) };
        if success { Ok(self) } else { Err(LastError::get()) }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-addaccessdeniedaceex)\]
    /// `AddAccessDeniedAceEx`
    pub fn add_access_denied_ace_ex<'acl, 'sid>(&'acl mut self,
        ace_revision:   acl::Revision,
        ace_flags:      ace::Flags,
        access_mask:    token::AccessRights,
        sid:            impl Into<sid::Ptr<'sid>>,
    ) -> Result<&'acl mut Self, LastError> {
        let success = 0 != unsafe { AddAccessDeniedAceEx(self.as_winapi(), ace_revision.into(), ace_flags.into(), access_mask.into(), sid.into().as_psid()) };
        if success { Ok(self) } else { Err(LastError::get()) }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-addaccessdeniedobjectace)\]
    /// `AddAccessDeniedObjectAce`
    pub fn add_access_denied_object_ace<'acl, 'sid>(&'acl mut self,
        ace_revision:               acl::Revision,
        ace_flags:                  ace::Flags,
        access_mask:                token::AccessRights,
        object_type_guid:           impl Into<Option<GUID>>,
        inherited_object_type_guid: impl Into<Option<GUID>>,
        sid:                        impl Into<sid::Ptr<'sid>>,
    ) -> Result<&'acl mut Self, LastError> {
        let success = 0 != unsafe { AddAccessDeniedObjectAce(
            self.as_winapi(),
            ace_revision.into(),
            ace_flags.into(),
            access_mask.into(),
            object_type_guid.into().as_mut().map_or(null_mut(), |g| g),
            inherited_object_type_guid.into().as_mut().map_or(null_mut(), |g| g),
            sid.into().as_psid()
        )};
        if success { Ok(self) } else { Err(LastError::get()) }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-addace)\]
    /// `AddAce` an entire Acl
    pub fn add_acl(&mut self, ace_revision: acl::Revision, starting_ace_index: u32, acl: acl::Ptr) -> Result<&mut Self, LastError> {
        if acl.as_pacl().is_null() { return Err(LastError(ERROR_INVALID_PARAMETER)); }
        let size = acl.get_acl_size_information().AclBytesInUse - (size_of::<ace::Header>() as u32);
        let ptr = acl.aces().as_ptr().cast();
        let success = 0 != unsafe { AddAce(self.as_winapi(), ace_revision.into(), starting_ace_index, ptr, size) };
        if success { Ok(self) } else { Err(LastError::get()) }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-addauditaccessobjectace)\]
    /// `AddAuditAccessObjectAce`
    pub fn add_audit_access_object_ace<'acl, 'sid>(&'acl mut self,
        ace_revision:               acl::Revision,
        ace_flags:                  ace::Flags,
        access_mask:                token::AccessRights,
        object_type_guid:           impl Into<Option<GUID>>,
        inherited_object_type_guid: impl Into<Option<GUID>>,
        sid:                        impl Into<sid::Ptr<'sid>>,
        audit_success:              bool,
        audit_failure:              bool,
    ) -> Result<&'acl mut Self, LastError> {
        let success = 0 != unsafe { AddAuditAccessObjectAce(
            self.as_winapi(),
            ace_revision.into(),
            ace_flags.into(),
            access_mask.into(),
            object_type_guid.into().as_mut().map_or(null_mut(), |g| g),
            inherited_object_type_guid.into().as_mut().map_or(null_mut(), |g| g),
            sid.into().as_psid(),
            audit_success.into(),
            audit_failure.into(),
        )};
        if success { Ok(self) } else { Err(LastError::get()) }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-addmandatoryace)\]
    /// `AddMandatoryAce`
    pub fn add_mandatory_ace<'acl, 'sid>(&'acl mut self,
        ace_revision:       acl::Revision,
        ace_flags:          ace::Flags,
        mandatory_policy:   u32, // TODO: strengthen type to limit to SYSTEM_MANDATORY_LABEL_*
        sid:                impl Into<sid::Ptr<'sid>>,
    ) -> Result<&'acl mut Self, LastError> {
        let success = 0 != unsafe { AddMandatoryAce(self.as_winapi(), ace_revision.into(), ace_flags.into(), mandatory_policy.into(), sid.into().as_psid()) };
        if success { Ok(self) } else { Err(LastError::get()) }
    }

    // TODO: https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-addresourceattributeace
    // TODO: https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-addscopedpolicyidace

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-deleteace)\]
    /// `DeleteAce`
    pub fn delete_ace(&mut self, ace_index: u32) -> Result<&mut Self, LastError> {
        let success = 0 != unsafe { DeleteAce(self.as_winapi(), ace_index) };
        if success { Ok(self) } else { Err(LastError::get()) }
    }

    // https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-findfirstfreeace

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-getace)\]
    /// `GetAce`
    pub fn get_ace(&self, ace_index: u32) -> Result<ace::Ptr, LastError> {
        let mut ace = null_mut();
        let success = 0 != unsafe { GetAce(self.as_winapi_const(), ace_index, &mut ace) };
        let ace = unsafe { ace::Ptr::from_raw(ace.cast()) };
        if success { ace.ok_or(LastError(ERROR_INVALID_HANDLE)) } else { Err(LastError::get()) }
    }

    // https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-setaclinformation
}
