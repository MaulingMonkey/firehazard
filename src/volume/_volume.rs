//! \[<strike>microsoft.com</strike>\]
//! APIs and constants related to volume management



// C:\Program Files (x86)\Windows Kits\10\Include\10.0.22621.0\um\WinBase.h
// line 718+

#[doc(alias = "VOLUME_NAME_DOS")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfinalpathnamebyhandlew)\]
/// VOLUME_NAME_DOS
///
pub const NAME_DOS   : u32 = 0x0;

#[doc(alias = "VOLUME_NAME_GUID")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfinalpathnamebyhandlew)\]
/// VOLUME_NAME_GUID
///
pub const NAME_GUID  : u32 = 0x1;

#[doc(alias = "VOLUME_NAME_NT")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfinalpathnamebyhandlew)\]
/// VOLUME_NAME_NT
///
pub const NAME_NT    : u32 = 0x2;

#[doc(alias = "VOLUME_NAME_NONE")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfinalpathnamebyhandlew)\]
/// VOLUME_NAME_NONE
///
pub const NAME_NONE  : u32 = 0x4;
