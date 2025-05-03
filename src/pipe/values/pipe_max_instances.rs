use crate::prelude::*;

use winapi::um::winbase::PIPE_UNLIMITED_INSTANCES;

use core::fmt::{self, Debug, Display, Formatter, Write};



#[doc(alias = "PIPE_UNLIMITED_INSTANCES")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createnamedpipew#parameters)\]
/// PIPE_UNLIMITED_INSTANCES
///
pub const UNLIMITED_INSTANCES : MaxInstances = MaxInstances(PIPE_UNLIMITED_INSTANCES);



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createnamedpipew)\]
/// 1 ..= 254 | ∞ | \<invalid\>
///
/// Exists mainly to represent [`pipe::named::create_*`](pipe::named::create)'s `max_instances` parameter.
///
/// | Value         | Description   |
/// |:-------------:| --------------|
/// | `0`           | Invalid       |
/// | `1 ..= 254`   | Finite        |
/// | `255`         | Infinite      |
/// | `256 ..`      | Invalid       |
///
#[repr(transparent)] #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)] pub struct MaxInstances(pub(crate) u32);



impl MaxInstances {
    /// Create a max instances from a compile time constant in the range `1 ..= 254`.
    ///
    /// This will cause compile time errors for the following values:
    /// *   `0` will be rejected (always invalid, will cause `ERROR_INVALID_PARAMETER`)
    /// *   `255` will be rejected (it'll be misinterpreted as `MaxInstances::UNLIMITED` - use it or `MaxInstances::from_unchecked(SOME_INDIRECT_CONSTANT)` if this is intentional)
    /// *   `256 ..` will be rejected (always invalid, will cause `ERROR_INVALID_PARAMETER`)
    ///
    pub const fn from_finite<const N : u32>() -> Self {
        const {
            assert!(N !=   0, "zero is not a valid number of max instances and will cause CreateNamedPipe et. all to fail with `ERROR_INVALID_PARAMETER`.");
            assert!(N != 255, "255 is reserved for `PIPE_UNLIMITED_INSTANCES`.  Use `MaxInstances::UNLIMITED` instead.");
            assert!(N <= 255, "254 is the maximum finite limit allowed by CreateNamedPipe.  Use that, or `MaxInstances::UNLIMITED` (255) for no limit.");
        };
        Self::from_unchecked(N)
    }

    /// Create a new instance limit.
    pub const fn from_unchecked(value: u32) -> Self { Self(value) }

    /// Create a new finite instance limit.
    ///
    /// Will succeed for:
    /// *   `1 ..= 254`
    ///
    /// Will fail for:
    /// *   `0`         (invalid)
    /// *   `255`       (infinite)
    /// *   `256 ..`    (invalid)
    ///
    pub const fn try_from_finite(instances: u32) -> Result<Self, ()> {
        match instances {
            0           => Err(()),             // invalid
            1 ..= 254   => Ok(Self(instances)), // finite
            255         => Err(()),             // infinite
            256 ..      => Err(()),             // invalid
        }
    }

    /// Returns <code>[Some]\(...\)</code> only for valid, finite values (in the range `1 ..= 254`)
    pub const fn finite(self) -> Option<u32> {
        match self.0 {
            v @ 1 ..= 254   => Some(v),
            _               => None,
        }
    }

    /// Returns `true` only for [`pipe::UNLIMITED_INSTANCES`]
    pub const fn is_infinite(self) -> bool { matches!(self, pipe::UNLIMITED_INSTANCES) }

    /// Shorthand for <code>[MaxInstances]::[finite](Self::finite)::&lt;1&gt;()</code>
    pub const ONE : Self = Self::from_finite::<1>();

    #[doc(alias = "PIPE_UNLIMITED_INSTANCES")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createnamedpipew#parameters)\]
    /// PIPE_UNLIMITED_INSTANCES
    ///
    pub const UNLIMITED : Self = Self(PIPE_UNLIMITED_INSTANCES);
}

impl Debug for MaxInstances {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self.0 {
            v @ 1 ..= 254 => write!(fmt, "{v}"),
            PIPE_UNLIMITED_INSTANCES => fmt.write_char('∞'),
            v => write!(fmt, "MaxInstances({v} (invalid))"),
        }
    }
}

impl Display for MaxInstances {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self.0 {
            PIPE_UNLIMITED_INSTANCES    => fmt.write_char('∞'),
            v                           => write!(fmt, "{v}"),
        }
    }
}
