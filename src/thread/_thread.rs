//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createthread)\]
//! Thread [`OwnedHandle`] and related fns

#[path = "thread_funcs.rs"]
pub(crate) mod funcs;                   pub use funcs::*;
mod thread_handle;                      pub use thread_handle::*;
mod thread_owned_handle;                pub use thread_owned_handle::*;
mod thread_psuedo_handle;               pub use thread_psuedo_handle::*;

pub type Id = u32;

/// [`OwnedHandle`] | [`PsuedoHandle`] | [`None`]
pub unsafe trait AsHandleOrNone                                 { fn as_handle_or_none(&self) -> Option<winapi::shared::ntdef::HANDLE>; }
unsafe impl AsHandleOrNone for Option<std::convert::Infallible> { fn as_handle_or_none(&self) -> Option<winapi::shared::ntdef::HANDLE> { None } }
unsafe impl<T: AsRef<Handle>> AsHandleOrNone for T              { fn as_handle_or_none(&self) -> Option<winapi::shared::ntdef::HANDLE> { Some(self.as_ref().as_handle()) } }
