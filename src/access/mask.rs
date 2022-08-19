use std::fmt::{self, Debug, Formatter};
use std::ops::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/secauthz/access-mask)\]
/// ACCESS_MASK/DWORD/[u32]: Access rights flags mask for removing or restricting rights
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct MaskMask(pub(super) u32);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/secauthz/access-mask)\]
/// ACCESS_MASK/DWORD/[u32]: Access rights flags
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Mask(pub(super) u32);

impl Mask {
    /// ### Safety
    /// *   Some APIs might theoretically assume access rights are a valid?
    pub const unsafe fn from_unchecked(rights: u32) -> Self { Self(rights) }

    pub fn as_u32(self) -> u32 { self.0 }
}

impl Debug for Mask {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        let mut v = self.0;
        if v == 0 { return write!(fmt, "0") }

        macro_rules! v { ($e:expr) => {{
            const E : u32 = $e;
            if v & E != 0 {
                write!(fmt, "{}", stringify!($e))?;
                v &= !E;
                if v != 0 { write!(fmt, " | ")?; }
            }
        }}}

        use winapi::um::winnt::*;

        v!(GENERIC_ALL);
        v!(GENERIC_EXECUTE);
        v!(GENERIC_WRITE);
        v!(GENERIC_READ);

        v!(MAXIMUM_ALLOWED);

        v!(ACCESS_SYSTEM_SECURITY);

        v!(DELETE);
        v!(READ_CONTROL);
        v!(WRITE_DAC);
        v!(WRITE_OWNER);
        v!(SYNCHRONIZE);

        if v != 0 { write!(fmt, "0x{:04x}", v)? }

        Ok(())
    }
}

impl From<()> for Mask { fn from(_: ()) -> Self { Self(0) } }
impl From<Mask> for u32 { fn from(ar: Mask) -> Self { ar.0 } }

impl BitAnd         for Mask { type Output = Self; fn bitand(self, rhs: Self) -> Self::Output { Self(self.0 & rhs.0) } }
impl BitXor         for Mask { type Output = Self; fn bitxor(self, rhs: Self) -> Self::Output { Self(self.0 ^ rhs.0) } }
impl BitOr          for Mask { type Output = Self; fn bitor (self, rhs: Self) -> Self::Output { Self(self.0 | rhs.0) } }
impl BitAndAssign   for Mask { fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0 } }
impl BitXorAssign   for Mask { fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0 } }
impl BitOrAssign    for Mask { fn bitor_assign (&mut self, rhs: Self) { self.0 |= rhs.0 } }

impl Not                            for Mask      { type Output = MaskMask; fn not(self) -> Self::Output { MaskMask(!self.0) } }
impl BitAnd<MaskMask>         for Mask      { type Output = Mask; fn bitand(self, rhs: MaskMask) -> Mask { Mask(self.0 & rhs.0) } }
impl BitAnd<Mask>             for MaskMask  { type Output = Mask; fn bitand(self, rhs: Mask    ) -> Mask { Mask(self.0 & rhs.0) } }
impl BitAndAssign<MaskMask>   for Mask      { fn bitand_assign(&mut self, rhs: MaskMask) { self.0 &= rhs.0 } }
