//! \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createthread)\]
//! Thread [`OwnedHandle`] and related fns

use crate::prelude::*;

pub use funcs::*;
pub(crate) mod funcs {
    use crate::prelude::*;
    include!(r"funcs\exit_thread.rs");
    include!(r"funcs\get_current_thread_id.rs");
    include!(r"funcs\get_current_thread.rs");
    include!(r"funcs\get_exit_code_thread.rs");
    include!(r"funcs\get_thread_id.rs");
    include!(r"funcs\is_thread_alive.rs");
    include!(r"funcs\resume_thread.rs");
    include!(r"funcs\suspend_thread.rs");
    include!(r"funcs\wait_for_thread.rs");
}

#[cfg(test)] mod tests {
    use crate::prelude::*;
    include!(r"tests\test_wait_exit.rs");
}

mod thread_handles;                     pub use thread_handles::*;

/// DWORD / u32 thread identifier.
pub type Id = u32;

#[doc(alias = "HANDLE")]
/// [`OwnedHandle`] | [`PseudoHandle`] | [`None`]
pub unsafe trait AsHandleOrNone                                 { fn as_handle_or_none(&self) -> Option<winapi::shared::ntdef::HANDLE>; }
unsafe impl AsHandleOrNone for Option<core::convert::Infallible>{ fn as_handle_or_none(&self) -> Option<winapi::shared::ntdef::HANDLE> { None } }
unsafe impl AsHandleOrNone for &'_ thread::OwnedHandle          { fn as_handle_or_none(&self) -> Option<winapi::shared::ntdef::HANDLE> { Some(self.as_handle()) } }
unsafe impl AsHandleOrNone for thread::Handle<'_>               { fn as_handle_or_none(&self) -> Option<winapi::shared::ntdef::HANDLE> { Some(self.as_handle()) } }
unsafe impl AsHandleOrNone for thread::PseudoHandle<'_>         { fn as_handle_or_none(&self) -> Option<winapi::shared::ntdef::HANDLE> { Some(self.as_handle()) } }
