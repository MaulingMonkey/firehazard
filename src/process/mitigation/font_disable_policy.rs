use super::*;
use crate::*;
use bytemuck::*;
use winapi::um::winnt::*;



#[doc(alias = "PROCESS_MITIGATION_FONT_DISABLE_POLICY")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-process_mitigation_font_disable_policy)\]
/// ≈ PROCESS_MITIGATION_FONT_DISABLE_POLICY
///
#[derive(Clone, Copy, Debug)]
#[derive(Default, Zeroable)]
pub struct FontDisablePolicy {
    pub disable_non_system_fonts:       bool,
    pub audit_non_system_font_loading:  bool,
    #[doc(hidden)] pub _reserved_flags: ()
}

unsafe impl GetPolicy for PROCESS_MITIGATION_FONT_DISABLE_POLICY {
    type Raw = Self;
    fn ty() -> process::mitigation::Policy { process::FontDisablePolicy }
    fn from_policy(p: Self::Raw) -> Self { p }
}

impl SetPolicy for PROCESS_MITIGATION_FONT_DISABLE_POLICY {
    fn into_policy(self) -> Self::Raw { self }
}

unsafe impl GetPolicy for FontDisablePolicy {
    type Raw = PROCESS_MITIGATION_FONT_DISABLE_POLICY;
    fn ty() -> process::mitigation::Policy { process::FontDisablePolicy }
    fn from_policy(p: Self::Raw) -> Self { p.into() }
}

impl SetPolicy for FontDisablePolicy {
    fn into_policy(self) -> Self::Raw { self.into() }
}

impl From<FontDisablePolicy> for PROCESS_MITIGATION_FONT_DISABLE_POLICY {
    fn from(i: FontDisablePolicy) -> Self {
        let mut o = Self::default();
        o.set_DisableNonSystemFonts     (i.disable_non_system_fonts         as u32);
        o.set_AuditNonSystemFontLoading (i.audit_non_system_font_loading    as u32);
        o
    }
}

impl From<PROCESS_MITIGATION_FONT_DISABLE_POLICY> for FontDisablePolicy {
    fn from(i: PROCESS_MITIGATION_FONT_DISABLE_POLICY) -> Self {
        let mut o = Self::default();
        o.disable_non_system_fonts      = i.DisableNonSystemFonts()     != 0;
        o.audit_non_system_font_loading = i.AuditNonSystemFontLoading() != 0;
        o
    }
}
