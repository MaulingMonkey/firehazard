use super::*;
use crate::*;
use bytemuck::*;
use winapi::um::winnt::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-process_mitigation_image_load_policy)\]
/// ~ [PROCESS_MITIGATION_IMAGE_LOAD_POLICY]
#[derive(Clone, Copy, Debug)]
#[derive(Default, Zeroable)]
pub struct ImageLoadPolicy {
    pub no_remote_images:               bool,
    pub no_low_mandatory_label_images:  bool,
    pub prefer_system32_images:         bool,
    #[doc(hidden)] pub _reserved_flags: ()
}

unsafe impl IntoPolicy for PROCESS_MITIGATION_IMAGE_LOAD_POLICY {
    type Raw = Self;
    fn ty() -> process::mitigation::Policy { process::ImageLoadPolicy }
    fn into_policy(self) -> Self::Raw { self }
    fn from_policy(p: Self::Raw) -> Self { p }
}

unsafe impl IntoPolicy for ImageLoadPolicy {
    type Raw = PROCESS_MITIGATION_IMAGE_LOAD_POLICY;
    fn ty() -> process::mitigation::Policy { process::ImageLoadPolicy }
    fn into_policy(self) -> Self::Raw { self.into() }
    fn from_policy(p: Self::Raw) -> Self { p.into() }
}

impl From<ImageLoadPolicy> for PROCESS_MITIGATION_IMAGE_LOAD_POLICY {
    fn from(i: ImageLoadPolicy) -> Self {
        let mut o = Self::default();
        o.set_NoRemoteImages            (i.no_remote_images                 as u32);
        o.set_NoLowMandatoryLabelImages (i.no_low_mandatory_label_images    as u32);
        o.set_PreferSystem32Images      (i.prefer_system32_images           as u32);
        o
    }
}

impl From<PROCESS_MITIGATION_IMAGE_LOAD_POLICY> for ImageLoadPolicy {
    fn from(i: PROCESS_MITIGATION_IMAGE_LOAD_POLICY) -> Self {
        let mut o = Self::default();
        o.no_remote_images              = i.NoRemoteImages()            != 0;
        o.no_low_mandatory_label_images = i.NoLowMandatoryLabelImages() != 0;
        o.prefer_system32_images        = i.PreferSystem32Images()      != 0;
        o
    }
}
