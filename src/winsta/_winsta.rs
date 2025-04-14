//! \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/winstation/window-stations)\]
//! Window Station APIs

mod winsta_access_rights;               pub use winsta_access_rights::*;
mod winsta_handles;                     pub use winsta_handles::*;

pub use funcs::*;
#[path = "winsta_funcs.rs"] pub(crate) mod funcs;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowstationa)\]
/// CreateWindowStationA Flags
///
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct CreateWindowFlags(u32);

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowstationa)\]
/// CWF_CREATE_ONLY
pub const CWF_CREATE_ONLY : CreateWindowFlags = CreateWindowFlags(winapi::um::winuser::CWF_CREATE_ONLY);

impl CreateWindowFlags {
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowstationa)\]
    /// 0
    pub const NONE          : Self = Self(0);

    #[doc(alias = "CWF_CREATE_ONLY")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowstationa)\]
    /// CWF_CREATE_ONLY
    pub const CREATE_ONLY   : Self = Self(winapi::um::winuser::CWF_CREATE_ONLY);
}

impl From<Option<core::convert::Infallible>> for CreateWindowFlags  { fn from(_: Option<core::convert::Infallible>   ) -> Self { Self(0) } }
impl From<()>                               for CreateWindowFlags   { fn from(_: ()                                 ) -> Self { Self(0) } }
impl From<CreateWindowFlags>                for u32                 { fn from(cwf: CreateWindowFlags                ) -> Self { cwf.0 } }

impl core::fmt::Debug for CreateWindowFlags {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        let friendly = match *self {
            Self::NONE          => "0",
            Self::CREATE_ONLY   => "CWF_CREATE_ONLY",
            _                   => "CWF_???",
        };
        write!(fmt, "{friendly}")
    }
}
