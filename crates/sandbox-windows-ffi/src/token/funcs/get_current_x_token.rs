// inline fns, not found in winapi, from:
// C:\Program Files (x86)\Windows Kits\10\Include\10.0.17134.0\um\processthreadsapi.h

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocesstoken)\] GetCurrentProcessToken
#[inline(always)] pub const fn get_current_process_token() -> crate::token::PsuedoHandle<'static> { unsafe { crate::token::PsuedoHandle::from_raw_const(-4) } }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentthreadtoken)\] GetCurrentThreadToken
#[inline(always)] pub const fn get_current_thread_token() -> crate::token::PsuedoHandle<'static> { unsafe { crate::token::PsuedoHandle::from_raw_const(-5) } }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentthreadeffectivetoken)\] GetCurrentThreadEffectiveToken
#[inline(always)] pub const fn get_current_thread_effective_token() -> crate::token::PsuedoHandle<'static> { unsafe { crate::token::PsuedoHandle::from_raw_const(-6) } }



#[cfg(std)] #[test] fn debug() {
    dbg!(dbg!(get_current_process_token()).clone());
    dbg!(dbg!(get_current_thread_token()).clone());
    dbg!(dbg!(get_current_thread_effective_token()).clone());
}
