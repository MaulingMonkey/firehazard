use crate::{self as firehazard};
use winapi::shared::winerror::ERROR_INVALID_HANDLE;
use core::fmt::{self, Debug, Display, Formatter};



/// Error converting from one handle type to another (e.g. converting a generic handle to a thread handle when the underlying object was actually a process.)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct HandleConversionError<H> {
    pub unconverted: H,
}

impl<H> From<HandleConversionError<H>> for firehazard::Error {
    fn from(_hce: HandleConversionError<H>) -> Self {
        Self(ERROR_INVALID_HANDLE)
    }
}

#[cfg(std)] impl<H> From<HandleConversionError<H>> for std::io::Error {
    fn from(_hce: HandleConversionError<H>) -> Self {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, "firehazard::HandleConversionError: unable to convert handle to a more specific type (wrong type of handle)")
    }
}

impl<H: Debug> core::error::Error for HandleConversionError<H> {
    fn description(&self) -> &str { "firehazard::HandleConversionError: unable to convert handle to a more specific type (wrong type of handle)" }
}

impl<H: Debug> Debug for HandleConversionError<H> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(
            fmt,
            "firehazard::HandleConversionError {{ unconverted: {:?} }}",
            self.unconverted,
        )
    }
}

impl<H: Debug> Display for HandleConversionError<H> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(
            fmt,
            "firehazard::HandleConversionError {{ unconverted: {:?} }}: unable to convert handle to a more specific type (wrong type of handle)",
            self.unconverted,
        )
    }
}
