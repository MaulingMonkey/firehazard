use crate::prelude::*;

use winapi::shared::ntdef::LUID;

use core::fmt::{self, Debug, Formatter};
use core::hash::Hash;



#[doc(alias = "LUID")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-luid)\]
/// Generic LUID (≈ a 32-bit aligned `u64` / "Locally Unique IDentifier")
///
/// LUIDs are used for multiple unrelated purpouses:
/// *   [`privilege::Luid`] &mdash; identifies a [privilege](https://learn.microsoft.com/en-us/windows/win32/secauthz/privilege-constants#constants) such as `"SeShutdownPrivilege"`
/// *   <strike style="opacity: 25%">`graphics::adapter::Luid`</strike> &mdash; identifies a
///     [Direct3D9](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3d9ex-getadapterluid) ..
///     [12](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-getadapterluid) or
///     [DXGI](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/ns-dxgi-dxgi_adapter_desc#:~:text=AdapterLuid)
///     graphics adapter
/// *   <strike style="opacity: 25%">`network::interface::Luid`</strike> &mdash; identifies a
///     [Network Adapter](https://learn.microsoft.com/en-us/windows/win32/fwp/filtering-condition-identifiers-#:~:text=LUID)
///     for the Windows Filtering Platform.
///
#[derive(Clone, Copy, Default)] #[repr(transparent)] pub struct Luid(pub(crate) LUID);

impl Debug      for Luid { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "Luid(0x{:08x})", u64::from(*self)) } }
impl From<u64>  for Luid { fn from(value: u64) -> Self { Self(LUID { HighPart: (value>>32) as _, LowPart: value as _ }) } }
impl From<LUID> for Luid { fn from(value: LUID) -> Self { Self(value) } }
impl From<Luid> for u64  { fn from(value: Luid) -> Self { (value.0.HighPart as u64) << 32 | value.0.LowPart as u64 } }
impl From<Luid> for LUID { fn from(value: Luid) -> Self { value.0 } }
impl PartialEq  for Luid { fn eq(&self, other: &Self) -> bool { u64::from(*self) == u64::from(*other) } }
impl Eq         for Luid {}
impl PartialOrd for Luid { fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> { u64::from(*self).partial_cmp(&u64::from(*other)) } }
impl Ord        for Luid { fn cmp(&self, other: &Self) -> core::cmp::Ordering { u64::from(*self).cmp(&u64::from(*other)) } }
impl Hash       for Luid { fn hash<H: core::hash::Hasher>(&self, state: &mut H) { u64::from(*self).hash(state) } }

structure!(@assert layout Luid => LUID {});
