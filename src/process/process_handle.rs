use winapi::um::winnt::*;

use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessw)\]
/// `HANDLE` to a process
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)] #[repr(transparent)] pub struct Handle(HANDLE);
// DO NOT IMPLEMENT: Clone, Copy

impl Handle {
    pub fn as_handle(&self) -> HANDLE { self.0 }
}

impl AsRef<HANDLE>  for Handle { fn as_ref(&self) -> &HANDLE { &self.0 } }
impl Debug          for Handle { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "process::Handle(0x{:08x})", self.0 as usize) } }

impl From<&'_ Handle> for HANDLE { fn from(process: &'_ Handle) -> Self { process.0 } }
