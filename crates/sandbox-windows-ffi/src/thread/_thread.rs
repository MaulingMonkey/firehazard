//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createthread)\]
//! Thread [`OwnedHandle`] and related fns

use crate::*;

#[path = "thread_funcs.rs"]
pub(crate) mod funcs;                   pub use funcs::*;
mod thread_handles;                     pub use thread_handles::*;

pub type Id = u32;

/// [`OwnedHandle`] | [`PsuedoHandle`] | [`None`]
pub unsafe trait AsHandleOrNone                                 { fn as_handle_or_none(&self) -> Option<winapi::shared::ntdef::HANDLE>; }
unsafe impl AsHandleOrNone for Option<core::convert::Infallible>{ fn as_handle_or_none(&self) -> Option<winapi::shared::ntdef::HANDLE> { None } }
unsafe impl AsHandleOrNone for &'_ thread::OwnedHandle          { fn as_handle_or_none(&self) -> Option<winapi::shared::ntdef::HANDLE> { Some(self.as_handle()) } }
unsafe impl AsHandleOrNone for thread::Handle<'_>               { fn as_handle_or_none(&self) -> Option<winapi::shared::ntdef::HANDLE> { Some(self.as_handle()) } }
unsafe impl AsHandleOrNone for thread::PsuedoHandle<'_>         { fn as_handle_or_none(&self) -> Option<winapi::shared::ntdef::HANDLE> { Some(self.as_handle()) } }
