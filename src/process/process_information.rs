use crate::*;
use winapi::um::processthreadsapi::PROCESS_INFORMATION;
use core::mem::{align_of, size_of, transmute};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/ns-processthreadsapi-process_information)\]
/// PROCESS_INFORMATION
#[derive(Debug)]
#[repr(C)] pub struct Information {
    pub process:    process::OwnedHandle,
    pub thread:     thread::OwnedHandle,
    pub process_id: process::Id,
    pub thread_id:  thread::Id,
}
const _ : () = assert!(align_of::<process::Information>() == align_of::<PROCESS_INFORMATION>());
const _ : () = assert!(size_of ::<process::Information>() == size_of ::<PROCESS_INFORMATION>());

impl AsRef<PROCESS_INFORMATION> for process::Information { fn as_ref(&self) -> &PROCESS_INFORMATION { unsafe { transmute(self) } } }

impl process::Information {
    pub unsafe fn from_raw(pi: PROCESS_INFORMATION) -> Self {
        assert!(!pi.hProcess.is_null()); // Self::process is NonNull
        assert!(!pi.hThread .is_null()); // Self::thread  is NonNull
        unsafe { transmute(pi) }
    }
}
