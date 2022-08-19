use crate::*;
use winapi::um::processthreadsapi::PROCESS_INFORMATION;
use std::mem::{align_of, size_of, transmute};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/ns-processthreadsapi-process_information)\]
/// PROCESS_INFORMATION
#[derive(Clone, Debug)] #[repr(C)] pub struct Information {
    pub process:    process::OwnedHandle,
    pub thread:     thread::OwnedHandle,
    pub process_id: process::Id,
    pub thread_id:  thread::Id,
}
const _ALIGN : () = assert!(align_of::<process::Information>() == align_of::<PROCESS_INFORMATION>());
const _SIZE  : () = assert!(size_of ::<process::Information>() == size_of ::<PROCESS_INFORMATION>());

impl AsRef<PROCESS_INFORMATION> for process::Information { fn as_ref(&self) -> &PROCESS_INFORMATION { unsafe { transmute(self) } } }

impl process::Information {
    pub unsafe fn from_raw(pi: PROCESS_INFORMATION) -> Self { unsafe { transmute(pi) } }
}
