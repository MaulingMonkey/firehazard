//! \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocessmitigationpolicy)\]
//! PROCESS_MITIGATION_\*
//!
//! Process mitigation policy types and functions

mod aslr_policy;                    pub use aslr_policy::*;
mod binary_signature_policy;        pub use binary_signature_policy::*;
mod child_process_policy;           pub use child_process_policy::*;
mod control_flow_guard_policy;      pub use control_flow_guard_policy::*;
mod dep_policy;                     pub use dep_policy::*;
mod dynamic_code_policy;            pub use dynamic_code_policy::*;
mod extension_point_disable_policy; pub use extension_point_disable_policy::*;
mod font_disable_policy;            pub use font_disable_policy::*;
mod image_load_policy;              pub use image_load_policy::*;
mod mitigation_options_mask;        pub use mitigation_options_mask::*;
mod payload_restriction_policy;     pub use payload_restriction_policy::*;
mod redirection_trust_policy;       pub use redirection_trust_policy::*;
mod sehop_policy;                   pub use sehop_policy::*;
mod side_channel_isolation_policy;  pub use side_channel_isolation_policy::*;
mod strict_handle_check_policy;     pub use strict_handle_check_policy::*;
mod system_call_disable_policy;     pub use system_call_disable_policy::*;
mod system_call_filter_policy;      pub use system_call_filter_policy::*;
mod user_pointer_auth_policy;       pub use user_pointer_auth_policy::*;
mod user_shadow_stack_policy;       pub use user_shadow_stack_policy::*;



#[doc(alias = "PROCESS_MITIGATION_POLICY")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ne-winnt-process_mitigation_policy)\]
/// PROCESS_MITIGATION_POLICY
///
#[non_exhaustive] #[repr(u32)] pub enum Policy {
    // C:\Program Files (x86)\Windows Kits\10\Include\10.0.22621.0\um\winnt.h
    #[doc(alias = "ProcessDEPPolicy"                    )] #[doc = "ProcessDEPPolicy"                   ] DEPPolicy,
    #[doc(alias = "ProcessASLRPolicy"                   )] #[doc = "ProcessASLRPolicy"                  ] ASLRPolicy,
    #[doc(alias = "ProcessDynamicCodePolicy"            )] #[doc = "ProcessDynamicCodePolicy"           ] DynamicCodePolicy,
    #[doc(alias = "ProcessStrictHandleCheckPolicy"      )] #[doc = "ProcessStrictHandleCheckPolicy"     ] StrictHandleCheckPolicy,
    #[doc(alias = "ProcessSystemCallDisablePolicy"      )] #[doc = "ProcessSystemCallDisablePolicy"     ] SystemCallDisablePolicy,
    #[doc(alias = "ProcessMitigationOptionsMask"        )] #[doc = "ProcessMitigationOptionsMask"       ] MitigationOptionsMask,
    #[doc(alias = "ProcessExtensionPointDisablePolicy"  )] #[doc = "ProcessExtensionPointDisablePolicy" ] ExtensionPointDisablePolicy,
    #[doc(alias = "ProcessControlFlowGuardPolicy"       )] #[doc = "ProcessControlFlowGuardPolicy"      ] ControlFlowGuardPolicy,
    #[doc(alias = "ProcessSignaturePolicy"              )] #[doc = "ProcessSignaturePolicy"             ] SignaturePolicy,
    #[doc(alias = "ProcessFontDisablePolicy"            )] #[doc = "ProcessFontDisablePolicy"           ] FontDisablePolicy,
    #[doc(alias = "ProcessImageLoadPolicy"              )] #[doc = "ProcessImageLoadPolicy"             ] ImageLoadPolicy,
    #[doc(alias = "ProcessSystemCallFilterPolicy"       )] #[doc = "ProcessSystemCallFilterPolicy"      ] SystemCallFilterPolicy,
    #[doc(alias = "ProcessPayloadRestrictionPolicy"     )] #[doc = "ProcessPayloadRestrictionPolicy"    ] PayloadRestrictionPolicy,
    #[doc(alias = "ProcessChildProcessPolicy"           )] #[doc = "ProcessChildProcessPolicy"          ] ChildProcessPolicy,
    #[doc(alias = "ProcessSideChannelIsolationPolicy"   )] #[doc = "ProcessSideChannelIsolationPolicy"  ] SideChannelIsolationPolicy,
    #[doc(alias = "ProcessUserShadowStackPolicy"        )] #[doc = "ProcessUserShadowStackPolicy"       ] UserShadowStackPolicy,
    #[doc(alias = "ProcessRedirectionTrustPolicy"       )] #[doc = "ProcessRedirectionTrustPolicy"      ] RedirectionTrustPolicy,
    #[doc(alias = "ProcessUserPointerAuthPolicy"        )] #[doc = "ProcessUserPointerAuthPolicy"       ] UserPointerAuthPolicy,
    #[doc(alias = "ProcessSEHOPPolicy"                  )] #[doc = "ProcessSEHOPPolicy"                 ] SEHOPPolicy,
    //MaxProcessMitigationPolicy
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ne-winnt-process_mitigation_policy)\]
/// MaxProcessMitigationPolicy
///
#[allow(non_upper_case_globals)]
pub const MaxProcessMitigationPolicy : u32 = Policy::SEHOPPolicy as u32 + 1;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocessmitigationpolicy)\]
/// GetProcessMitigationPolicy parameters
///
/// ### Safety
/// `Self::Raw` must be ABI-compatible with whatever policy enumerand `GetPolicy::ty()` specifies.
///
pub unsafe trait GetPolicy {
    /// POD type that will be requested from GetProcessMitigationPolicy.
    /// If [`winapi`] implemented [`bytemuck::Pod`], I would require it.
    type Raw : Default;

    /// Specifies the type of [`GetPolicy::Raw`]
    fn ty() -> Policy;

    /// Constructs `Self` from `Self::Raw`
    fn from_policy(p: Self::Raw) -> Self;
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessmitigationpolicy)\]
/// SetProcessMitigationPolicy parameters
///
pub trait SetPolicy : GetPolicy {
    fn into_policy(self) -> Self::Raw;
}

// See also "Exploit protection" settings app for human readable descriptions of some settings
// N.b. individual "Program settings" display more options than "System settings"
