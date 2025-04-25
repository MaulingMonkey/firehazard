#[doc(alias = "HANDLE")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createnamedpipew)\]
/// _Borrowed_, _non-null_ `HANDLE` to a _pipe_ reading a single message, <br>
/// or `INVALID_HANDLE_VALUE` if the message has been completely read.
///
#[repr(transparent)] pub struct MessageReader<'m>(pub(crate) HANDLENN, PhantomData<&'m mut ()>);

impl Drop for MessageReader<'_> {
    fn drop(&mut self) {
        // always read an entire message
        use io::Read;
        let mut buf = [0u8; 1024];
        while let Ok(n) = self.read(&mut buf[..]) { if n == 0 { break } }
    }
}

handles!(unsafe impl *LocalHandleNN<c_void>     for self::{MessageReader<'_>});
handles!(unsafe impl {Send, Sync}               for self::{MessageReader<'_>});
handles!(       impl Debug                      for self::{MessageReader<'_>});

handles!(unsafe impl @convert self::MessageReader<'_> => handle::Borrowed<'_> );
handles!(unsafe impl @convert self::MessageReader<'_> => handle::Pseudo<'_>   );

//pl crate::os::windows::io::AsHandle       for MessageReader<'_> { fn as_handle(&self) -> crate::os::windows::io::BorrowedHandle { unsafe { crate::os::windows::io::BorrowedHandle::borrow_raw(self.0.as_ptr().cast()) } } }
//pl crate::os::windows::io::AsRawHandle    for MessageReader<'_> { fn as_raw_handle(&self) -> crate::os::windows::io::RawHandle { self.0.as_ptr().cast() } }
impl crate::os::windows::io::FromRawHandle  for MessageReader<'_> { unsafe fn from_raw_handle(handle: crate::os::windows::io::RawHandle) -> Self { Self(HANDLENN::new(handle.cast()).expect("undefined behavior: null is not an open, owned handle"), PhantomData) } }
//pl crate::os::windows::io::IntoRawHandle  for MessageReader<'_> { fn into_raw_handle(self) -> crate::os::windows::io::RawHandle { self.into_handle().cast() } }

unsafe impl valrow::Borrowable for MessageReader<'_> { type Abi = HANDLENN; }



impl io::Read for MessageReader<'_> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let invalid = NonNull::new(-1_isize as _).unwrap();
        if self.0 == invalid {
            Ok(0)
        } else {
            let len32 = u32::try_from(buf.len()).unwrap_or(u32::MAX);
            let mut read = 0;
            match firehazard::Error::get_last_if(0 == unsafe { winapi::um::fileapi::ReadFile(
                self.0.as_ptr().cast(),
                buf.as_mut_ptr().cast(),
                len32,
                &mut read,
                null_mut(),
            )}).map_err(u32::from) {
                Ok(())                  => { self.0 = invalid; Ok(usize::from32(read)) },
                Err(ERROR_MORE_DATA)    => {                   Ok(usize::from32(read)) },
                Err(err)                => { self.0 = invalid; Err(io::Error::from_raw_os_error(err as _)) },
            }
        }
    }
}
