#![allow(non_upper_case_globals)]

use winapi::um::winnt::*;

use core::fmt::{self, Debug, Formatter};



#[doc(alias = "TOKEN_ELEVATION_TYPE")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ne-winnt-token_elevation_type)\]
/// TOKEN_ELEVATION_TYPE
///
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct ElevationType(TOKEN_ELEVATION_TYPE);

impl Default for ElevationType { fn default() -> Self { Self::Default } } // nonzero

impl ElevationType {
    /// ### Safety
    ///
    /// Some APIs might assume [`ElevationType`] is a valid token type.
    ///
    pub const unsafe fn from_unchecked(ty: TOKEN_ELEVATION_TYPE) -> Self { Self(ty) }



    #[doc(alias = "TokenElevationTypeDefault")]
    /// TokenElevationTypeDefault
    ///
    pub const Default : ElevationType = ElevationType(TokenElevationTypeDefault);



    #[doc(alias = "TokenElevationTypeFull")]
    /// TokenElevationTypeFull
    ///
    pub const Full : ElevationType = ElevationType(TokenElevationTypeFull);



    #[doc(alias = "TokenElevationTypeLimited")]
    /// TokenElevationTypeLimited
    ///
    pub const Limited : ElevationType = ElevationType(TokenElevationTypeLimited);
}


impl From<ElevationType> for TOKEN_ELEVATION_TYPE { fn from(ty: ElevationType) -> Self { ty.0 } }

impl Debug for ElevationType {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match *self {
            ElevationType::Default  => write!(fmt, "token::ElevationType::Default"),
            ElevationType::Full     => write!(fmt, "token::ElevationType::Full"),
            ElevationType::Limited  => write!(fmt, "token::ElevationType::Limited"),
            other                   => write!(fmt, "token::ElevationType({})", other.0),
        }
    }
}
