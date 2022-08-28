use crate::*;
use winapi::ctypes::c_void;
use winapi::shared::minwindef::FALSE;
use winapi::um::fileapi::{ReadFile, WriteFile};
use core::ptr::null_mut;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createpipe)\] Owned anonymous pipe `HANDLE` (not repr transparent! [Read]able end)
pub struct ReadPipe (pub(super) HANDLENN);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createpipe)\] Owned anonymous pipe `HANDLE` (not repr transparent! [Write]able end)
pub struct WritePipe(pub(super) HANDLENN);

handles!(impl *LocalHandleNN<c_void>    for io::{ReadPipe});
handles!(impl Debug                     for io::{ReadPipe});
handles!(impl {AsRef, From}             for io::{ReadPipe});
handles!(impl {AsRef<@base>, From}      for io::{ReadPipe});

handles!(impl *LocalHandleNN<c_void>    for io::{WritePipe});
handles!(impl Debug                     for io::{WritePipe});
handles!(impl {AsRef, From}             for io::{WritePipe});
handles!(impl {AsRef<@base>, From}      for io::{WritePipe});

impl Drop for ReadPipe  { fn drop(&mut self) { unsafe { drop_close_handle_nn(self) } } }
impl Drop for WritePipe { fn drop(&mut self) { unsafe { drop_close_handle_nn(self) } } }

impl io::Read for ReadPipe {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-readfile)\]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut read = 0;
        Error::get_last_if(FALSE == unsafe { ReadFile(self.0.as_ptr(), buf.as_mut_ptr().cast(), buf.len().try_into().unwrap_or(!0u32), &mut read, null_mut()) })?;
        Ok(usize::from32(read))
    }
}

impl io::Write for WritePipe {
    fn flush(&mut self) -> io::Result<()> { Ok(()) }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-writefile)\]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut written = 0;
        Error::get_last_if(FALSE == unsafe { WriteFile(self.0.as_ptr(), buf.as_ptr().cast(), buf.len().try_into().unwrap_or(!0u32), &mut written, null_mut()) })?;
        Ok(usize::from32(written))
    }
}
