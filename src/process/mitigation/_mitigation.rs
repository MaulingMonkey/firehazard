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
mod system_call_filter_policy;
mod user_pointer_auth_policy;       pub use user_pointer_auth_policy::*;
mod user_shadow_stack_policy;       pub use user_shadow_stack_policy::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ne-winnt-process_mitigation_policy)\]
/// PROCESS_MITIGATION_POLICY
///
/// `C:\Program Files (x86)\Windows Kits\10\Include\10.0.22621.0\um\winnt.h`
#[non_exhaustive] #[repr(u32)] pub enum Policy {
    /* Process */ DEPPolicy,
    /* Process */ ASLRPolicy,
    /* Process */ DynamicCodePolicy,
    /* Process */ StrictHandleCheckPolicy,
    /* Process */ SystemCallDisablePolicy,
    /* Process */ MitigationOptionsMask,
    /* Process */ ExtensionPointDisablePolicy,
    /* Process */ ControlFlowGuardPolicy,
    /* Process */ SignaturePolicy,
    /* Process */ FontDisablePolicy,
    /* Process */ ImageLoadPolicy,
    /* Process */ SystemCallFilterPolicy,
    /* Process */ PayloadRestrictionPolicy,
    /* Process */ ChildProcessPolicy,
    /* Process */ SideChannelIsolationPolicy,
    /* Process */ UserShadowStackPolicy,
    /* Process */ RedirectionTrustPolicy,
    /* Process */ UserPointerAuthPolicy,
    /* Process */ SEHOPPolicy,
    //MaxProcessMitigationPolicy
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocessmitigationpolicy)\] GetProcessMitigationPolicy parameters
///
/// ### Safety
/// `Self::Raw` must be ABI-compatible with whatever policy enumerand `GetPolicy::ty()` specifies.
pub unsafe trait GetPolicy {
    /// POD type that will be requested from GetProcessMitigationPolicy.
    /// If [`winapi`] implemented [`bytemuck::Pod`], I would require it.
    type Raw : Default;

    /// Specifies the type of [`GetPolicy::Raw`]
    fn ty() -> Policy;

    /// Constructs `Self` from `Self::Raw`
    fn from_policy(p: Self::Raw) -> Self;
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessmitigationpolicy)\] SetProcessMitigationPolicy parameters
pub trait SetPolicy : GetPolicy {
    fn into_policy(self) -> Self::Raw;
}
