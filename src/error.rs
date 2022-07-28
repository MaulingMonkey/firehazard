use winapi::um::errhandlingapi::GetLastError;
use std::fmt::{self, Debug, Formatter};



#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct LastError(pub(crate) u32);

impl LastError {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror)\] GetLastError
    pub fn get() -> Self { Self(get_last_error()) }

    pub fn as_u32(self) -> u32 { self.0 }
}

impl Debug for LastError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "LastError({})", self.0)
    }
}

impl From<LastError> for u32 { fn from(err: LastError) -> Self { err.0 } }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror)\] GetLastError
pub(crate) fn get_last_error() -> u32 {
    unsafe { GetLastError() }
}
