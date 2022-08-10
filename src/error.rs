//! Error handling types and functions

use winapi::shared::winerror::*;
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
        let name = match self.0 {
            ERROR_ACCESS_DENIED         => "ERROR_ACCESS_DENIED",       // 5
            ERROR_BAD_LENGTH            => "ERROR_BAD_LENGTH",          // 24
            ERROR_INVALID_PARAMETER     => "ERROR_INVALID_PARAMETER",   // 87
            ERROR_INSUFFICIENT_BUFFER   => "ERROR_INSUFFICIENT_BUFFER", // 122
            ERROR_NO_TOKEN              => "ERROR_NO_TOKEN",            // 1008
            ERROR_PRIVILEGE_NOT_HELD    => "ERROR_PRIVILEGE_NOT_HELD",  // 1314
            ERROR_BAD_TOKEN_TYPE        => "ERROR_BAD_TOKEN_TYPE",      // 1349
            ERROR_INCORRECT_SIZE        => "ERROR_INCORRECT_SIZE",      // 1462
            _                           => "ERROR_???",
        };
        write!(fmt, "LastError({} {name})", self.0)
    }
}

impl From<LastError> for u32 { fn from(err: LastError) -> Self { err.0 } }
impl PartialEq<u32> for LastError { fn eq(&self, other: &u32) -> bool { self.0 == *other } }
impl PartialEq<LastError> for u32 { fn eq(&self, other: &LastError) -> bool { *self == other.0 } }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror)\] GetLastError
pub(crate) fn get_last_error() -> u32 {
    unsafe { GetLastError() }
}
