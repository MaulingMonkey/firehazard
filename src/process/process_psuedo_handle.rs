use crate::process::Handle;

use winapi::um::winnt::*;

use std::fmt::{self, Debug, Formatter};
use std::ops::Deref;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessw)\]
/// `HANDLE` to a process
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)] #[repr(transparent)] pub struct PsuedoHandle(HANDLE);

impl PsuedoHandle {
    /// ### Safety
    /// `handle` must be a valid process handle.
    ///
    /// This takes over ownership of `handle` and will `CloseHandle` it on drop.
    /// The caller must ensure no other code attempts to claim ownership over the same handle.
    pub const unsafe fn from_raw_unchecked(handle: HANDLE) -> Self { Self(handle) }

    pub fn as_handle(&self) -> HANDLE { self.0 }
}

impl AsRef<Handle>  for PsuedoHandle { fn as_ref(&self) -> &Handle { unsafe { std::mem::transmute(self) } } }
impl AsRef<HANDLE>  for PsuedoHandle { fn as_ref(&self) -> &HANDLE { &self.0 } }
impl Debug          for PsuedoHandle { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "process::PsuedoHandle(0x{:08x})", self.0 as usize) } }
impl Deref          for PsuedoHandle { type Target = Handle; fn deref(&self) -> &Self::Target { unsafe { std::mem::transmute(self) } } }

impl From<PsuedoHandle>  for HANDLE { fn from(process: PsuedoHandle) -> Self { process.0 } }
