use crate::*;
use crate::policy::*;
use bytemuck::*;
use winapi::um::winnt::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-process_mitigation_font_disable_policy)\]
/// ~ [PROCESS_MITIGATION_FONT_DISABLE_POLICY]
#[derive(Clone, Copy, Debug)]
#[derive(Default, Zeroable)]
pub struct FontDisablePolicy {
    pub disable_non_system_fonts:       bool,
    pub audit_non_system_font_loading:  bool,
    #[doc(hidden)] pub _reserved_flags: ()
}

unsafe impl IntoPolicy for PROCESS_MITIGATION_FONT_DISABLE_POLICY {
    type Policy = Self;
    fn into_policy(self) -> (process::mitigation::Policy, Self::Policy) { (process::FontDisablePolicy, self) }
}

unsafe impl IntoPolicy for FontDisablePolicy {
    type Policy = PROCESS_MITIGATION_FONT_DISABLE_POLICY;
    fn into_policy(self) -> (process::mitigation::Policy, Self::Policy) { (process::FontDisablePolicy, self.into()) }
}

impl From<FontDisablePolicy> for PROCESS_MITIGATION_FONT_DISABLE_POLICY {
    fn from(i: FontDisablePolicy) -> Self {
        let mut o = PROCESS_MITIGATION_FONT_DISABLE_POLICY::default();
        o.set_DisableNonSystemFonts(i.disable_non_system_fonts as u32);
        o.set_AuditNonSystemFontLoading(i.audit_non_system_font_loading as u32);
        o
    }
}
