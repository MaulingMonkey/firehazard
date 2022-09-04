use crate::*;
use winapi::um::processthreadsapi::PROCESS_INFORMATION;
use core::mem::{transmute};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/ns-processthreadsapi-process_information)\]
/// PROCESS_INFORMATION
#[derive(Debug)]
#[repr(C)] pub struct Information {
    pub process:    process::OwnedHandle,
    pub thread:     thread::OwnedHandle,
    pub process_id: process::Id,
    pub thread_id:  thread::Id,
}

structure!(@assert layout Information => PROCESS_INFORMATION {
    process     == hProcess,
    thread      == hThread,
    process_id  == dwProcessId,
    thread_id   == dwThreadId,
});

impl AsRef<PROCESS_INFORMATION> for process::Information { fn as_ref(&self) -> &PROCESS_INFORMATION { unsafe { transmute(self) } } }

impl process::Information {
    pub unsafe fn from_raw(pi: PROCESS_INFORMATION) -> Self {
        assert!(!pi.hProcess.is_null()); // Self::process is NonNull
        assert!(!pi.hThread .is_null()); // Self::thread  is NonNull
        unsafe { transmute(pi) }
    }
}
