use crate::*;
use winapi::ctypes::c_void;
use core::fmt::{self, Debug, Formatter};
use core::marker::PhantomData;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/secauthz/access-tokens)\]
/// _Owned_, _nullable_ `HANDLE` to an _Access Token_
#[repr(transparent)] pub struct OwnedHandle(HANDLENN);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/secauthz/access-tokens)\]
/// _Borrowed, _nullable_ `HANDLE` to an _Access Token_
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Handle<'a>(HANDLENN, PhantomData<&'a HANDLENN>);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/secauthz/access-tokens)\]
/// _Borrowed or psuedo, _nullable_ `HANDLE` to an _Access Token_
///
/// ### References: Local
/// *   [get_current_process_token]
/// *   [get_current_thread_token]
/// *   [get_current_thread_effective_token]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct PsuedoHandle<'a>(HANDLENN, PhantomData<&'a HANDLENN>);

handles!(unsafe impl *LocalHandleNN<c_void> for token::{OwnedHandle, Handle, PsuedoHandle});
handles!(unsafe impl {AsRef, From}          for token::{OwnedHandle, Handle, PsuedoHandle});
handles!(unsafe impl {AsRef<@base>, From}   for token::{OwnedHandle, Handle, PsuedoHandle});
handles!(impl Debug                         for token::{OwnedHandle, Handle}); // XXX: PsuedoHandle specially classed

impl PsuedoHandle<'static> { pub(crate) const unsafe fn from_raw_const(c: isize) -> Self { assert!(c != 0); Self(unsafe{core::ptr::NonNull::new_unchecked(c as _)}, PhantomData) } }
impl Drop for OwnedHandle { fn drop(&mut self) { unsafe { drop_close_handle_nn(self) } } }

impl Debug for PsuedoHandle<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self.0.as_ptr() as isize {
            -4  => write!(fmt, "token::PsuedoHandle(-4 aka GetCurrentProcessToken())"),
            -5  => write!(fmt, "token::PsuedoHandle(-5 aka GetCurrentThreadToken())"),
            -6  => write!(fmt, "token::PsuedoHandle(-6 aka GetCurrentThreadEffectiveToken())"),
            o   => write!(fmt, "token::PsuedoHandle(0x{:08x})", o as usize),
        }
    }
}
