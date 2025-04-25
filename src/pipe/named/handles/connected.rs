#[doc(alias = "HANDLE")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createnamedpipew)\]
#[repr(transparent)] pub struct Connected(pub(crate) HANDLENN);

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle)\]
/// CloseHandle
impl Drop for Connected { fn drop(&mut self) { unsafe { drop_close_handle_nn(self) } } }

handles!(unsafe impl *LocalHandleNN<c_void>     for self::{Connected});
handles!(unsafe impl {Send, Sync}               for self::{Connected});
handles!(       impl Debug                      for self::{Connected});

handles!(unsafe impl @convert     self::Connected => handle::Owned        );
handles!(unsafe impl @convert &'_ self::Connected => handle::Borrowed<'_> );
handles!(unsafe impl @convert &'_ self::Connected => handle::Pseudo<'_>   );

//pl crate::os::windows::io::AsHandle       for Connected { fn as_handle(&self) -> crate::os::windows::io::BorrowedHandle { unsafe { crate::os::windows::io::BorrowedHandle::borrow_raw(self.0.as_ptr().cast()) } } }
//pl crate::os::windows::io::AsRawHandle    for Connected { fn as_raw_handle(&self) -> crate::os::windows::io::RawHandle { self.0.as_ptr().cast() } }
impl crate::os::windows::io::FromRawHandle  for Connected { unsafe fn from_raw_handle(handle: crate::os::windows::io::RawHandle) -> Self { Self(HANDLENN::new(handle.cast()).expect("undefined behavior: null is not an open, owned handle")) } }
impl crate::os::windows::io::IntoRawHandle  for Connected { fn into_raw_handle(self) -> crate::os::windows::io::RawHandle { self.into_handle().cast() } }

unsafe impl valrow::Borrowable for Connected { type Abi = HANDLENN; }



impl Connected {
    // as_messages
    // as_bytes

    pub fn bytes(&mut self) -> BytesReader { unsafe { BytesReader::from_raw_nn(self.0.cast()) } }
    pub fn next_message(&mut self) -> MessageReader { unsafe { MessageReader::from_raw_nn(self.0.cast()) } }

    fn write_message(&self, message: &[u8]) -> io::Result<usize> {
        u32::try_from(message.len()).map_err(|_| io::Error::new(io::ErrorKind::OutOfMemory, "cannot write 4 GiB in a single pipe message"))?;
        Ok(usize::from32(unsafe { write_file(self, message, None) }?))
    }



    #[doc(alias = "FlushFileBuffers")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-flushfilebuffers)\]
    /// FlushFileBuffers(self)
    pub fn flush_file_buffers(&self) -> firehazard::Result<()> { flush_file_buffers(self) }



    #[doc(alias = "DisconnectNamedPipe")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-disconnectnamedpipe)\]
    /// DisconnectNamedPipe(self)
    pub fn disconnect(self) -> Result<pipe::named::Listener, (Self, firehazard::Error)> {
        if let Err(err) = pipe::named::disconnect(&self) {
            Err((self, err))
        } else {
            Ok(pipe::named::Listener(core::mem::ManuallyDrop::new(self).0))
        }
    }



    // XXX: deadlock bait
    // #[doc(alias = "DuplicateHandle")]
    // /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-duplicatehandle)\]
    // /// DuplicateHandle(..., self, ...)
    // pub fn try_clone(&self) -> firehazard::Result<Self> { Ok(Self(duplicate_handle_local_same_access(self, false)?.into_handle_nn())) }
}

impl io::Write for Connected { fn flush(&mut self) -> io::Result<()> { Ok(self.flush_file_buffers()?) } fn write(&mut self, buf: &[u8]) -> io::Result<usize> { self.write_message(buf) } }
