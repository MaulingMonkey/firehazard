use crate::*;

use winapi::um::winnt::*;

use core::marker::PhantomData;
use core::ptr::null_mut;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/sysinfo/kernel-objects)\]
/// Owned `HANDLE` to a kernel object (will [`CloseHandle`] on [`Drop`])
///
/// [`CloseHandle`]:    https://docs.microsoft.com/en-us/wsindows/win32/api/handleapi/nf-handleapi-closehandle
#[repr(transparent)] pub struct Owned(HANDLE);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/sysinfo/kernel-objects)\]
/// Borrowed `HANDLE` to a kernel object
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)] #[repr(transparent)] pub struct Borrowed<'a>(HANDLE, PhantomData<&'a HANDLE>);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/sysinfo/kernel-objects)\]
/// Borrowed or psuedo-`HANDLE` to a kernel object
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)] #[repr(transparent)] pub struct Psuedo<'a>(HANDLE, PhantomData<&'a HANDLE>);

handles!(impl *LocalHandle<HANDLE>  for handle::{Owned, Borrowed, Psuedo});
handles!(impl Debug                 for handle::{Owned, Borrowed, Psuedo});
handles!(impl {AsRef, From}         for handle::{Owned, Borrowed, Psuedo});

impl Drop for Owned { fn drop(&mut self) { unsafe { drop_close_handle(self.0) } } }
impl Default for Owned { fn default() -> Self { Self(null_mut()) } }
