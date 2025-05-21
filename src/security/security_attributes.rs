use crate::prelude::*;
use winapi::um::minwinbase::SECURITY_ATTRIBUTES;



#[doc(alias = "SECURITY_ATTRIBUTES")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/wtypesbase/ns-wtypesbase-security_attributes)\]
/// SECURITY_ATTRIBUTES
///
#[derive(Clone, Copy)]
#[repr(transparent)] pub struct Attributes<'sd> {
    attributes:  SECURITY_ATTRIBUTES,
    phantom:     PhantomData<security::Descriptor<'sd>>,
}

unsafe impl Send for Attributes<'_> {}
unsafe impl Sync for Attributes<'_> {}

impl<'sd> Attributes<'sd> {
    pub fn new(security_descriptor: Option<&'sd security::Descriptor<'sd>>, inherit_handle: bool) -> Self {
        let security_descriptor = security_descriptor.map_or(null(), |d| d);
        Self {
            attributes: SECURITY_ATTRIBUTES {
                nLength:                size_of::<SECURITY_ATTRIBUTES>() as _,
                lpSecurityDescriptor:   security_descriptor as *mut _,
                bInheritHandle:         inherit_handle as _,
            },
            phantom: PhantomData
        }
    }

    pub fn inherit_handle(&self) -> bool { self.attributes.bInheritHandle != 0 }
    pub fn set_inherit_handle(&mut self, inherit: bool) { self.attributes.bInheritHandle = inherit as _ }

    pub fn security_descriptor(&self) -> Option<&'sd security::Descriptor<'sd>> {
        if self.attributes.lpSecurityDescriptor.is_null() { return None }
        Some(unsafe { core::mem::transmute(self.attributes.lpSecurityDescriptor) })
    }

    pub fn set_security_descriptor(&mut self, sd: Option<&'sd security::Descriptor<'sd>>) {
        let sd = sd.map_or(null(), |d| d);
        self.attributes.lpSecurityDescriptor = sd as *mut _;
    }

    pub fn with_security_descriptor<'d>(self, sd: Option<&'d security::Descriptor<'d>>) -> Attributes<'d> where 'sd: 'd {
        let mut a : Attributes<'d> = self;
        a.set_security_descriptor(sd);
        a
    }
}

impl Default for Attributes<'_> { fn default() -> Self { Self::new(None, false) } } // sets attributes.nLength

impl From<&    Attributes<'_>> for *const SECURITY_ATTRIBUTES { fn from(a: &    Attributes) -> Self { &    a.attributes } }
impl From<&mut Attributes<'_>> for *mut   SECURITY_ATTRIBUTES { fn from(a: &mut Attributes) -> Self { &mut a.attributes } }

impl<'sd> From<(       &'sd security::Descriptor<'sd> , bool)> for Attributes<'sd> { fn from((desc, inherit): (       &'sd security::Descriptor<'sd> , bool)) -> Self { Self::new(Some(desc), inherit) } }
impl<'sd> From<(Option<&'sd security::Descriptor<'sd>>, bool)> for Attributes<'sd> { fn from((desc, inherit): (Option<&'sd security::Descriptor<'sd>>, bool)) -> Self { Self::new(desc, inherit) } }

// Debug? Default?
