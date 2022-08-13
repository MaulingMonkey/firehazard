//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createthread)\]
//! Thread [`OwnedHandle`] and related fns

#[path = "thread_funcs.rs"]
pub(crate) mod funcs;                   pub use funcs::*;
mod thread_owned_handle;                pub use thread_owned_handle::*;
mod thread_psuedo_handle;               pub use thread_psuedo_handle::*;

pub type Id = u32;

/// [`OwnedHandle`] | [`PsuedoHandle`]
pub unsafe trait AsHandle               { fn as_handle(&self) -> winapi::shared::ntdef::HANDLE; }
unsafe impl AsHandle for OwnedHandle    { fn as_handle(&self) -> winapi::shared::ntdef::HANDLE { Self::as_handle(self) } }
unsafe impl AsHandle for PsuedoHandle   { fn as_handle(&self) -> winapi::shared::ntdef::HANDLE { Self::as_handle(self) } }
unsafe impl<T: AsHandle> AsHandle for &'_ T { fn as_handle(&self) -> winapi::shared::ntdef::HANDLE { T::as_handle(*self) } }

/// [`OwnedHandle`] | [`PsuedoHandle`] | [`None`]
pub unsafe trait AsHandleOrNone                                 { fn as_handle_or_none(&self) -> Option<winapi::shared::ntdef::HANDLE>; }
unsafe impl AsHandleOrNone for Option<std::convert::Infallible> { fn as_handle_or_none(&self) -> Option<winapi::shared::ntdef::HANDLE> { None } }
unsafe impl<T: AsHandle> AsHandleOrNone for T                   { fn as_handle_or_none(&self) -> Option<winapi::shared::ntdef::HANDLE> { Some(T::as_handle(self)) } }
