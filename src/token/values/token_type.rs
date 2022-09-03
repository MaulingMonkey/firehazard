#![allow(non_upper_case_globals)]

use winapi::um::winnt::*;

use core::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ne-winnt-token_type)\]
/// TOKEN_TYPE
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Type(TOKEN_TYPE);

impl Type {
    /// ### Safety
    ///
    /// Some APIs might assume [`Type`] is a valid token type.
    pub const unsafe fn from_unchecked(ty: TOKEN_TYPE) -> Self { Self(ty) }

    /// TokenPrimary
    pub const Primary       : Type = Type(TokenPrimary);

    /// TokenImpersonation
    pub const Impersonation : Type = Type(TokenImpersonation);
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-duplicatetokenex)\]
/// TokenPrimary
pub const Primary       : Type = Type(TokenPrimary);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-duplicatetokenex)\]
/// TokenImpersonation
pub const Impersonation : Type = Type(TokenImpersonation);

impl From<Type> for TOKEN_TYPE { fn from(ty: Type) -> Self { ty.0 } }

impl Debug for Type {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match *self {
            Type::Primary       => write!(fmt, "token::Type::Primary"),
            Type::Impersonation => write!(fmt, "token::Type::Impersonation"),
            other               => write!(fmt, "token::Type({})", other.0),
        }
    }
}
