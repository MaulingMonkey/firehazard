use crate::*;

use winapi::um::winnt::*;

#[cfg(std)] use std::os::windows::io::IntoRawHandle;
#[cfg(std)] use std::thread::JoinHandle;

use core::marker::PhantomData;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createthread)\]
/// `HANDLE` to a thread
#[repr(transparent)] pub struct OwnedHandle(HANDLE);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createthread)\]
/// `HANDLE` to a thread
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Handle<'a>(HANDLE, PhantomData<&'a HANDLE>);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createthread)\]
/// `HANDLE` to a thread
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct PsuedoHandle<'a>(HANDLE, PhantomData<&'a HANDLE>);

handles!(impl *LocalHandle<HANDLE>  for thread::{OwnedHandle, Handle, PsuedoHandle});
handles!(impl Debug                 for thread::{OwnedHandle, Handle, PsuedoHandle});
handles!(impl {AsRef, From}         for thread::{OwnedHandle, Handle, PsuedoHandle});
handles!(impl {AsRef<@base>, From}  for thread::{OwnedHandle, Handle, PsuedoHandle});

impl Drop for OwnedHandle { fn drop(&mut self) { unsafe { drop_close_handle(self.0) } } }
#[cfg(std)] impl<T> From<JoinHandle<T>> for OwnedHandle { fn from(jh: JoinHandle<T>) -> Self { unsafe { Self::from_raw(jh.into_raw_handle().cast()).unwrap() } } }
