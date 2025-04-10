use crate::*;

use winapi::ctypes::c_void;

use core::marker::PhantomData;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/procthread/job-objects)\]
/// _Owned_ `HANDLE` to a job
#[repr(transparent)] pub struct OwnedHandle(HANDLENN);

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/procthread/job-objects)\]
/// _Borrowed_ `HANDLE` to a job
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Handle<'a>(HANDLENN, PhantomData<&'a HANDLENN>);

// No psuedo job handles?

handles!(unsafe impl *LocalHandleNN<c_void>         for job::{OwnedHandle, Handle<'_>});
handles!(unsafe impl AsRef<Self>                    for job::{OwnedHandle, Handle<'_>});
handles!(unsafe impl Send                           for job::{OwnedHandle});
handles!(unsafe impl {AsRef, From}                  for job::{OwnedHandle, Handle<'_>});
handles!(unsafe impl {AsRef<@base>, From<@base>}    for job::{OwnedHandle, Handle<'_>});
handles!(impl Debug                                 for job::{OwnedHandle, Handle<'_>});

impl Drop for OwnedHandle { fn drop(&mut self) { unsafe { drop_close_handle_nn(self) } } }

unsafe impl valrow::Borrowable for OwnedHandle   { type Abi = HANDLENN; }
unsafe impl valrow::Borrowable for Handle<'_>    { type Abi = HANDLENN; }
