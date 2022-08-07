use winapi::um::winnt::HANDLE;

use std::fmt::{self, Debug, Formatter};



/// An Access Token psuedo-HANDLE
///
/// Such a psuedo-handle:
/// *   Cannot and need not be `CloseHandle()`d
/// *   Cannot be `Duplicate{Handle,Token,TokenEx}()`d (just copy the psuedo-handle as appropriate?)
///
/// ### References: Local
/// *   [get_current_process_token]
/// *   [get_current_thread_token]
/// *   [get_current_thread_effective_token]
/// *   <https://docs.microsoft.com/en-us/windows/win32/secauthz/access-tokens>
/// *   <https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocesstoken>
#[derive(Clone, Copy)] #[repr(transparent)] pub struct PsuedoHandle(HANDLE);

impl PsuedoHandle {
    pub const unsafe fn from_raw(handle: HANDLE) -> Self { Self(handle) }
    pub fn as_handle(self) -> HANDLE { self.0 }
}

impl Debug for PsuedoHandle {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self.0 as isize {
            -4  => write!(fmt, "token::PsuedoHandle(GetCurrentProcessToken())"),
            -5  => write!(fmt, "token::PsuedoHandle(GetCurrentThreadToken())"),
            -6  => write!(fmt, "token::PsuedoHandle(GetCurrentThreadEffectiveToken())"),
            o   => write!(fmt, "token::PsuedoHandle({})", o),
        }
    }
}

impl From<PsuedoHandle> for HANDLE {
    fn from(token: PsuedoHandle) -> Self { token.0 }
}

#[test] fn clone_debug() {
    use crate::*;
    let p : PsuedoHandle = get_current_process_token();
    let _p2 = dbg!(p.clone());
}
