use super::*;
use crate::*;
use bytemuck::*;
use winapi::um::winnt::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-process_mitigation_payload_restriction_policy)\]
/// ~ [PROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY]
#[derive(Clone, Copy, Debug)]
#[derive(Default, Zeroable)]
pub struct PayloadRestrictionPolicy {
    pub enable_export_address_filter:       bool,
    pub audit_export_address_filter:        bool,
    pub enable_export_address_filter_plus:  bool,
    pub audit_export_address_filter_plus:   bool,
    pub enable_import_address_filter:       bool,
    pub audit_import_address_filter:        bool,
    pub enable_rop_stack_pivot:             bool,
    pub audit_rop_stack_pivot:              bool,
    pub enable_rop_caller_check:            bool,
    pub audit_rop_caller_check:             bool,
    pub enable_rop_sim_exec:                bool,
    pub audit_rop_sim_exec:                 bool,
    #[doc(hidden)] pub _reserved_flags: ()
}

unsafe impl IntoPolicy for PROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY {
    type Policy = Self;
    fn ty() -> process::mitigation::Policy { process::PayloadRestrictionPolicy }
    fn into_policy(self) -> Self::Policy { self }
    fn from_policy(p: Self::Policy) -> Self { p }
}

unsafe impl IntoPolicy for PayloadRestrictionPolicy {
    type Policy = PROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY;
    fn ty() -> process::mitigation::Policy { process::PayloadRestrictionPolicy }
    fn into_policy(self) -> Self::Policy { self.into() }
    fn from_policy(p: Self::Policy) -> Self { p.into() }
}

impl From<PayloadRestrictionPolicy> for PROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY {
    fn from(i: PayloadRestrictionPolicy) -> Self {
        let mut o = Self::default();
        o.set_EnableExportAddressFilter     (i.enable_export_address_filter         as u32);
        o.set_AuditExportAddressFilter      (i.audit_export_address_filter          as u32);
        o.set_EnableExportAddressFilterPlus (i.enable_export_address_filter_plus    as u32);
        o.set_AuditExportAddressFilterPlus  (i.audit_export_address_filter_plus     as u32);
        o.set_EnableImportAddressFilter     (i.enable_import_address_filter         as u32);
        o.set_AuditImportAddressFilter      (i.audit_import_address_filter          as u32);
        o.set_EnableRopStackPivot           (i.enable_rop_stack_pivot               as u32);
        o.set_AuditRopStackPivot            (i.audit_rop_stack_pivot                as u32);
        o.set_EnableRopCallerCheck          (i.enable_rop_caller_check              as u32);
        o.set_AuditRopCallerCheck           (i.audit_rop_caller_check               as u32);
        o.set_EnableRopSimExec              (i.enable_rop_sim_exec                  as u32);
        o.set_AuditRopSimExec               (i.audit_rop_sim_exec                   as u32);
        o
    }
}

impl From<PROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY> for PayloadRestrictionPolicy {
    fn from(i: PROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY) -> Self {
        let mut o = Self::default();
        o.enable_export_address_filter      = i.EnableExportAddressFilter()     != 0;
        o.audit_export_address_filter       = i.AuditExportAddressFilter()      != 0;
        o.enable_export_address_filter_plus = i.EnableExportAddressFilterPlus() != 0;
        o.audit_export_address_filter_plus  = i.AuditExportAddressFilterPlus()  != 0;
        o.enable_import_address_filter      = i.EnableImportAddressFilter()     != 0;
        o.audit_import_address_filter       = i.AuditImportAddressFilter()      != 0;
        o.enable_rop_stack_pivot            = i.EnableRopStackPivot()           != 0;
        o.audit_rop_stack_pivot             = i.AuditRopStackPivot()            != 0;
        o.enable_rop_caller_check           = i.EnableRopCallerCheck()          != 0;
        o.audit_rop_caller_check            = i.AuditRopCallerCheck()           != 0;
        o.enable_rop_sim_exec               = i.EnableRopSimExec()              != 0;
        o.audit_rop_sim_exec                = i.AuditRopSimExec()               != 0;
        o
    }
}
