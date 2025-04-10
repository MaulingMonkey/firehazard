use crate::*;

use winapi::ctypes::c_void;

use core::marker::PhantomData;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/sysinfo/kernel-objects)\]
/// Owned `HANDLE` to a kernel object (will [`CloseHandle`] on [`Drop`])
///
/// [`CloseHandle`]:    https://learn.microsoft.com/en-us/wsindows/win32/api/handleapi/nf-handleapi-closehandle
#[repr(transparent)] pub struct Owned(HANDLENN);

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/sysinfo/kernel-objects)\]
/// Borrowed `HANDLE` to a kernel object
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)] #[repr(transparent)] pub struct Borrowed<'a>(HANDLENN, PhantomData<&'a HANDLENN>);

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/sysinfo/kernel-objects)\]
/// Borrowed or psuedo-`HANDLE` to a kernel object
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)] #[repr(transparent)] pub struct Psuedo<'a>(HANDLENN, PhantomData<&'a HANDLENN>);

handles!(unsafe impl *LocalHandleNN<c_void> for handle::{Owned, Borrowed<'_>, Psuedo<'_>});
handles!(unsafe impl AsRef<Self>            for handle::{Owned, Borrowed<'_>, Psuedo<'_>});
handles!(unsafe impl {AsRef, From}          for handle::{Owned, Borrowed<'_>, Psuedo<'_>});
handles!(impl Debug                         for handle::{Owned, Borrowed<'_>, Psuedo<'_>});

impl Drop for Owned { fn drop(&mut self) { unsafe { drop_close_handle_nn(self) } } }

unsafe impl valrow::Borrowable for Owned         { type Abi = HANDLENN; }
unsafe impl valrow::Borrowable for Borrowed<'_>  { type Abi = HANDLENN; }
unsafe impl valrow::Borrowable for Psuedo<'_>    { type Abi = HANDLENN; }



#[cfg(test)] pub(crate) mod invalid {
    use crate::*;

    #[repr(transparent)] struct Invalid(HANDLE);
    #[repr(transparent)] struct InvalidNN(HANDLENN);

    impl AsLocalHandle   for Invalid   { fn as_handle(&self) -> HANDLE { self.0 } }
    impl AsLocalHandleNN for InvalidNN { fn as_handle_nn(&self) -> HANDLENN { self.0 } }

    pub(crate) fn null()                    -> impl AsLocalHandle                   { Invalid(core::ptr::null_mut()) }
    pub(crate) fn invalid_value()           -> impl AsLocalHandle + AsLocalHandleNN { InvalidNN(unsafe { core::ptr::NonNull::new_unchecked(winapi::um::handleapi::INVALID_HANDLE_VALUE.cast()) }) }
    pub(crate) fn never_valid()             -> impl AsLocalHandle + AsLocalHandleNN { InvalidNN(unsafe { core::ptr::NonNull::new_unchecked(0x12345678_usize as *mut _) }) }
    #[cfg(std)] pub(crate) fn dangling()    -> impl AsLocalHandle + AsLocalHandleNN {
        use std::os::windows::io::AsRawHandle;
        let file = std::fs::File::open("Readme.md").unwrap();
        let dangling = file.as_raw_handle().cast();
        drop(file);
        InvalidNN(core::ptr::NonNull::new(dangling).unwrap())
    }
}
