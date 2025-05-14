/// <code>[Ok]\(T\)</code> | <code>[Err]\([firehazard]::[Error]\)</code>
pub type Result<T> = core::result::Result<T, Error>;



pub(crate) trait ResultErrorExt<R>      { fn unerr(self, err: u32, remap: R) -> Self; }
impl<R> ResultErrorExt<R> for Result<R> { fn unerr(self, err: u32, remap: R) -> Self { match self { Err(e) if e == err => Ok(remap), r => r } } }



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror)\]
/// DWORD/[u32], typically from GetLastError, representing an error code, hresult, or ntstatus
///
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Error(pub(crate) u32);

impl Error {
    #[doc(alias = "GetLastError")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror)\]
    /// GetLastError
    ///
    pub fn get_last() -> Self { Self(unsafe { winapi::um::errhandlingapi::GetLastError() }) }

    #[doc(alias = "GetLastError")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror)\]
    /// GetLastError if `error`
    ///
    pub fn get_last_if(error: bool) -> Result<()> { if !error { Ok(()) } else { Err(Self::get_last()) } }

    pub fn friendly(self) -> &'static str {
        use winapi::shared::ntstatus::*;
        use winapi::shared::winerror::*;
        match self.0 {
            NO_ERROR                        => "NO_ERROR",                      // 0
            ERROR_FILE_NOT_FOUND            => "ERROR_FILE_NOT_FOUND",          // 2
            ERROR_PATH_NOT_FOUND            => "ERROR_PATH_NOT_FOUND",          // 3
            ERROR_ACCESS_DENIED             => "ERROR_ACCESS_DENIED",           // 5
            ERROR_INVALID_HANDLE            => "ERROR_INVALID_HANDLE",          // 6
            ERROR_BAD_ENVIRONMENT           => "ERROR_BAD_ENVIRONMENT",         // 10
            ERROR_BAD_LENGTH                => "ERROR_BAD_LENGTH",              // 24
            ERROR_NOT_SUPPORTED             => "ERROR_NOT_SUPPORTED",           // 50
            ERROR_INVALID_PARAMETER         => "ERROR_INVALID_PARAMETER",       // 87
            ERROR_EXCL_SEM_ALREADY_OWNED    => "ERROR_EXCL_SEM_ALREADY_OWNED",  // 101
            ERROR_CALL_NOT_IMPLEMENTED      => "ERROR_CALL_NOT_IMPLEMENTED",    // 120
            ERROR_INSUFFICIENT_BUFFER       => "ERROR_INSUFFICIENT_BUFFER",     // 122
            ERROR_BAD_ARGUMENTS             => "ERROR_BAD_ARGUMENTS",           // 160
            ERROR_BAD_PATHNAME              => "ERROR_BAD_PATHNAME",            // 161
            ERROR_BUSY                      => "ERROR_BUSY",                    // 170
            ERROR_ALREADY_EXISTS            => "ERROR_ALREADY_EXISTS",          // 183
            ERROR_PIPE_LOCAL                => "ERROR_PIPE_LOCAL",              // 229
            ERROR_PARTIAL_COPY              => "ERROR_PARTIAL_COPY",            // 299
            367                             => "ERROR_CHILD_PROCESS_BLOCKED",   // 367
            ERROR_ILLEGAL_CHARACTER         => "ERROR_ILLEGAL_CHARACTER",       // 582
            ERROR_NO_TOKEN                  => "ERROR_NO_TOKEN",                // 1008
            ERROR_NO_SUCH_PRIVILEGE         => "ERROR_NO_SUCH_PRIVILEGE",       // 1313
            ERROR_PRIVILEGE_NOT_HELD        => "ERROR_PRIVILEGE_NOT_HELD",      // 1314
            ERROR_INVALID_SUB_AUTHORITY     => "ERROR_INVALID_SUB_AUTHORITY",   // 1335
            ERROR_INVALID_ACL               => "ERROR_INVALID_ACL",             // 1336
            ERROR_INVALID_SID               => "ERROR_INVALID_SID",             // 1337
            ERROR_INVALID_SECURITY_DESCR    => "ERROR_INVALID_SECURITY_DESCR",  // 1338
            ERROR_BAD_INHERITANCE_ACL       => "ERROR_BAD_INHERITANCE_ACL",     // 1340
            ERROR_INVALID_ID_AUTHORITY      => "ERROR_INVALID_ID_AUTHORITY",    // 1343
            ERROR_ALLOTTED_SPACE_EXCEEDED   => "ERROR_ALLOTTED_SPACE_EXCEEDED", // 1344
            ERROR_BAD_TOKEN_TYPE            => "ERROR_BAD_TOKEN_TYPE",          // 1349
            ERROR_INCORRECT_SIZE            => "ERROR_INCORRECT_SIZE",          // 1462
            ERROR_DYNAMIC_CODE_BLOCKED      => "ERROR_DYNAMIC_CODE_BLOCKED",    // 1655
            1657                            => "ERROR_STRICT_CFG_VIOLATION",    // 1657
            ERROR_INVALID_ENVIRONMENT       => "ERROR_INVALID_ENVIRONMENT",     // 1805 - prefer ERROR_BAD_ENVIRONMENT ?
            ERROR_NOT_APPCONTAINER          => "ERROR_NOT_APPCONTAINER",        // 4250
            0x800700B7                      => "HRESULT_FROM_WIN32(ERROR_ALREADY_EXISTS)",
            _                               => match self.0 as _ {
                STATUS_BUFFER_OVERFLOW          => "STATUS_BUFFER_OVERFLOW",            // 0x80000005
                E_STRING_NOT_NULL_TERMINATED    => "E_STRING_NOT_NULL_TERMINATED",      // 0x80000017
                E_INVALIDARG                    => "E_INVALIDARG",                      // 0x80070057
                STATUS_INVALID_INFO_CLASS       => "STATUS_INVALID_INFO_CLASS",         // 0xC0000003
                STATUS_INFO_LENGTH_MISMATCH     => "STATUS_INFO_LENGTH_MISMATCH",       // 0xC0000004
                STATUS_ACCESS_VIOLATION         => "STATUS_ACCESS_VIOLATION",           // 0xC0000005
                STATUS_INVALID_HANDLE           => "STATUS_INVALID_HANDLE",             // 0xC0000008
                STATUS_ACCESS_DENIED            => "STATUS_ACCESS_DENIED",              // 0xC0000022
                STATUS_BUFFER_TOO_SMALL         => "STATUS_BUFFER_TOO_SMALL",           // 0xC0000023
                STATUS_OBJECT_TYPE_MISMATCH     => "STATUS_OBJECT_TYPE_MISMATCH",       // 0xC0000024
                STATUS_NONE_MAPPED              => "STATUS_NONE_MAPPED",                // 0xC0000073
                STATUS_BAD_IMPERSONATION_LEVEL  => "STATUS_BAD_IMPERSONATION_LEVEL",    // 0xC00000A5
                STATUS_DLL_NOT_FOUND            => "STATUS_DLL_NOT_FOUND",              // 0xC0000135
                STATUS_DLL_INIT_FAILED          => "STATUS_DLL_INIT_FAILED",            // 0xC0000142
                _ if self.0 & 0xF000_0000 == 0x8000_0000    => "E_???",                 // 0x8???????
                _ if self.0 & 0xF000_0000 == 0xC000_0000    => "STATUS_???",            // 0xC???????
                _                                           => "ERROR_???",             // 0x????????
            },
        }
    }
}

