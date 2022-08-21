#![allow(non_upper_case_globals)]
use crate::*;
use winapi::um::winnt::*;
use core::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ne-winnt-security_impersonation_level)\]
/// SECURITY_IMPERSONATION_LEVEL
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct ImpersonationLevel(SECURITY_IMPERSONATION_LEVEL);

impl Debug for ImpersonationLevel {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        let friendly = match *self {
            security::Anonymous         => "SecurityAnonymous",
            security::Identification    => "SecurityIdentification",
            security::Impersonation     => "SecurityImpersonation",
            security::Delegation        => "SecurityDelegation",
            _                           => "Security???",
        };
        write!(fmt, "{friendly}")
    }
}

impl From<ImpersonationLevel> for SECURITY_IMPERSONATION_LEVEL { fn from(il: ImpersonationLevel) -> Self { il.0 } }

/// SecurityAnonymous
pub const Anonymous : ImpersonationLevel = ImpersonationLevel(SecurityAnonymous); // default

/// SecurityIdentification
pub const Identification : ImpersonationLevel = ImpersonationLevel(SecurityIdentification);

/// SecurityImpersonation
pub const Impersonation : ImpersonationLevel = ImpersonationLevel(SecurityImpersonation);

/// SecurityDelegation
pub const Delegation : ImpersonationLevel = ImpersonationLevel(SecurityDelegation);
