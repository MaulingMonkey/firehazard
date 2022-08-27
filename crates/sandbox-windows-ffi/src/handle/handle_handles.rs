use crate::*;

use winapi::ctypes::c_void;

use core::marker::PhantomData;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/sysinfo/kernel-objects)\]
/// Owned `HANDLE` to a kernel object (will [`CloseHandle`] on [`Drop`])
///
/// [`CloseHandle`]:    https://docs.microsoft.com/en-us/wsindows/win32/api/handleapi/nf-handleapi-closehandle
#[repr(transparent)] pub struct Owned(HANDLENN);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/sysinfo/kernel-objects)\]
/// Borrowed `HANDLE` to a kernel object
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)] #[repr(transparent)] pub struct Borrowed<'a>(HANDLENN, PhantomData<&'a HANDLENN>);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/sysinfo/kernel-objects)\]
/// Borrowed or psuedo-`HANDLE` to a kernel object
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)] #[repr(transparent)] pub struct Psuedo<'a>(HANDLENN, PhantomData<&'a HANDLENN>);

handles!(impl *LocalHandleNN<c_void>    for handle::{Owned, Borrowed, Psuedo});
handles!(impl Debug                     for handle::{Owned, Borrowed, Psuedo});
handles!(impl {AsRef, From}             for handle::{Owned, Borrowed, Psuedo});

impl Drop for Owned { fn drop(&mut self) { unsafe { drop_close_handle_nn(self) } } }