impl core::fmt::Debug for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        if self.0 < 0x8000_0000 {
            write!(fmt, "Error({} {})", self.0, self.friendly())
        } else {
            write!(fmt, "Error(0x{:X} {})", self.0, self.friendly())
        }
    }
}

impl From<Error> for i32 { fn from(err: Error) -> Self { err.0 as _ } }
impl From<Error> for u32 { fn from(err: Error) -> Self { err.0 as _ } }
impl From<Error> for crate::io::Error { fn from(err: Error) -> Self { Self::from_raw_os_error(err.into()) } }
impl From<i32> for Error { fn from(err: i32      ) -> Self { Self(err as _) } }
impl From<u32> for Error { fn from(err: u32      ) -> Self { Self(err as _) } }
impl From<abistr::InteriorNulError> for Error { fn from(_: abistr::InteriorNulError) -> Self { Self(ERROR_ILLEGAL_CHARACTER) } }
impl From<NtStatus> for Error { fn from(status: NtStatus) -> Self { Self(status.into()) } }
impl From<winresult::ErrorCode> for Error { fn from(code: winresult::ErrorCode) -> Self { Self(code.into()) } }
impl PartialEq<i32> for Error { fn eq(&self, other: &i32) -> bool { *self == Error::from(*other) } }
impl PartialEq<u32> for Error { fn eq(&self, other: &u32) -> bool { *self == Error::from(*other) } }
impl PartialEq<Error> for i32 { fn eq(&self, other: &Error) -> bool { Error::from(*self) == *other } }
impl PartialEq<Error> for u32 { fn eq(&self, other: &Error) -> bool { Error::from(*self) == *other } }
