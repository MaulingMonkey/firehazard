use crate::prelude::*;

// C:\Program Files (x86)\Windows Kits\10\Include\10.0.22621.0\um\WinBase.h
// line 787+

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfiletype)\]
/// FILE_TYPE_{[DISK](Self::DISK), [CHAR](Self::CHAR), [PIPE](Self::PIPE), [UNKNOWN](Self::UNKNOWN)} | FILE_TYPE_REMOTE?
///
#[derive(Clone, Copy, bytemuck::Pod, Default, bytemuck::Zeroable, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Type(u32);

impl Type {
    #[doc(alias = "FILE_TYPE_UNKNOWN")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfiletype)\]
    /// FILE_TYPE_UNKNOWN
    ///
    pub const UNKNOWN   : Self = Self(0x0000);

    #[doc(alias = "FILE_TYPE_DISK")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfiletype)\]
    /// FILE_TYPE_DISK
    ///
    /// A typical file, presumably stored on a disk.
    ///
    pub const DISK      : Self = Self(0x0001);

    #[doc(alias = "FILE_TYPE_CHAR")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfiletype)\]
    /// FILE_TYPE_CHAR
    ///
    /// A character-based file handle, such as various console handles.
    ///
    pub const CHAR      : Self = Self(0x0002);

    #[doc(alias = "FILE_TYPE_PIPE")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfiletype)\]
    /// FILE_TYPE_PIPE
    ///
    /// A [pipe] or socket handle (won't be [`Seek`](std::io::Seek)able)
    ///
    pub const PIPE      : Self = Self(0x0003);

    #[doc(alias = "FILE_TYPE_REMOTE")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfiletype)\]
    /// FILE_TYPE_REMOTE
    ///
    /// Documented as unused by `GetFileType`, presumably a flag that's bitwise ORed against one of the other types?
    ///
    pub const REMOTE    : Self = Self(0x8000);
}

impl Type {
    /// self & FILE_TYPE_REMOTE != 0
    pub fn is_remote(self) -> bool { self.0 & Self::REMOTE.0 != 0 }

    /// self & ~FILE_TYPE_REMOTE
    pub fn without_remote(self) -> Self { Self(self.0 & !Self::REMOTE.0) }
}

impl core::fmt::Debug for Type {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        let mut v = self.0;

        let remote = v & file::Type::REMOTE.0 != 0;
        v &= !file::Type::REMOTE.0;

        match Self(v) {
            Self::UNKNOWN   => write!(fmt, "FILE_TYPE_UNKNOWN")?,
            Self::DISK      => write!(fmt, "FILE_TYPE_DISK")?,
            Self::CHAR      => write!(fmt, "FILE_TYPE_CHAR")?,
            Self::PIPE      => write!(fmt, "FILE_TYPE_PIPE")?,
            _other          => write!(fmt, "FILE_TYPE_??? ({v})")?,
        }

        if remote {
            write!(fmt, " | FILE_TYPE_REMOTE")?;
        }
        Ok(())
    }
}

impl From<u32> for Type { fn from(value: u32) -> Self { Self(value) } }
impl From<Type> for u32 { fn from(value: Type) -> Self { value.0 } }
