#[cfg(doc)] use crate::*;

use bytemuck::{Pod, Zeroable};
use core::fmt::{self, Debug, Formatter};



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createnamedpipew)\]
/// **N**a**m**ed **P**ipe **Wait**
/// &mdash;
/// [`NMPWAIT::USE_DEFAULT_WAIT`] | 2+ ms | ∞
///
/// Exists mainly to represent [`call_named_pipe`]'s et. all's `timeout` parameter.
///
/// | Value                 | Description                   |
/// |:---------------------:| ------------------------------|
/// | `0`                   | [`NMPWAIT::USE_DEFAULT_WAIT`] |
/// | `1`                   | [`NMPWAIT::NOWAIT`]           |
/// | `2 ..= 0xFFFFFFFE`    | a number of milliseconds      |
/// | `0xFFFFFFFF`          | [`NMPWAIT::WAIT_FOREVER`]     |
///
#[repr(transparent)] #[derive(Clone, Copy, Pod, Zeroable, Default, PartialEq, Eq, PartialOrd, Ord, Hash)] pub struct NMPWAIT(pub(crate) u32);



impl NMPWAIT {
    /// Create a wait duration in milliseconds from a compile time constant in the range `1 ..= !0-1`.
    ///
    /// This will cause compile time errors for the following values:
    /// *   `0` will be rejected (this will be misinterpreted as [`NMPWAIT::USE_DEFAULT_WAIT`].)
    /// *   `1` will be rejected (this will be misinterpreted as [`NMPWAIT::NOWAIT`].)
    /// *   `!0` will be rejected (this will be misinterpreted as [`NMPWAIT::WAIT_FOREVER`].)
    ///
    pub const fn from_finite_ms<const N : u32>() -> Self {
        const {
            assert!(N !=  0, "0 is reserved for `NMPWAIT::USE_DEFAULT_WAIT`");
            assert!(N !=  1, "1 is reserved for `NMPWAIT::NOWAIT`");
            assert!(N != !0, "0xFFFFFFFF is reserved for `NMPWAIT::WAIT_FOREVER`");
        };
        Self::from_unchecked(N)
    }

    /// Create a wait duration (either in milliseconds, or from a magic constant)
    pub const fn from_unchecked(value: u32) -> Self { Self(value) }

    /// Create a new finite instance limit.
    ///
    /// Will succeed for `milliseconds`:
    /// *   `2 ..= 0xFFFFFFFE`
    ///
    /// Will fail for `milliseconds`:
    /// *   `0` (invalid)
    /// *   `1` (no wait)
    /// *   `0xFFFFFFFF` (forever)
    ///
    pub const fn try_from_finite_ms(milliseconds: u32) -> Result<Self, ()> {
        match milliseconds {
            0                   => Err(()), // NMPWAIT::USE_DEFAULT_WAIT
            1                   => Err(()), // NMPWAIT::NOWAIT
            2 ..= 0xFFFFFFFE    => Ok(Self(milliseconds)),
            0xFFFFFFFF          => Err(()), // NMPWAIT::WAIT_FOREVER
        }
    }

    /// Returns <code>[Some]\(...\)</code> only for valid, finite values (in the range `2 ..= 0xFFFFFFFE`)
    pub const fn finite_ms(self) -> Option<u32> {
        match self.0 {
            v @ 2 ..= 0xFFFFFFFE    => Some(v),
            _                       => None,
        }
    }

    /// Returns `true` only for [`pipe::UNLIMITED_INSTANCES`]
    pub const fn is_infinite(self) -> bool { matches!(self, NMPWAIT::WAIT_FOREVER) }



    // constants from C:\Program Files (x86)\Windows Kits\10\Include\10.0.22621.0\um\WinBase.h
    // #define NMPWAIT_WAIT_FOREVER            0xffffffff
    // #define NMPWAIT_NOWAIT                  0x00000001
    // #define NMPWAIT_USE_DEFAULT_WAIT        0x00000000

    /// NMPWAIT_WAIT_FOREVER
    ///
    pub const WAIT_FOREVER      : Self = Self::from_unchecked(0xffffffff);

    /// NMPWAIT_USE_DEFAULT_WAIT
    ///
    pub const USE_DEFAULT_WAIT  : Self = Self::from_unchecked(0x00000000);

    /// NMPWAIT_NOWAIT
    ///
    pub const NOWAIT            : Self = Self::from_unchecked(0x00000001);
}

impl Debug for NMPWAIT {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match *self {
            NMPWAIT::NOWAIT             => write!(fmt, "NMPWAIT::NOWAIT"),
            NMPWAIT::WAIT_FOREVER       => write!(fmt, "NMPWAIT::WAIT_FOREVER"),
            NMPWAIT::USE_DEFAULT_WAIT   => write!(fmt, "NMPWAIT::USE_DEFAULT_WAIT"),
            NMPWAIT(ms)                 => write!(fmt, "NMPWAIT({ms} ms)"),
        }
    }
}

impl From<Option<core::convert::Infallible>> for NMPWAIT {
    fn from(_none: Option<core::convert::Infallible>) -> Self { NMPWAIT::USE_DEFAULT_WAIT }
}
