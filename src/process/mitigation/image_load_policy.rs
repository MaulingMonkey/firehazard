use crate::*;
use crate::policy::*;
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
    type Policy = Self;
    fn into_policy(self) -> (process::mitigation::Policy, Self::Policy) { (process::ImageLoadPolicy, self) }
}

unsafe impl IntoPolicy for ImageLoadPolicy {
    type Policy = PROCESS_MITIGATION_IMAGE_LOAD_POLICY;
    fn into_policy(self) -> (process::mitigation::Policy, Self::Policy) { (process::ImageLoadPolicy, self.into()) }
}

impl From<ImageLoadPolicy> for PROCESS_MITIGATION_IMAGE_LOAD_POLICY {
    fn from(i: ImageLoadPolicy) -> Self {
        let mut o = PROCESS_MITIGATION_IMAGE_LOAD_POLICY::default();
        o.set_NoRemoteImages            (i.no_remote_images as u32);
        o.set_NoLowMandatoryLabelImages (i.no_low_mandatory_label_images as u32);
        o.set_PreferSystem32Images      (i.prefer_system32_images as u32);
        o
    }
}
