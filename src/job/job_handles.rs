use crate::*;

use winapi::ctypes::c_void;

use core::marker::PhantomData;



#[doc(alias = "HANDLE")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/procthread/job-objects)\]
/// _Owned_, _non-null_, `HANDLE` to a *job*.
///
#[repr(transparent)] pub struct OwnedHandle(HANDLENN);



#[doc(alias = "HANDLE")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/procthread/job-objects)\]
/// _Borrowed_, _non-null_, `HANDLE` to a *job*.
///
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Handle<'a>(HANDLENN, PhantomData<&'a HANDLENN>);



// No psuedo job handles?



handles!(unsafe impl *LocalHandleNN<c_void>         for job::{OwnedHandle, Handle<'_>});
handles!(       impl AsRef<Self>                    for job::{OwnedHandle, Handle<'_>});
handles!(unsafe impl Send                           for job::{OwnedHandle});
handles!(       impl Debug                          for job::{OwnedHandle, Handle<'_>});

handles!(unsafe impl @convert &'_ job::OwnedHandle  => job::Handle<'_>      );
handles!(unsafe impl @convert     job::OwnedHandle  => handle::Owned        );
handles!(unsafe impl @convert &'_ job::OwnedHandle  => handle::Borrowed<'_> );
handles!(unsafe impl @convert &'_ job::OwnedHandle  => handle::Psuedo<'_>   );
handles!(unsafe impl @convert job::Handle<'_>       => handle::Borrowed<'_> );
handles!(unsafe impl @convert job::Handle<'_>       => handle::Psuedo<'_>   );

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle)\]
/// CloseHandle
impl Drop for OwnedHandle { fn drop(&mut self) { unsafe { drop_close_handle_nn(self) } } }

unsafe impl valrow::Borrowable for OwnedHandle   { type Abi = HANDLENN; }
unsafe impl valrow::Borrowable for Handle<'_>    { type Abi = HANDLENN; }
