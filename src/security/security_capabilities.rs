use crate::prelude::*;
use winapi::um::winnt::*;



#[doc(alias = "SECURITY_CAPABILITIES")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-security_capabilities)\]
/// [SECURITY_CAPABILITIES]
///
#[derive(Clone, Copy, Default)]
#[repr(transparent)] pub struct Capabilities<'s> {
    capabilities:   SECURITY_CAPABILITIES,
    phantom:        PhantomData<sid::Ptr<'s>>,
}

impl<'s> Capabilities<'s> {
    pub fn app_container_sid(&self) -> sid::Ptr<'s> { unsafe { sid::Ptr::from_raw_unchecked(self.capabilities.AppContainerSid.cast()) } }
    pub fn capabilities(&self) -> &[sid::AndAttributes<'s>] { unsafe { slice::from_nullable_len_ref(self.capabilities.Capabilities.cast(), usize::from32(self.capabilities.CapabilityCount)) } }
}

impl From<&    Capabilities<'_>> for *const SECURITY_CAPABILITIES { fn from(c: &    Capabilities) -> Self { &    c.capabilities } }
impl From<&mut Capabilities<'_>> for *mut   SECURITY_CAPABILITIES { fn from(c: &mut Capabilities) -> Self { &mut c.capabilities } }

impl<'s> From<(&'s sid::Value, &'s [sid::AndAttributes<'s>])> for Capabilities<'s> {
    fn from((app_container_sid, capabilities): (&'s sid::Value, &'s [sid::AndAttributes<'s>])) -> Self {
        let count32 : u32 = capabilities.len().try_into().expect("capabilities.len() > 4GB");
        Self { phantom: PhantomData, capabilities: SECURITY_CAPABILITIES {
            AppContainerSid:    app_container_sid.as_psid(),
            Capabilities:       if count32 == 0 { null_mut() } else { capabilities.as_ptr() as *mut _ },
            CapabilityCount:    count32,
            Reserved:           0
        }}
    }
}

impl<'s> From<(sid::Ptr<'s>, &'s [sid::AndAttributes<'s>])> for Capabilities<'s> {
    fn from((app_container_sid, capabilities): (sid::Ptr<'s>, &'s [sid::AndAttributes<'s>])) -> Self {
        let count32 : u32 = capabilities.len().try_into().expect("capabilities.len() > 4GB");
        Self { phantom: PhantomData, capabilities: SECURITY_CAPABILITIES {
            AppContainerSid:    app_container_sid.as_psid(),
            Capabilities:       if count32 == 0 { null_mut() } else { capabilities.as_ptr() as *mut _ },
            CapabilityCount:    count32,
            Reserved:           0
        }}
    }
}
