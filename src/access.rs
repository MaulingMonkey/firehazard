//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/secauthz/access-mask)\]
//! ACCESS_MASK generic types, functions, and constants
//!
//! ### References
//! *   [`ACCESS_MASK`](https://docs.microsoft.com/en-us/windows/win32/secauthz/access-mask)
//! *   The Old New Thing / Raymond Chen
//!     *   [Is GENERIC_ALL equivalent to GENERIC_READ | GENERIC_WRITE | GENERIC_EXECUTE?](https://devblogs.microsoft.com/oldnewthing/20170310-00/?p=95705)
//!     *   [Anybody can make up a generic mapping](https://devblogs.microsoft.com/oldnewthing/?p=20733)
//!     *   [If you ask for STANDARD_RIGHTS_REQUIRED, you may as well ask for the moon](https://devblogs.microsoft.com/oldnewthing/20080227-00/?p=23303)

use std::fmt::{self, Debug, Formatter};
use std::ops::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/secauthz/access-mask)\]
/// ACCESS_MASK/DWORD/[u32]: Access rights flags mask for removing or restricting rights
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct MaskMask(u32);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/secauthz/access-mask)\]
/// ACCESS_MASK/DWORD/[u32]: Access rights flags
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Mask(u32);

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

pub const DELETE                    : Mask      = Mask    (winapi::um::winnt::DELETE                    ); // 0x00010000
pub const READ_CONTROL              : Mask      = Mask    (winapi::um::winnt::READ_CONTROL              ); // 0x00020000
pub const WRITE_DAC                 : Mask      = Mask    (winapi::um::winnt::WRITE_DAC                 ); // 0x00040000
pub const WRITE_OWNER               : Mask      = Mask    (winapi::um::winnt::WRITE_OWNER               ); // 0x00080000
pub const SYNCHRONIZE               : Mask      = Mask    (winapi::um::winnt::SYNCHRONIZE               ); // 0x00100000

pub const STANDARD_RIGHTS_REQUIRED  : MaskMask  = MaskMask(winapi::um::winnt::STANDARD_RIGHTS_REQUIRED  ); // 0x000F0000
pub const STANDARD_RIGHTS_READ      : Mask      = Mask    (winapi::um::winnt::STANDARD_RIGHTS_READ      ); // READ_CONTROL - this seems... buggy? on the windows sdk side of things?
pub const STANDARD_RIGHTS_WRITE     : Mask      = Mask    (winapi::um::winnt::STANDARD_RIGHTS_WRITE     ); // READ_CONTROL - this seems... buggy? on the windows sdk side of things?
pub const STANDARD_RIGHTS_EXECUTE   : Mask      = Mask    (winapi::um::winnt::STANDARD_RIGHTS_EXECUTE   ); // READ_CONTROL - this seems... buggy? on the windows sdk side of things?
pub const STANDARD_RIGHTS_ALL       : MaskMask  = MaskMask(winapi::um::winnt::STANDARD_RIGHTS_ALL       ); // 0x001F0000

pub const SPECIFIC_RIGHTS_ALL       : MaskMask  = MaskMask(winapi::um::winnt::SPECIFIC_RIGHTS_ALL       ); // 0x0000FFFF

pub const ACCESS_SYSTEM_SECURITY    : Mask      = Mask    (winapi::um::winnt::ACCESS_SYSTEM_SECURITY    ); // 0x01000000
pub const MAXIMUM_ALLOWED           : Mask      = Mask    (winapi::um::winnt::MAXIMUM_ALLOWED           ); // 0x02000000

pub const GENERIC_READ              : Mask      = Mask    (winapi::um::winnt::GENERIC_READ              ); // 0x80000000
pub const GENERIC_WRITE             : Mask      = Mask    (winapi::um::winnt::GENERIC_WRITE             ); // 0x40000000
pub const GENERIC_EXECUTE           : Mask      = Mask    (winapi::um::winnt::GENERIC_EXECUTE           ); // 0x20000000
pub const GENERIC_ALL               : Mask      = Mask    (winapi::um::winnt::GENERIC_ALL               ); // 0x10000000
