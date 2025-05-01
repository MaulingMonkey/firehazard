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
