use crate::*;
use winapi::um::winnt::*;
use core::fmt::{self, Debug, Formatter};
use core::marker::PhantomData;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/secauthz/access-tokens)\]
/// _Owned_, _nullable_ `HANDLE` to an _Access Token_
#[repr(transparent)] pub struct OwnedHandle(HANDLE);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/secauthz/access-tokens)\]
/// _Borrowed, _nullable_ `HANDLE` to an _Access Token_
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)] #[repr(transparent)] pub struct Handle<'a>(HANDLE, PhantomData<&'a HANDLE>);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/secauthz/access-tokens)\]
/// _Borrowed or psuedo, _nullable_ `HANDLE` to an _Access Token_
///
/// ### References: Local
/// *   [get_current_process_token]
/// *   [get_current_thread_token]
/// *   [get_current_thread_effective_token]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)] #[repr(transparent)] pub struct PsuedoHandle<'a>(HANDLE, PhantomData<&'a HANDLE>);

handles!(impl *LocalHandle<HANDLE>  for token::{OwnedHandle, Handle, PsuedoHandle});
handles!(impl Debug                 for token::{OwnedHandle, Handle}); // XXX: PsuedoHandle specially classed
handles!(impl {AsRef, From}         for token::{OwnedHandle, Handle, PsuedoHandle});
handles!(impl {AsRef<@base>, From}  for token::{OwnedHandle, Handle, PsuedoHandle});

impl PsuedoHandle<'static> { pub(crate) const unsafe fn from_raw_const(c: isize) -> Self { assert!(c != 0); Self(c as _, PhantomData) } }
impl Drop for OwnedHandle { fn drop(&mut self) { unsafe { drop_close_handle(self.0) } } }

impl Debug for PsuedoHandle<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self.0 as isize {
            -4  => write!(fmt, "token::PsuedoHandle(-4 aka GetCurrentProcessToken())"),
            -5  => write!(fmt, "token::PsuedoHandle(-5 aka GetCurrentThreadToken())"),
            -6  => write!(fmt, "token::PsuedoHandle(-6 aka GetCurrentThreadEffectiveToken())"),
            o   => write!(fmt, "token::PsuedoHandle(0x{:08x})", o as usize),
        }
    }
}
