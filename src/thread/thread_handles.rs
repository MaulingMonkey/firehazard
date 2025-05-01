use crate::prelude::*;

#[cfg(std)] use std::os::windows::io::{AsRawHandle, IntoRawHandle};



#[doc(alias = "HANDLE")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createthread)\]
/// _Owned_, _non-null_ `HANDLE` to a _thread_.
///
/// ### Alternatives
///
/// *   <code>std::thread::[Thread](std::thread::Thread)</code> &mdash; cross platform, no stable ABI, no access to the underlying system handle
///
#[repr(transparent)] pub struct OwnedHandle(HANDLENN);

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle)\]
/// CloseHandle
impl Drop for OwnedHandle { fn drop(&mut self) { unsafe { drop_close_handle_nn(self) } } }



#[doc(alias = "HANDLE")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createthread)\]
/// _Borrowed_, _non-null_ `HANDLE` to a _thread_.
///
/// ### Alternatives
///
/// *   <code>&amp;std::thread::[Thread](std::thread::Thread)</code> &mdash; cross platform, no stable ABI, no access to the underlying system handle
///
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Handle<'a>(HANDLENN, PhantomData<&'a HANDLENN>);



#[doc(alias = "HANDLE")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createthread)\]
/// _Borrowed_ or _[pseudo](handle::Pseudo)_, _non-null_ `HANDLE` to a _thread_.
///
/// The only currently known thread pseudo handle is <code>[get_current_thread]\(\)</code> (currently -2).
///
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct PseudoHandle<'a>(HANDLENN, PhantomData<&'a HANDLENN>);

#[doc(hidden)] #[deprecated = "it's spelled PseudoHandle"] pub use PseudoHandle as PsuedoHandle;



handles!(unsafe impl *LocalHandleNN<c_void>         for thread::{OwnedHandle, Handle<'_>, PseudoHandle<'_>});
handles!(unsafe impl TryCloneToOwned<OwnedHandle>   for thread::{OwnedHandle, Handle<'_>, PseudoHandle<'_>});
handles!(unsafe impl Send                           for thread::{OwnedHandle, Handle<'_>}); // PseudoHandle excluded: includes "GetCurrentThread", which shouldn't be sent.
handles!(       impl Debug                          for thread::{OwnedHandle, Handle<'_>, PseudoHandle<'_>});

handles!(unsafe impl @convert &'_ thread::OwnedHandle   => thread::Handle<'_>       );
handles!(unsafe impl @convert &'_ thread::OwnedHandle   => thread::PseudoHandle<'_> );
handles!(unsafe impl @convert thread::Handle<'_>        => thread::PseudoHandle<'_> );

handles!(unsafe impl @convert     thread::OwnedHandle   => handle::Owned            );
handles!(unsafe impl @convert &'_ thread::OwnedHandle   => handle::Borrowed<'_>     );
handles!(unsafe impl @convert &'_ thread::OwnedHandle   => handle::Pseudo<'_>       );
handles!(unsafe impl @convert thread::Handle<'_>        => handle::Borrowed<'_>     );
handles!(unsafe impl @convert thread::Handle<'_>        => handle::Pseudo<'_>       );
handles!(unsafe impl @convert thread::PseudoHandle<'_>  => handle::Pseudo<'_>       );

#[cfg(std)] impl<T> AsLocalHandleNN for std::thread::JoinHandle<T> { fn as_handle_nn(&self) -> HANDLENN { HANDLENN::new(self.as_raw_handle().cast()).expect("undefined behavior? std::thread::JoinHandle somehow had a null handle") } }

#[cfg(std)] impl<    T> From<    std::thread::JoinHandle<T>> for thread::OwnedHandle        { fn from(h:     std::thread::JoinHandle<T>) -> Self { unsafe { Self::from_raw(h.into_raw_handle().cast()).expect("undefined behavior? std::thread::JoinHandle somehow had a null handle") } } }
#[cfg(std)] impl<'a, T> From<&'a std::thread::JoinHandle<T>> for thread::Handle<'a>         { fn from(h: &'a std::thread::JoinHandle<T>) -> Self { unsafe { Self::from_raw_nn(h.as_handle_nn().cast()) } } }
#[cfg(std)] impl<'a, T> From<&'a std::thread::JoinHandle<T>> for thread::PseudoHandle<'a>   { fn from(h: &'a std::thread::JoinHandle<T>) -> Self { unsafe { Self::from_raw_nn(h.as_handle_nn().cast()) } } }

impl TryFrom<handle::Owned> for thread::OwnedHandle {
    type Error = HandleConversionError<handle::Owned>;
    fn try_from(handle: handle::Owned) -> Result<Self, Self::Error> {
        if !is_thread_handle(&handle) { return Err(HandleConversionError { unconverted: handle }) }
        Ok(unsafe { core::mem::transmute(handle) })
    }
}

impl<'a> TryFrom<handle::Borrowed<'a>> for thread::Handle<'a> {
    type Error = HandleConversionError<handle::Borrowed<'a>>;
    fn try_from(handle: handle::Borrowed<'a>) -> Result<Self, Self::Error> {
        if !is_thread_handle(&handle) { return Err(HandleConversionError { unconverted: handle }) }
        Ok(unsafe { core::mem::transmute(handle) })
    }
}

impl<'a> TryFrom<handle::Pseudo<'a>> for thread::PseudoHandle<'a> {
    type Error = HandleConversionError<handle::Pseudo<'a>>;
    fn try_from(handle: handle::Pseudo<'a>) -> Result<Self, Self::Error> {
        if !is_thread_handle(&handle) { return Err(HandleConversionError { unconverted: handle }) }
        Ok(unsafe { core::mem::transmute(handle) })
    }
}



unsafe impl valrow::Borrowable for OwnedHandle       { type Abi = HANDLENN; }
unsafe impl valrow::Borrowable for Handle<'_>        { type Abi = HANDLENN; }
unsafe impl valrow::Borrowable for PseudoHandle<'_>  { type Abi = HANDLENN; }

impl CloneToOwned for OwnedHandle       {}
impl CloneToOwned for Handle<'_>        {}
impl CloneToOwned for PseudoHandle<'_>  {}
