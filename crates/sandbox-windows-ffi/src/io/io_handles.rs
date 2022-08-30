use crate::*;

use winapi::ctypes::c_void;
use winapi::shared::minwindef::FALSE;
use winapi::um::fileapi::{ReadFile, WriteFile};

#[cfg(std)] use std::os::windows::prelude::*;

use core::marker::PhantomData;
use core::ptr::null_mut;

// TODO: consider implementing std::os::windows::io::* interop traits
// https://doc.rust-lang.org/std/os/windows/io/index.html



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilea)\] Owned non-null file `HANDLE`
#[repr(transparent)] pub struct File(pub(super) HANDLENN);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createpipe)\] Owned anonymous non-null pipe `HANDLE` ([io::Read]able end)
#[repr(transparent)] pub struct ReadPipe (pub(super) HANDLENN);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createpipe)\] Owned anonymous non-null pipe `HANDLE` ([io::Write]able end)
#[repr(transparent)] pub struct WritePipe(pub(super) HANDLENN);

/// Borrowed non-null readable pipe or file `HANDLE`
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct ReadHandle<'a>(pub(super) HANDLENN, PhantomData<&'a HANDLENN>);

/// Borrowed non-null writeable pipe or file `HANDLE`
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct WriteHandle<'a>(pub(super) HANDLENN, PhantomData<&'a HANDLENN>);

handles!(unsafe impl *LocalHandleNN<c_void>         for io::{File});
handles!(unsafe impl AsRef<Self>                    for io::{File});
handles!(unsafe impl {Send, Sync}                   for io::{File});
handles!(unsafe impl {AsRef, From}                  for io::{File});
handles!(unsafe impl {AsRef<@base>, From<@base>}    for io::{File});
handles!(       impl Debug                          for io::{File});

handles!(unsafe impl *LocalHandleNN<c_void>         for io::{ReadPipe, ReadHandle<'_>});
handles!(unsafe impl AsRef<Self>                    for io::{ReadPipe, ReadHandle<'_>});
handles!(unsafe impl {Send, Sync}                   for io::{ReadPipe, ReadHandle<'_>});
handles!(unsafe impl {AsRef, From}                  for io::{ReadPipe, ReadHandle<'_>});
handles!(unsafe impl {AsRef<@base>, From<@base>}    for io::{ReadPipe, ReadHandle<'_>});
handles!(       impl Debug                          for io::{ReadPipe, ReadHandle<'_>});

handles!(unsafe impl *LocalHandleNN<c_void>         for io::{WritePipe, WriteHandle<'_>});
handles!(unsafe impl AsRef<Self>                    for io::{WritePipe, WriteHandle<'_>});
handles!(unsafe impl {Send, Sync}                   for io::{WritePipe, WriteHandle<'_>});
handles!(unsafe impl {AsRef, From}                  for io::{WritePipe, WriteHandle<'_>});
handles!(unsafe impl {AsRef<@base>, From<@base>}    for io::{WritePipe, WriteHandle<'_>});
handles!(       impl Debug                          for io::{WritePipe, WriteHandle<'_>});

handles!(unsafe impl @convert io::File => io::WritePipe );
handles!(unsafe impl @convert io::File => io::ReadPipe  );
handles!(unsafe impl @convert &'_ io::File => io::WriteHandle<'_>   );
handles!(unsafe impl @convert &'_ io::File => io::ReadHandle<'_>    );

impl Drop       for File        { fn drop(&mut self) { unsafe { drop_close_handle_nn(self) } } }
impl Drop       for ReadPipe    { fn drop(&mut self) { unsafe { drop_close_handle_nn(self) } } }
impl Drop       for WritePipe   { fn drop(&mut self) { unsafe { drop_close_handle_nn(self) } } }

impl io::Read   for File            { fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { unsafe { read_file(self.0, buf) } } }
impl io::Read   for ReadPipe        { fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { unsafe { read_file(self.0, buf) } } }
impl io::Read   for ReadHandle<'_>  { fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { unsafe { read_file(self.0, buf) } } }

impl io::Write  for File            { fn write(&mut self, buf: &[u8]) -> io::Result<usize> { unsafe { write_file(self.0, buf) } } fn flush(&mut self) -> io::Result<()> { Ok(()) } }
impl io::Write  for WritePipe       { fn write(&mut self, buf: &[u8]) -> io::Result<usize> { unsafe { write_file(self.0, buf) } } fn flush(&mut self) -> io::Result<()> { Ok(()) } }
impl io::Write  for WriteHandle<'_> { fn write(&mut self, buf: &[u8]) -> io::Result<usize> { unsafe { write_file(self.0, buf) } } fn flush(&mut self) -> io::Result<()> { Ok(()) } }

#[cfg(std)] impl From<std::fs::File> for File { fn from(file: std::fs::File ) -> Self { Self(HANDLENN::new(file.into_raw_handle().cast()).unwrap()) } }
#[cfg(std)] impl From<File> for std::fs::File { fn from(file: File          ) -> Self { unsafe { std::fs::File::from_raw_handle(file.as_handle().cast()) } } }




/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-readfile)\] ReadFile
unsafe fn read_file(h: HANDLENN, buf: &mut [u8]) -> io::Result<usize> {
    let mut read = 0;
    Error::get_last_if(FALSE == unsafe { ReadFile(h.as_ptr(), buf.as_mut_ptr().cast(), buf.len().try_into().unwrap_or(!0u32), &mut read, null_mut()) })?;
    Ok(usize::from32(read))
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-writefile)\] WriteFile
unsafe fn write_file(h: HANDLENN, buf: &[u8]) -> io::Result<usize> {
    let mut written = 0;
    Error::get_last_if(FALSE == unsafe { WriteFile(h.as_ptr(), buf.as_ptr().cast(), buf.len().try_into().unwrap_or(!0u32), &mut written, null_mut()) })?;
    Ok(usize::from32(written))
}
