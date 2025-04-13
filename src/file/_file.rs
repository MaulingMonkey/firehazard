//! \[<strike>microsoft.com</strike>\]
//! APIs and constants related to file management



// C:\Program Files (x86)\Windows Kits\10\Include\10.0.22621.0\um\WinBase.h
// line 723+

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfinalpathnamebyhandlew)\]
/// FILE_NAME_NORMALIZED
///
pub const NAME_NORMALIZED  : u32 = 0;

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfinalpathnamebyhandlew)\]
/// FILE_NAME_OPENED
///
pub const NAME_OPENED      : u32 = 8;



// C:\Program Files (x86)\Windows Kits\10\Include\10.0.22621.0\um\WinBase.h
// line 129+

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilea)\]
/// FILE_FLAG_WRITE_THROUGH
///
pub const FLAG_WRITE_THROUGH : u32 = winapi::um::winbase::FILE_FLAG_WRITE_THROUGH;

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-createnamedpipea)\]
/// FILE_FLAG_FIRST_PIPE_INSTANCE
///
pub const FLAG_FIRST_PIPE_INSTANCE : u32 = winapi::um::winbase::FILE_FLAG_FIRST_PIPE_INSTANCE;

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilea)\]
/// FILE_FLAG_OVERLAPPED
///
pub const FLAG_OVERLAPPED : u32 = winapi::um::winbase::FILE_FLAG_OVERLAPPED;
