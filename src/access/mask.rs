use bytemuck::*;
use core::fmt::{self, Debug, Formatter};



#[doc(alias = "ACCESS_MASK")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/secauthz/access-mask)\]
/// ACCESS_MASK/DWORD/[u32]: Access rights flags mask for removing or restricting rights
///
#[derive(Clone, Copy, Pod, Debug, Default, Zeroable, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct MaskMask(pub(super) u32);

#[doc(alias = "ACCESS_MASK")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/secauthz/access-mask)\]
/// ACCESS_MASK/DWORD/[u32]: Access rights flags
///
#[derive(Clone, Copy, Pod, Default, Zeroable, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Mask(pub(super) u32);

flags!(impl .. for Mask(u32) - MaskMask);

impl Mask {
    /// ### Safety
    /// *   Some APIs might theoretically assume access rights are a valid?
    pub const unsafe fn from_unchecked(rights: u32) -> Self { Self(rights) }
}

impl Debug for Mask {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        use winapi::um::winnt::*;
        flags!(self.0, fmt, "0x{:04X}", [
            GENERIC_ALL,
            GENERIC_EXECUTE,
            GENERIC_WRITE,
            GENERIC_READ,

            MAXIMUM_ALLOWED,

            ACCESS_SYSTEM_SECURITY,

            DELETE,
            READ_CONTROL,
            WRITE_DAC,
            WRITE_OWNER,
            SYNCHRONIZE,
        ])
    }
}
