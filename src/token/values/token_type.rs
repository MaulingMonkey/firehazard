#![allow(non_upper_case_globals)]

use winapi::um::winnt::*;

use core::fmt::{self, Debug, Formatter};



#[doc(alias = "TOKEN_TYPE")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ne-winnt-token_type)\]
/// TOKEN_TYPE
///
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Type(TOKEN_TYPE);

impl Type {
    /// ### Safety
    ///
    /// Some APIs might assume [`Type`] is a valid token type.
    pub const unsafe fn from_unchecked(ty: TOKEN_TYPE) -> Self { Self(ty) }

    #[doc(alias = "TokenPrimary")]
    /// TokenPrimary
    pub const Primary       : Type = Type(TokenPrimary);

    #[doc(alias = "TokenImpersonation")]
    /// TokenImpersonation
    pub const Impersonation : Type = Type(TokenImpersonation);
}

#[doc(alias = "TokenPrimary")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-duplicatetokenex)\]
/// TokenPrimary
pub const Primary       : Type = Type(TokenPrimary);

#[doc(alias = "TokenImpersonation")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-duplicatetokenex)\]
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
