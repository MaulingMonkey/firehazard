use crate::*;
use winapi::ctypes::c_void;
use core::marker::PhantomData;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/secauthz/access-tokens)\]
/// _Owned_, _non-null_ `HANDLE` to an _Access Token_
#[repr(transparent)] pub struct OwnedHandle(HANDLENN);

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/secauthz/access-tokens)\]
/// _Borrowed, _non-null_ `HANDLE` to an _Access Token_
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Handle<'a>(HANDLENN, PhantomData<&'a HANDLENN>);

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/secauthz/access-tokens)\]
/// _Borrowed or psuedo, _non-null_ `HANDLE` to an _Access Token_
///
/// ### References: Local
/// *   [get_current_process_token]
/// *   [get_current_thread_token]
/// *   [get_current_thread_effective_token]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct PsuedoHandle<'a>(HANDLENN, PhantomData<&'a HANDLENN>);

handles!(unsafe impl *LocalHandleNN<c_void>         for token::{OwnedHandle, Handle<'_>, PsuedoHandle<'_>});
handles!(unsafe impl AsRef<Self>                    for token::{OwnedHandle, Handle<'_>, PsuedoHandle<'_>});
handles!(unsafe impl Send                           for token::{OwnedHandle});
handles!(unsafe impl {AsRef, From}                  for token::{OwnedHandle, Handle<'_>, PsuedoHandle<'_>});
handles!(unsafe impl {AsRef<@base>, From<@base>}    for token::{OwnedHandle, Handle<'_>}); // XXX: token PsuedoHandles cannot be DuplicateHandle()d, so exclude them from conversion to generic handle::Psuedo s - see duplicate_handle_local[_same_access]
handles!(impl Debug                                 for token::{OwnedHandle, Handle<'_>, PsuedoHandle<'_>});

impl PsuedoHandle<'static> { pub(crate) const unsafe fn from_raw_const(c: isize) -> Self { assert!(c != 0); Self(unsafe{core::ptr::NonNull::new_unchecked(c as _)}, PhantomData) } }
impl Drop for OwnedHandle { fn drop(&mut self) { unsafe { drop_close_handle_nn(self) } } }

unsafe impl valrow::Borrowable for OwnedHandle       { type Abi = HANDLENN; }
unsafe impl valrow::Borrowable for Handle<'_>        { type Abi = HANDLENN; }
unsafe impl valrow::Borrowable for PsuedoHandle<'_>  { type Abi = HANDLENN; }
