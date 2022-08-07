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



// inline fns, not found in winapi, from:
// C:\Program Files (x86)\Windows Kits\10\Include\10.0.17134.0\um\processthreadsapi.h

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocesstoken)\] GetCurrentProcessToken
#[inline(always)] pub const fn get_current_process_token() -> PsuedoHandle { PsuedoHandle(-4_isize as _) }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentthreadtoken)\] GetCurrentThreadToken
#[inline(always)] pub const fn get_current_thread_token() -> PsuedoHandle { PsuedoHandle(-5_isize as _) }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentthreadeffectivetoken)\] GetCurrentThreadEffectiveToken
#[inline(always)] pub const fn get_current_thread_effective_token() -> PsuedoHandle { PsuedoHandle(-6_isize as _) }



#[test] fn debug() {
    dbg!(dbg!(get_current_process_token()).clone());
    dbg!(dbg!(get_current_thread_token()).clone());
    dbg!(dbg!(get_current_thread_effective_token()).clone());
}
