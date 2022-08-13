//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessw)\]
//! Process [`OwnedHandle`] and related fns

#[path = "process_funcs.rs"]
pub(crate) mod funcs;                   pub use funcs::*;
mod process_information;                pub use process_information::*;
mod process_owned_handle;               pub use process_owned_handle::*;
mod process_psuedo_handle;              pub use process_psuedo_handle::*;

pub type Id = u32;

/// [`OwnedHandle`] | [`PsuedoHandle`]
pub unsafe trait AsHandle               { fn as_handle(&self) -> winapi::shared::ntdef::HANDLE; }
unsafe impl AsHandle for OwnedHandle    { fn as_handle(&self) -> winapi::shared::ntdef::HANDLE { Self::as_handle(self) } }
unsafe impl AsHandle for PsuedoHandle   { fn as_handle(&self) -> winapi::shared::ntdef::HANDLE { Self::as_handle(self) } }
