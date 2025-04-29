#[doc(alias = "HANDLE")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createnamedpipew)\]
#[repr(transparent)] pub struct Listener(pub(crate) HANDLENN);

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle)\]
/// CloseHandle
impl Drop for pipe::named::Listener { fn drop(&mut self) { unsafe { drop_close_handle_nn(self) } } }

handles!(unsafe impl *LocalHandleNN<c_void>     for pipe::named::{Listener});
handles!(unsafe impl TryCloneToOwned<Listener>  for pipe::named::{Listener});
handles!(unsafe impl {Send, Sync}               for pipe::named::{Listener});
handles!(       impl Debug                      for pipe::named::{Listener});

handles!(unsafe impl @convert     pipe::named::Listener => handle::Owned        );
handles!(unsafe impl @convert &'_ pipe::named::Listener => handle::Borrowed<'_> );
handles!(unsafe impl @convert &'_ pipe::named::Listener => handle::Pseudo<'_>   );

//pl crate::os::windows::io::AsHandle       for pipe::named::Listener { fn as_handle(&self) -> crate::os::windows::io::BorrowedHandle { unsafe { crate::os::windows::io::BorrowedHandle::borrow_raw(self.0.as_ptr().cast()) } } }
//pl crate::os::windows::io::AsRawHandle    for pipe::named::Listener { fn as_raw_handle(&self) -> crate::os::windows::io::RawHandle { self.0.as_ptr().cast() } }
impl crate::os::windows::io::FromRawHandle  for pipe::named::Listener { unsafe fn from_raw_handle(handle: crate::os::windows::io::RawHandle) -> Self { Self(HANDLENN::new(handle.cast()).expect("undefined behavior: null is not an open, owned handle")) } }
impl crate::os::windows::io::IntoRawHandle  for pipe::named::Listener { fn into_raw_handle(self) -> crate::os::windows::io::RawHandle { self.into_handle().cast() } }

unsafe impl valrow::Borrowable for pipe::named::Listener { type Abi = HANDLENN; }

impl CloneToOwned for pipe::named::Listener {}



impl pipe::named::Listener {
    #[doc(alias = "ConnectNamedPipe")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-connectnamedpipe)\]
    /// ConnectNamedPipe(handle, nullptr)
    ///
    pub fn accept(self) -> Result<pipe::named::Connected, (pipe::named::Listener, firehazard::Error)> {
        if let Err(err) = pipe::named::connect(&self, None) {
            Err((self, err))
        } else {
            Ok(pipe::named::Connected(core::mem::ManuallyDrop::new(self).0))
        }
    }
}
