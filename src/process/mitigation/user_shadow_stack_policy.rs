use crate::*;
use crate::policy::*;
use bytemuck::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-process_mitigation_user_shadow_stack_policy)\]
/// ~ PROCESS_MITIGATION_USER_SHADOW_STACK_POLICY
#[derive(Clone, Copy, Debug)]
#[derive(Default, Zeroable)]
pub struct UserShadowStackPolicy {
    pub enable_user_shadow_stack:               bool,
    pub audit_user_shadow_stack:                bool,
    pub set_context_ip_validation:              bool,
    pub audit_set_context_ip_validation:        bool,
    pub enable_user_shadow_stack_strict_mode:   bool,
    pub block_non_cet_binaries:                 bool,
    pub block_non_cet_binaries_non_ehcont:      bool,
    pub audit_block_non_cet_binaries:           bool,
    pub cet_dynamic_apis_out_of_proc_only:      bool,
    pub set_context_ip_validation_relaxed_mode: bool,
    #[doc(hidden)] pub _reserved_flags: ()
}

unsafe impl IntoPolicy for PROCESS_MITIGATION_USER_SHADOW_STACK_POLICY {
    type Policy = Self;
    fn ty() -> process::mitigation::Policy { process::UserShadowStackPolicy }
    fn into_policy(self) -> Self::Policy { self }
    fn from_policy(p: Self::Policy) -> Self { p }
}

unsafe impl IntoPolicy for UserShadowStackPolicy {
    type Policy = u32; // XXX
    fn ty() -> process::mitigation::Policy { process::UserShadowStackPolicy }
    fn into_policy(self) -> Self::Policy { PROCESS_MITIGATION_USER_SHADOW_STACK_POLICY::from(self).Flags }
    fn from_policy(p: Self::Policy) -> Self { PROCESS_MITIGATION_USER_SHADOW_STACK_POLICY { Flags: p }.into() }
}

impl From<UserShadowStackPolicy> for PROCESS_MITIGATION_USER_SHADOW_STACK_POLICY {
    fn from(i: UserShadowStackPolicy) -> Self {
        let mut o = PROCESS_MITIGATION_USER_SHADOW_STACK_POLICY::default();
        o.set_EnableUserShadowStack             (i.enable_user_shadow_stack                 as u32);
        o.set_AuditUserShadowStack              (i.audit_user_shadow_stack                  as u32);
        o.set_SetContextIpValidation            (i.set_context_ip_validation                as u32);
        o.set_AuditSetContextIpValidation       (i.audit_set_context_ip_validation          as u32);
        o.set_EnableUserShadowStackStrictMode   (i.enable_user_shadow_stack_strict_mode     as u32);
        o.set_BlockNonCetBinaries               (i.block_non_cet_binaries                   as u32);
        o.set_BlockNonCetBinariesNonEhcont      (i.block_non_cet_binaries_non_ehcont        as u32);
        o.set_AuditBlockNonCetBinaries          (i.audit_block_non_cet_binaries             as u32);
        o.set_CetDynamicApisOutOfProcOnly       (i.cet_dynamic_apis_out_of_proc_only        as u32);
        o.set_SetContextIpValidationRelaxedMode (i.set_context_ip_validation_relaxed_mode   as u32);
        o
    }
}

impl From<PROCESS_MITIGATION_USER_SHADOW_STACK_POLICY> for UserShadowStackPolicy {
    fn from(i: PROCESS_MITIGATION_USER_SHADOW_STACK_POLICY) -> Self {
        let mut o = Self::default();
        o.enable_user_shadow_stack                  = i.EnableUserShadowStack()             != 0;
        o.audit_user_shadow_stack                   = i.AuditUserShadowStack()              != 0;
        o.set_context_ip_validation                 = i.SetContextIpValidation()            != 0;
        o.audit_set_context_ip_validation           = i.AuditSetContextIpValidation()       != 0;
        o.enable_user_shadow_stack_strict_mode      = i.EnableUserShadowStackStrictMode()   != 0;
        o.block_non_cet_binaries                    = i.BlockNonCetBinaries()               != 0;
        o.block_non_cet_binaries_non_ehcont         = i.BlockNonCetBinariesNonEhcont()      != 0;
        o.audit_block_non_cet_binaries              = i.AuditBlockNonCetBinaries()          != 0;
        o.cet_dynamic_apis_out_of_proc_only         = i.CetDynamicApisOutOfProcOnly()       != 0;
        o.set_context_ip_validation_relaxed_mode    = i.SetContextIpValidationRelaxedMode() != 0;
        o
    }
}

