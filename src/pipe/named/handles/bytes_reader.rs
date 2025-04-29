#[doc(alias = "HANDLE")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createnamedpipew)\]
/// _Borrowed_, _non-null_ `HANDLE` to a _pipe_ reading bytes. <br>
/// Message framing / `ERROR_MORE_DATA` will be ignored.
///
#[repr(transparent)] pub struct BytesReader<'m>(pub(crate) HANDLENN, PhantomData<&'m mut ()>);

handles!(unsafe impl *LocalHandleNN<c_void>     for pipe::named::{BytesReader<'_>});
handles!(unsafe impl {Send, Sync}               for pipe::named::{BytesReader<'_>});
handles!(       impl Debug                      for pipe::named::{BytesReader<'_>});

handles!(unsafe impl @convert pipe::named::BytesReader<'_> => handle::Borrowed<'_> );
handles!(unsafe impl @convert pipe::named::BytesReader<'_> => handle::Pseudo<'_>   );

//pl crate::os::windows::io::AsHandle       for BytesReader<'_> { fn as_handle(&self) -> crate::os::windows::io::BorrowedHandle { unsafe { crate::os::windows::io::BorrowedHandle::borrow_raw(self.0.as_ptr().cast()) } } }
//pl crate::os::windows::io::AsRawHandle    for BytesReader<'_> { fn as_raw_handle(&self) -> crate::os::windows::io::RawHandle { self.0.as_ptr().cast() } }
impl crate::os::windows::io::FromRawHandle  for BytesReader<'_> { unsafe fn from_raw_handle(handle: crate::os::windows::io::RawHandle) -> Self { Self(HANDLENN::new(handle.cast()).expect("undefined behavior: null is not an open, owned handle"), PhantomData) } }
//pl crate::os::windows::io::IntoRawHandle  for BytesReader<'_> { fn into_raw_handle(self) -> crate::os::windows::io::RawHandle { self.into_handle().cast() } }

unsafe impl valrow::Borrowable for BytesReader<'_> { type Abi = HANDLENN; }



impl io::Read for BytesReader<'_> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let len32 = u32::try_from(buf.len()).unwrap_or(u32::MAX);
        let mut read = 0;
        match firehazard::Error::get_last_if(0 == unsafe { winapi::um::fileapi::ReadFile(
            self.0.as_ptr().cast(),
            buf.as_mut_ptr().cast(),
            len32,
            &mut read,
            null_mut(),
        )}).map_err(u32::from) {
            Ok(())                  => { Ok(usize::from32(read)) },
            Err(ERROR_MORE_DATA)    => { Ok(usize::from32(read)) },
            Err(err)                => { Err(io::Error::from_raw_os_error(err as _)) },
        }
    }
}
