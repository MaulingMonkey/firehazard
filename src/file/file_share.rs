/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilew#parameters)\]
/// DWORD/[u32] bitset of `FILE_SHARE_*` values
///
#[derive(Clone, Copy, Default, bytemuck::Zeroable, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Share(u32);

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilew#parameters)\]
/// DWORD/[u32] mask for `FILE_SHARE_*` values
///
#[derive(Clone, Copy, Debug, Default, bytemuck::Zeroable, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct ShareMask(u32);

flags!(impl .. for Share(u32) - ShareMask);

impl Share {
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilew#parameters)\]
    /// 0
    pub const NONE : Self = Self(0);

    // C:\Program Files (x86)\Windows Kits\10\Include\10.0.22621.0\um\winnt.h
    // line 14355+

    #[doc(alias = "FILE_SHARE_READ")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilew#parameters)\]
    /// FILE_SHARE_READ
    ///
    pub const READ : Self = Self(0x00000001);

    #[doc(alias = "FILE_SHARE_WRITE")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilew#parameters)\]
    /// FILE_SHARE_WRITE
    ///
    pub const WRITE : Self = Self(0x00000002);

    #[doc(alias = "FILE_SHARE_DELETE")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilew#parameters)\]
    /// FILE_SHARE_DELETE
    ///
    pub const DELETE : Self = Self(0x00000004);
}

impl core::fmt::Debug for Share {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        const FILE_SHARE_READ   : u32 = Share::READ  .0;
        const FILE_SHARE_WRITE  : u32 = Share::WRITE .0;
        const FILE_SHARE_DELETE : u32 = Share::DELETE.0;
        flags!(self.0, fmt, "0x{:X}", [
            FILE_SHARE_READ,
            FILE_SHARE_WRITE,
            FILE_SHARE_DELETE,
        ])
    }
}
