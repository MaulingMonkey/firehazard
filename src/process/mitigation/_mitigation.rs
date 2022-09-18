//! PROCESS_MITIGATION_\*

mod aslr_policy;                    pub use aslr_policy::*;
mod binary_signature_policy;        pub use binary_signature_policy::*;
mod child_process_policy;           pub use child_process_policy::*;
mod control_flow_guard_policy;      pub use control_flow_guard_policy::*;
mod dep_policy;                     pub use dep_policy::*;
mod dynamic_code_policy;            pub use dynamic_code_policy::*;
mod extension_point_disable_policy; pub use extension_point_disable_policy::*;
mod font_disable_policy;            pub use font_disable_policy::*;
mod image_load_policy;              pub use image_load_policy::*;
mod winapi;



/// \[[docs.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ne-winnt-process_mitigation_policy)\]
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
