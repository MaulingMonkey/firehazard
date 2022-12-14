use crate::*;

use winapi::ctypes::c_void;

#[cfg(std)] use std::os::windows::io::IntoRawHandle;
#[cfg(std)] use std::process::Child;

use core::marker::PhantomData;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessw)\]
/// _Owned_, _non-null_, `HANDLE` to a _process_
#[repr(transparent)] pub struct OwnedHandle(HANDLENN);

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessw)\]
/// _Borrowed_, _non-null_, `HANDLE` to a _process_
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Handle<'a>(HANDLENN, PhantomData<&'a HANDLENN>);

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessw)\]
/// _Borrowed or psuedo_, _non-null_, `HANDLE` to a _process_
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct PsuedoHandle<'a>(HANDLENN, PhantomData<&'a HANDLENN>);

handles!(unsafe impl *LocalHandleNN<c_void>         for process::{OwnedHandle, Handle<'_>, PsuedoHandle<'_>});
handles!(unsafe impl AsRef<Self>                    for process::{OwnedHandle, Handle<'_>, PsuedoHandle<'_>});
handles!(unsafe impl Send                           for process::{OwnedHandle});
handles!(unsafe impl {AsRef, From}                  for process::{OwnedHandle, Handle<'_>, PsuedoHandle<'_>});
handles!(unsafe impl {AsRef<@base>, From<@base>}    for process::{OwnedHandle, Handle<'_>, PsuedoHandle<'_>});
handles!(impl Debug                                 for process::{OwnedHandle, Handle<'_>, PsuedoHandle<'_>});

impl Drop for OwnedHandle { fn drop(&mut self) { unsafe { drop_close_handle_nn(self) } } }
#[cfg(std)] impl From<Child> for OwnedHandle { fn from(c: Child) -> Self { unsafe { Self::from_raw(c.into_raw_handle().cast()).unwrap() } } }
