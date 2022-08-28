use core::fmt::{self, Debug, Formatter};
use std::fs::File;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createpipe)\] Owned anonymous pipe `HANDLE` (not repr transparent! [Read]able end)
pub struct ReadPipe (pub(super) File);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createpipe)\] Owned anonymous pipe `HANDLE` (not repr transparent! [Write]able end)
pub struct WritePipe(pub(super) File);

impl std::io::Read for ReadPipe {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> { self.0.read(buf) }
}

impl std::io::Write for WritePipe {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> { self.0.write(buf) }
    fn flush(&mut self) -> std::io::Result<()> { self.0.flush() }
}

impl Debug for ReadPipe     { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { fmt.debug_struct("ReadPipe").finish_non_exhaustive() } }
impl Debug for WritePipe    { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { fmt.debug_struct("WritePipe").finish_non_exhaustive() } }
