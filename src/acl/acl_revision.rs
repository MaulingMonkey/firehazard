use winapi::um::winnt::{ACL_REVISION, ACL_REVISION_DS};

use core::fmt::{self, Debug, Formatter};



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-initializeacl#parameters)\]
/// ACL_REVISION{,DS}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Revision(u8);

impl From<Revision> for u8  { fn from(r: Revision) -> Self { r.0 } }
impl From<Revision> for u32 { fn from(r: Revision) -> Self { r.0 as _ } }

impl Revision {
    /// ### Safety
    ///
    /// Some APIs might assume [`Revision`] is a valid ACL revision type.
    pub const unsafe fn from_unchecked(ty: u8) -> Self { Self(ty) }
}

pub const REVISION      : Revision = Revision(ACL_REVISION as _);
pub const REVISION_DS   : Revision = Revision(ACL_REVISION_DS as _);

// pub const ACL_REVISION: BYTE = 2;
// pub const ACL_REVISION_DS: BYTE = 4;
// pub const ACL_REVISION1: BYTE = 1;
// pub const MIN_ACL_REVISION: BYTE = ACL_REVISION2;
// pub const ACL_REVISION2: BYTE = 2;
// pub const ACL_REVISION3: BYTE = 3;
// pub const ACL_REVISION4: BYTE = 4;
// pub const MAX_ACL_REVISION: BYTE = ACL_REVISION4;

impl Debug for Revision {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "ACL_REVISION{}", self.0)
    }
}
