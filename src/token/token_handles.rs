use crate::prelude::*;



#[doc(alias = "HANDLE")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/secauthz/access-tokens)\]
/// _Owned_, _non-null_ `HANDLE` to an _Access Token_.
///
#[repr(transparent)] pub struct OwnedHandle(HANDLENN);

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle)\]
/// CloseHandle
impl Drop for OwnedHandle { fn drop(&mut self) { unsafe { drop_close_handle_nn(self) } } }



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
handles!(unsafe impl TryCloneToOwned<OwnedHandle>   for token::{OwnedHandle, Handle<'_>, PseudoHandle<'_>});
handles!(unsafe impl Send                           for token::{OwnedHandle});
handles!(       impl Debug                          for token::{OwnedHandle, Handle<'_>, PseudoHandle<'_>});

handles!(unsafe impl @convert &'_ token::OwnedHandle    => token::Handle<'_>        );
handles!(unsafe impl @convert &'_ token::OwnedHandle    => token::PseudoHandle<'_>  );
handles!(unsafe impl @convert token::Handle<'_>         => token::PseudoHandle<'_>  );

handles!(unsafe impl @convert     token::OwnedHandle    => handle::Owned            );
handles!(unsafe impl @convert &'_ token::OwnedHandle    => handle::Borrowed<'_>     );
handles!(unsafe impl @convert &'_ token::OwnedHandle    => handle::Pseudo<'_>       );
handles!(unsafe impl @convert token::Handle<'_>         => handle::Borrowed<'_>     );
handles!(unsafe impl @convert token::Handle<'_>         => handle::Pseudo<'_>       );
handles!(unsafe impl @convert token::PseudoHandle<'_>   => handle::Pseudo<'_>       );

impl PseudoHandle<'static> { pub(crate) const unsafe fn from_raw_const(c: isize) -> Self { assert!(c != 0); Self(unsafe{NonNull::new_unchecked(c as _)}, PhantomData) } }

unsafe impl valrow::Borrowable for OwnedHandle       { type Abi = HANDLENN; }
unsafe impl valrow::Borrowable for Handle<'_>        { type Abi = HANDLENN; }
unsafe impl valrow::Borrowable for PseudoHandle<'_>  { type Abi = HANDLENN; }

impl CloneToOwned for OwnedHandle       {}
impl CloneToOwned for Handle<'_>        {}
//pl CloneToOwned for PseudoHandle<'_>  {} // token pseudo handles can't be DuplicateHandle d!
