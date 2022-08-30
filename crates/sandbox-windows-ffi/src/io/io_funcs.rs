use crate::*;

use winapi::shared::minwindef::FALSE;
use winapi::shared::winerror::ERROR_INVALID_HANDLE;
use winapi::um::fileapi::{GetFinalPathNameByHandleA, GetFinalPathNameByHandleW};
use winapi::um::namedpipeapi::*;

#[cfg(std)] use std::ffi::*;
#[cfg(std)] use std::os::windows::prelude::OsStringExt;
#[cfg(std)] use std::path::*;

use core::ptr::{null_mut, NonNull};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createpipe)\] CreatePipe
///
/// ### Example
/// ```
/// # use sandbox_windows_ffi::*;
/// let (read, write) = create_pipe(None, 0).unwrap();
/// ```
pub fn create_pipe(pipe_attributes: Option<&security::Attributes>, size: u32) -> Result<(io::ReadPipe, io::WritePipe), Error> {
    let mut read = null_mut();
    let mut write = null_mut();
    Error::get_last_if(FALSE == unsafe { CreatePipe(&mut read, &mut write, pipe_attributes.map_or(null_mut(), |a| a as *const _ as *mut _), size) })?;
    let read  = NonNull::new(read ).map(|nn| io::ReadPipe (nn)).ok_or(Error(ERROR_INVALID_HANDLE));
    let write = NonNull::new(write).map(|nn| io::WritePipe(nn)).ok_or(Error(ERROR_INVALID_HANDLE));
    Ok((read?, write?))
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-createnamedpipea)\] <strike>CreateNamedPipeA</strike>
#[cfg(doc)] pub fn create_named_pipe_a() { unimplemented!() }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createnamedpipew)\] <strike>CreateNamedPipeW</strike>
#[cfg(doc)] pub fn create_named_pipe_w() { unimplemented!() }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-disconnectnamedpipe)\] <strike>DisconnectNamedPipe</strike>
#[cfg(doc)] pub fn disconnect_named_pipe(handle: ()) -> Result<(), Error> { unimplemented!() }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfinalpathnamebyhandlea)\] GetFinalPathNameByHandleA
pub fn get_final_path_name_by_handle_a_inplace(
    handle: impl AsRef<io::File>,
    path:   &mut [u8],
    flags:  u32, // TODO: type
) -> Result<&[u8], Error> {
    let buf_chars = path.len().try_into().unwrap_or(!0_u32);
    let full_chars = usize::from32(unsafe { GetFinalPathNameByHandleA(handle.as_ref().as_handle(), path.as_mut_ptr().cast(), buf_chars, flags) });
    Error::get_last_if(full_chars == 0 || full_chars > path.len())?;
    Ok(&path[..full_chars])
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfinalpathnamebyhandlea)\] GetFinalPathNameByHandleW
pub fn get_final_path_name_by_handle_w_inplace(
    handle: impl AsRef<io::File>,
    path:   &mut [u16],
    flags:  u32, // TODO: type
) -> Result<&[u16], Error> {
    let buf_chars = path.len().try_into().unwrap_or(!0_u32);
    let full_chars = usize::from32(unsafe { GetFinalPathNameByHandleW(handle.as_ref().as_handle(), path.as_mut_ptr().cast(), buf_chars, flags) });
    Error::get_last_if(full_chars == 0 || full_chars > path.len())?;
    Ok(&path[..full_chars])
}

#[cfg(std)]
/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfinalpathnamebyhandlea)\] GetFinalPathNameByHandleW
pub fn get_final_path_name_by_handle(handle: impl AsRef<io::File>, flags: u32) -> Result<PathBuf, Error> {
    let handle = handle.as_ref();

    let mut buf = [0u16; 260];
    let full_chars = usize::from32(unsafe { GetFinalPathNameByHandleW(handle.as_handle(), buf.as_mut_ptr().cast(), buf.len() as _, flags) });
    Error::get_last_if(full_chars == 0)?;
    if full_chars <= buf.len() { return Ok(PathBuf::from(OsString::from_wide(&buf[..full_chars]))) }

    // else `buf` was too small:
    let mut buf = vec![0u16; full_chars+1];
    let path = get_final_path_name_by_handle_w_inplace(handle, &mut buf[..], flags)?;
    Ok(PathBuf::from(OsString::from_wide(path)))
}



//  TODO:
// CallNamedPipeA
// CallNamedPipeW
// ConnectNamedPipe
// DisconnectNamedPipe
// GetNamedPipeClientComputerNameA
// GetNamedPipeClientComputerNameW
// GetNamedPipeClientProcessId
// GetNamedPipeClientSessionId
// GetNamedPipeHandleStateA
// GetNamedPipeHandleStateW
// GetNamedPipeInfo
// GetNamedPipeServerProcessId
// GetNamedPipeServerSessionId
// ImpersonateNamedPipeClient
// PeekNamedPipe
// SetNamedPipeHandleState
// TransactNamedPipe
// WaitNamedPipeA
// WaitNamedPipeW
