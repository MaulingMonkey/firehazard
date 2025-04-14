use super::*;
use crate::*;
use bytemuck::*;
use winapi::um::winnt::*;



#[doc(alias = "PROCESS_MITIGATION_IMAGE_LOAD_POLICY")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-process_mitigation_image_load_policy)\]
/// ≈ [PROCESS_MITIGATION_IMAGE_LOAD_POLICY]
///
#[derive(Clone, Copy, Debug)]
#[derive(Default, Zeroable)]
pub struct ImageLoadPolicy {
    pub no_remote_images:                       bool,
    pub no_low_mandatory_label_images:          bool,
    pub prefer_system32_images:                 bool,
    pub audit_no_remote_images:                 bool,
    pub audit_no_low_mandatory_label_images:    bool,
    #[doc(hidden)] pub _reserved_flags: ()
}

unsafe impl GetPolicy for PROCESS_MITIGATION_IMAGE_LOAD_POLICY {
    type Raw = Self;
    fn ty() -> process::mitigation::Policy { process::ImageLoadPolicy }
    fn from_policy(p: Self::Raw) -> Self { p }
}

impl SetPolicy for PROCESS_MITIGATION_IMAGE_LOAD_POLICY {
    fn into_policy(self) -> Self::Raw { self }
}

unsafe impl GetPolicy for ImageLoadPolicy {
    type Raw = PROCESS_MITIGATION_IMAGE_LOAD_POLICY;
    fn ty() -> process::mitigation::Policy { process::ImageLoadPolicy }
    fn from_policy(p: Self::Raw) -> Self { p.into() }
}

impl SetPolicy for ImageLoadPolicy {
    fn into_policy(self) -> Self::Raw { self.into() }
}

impl From<ImageLoadPolicy> for PROCESS_MITIGATION_IMAGE_LOAD_POLICY {
    fn from(i: ImageLoadPolicy) -> Self {
        let mut o = Self::default();
        o.Flags = 0
            | ((i.no_remote_images                      as u32) << 0)
            | ((i.no_low_mandatory_label_images         as u32) << 1)
            | ((i.prefer_system32_images                as u32) << 2)
            | ((i.audit_no_remote_images                as u32) << 3)
            | ((i.audit_no_low_mandatory_label_images   as u32) << 4)
            ;
        o
    }
}

impl From<PROCESS_MITIGATION_IMAGE_LOAD_POLICY> for ImageLoadPolicy {
    fn from(i: PROCESS_MITIGATION_IMAGE_LOAD_POLICY) -> Self {
        let mut o = Self::default();
        o.no_remote_images                      = (i.Flags & (1 << 0)) != 0;
        o.no_low_mandatory_label_images         = (i.Flags & (1 << 1)) != 0;
        o.prefer_system32_images                = (i.Flags & (1 << 2)) != 0;
        o.audit_no_remote_images                = (i.Flags & (1 << 3)) != 0;
        o.audit_no_low_mandatory_label_images   = (i.Flags & (1 << 4)) != 0;
        o
    }
}
