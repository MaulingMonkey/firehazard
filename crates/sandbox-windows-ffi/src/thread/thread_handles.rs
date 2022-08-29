use crate::*;

use winapi::ctypes::c_void;

#[cfg(std)] use std::os::windows::io::IntoRawHandle;
#[cfg(std)] use std::thread::JoinHandle;

use core::marker::PhantomData;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createthread)\]
/// `HANDLE` to a thread
#[repr(transparent)] pub struct OwnedHandle(HANDLENN);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createthread)\]
/// `HANDLE` to a thread
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Handle<'a>(HANDLENN, PhantomData<&'a HANDLENN>);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createthread)\]
/// `HANDLE` to a thread
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct PsuedoHandle<'a>(HANDLENN, PhantomData<&'a HANDLENN>);

handles!(unsafe impl *LocalHandleNN<c_void>         for thread::{OwnedHandle, Handle<'_>, PsuedoHandle<'_>});
handles!(unsafe impl AsRef<Self>                    for thread::{OwnedHandle, Handle<'_>, PsuedoHandle<'_>});
handles!(unsafe impl {Send, Sync}                   for thread::{OwnedHandle, Handle<'_>, PsuedoHandle<'_>});
handles!(unsafe impl {AsRef, From}                  for thread::{OwnedHandle, Handle<'_>, PsuedoHandle<'_>});
handles!(unsafe impl {AsRef<@base>, From<@base>}    for thread::{OwnedHandle, Handle<'_>, PsuedoHandle<'_>});
handles!(impl Debug                                 for thread::{OwnedHandle, Handle<'_>, PsuedoHandle<'_>});

impl Drop for OwnedHandle { fn drop(&mut self) { unsafe { drop_close_handle_nn(self) } } }
#[cfg(std)] impl<T> From<JoinHandle<T>> for OwnedHandle { fn from(jh: JoinHandle<T>) -> Self { unsafe { Self::from_raw(jh.into_raw_handle().cast()).unwrap() } } }