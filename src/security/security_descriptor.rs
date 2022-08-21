use crate::*;
use winapi::um::winnt::*;
use core::marker::PhantomData;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-security_descriptor)\]
/// SECURITY_DESCRIPTOR
#[repr(transparent)] pub struct Descriptor<'b> {
    pub(super) desc:    SECURITY_DESCRIPTOR,
    pub(super) phantom: PhantomData<(sid::Ptr<'b>, acl::Ptr<'b>)>,
}
