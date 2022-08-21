use winapi::um::winnt::*;

use core::fmt::{self, Debug, Formatter};
#[allow(unused_imports)] use core::ops::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/sysinfo/kernel-objects)\]
/// `HANDLE` to a kernel object
/// (never instantiated: accessed only via [`AsRef`], [`Deref`], or other borrow)
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)] #[repr(transparent)] pub struct Handle(HANDLE);
// DO NOT IMPLEMENT: Clone, Copy

impl Handle {
    pub fn as_handle(&self) -> HANDLE { self.0 }
}

impl AsRef<HANDLE>  for Handle { fn as_ref(&self) -> &HANDLE { &self.0 } }
impl Debug          for Handle { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "process::Handle(0x{:08x})", self.0 as usize) } }

impl From<&'_ Handle> for HANDLE { fn from(process: &'_ Handle) -> Self { process.0 } }
