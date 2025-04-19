// inline fns, not found in winapi, from:
// C:\Program Files (x86)\Windows Kits\10\Include\10.0.17134.0\um\processthreadsapi.h



#[doc(alias = "GetCurrentProcessToken")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocesstoken)\]
/// GetCurrentProcessToken
///
#[inline(always)] pub const fn get_current_process_token() -> crate::token::PseudoHandle<'static> { unsafe { crate::token::PseudoHandle::from_raw_const(-4) } }



#[doc(alias = "GetCurrentThreadToken")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentthreadtoken)\]
/// GetCurrentThreadToken
///
#[inline(always)] pub const fn get_current_thread_token() -> crate::token::PseudoHandle<'static> { unsafe { crate::token::PseudoHandle::from_raw_const(-5) } }



#[doc(alias = "GetCurrentThreadEffectiveToken")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentthreadeffectivetoken)\]
/// GetCurrentThreadEffectiveToken
///
#[inline(always)] pub const fn get_current_thread_effective_token() -> crate::token::PseudoHandle<'static> { unsafe { crate::token::PseudoHandle::from_raw_const(-6) } }



#[cfg(std)] #[test] fn debug() {
    use std::dbg;
    dbg!(dbg!(get_current_process_token()).clone());
    dbg!(dbg!(get_current_thread_token()).clone());
    dbg!(dbg!(get_current_thread_effective_token()).clone());
}
