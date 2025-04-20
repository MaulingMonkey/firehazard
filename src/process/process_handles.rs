use crate::prelude::*;

#[cfg(std)] use std::os::windows::io::{AsRawHandle, IntoRawHandle};



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

#[cfg(std)] impl AsLocalHandleNN for std::process::Child { fn as_handle_nn(&self) -> HANDLENN { HANDLENN::new(self.as_raw_handle().cast()).expect("undefined behavior? std::process::Child somehow had a null handle") } }

#[cfg(std)] impl     From<    std::process::Child> for process::OwnedHandle         { fn from(h:     std::process::Child) -> Self { unsafe { Self::from_raw(h.into_raw_handle().cast()).expect("undefined behavior? std::process::Child somehow had a null handle") } } }
#[cfg(std)] impl<'a> From<&'a std::process::Child> for process::Handle<'a>          { fn from(h: &'a std::process::Child) -> Self { unsafe { Self::from_raw_nn(h.as_handle_nn().cast()) } } }
#[cfg(std)] impl<'a> From<&'a std::process::Child> for process::PseudoHandle<'a>    { fn from(h: &'a std::process::Child) -> Self { unsafe { Self::from_raw_nn(h.as_handle_nn().cast()) } } }



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle)\]
/// CloseHandle
impl Drop for OwnedHandle { fn drop(&mut self) { unsafe { drop_close_handle_nn(self) } } }

unsafe impl valrow::Borrowable for OwnedHandle       { type Abi = HANDLENN; }
unsafe impl valrow::Borrowable for Handle<'_>        { type Abi = HANDLENN; }
unsafe impl valrow::Borrowable for PseudoHandle<'_>  { type Abi = HANDLENN; }

impl OwnedHandle        { #[doc(alias = "DuplicateHandle")] #[doc = r"\[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-duplicatehandle)\] DuplicateHandle"] pub fn try_clone(&self)           -> firehazard::Result<OwnedHandle> { Ok(OwnedHandle(duplicate_handle_local_same_access( self, false)?.into_handle_nn())) } }
impl Handle<'_>         { #[doc(alias = "DuplicateHandle")] #[doc = r"\[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-duplicatehandle)\] DuplicateHandle"] pub fn try_clone_to_owned(&self)  -> firehazard::Result<OwnedHandle> { Ok(OwnedHandle(duplicate_handle_local_same_access(*self, false)?.into_handle_nn())) } }
impl PseudoHandle<'_>   { #[doc(alias = "DuplicateHandle")] #[doc = r"\[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-duplicatehandle)\] DuplicateHandle"] pub fn try_clone_to_owned(&self)  -> firehazard::Result<OwnedHandle> { Ok(OwnedHandle(duplicate_handle_local_same_access(*self, false)?.into_handle_nn())) } }