// XXX: not (yet?) defined by winapi
#[allow(non_snake_case)] #[derive(Clone, Copy, Debug, Default)] #[repr(C)] struct PROCESS_MITIGATION_USER_SHADOW_STACK_POLICY { Flags: u32 }
#[allow(non_snake_case)] impl PROCESS_MITIGATION_USER_SHADOW_STACK_POLICY {
    pub fn set_EnableUserShadowStack                (&mut self, value: u32) { const M : u32 = 1 << 0; if value == 0 { self.Flags &= !M } else { self.Flags |= M } }
    pub fn set_AuditUserShadowStack                 (&mut self, value: u32) { const M : u32 = 1 << 1; if value == 0 { self.Flags &= !M } else { self.Flags |= M } }
    pub fn set_SetContextIpValidation               (&mut self, value: u32) { const M : u32 = 1 << 2; if value == 0 { self.Flags &= !M } else { self.Flags |= M } }
    pub fn set_AuditSetContextIpValidation          (&mut self, value: u32) { const M : u32 = 1 << 3; if value == 0 { self.Flags &= !M } else { self.Flags |= M } }
    pub fn set_EnableUserShadowStackStrictMode      (&mut self, value: u32) { const M : u32 = 1 << 4; if value == 0 { self.Flags &= !M } else { self.Flags |= M } }
    pub fn set_BlockNonCetBinaries                  (&mut self, value: u32) { const M : u32 = 1 << 5; if value == 0 { self.Flags &= !M } else { self.Flags |= M } }
    pub fn set_BlockNonCetBinariesNonEhcont         (&mut self, value: u32) { const M : u32 = 1 << 6; if value == 0 { self.Flags &= !M } else { self.Flags |= M } }
    pub fn set_AuditBlockNonCetBinaries             (&mut self, value: u32) { const M : u32 = 1 << 7; if value == 0 { self.Flags &= !M } else { self.Flags |= M } }
    pub fn set_CetDynamicApisOutOfProcOnly          (&mut self, value: u32) { const M : u32 = 1 << 8; if value == 0 { self.Flags &= !M } else { self.Flags |= M } }
    pub fn set_SetContextIpValidationRelaxedMode    (&mut self, value: u32) { const M : u32 = 1 << 9; if value == 0 { self.Flags &= !M } else { self.Flags |= M } }

    pub fn EnableUserShadowStack                (&self) -> u32 { (self.Flags >> 0) & 1 }
    pub fn AuditUserShadowStack                 (&self) -> u32 { (self.Flags >> 1) & 1 }
    pub fn SetContextIpValidation               (&self) -> u32 { (self.Flags >> 2) & 1 }
    pub fn AuditSetContextIpValidation          (&self) -> u32 { (self.Flags >> 3) & 1 }
    pub fn EnableUserShadowStackStrictMode      (&self) -> u32 { (self.Flags >> 4) & 1 }
    pub fn BlockNonCetBinaries                  (&self) -> u32 { (self.Flags >> 5) & 1 }
    pub fn BlockNonCetBinariesNonEhcont         (&self) -> u32 { (self.Flags >> 6) & 1 }
    pub fn AuditBlockNonCetBinaries             (&self) -> u32 { (self.Flags >> 7) & 1 }
    pub fn CetDynamicApisOutOfProcOnly          (&self) -> u32 { (self.Flags >> 8) & 1 }
    pub fn SetContextIpValidationRelaxedMode    (&self) -> u32 { (self.Flags >> 9) & 1 }
}
