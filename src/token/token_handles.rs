use crate::*;
use winapi::ctypes::c_void;
use core::marker::PhantomData;



#[doc(alias = "HANDLE")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/secauthz/access-tokens)\]
/// _Owned_, _non-null_ `HANDLE` to an _Access Token_.
///
#[repr(transparent)] pub struct OwnedHandle(HANDLENN);



#[doc(alias = "HANDLE")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/secauthz/access-tokens)\]
/// _Borrowed_, _non-null_ `HANDLE` to an _Access Token_.
///
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Handle<'a>(HANDLENN, PhantomData<&'a HANDLENN>);



#[doc(alias = "HANDLE")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/secauthz/access-tokens)\]
/// _Borrowed_ or _[pseudo](handle::Pseudo)_, _non-null_ `HANDLE` to an _Access Token_.
///
/// Known access token pseudo handles include:
/// *   [get_current_process_token]             (currently -4)
/// *   [get_current_thread_token]              (currently -5)
/// *   [get_current_thread_effective_token]    (currently -6)
///
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct PseudoHandle<'a>(HANDLENN, PhantomData<&'a HANDLENN>);

#[doc(hidden)] #[deprecated = "it's spelled PseudoHandle"] pub use PseudoHandle as PsuedoHandle;



handles!(unsafe impl *LocalHandleNN<c_void>         for token::{OwnedHandle, Handle<'_>, PseudoHandle<'_>});
handles!(       impl AsRef<Self>                    for token::{OwnedHandle, Handle<'_>, PseudoHandle<'_>});
handles!(unsafe impl Send                           for token::{OwnedHandle});
handles!(       impl Debug                          for token::{OwnedHandle, Handle<'_>, PseudoHandle<'_>});

handles!(unsafe impl @convert &'_ token::OwnedHandle    => token::Handle<'_>        );
handles!(unsafe impl @convert &'_ token::OwnedHandle    => token::PseudoHandle<'_>  );
handles!(unsafe impl @convert token::Handle<'_>         => token::PseudoHandle<'_>  );

handles!(unsafe impl @convert     token::OwnedHandle    => handle::Owned            );
handles!(unsafe impl @convert &'_ token::OwnedHandle    => handle::Borrowed<'_>     );
//ndles!(unsafe impl @convert &'_ token::OwnedHandle    => handle::Pseudo<'_>       ); // XXX: token PseudoHandles cannot be DuplicateHandle()d, so exclude them from conversion to generic handle::Pseudo s - see duplicate_handle_local[_same_access]
handles!(unsafe impl @convert token::Handle<'_>         => handle::Borrowed<'_>     );
//ndles!(unsafe impl @convert token::Handle<'_>         => handle::Pseudo<'_>       ); // XXX: token PseudoHandles cannot be DuplicateHandle()d, so exclude them from conversion to generic handle::Pseudo s - see duplicate_handle_local[_same_access]
//ndles!(unsafe impl @convert token::PseudoHandle<'_>   => handle::Pseudo<'_>       ); // XXX: token PseudoHandles cannot be DuplicateHandle()d, so exclude them from conversion to generic handle::Pseudo s - see duplicate_handle_local[_same_access]

impl PseudoHandle<'static> { pub(crate) const unsafe fn from_raw_const(c: isize) -> Self { assert!(c != 0); Self(unsafe{core::ptr::NonNull::new_unchecked(c as _)}, PhantomData) } }

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle)\]
/// CloseHandle
impl Drop for OwnedHandle { fn drop(&mut self) { unsafe { drop_close_handle_nn(self) } } }

unsafe impl valrow::Borrowable for OwnedHandle       { type Abi = HANDLENN; }
unsafe impl valrow::Borrowable for Handle<'_>        { type Abi = HANDLENN; }
unsafe impl valrow::Borrowable for PseudoHandle<'_>  { type Abi = HANDLENN; }
