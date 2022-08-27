use crate::*;

use winapi::um::winnt::*;

#[cfg(std)] use std::os::windows::io::IntoRawHandle;
#[cfg(std)] use std::process::Child;

use core::marker::PhantomData;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessw)\]
/// _Owned_, _nullable_, `HANDLE` to a _process_
#[repr(transparent)] pub struct OwnedHandle(HANDLE);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessw)\]
/// _Borrowed_, _nullable_, `HANDLE` to a _process_
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Handle<'a>(HANDLE, PhantomData<&'a HANDLE>);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessw)\]
/// _Borrowed or psuedo_, _nullable_, `HANDLE` to a _process_
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct PsuedoHandle<'a>(HANDLE, PhantomData<&'a HANDLE>);

handles!(impl *LocalHandle<HANDLE>  for process::{OwnedHandle, Handle, PsuedoHandle});
handles!(impl Debug                 for process::{OwnedHandle, Handle, PsuedoHandle});
handles!(impl {AsRef, From}         for process::{OwnedHandle, Handle, PsuedoHandle});
handles!(impl {AsRef<@base>, From}  for process::{OwnedHandle, Handle, PsuedoHandle});

impl Drop for OwnedHandle { fn drop(&mut self) { unsafe { drop_close_handle(self.0) } } }
#[cfg(std)] impl From<Child> for OwnedHandle { fn from(c: Child) -> Self { unsafe { Self::from_raw(c.into_raw_handle().cast()).unwrap() } } }
