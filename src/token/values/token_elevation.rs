use abibool::*;
use winapi::um::winnt::TOKEN_ELEVATION;



#[doc(alias = "TOKEN_ELEVATION")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_elevation)\]
/// TOKEN_ELEVATION
///
#[derive(Clone, Copy, Default, Debug)]
#[repr(C)] pub struct Elevation {
    pub token_is_elevated:  bool32,
}

structure!(@assert layout Elevation => TOKEN_ELEVATION {
    token_is_elevated   == TokenIsElevated,
});

impl AsRef<TOKEN_ELEVATION> for Elevation { fn as_ref(&    self) -> &    TOKEN_ELEVATION { unsafe { core::mem::transmute(self) } } }
impl AsMut<TOKEN_ELEVATION> for Elevation { fn as_mut(&mut self) -> &mut TOKEN_ELEVATION { unsafe { core::mem::transmute(self) } } }
impl AsRef<Elevation> for TOKEN_ELEVATION { fn as_ref(&    self) -> &    Elevation  { unsafe { core::mem::transmute(self) } } }
impl AsMut<Elevation> for TOKEN_ELEVATION { fn as_mut(&mut self) -> &mut Elevation  { unsafe { core::mem::transmute(self) } } }
impl From<Elevation> for TOKEN_ELEVATION { fn from(ts: Elevation ) -> Self { *ts.as_ref() } }
impl From<TOKEN_ELEVATION> for Elevation { fn from(ts: TOKEN_ELEVATION) -> Self { *ts.as_ref() } }
