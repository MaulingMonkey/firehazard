// C:\Program Files (x86)\Windows Kits\10\Include\10.0.22621.0\um\WinBase.h
// line 723+

#[doc(alias = "FILE_NAME_NORMALIZED")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfinalpathnamebyhandlew)\]
/// FILE_NAME_NORMALIZED
///
pub const NAME_NORMALIZED  : u32 = 0;

#[doc(alias = "FILE_NAME_OPENED")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfinalpathnamebyhandlew)\]
/// FILE_NAME_OPENED
///
pub const NAME_OPENED      : u32 = 8;



// C:\Program Files (x86)\Windows Kits\10\Include\10.0.22621.0\um\WinBase.h
// line 787+

#[doc(alias = "FILE_TYPE_UNKNOWN")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfiletype)\]
/// FILE_TYPE_UNKNOWN
///
pub const TYPE_UNKNOWN : u32 = 0x0000;

#[doc(alias = "FILE_TYPE_DISK")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfiletype)\]
/// FILE_TYPE_DISK
///
pub const TYPE_DISK    : u32 = 0x0001;

#[doc(alias = "FILE_TYPE_CHAR")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfiletype)\]
/// FILE_TYPE_CHAR
///
pub const TYPE_CHAR    : u32 = 0x0002;

#[doc(alias = "FILE_TYPE_PIPE")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfiletype)\]
/// FILE_TYPE_PIPE
///
pub const TYPE_PIPE    : u32 = 0x0003;

#[doc(alias = "FILE_TYPE_REMOTE")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfiletype)\]
/// FILE_TYPE_REMOTE
///
pub const TYPE_REMOTE  : u32 = 0x8000;




// C:\Program Files (x86)\Windows Kits\10\Include\10.0.22621.0\um\WinBase.h
// line 129+

#[doc(alias = "FILE_FLAG_WRITE_THROUGH")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilea)\]
/// FILE_FLAG_WRITE_THROUGH
///
pub const FLAG_WRITE_THROUGH : u32 = 0x80000000;
const _ : () = assert!(FLAG_WRITE_THROUGH == winapi::um::winbase::FILE_FLAG_WRITE_THROUGH);

#[doc(alias = "FILE_FLAG_FIRST_PIPE_INSTANCE")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-createnamedpipea)\]
/// FILE_FLAG_FIRST_PIPE_INSTANCE
///
pub const FLAG_FIRST_PIPE_INSTANCE : u32 = 0x00080000;
const _ : () = assert!(FLAG_FIRST_PIPE_INSTANCE == winapi::um::winbase::FILE_FLAG_FIRST_PIPE_INSTANCE);

#[doc(alias = "FILE_FLAG_OVERLAPPED")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilea)\]
/// FILE_FLAG_OVERLAPPED
///
pub const FLAG_OVERLAPPED : u32 = 0x40000000;
const _ : () = assert!(FLAG_OVERLAPPED == winapi::um::winbase::FILE_FLAG_OVERLAPPED);
