use crate::*;

use winapi::ctypes::c_void;

#[cfg(std)] use std::os::windows::io::IntoRawHandle;
#[cfg(std)] use std::process::Child;

use core::marker::PhantomData;



#[doc(alias = "HANDLE")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessw)\]
/// _Owned_, _non-null_ `HANDLE` to a _process_.
///
#[repr(transparent)] pub struct OwnedHandle(HANDLENN);



#[doc(alias = "HANDLE")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessw)\]
/// _Borrowed_, _non-null_ `HANDLE` to a _process_.
///
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Handle<'a>(HANDLENN, PhantomData<&'a HANDLENN>);



#[doc(alias = "HANDLE")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessw)\]
/// _Borrowed_ or _[pseudo](handle::Pseudo)_, _non-null_ `HANDLE` to a _process_.
///
/// The only currently known process pseudo handle is <code>[get_current_process]\(\)</code> (currently -1).
///
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct PseudoHandle<'a>(HANDLENN, PhantomData<&'a HANDLENN>);

#[doc(hidden)] #[deprecated = "it's spelled PseudoHandle"] pub use PseudoHandle as PsuedoHandle;



handles!(unsafe impl *LocalHandleNN<c_void>         for process::{OwnedHandle, Handle<'_>, PseudoHandle<'_>});
handles!(       impl AsRef<Self>                    for process::{OwnedHandle, Handle<'_>, PseudoHandle<'_>});
handles!(unsafe impl Send                           for process::{OwnedHandle, Handle<'_>, PseudoHandle<'_>}); // sending GetCurrentProcess between threads is sane enough
handles!(       impl Debug                          for process::{OwnedHandle, Handle<'_>, PseudoHandle<'_>});

handles!(unsafe impl @convert &'_ process::OwnedHandle  => process::Handle<'_>          );
handles!(unsafe impl @convert &'_ process::OwnedHandle  => process::PseudoHandle<'_>    );
handles!(unsafe impl @convert     process::OwnedHandle  => handle::Owned                );
handles!(unsafe impl @convert &'_ process::OwnedHandle  => handle::Borrowed<'_>         );
handles!(unsafe impl @convert &'_ process::OwnedHandle  => handle::Pseudo<'_>           );

handles!(unsafe impl @convert process::Handle<'_>       => process::PseudoHandle<'_>    );
handles!(unsafe impl @convert process::Handle<'_>       => handle::Borrowed<'_>         );
handles!(unsafe impl @convert process::Handle<'_>       => handle::Pseudo<'_>           );

handles!(unsafe impl @convert process::PseudoHandle<'_> => handle::Pseudo<'_>           );



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle)\]
/// CloseHandle
impl Drop for OwnedHandle { fn drop(&mut self) { unsafe { drop_close_handle_nn(self) } } }

#[cfg(std)] impl From<Child> for OwnedHandle { fn from(c: Child) -> Self { unsafe { Self::from_raw(c.into_raw_handle().cast()).unwrap() } } }

unsafe impl valrow::Borrowable for OwnedHandle       { type Abi = HANDLENN; }
unsafe impl valrow::Borrowable for Handle<'_>        { type Abi = HANDLENN; }
unsafe impl valrow::Borrowable for PseudoHandle<'_>  { type Abi = HANDLENN; }
