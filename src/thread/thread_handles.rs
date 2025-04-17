use crate::*;

use winapi::ctypes::c_void;

#[cfg(std)] use std::os::windows::io::IntoRawHandle;
#[cfg(std)] use std::thread::JoinHandle;

use core::marker::PhantomData;



#[doc(alias = "HANDLE")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createthread)\]
/// _Owned_, _non-null_ `HANDLE` to a _thread_.
///
#[repr(transparent)] pub struct OwnedHandle(HANDLENN);



#[doc(alias = "HANDLE")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createthread)\]
/// _Borrowed_, _non-null_ `HANDLE` to a _thread_.
///
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Handle<'a>(HANDLENN, PhantomData<&'a HANDLENN>);



#[doc(alias = "HANDLE")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createthread)\]
/// _Borrowed_ or _[psuedo](handle::Psuedo)_, _non-null_ `HANDLE` to a _thread_.
///
/// The only currently known thread psuedo handle is <code>[get_current_thread]\(\)</code> (currently -2).
///
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct PsuedoHandle<'a>(HANDLENN, PhantomData<&'a HANDLENN>);



handles!(unsafe impl *LocalHandleNN<c_void>         for thread::{OwnedHandle, Handle<'_>, PsuedoHandle<'_>});
handles!(       impl AsRef<Self>                    for thread::{OwnedHandle, Handle<'_>, PsuedoHandle<'_>});
handles!(unsafe impl Send                           for thread::{OwnedHandle, Handle<'_>}); // PsuedoHandle excluded: includes "GetCurrentThread", which shouldn't be sent.
handles!(       impl Debug                          for thread::{OwnedHandle, Handle<'_>, PsuedoHandle<'_>});

handles!(unsafe impl @convert &'_ thread::OwnedHandle   => thread::Handle<'_>       );
handles!(unsafe impl @convert &'_ thread::OwnedHandle   => thread::PsuedoHandle<'_> );
handles!(unsafe impl @convert thread::Handle<'_>        => thread::PsuedoHandle<'_> );

handles!(unsafe impl @convert     thread::OwnedHandle   => handle::Owned            );
handles!(unsafe impl @convert &'_ thread::OwnedHandle   => handle::Borrowed<'_>     );
handles!(unsafe impl @convert &'_ thread::OwnedHandle   => handle::Psuedo<'_>       );
handles!(unsafe impl @convert thread::Handle<'_>        => handle::Borrowed<'_>     );
handles!(unsafe impl @convert thread::Handle<'_>        => handle::Psuedo<'_>       );
handles!(unsafe impl @convert thread::PsuedoHandle<'_>  => handle::Psuedo<'_>       );



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle)\]
/// CloseHandle
impl Drop for OwnedHandle { fn drop(&mut self) { unsafe { drop_close_handle_nn(self) } } }

#[cfg(std)] impl<T> From<JoinHandle<T>> for OwnedHandle { fn from(jh: JoinHandle<T>) -> Self { unsafe { Self::from_raw(jh.into_raw_handle().cast()).unwrap() } } }

unsafe impl valrow::Borrowable for OwnedHandle       { type Abi = HANDLENN; }
unsafe impl valrow::Borrowable for Handle<'_>        { type Abi = HANDLENN; }
unsafe impl valrow::Borrowable for PsuedoHandle<'_>  { type Abi = HANDLENN; }
