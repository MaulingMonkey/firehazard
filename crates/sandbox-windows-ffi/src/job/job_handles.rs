use crate::*;

use winapi::um::winnt::*;

use core::marker::PhantomData;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/procthread/job-objects)\]
/// _Owned_ `HANDLE` to a job
#[repr(transparent)] pub struct OwnedHandle(HANDLE);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/procthread/job-objects)\]
/// _Borrowed_ `HANDLE` to a job
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Handle<'a>(HANDLE, PhantomData<&'a HANDLE>);

// No psuedo job handles?

handles!(impl *LocalHandle<HANDLE>  for job::{OwnedHandle, Handle});
handles!(impl Debug                 for job::{OwnedHandle, Handle});
handles!(impl {AsRef, From}         for job::{OwnedHandle, Handle});
handles!(impl {AsRef<@base>, From}  for job::{OwnedHandle, Handle});

impl Drop for OwnedHandle { fn drop(&mut self) { unsafe { drop_close_handle(self.0) } } }
