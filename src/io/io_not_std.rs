use core::fmt::{self, Debug, Display, Formatter};



pub trait Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
    // TODO: lots
}

pub trait Seek {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64>;
    // TODO: lots
}

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;
    // TODO: lots
}

pub struct Error(ErrorImpl);

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum ErrorImpl {
    Code(i32),
    Kind(ErrorKind),
}

impl Error {
    pub fn new(kind: ErrorKind, _message: &'static str) -> Self { Self(ErrorImpl::Kind(kind)) }
    // fn other
    pub fn last_os_error() -> Self { Self::from_raw_os_error(crate::Error::get_last().into()) }
    pub fn from_raw_os_error(code: i32) -> Self { Self(ErrorImpl::Code(code)) }
    pub fn raw_os_error(&self) -> Option<i32> { match self.0 { ErrorImpl::Code(c) => Some(c), _ => None } }
    // fn get_ref
    // fn get_mut
    // fn into_inner
    pub fn kind(&self) -> ErrorKind { match self.0 { ErrorImpl::Kind(k) => k, _ => ErrorKind::Other } }
}

impl From<ErrorKind> for Error { fn from(kind: ErrorKind) -> Self { Self(ErrorImpl::Kind(kind)) } }

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ErrorKind {
    NotFound,
    PermissionDenied,
    ConnectionRefused,
    ConnectionReset,
    HostUnreachable,
    NetworkUnreachable,
    ConnectionAborted,
    NotConnected,
    AddrInUse,
    AddrNotAvailable,
    NetworkDown,
    BrokenPipe,
    AlreadyExists,
    WouldBlock,
    NotADirectory,
    IsADirectory,
    DirectoryNotEmpty,
    ReadOnlyFilesystem,
    FilesystemLoop,
    StaleNetworkFileHandle,
    InvalidInput,
    InvalidData,
    TimedOut,
    WriteZero,
    StorageFull,
    NotSeekable,
    FilesystemQuotaExceeded,
    FileTooLarge,
    ResourceBusy,
    ExecutableFileBusy,
    Deadlock,
    CrossesDevices,
    TooManyLinks,
    InvalidFilename,
    ArgumentListTooLong,
    Interrupted,
    Unsupported,
    UnexpectedEof,
    OutOfMemory,
    Other,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SeekFrom {
    Start(u64),
    End(i64),
    Current(i64),
}



impl Debug for Error {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self.0 {
            ErrorImpl::Code(code) => write!(fmt, "io::Error {{ os: {code} }}"),
            ErrorImpl::Kind(kind) => write!(fmt, "io::Error {{ kind: {kind} }}"),
        }
    }
}

impl Display for Error {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self.0 {
            ErrorImpl::Code(code) => write!(fmt, "io::Error {{ os: {code} }}"),
            ErrorImpl::Kind(kind) => write!(fmt, "io::Error {{ kind: {kind} }}"),
        }
    }
}

impl Display for ErrorKind {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, fmt)
    }
}
