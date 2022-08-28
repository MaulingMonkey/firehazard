use crate::*;
use crate::pipe::*;

use winapi::shared::minwindef::FALSE;
use winapi::um::namedpipeapi::*;

use core::ptr::null_mut;

use std::fs::File;
use std::os::windows::io::FromRawHandle;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createpipe)\] CreatePipe
///
/// ### Example
/// ```
/// # use sandbox_windows_ffi::*;
/// let (read, write) = create_pipe(None, 0).unwrap();
/// ```
pub fn create_pipe(pipe_attributes: Option<&security::Attributes>, size: u32) -> Result<(ReadPipe, WritePipe), Error> {
    let mut read = null_mut();
    let mut write = null_mut();
    Error::get_last_if(FALSE == unsafe { CreatePipe(&mut read, &mut write, pipe_attributes.map_or(null_mut(), |a| a as *const _ as *mut _), size) })?;
    let read  = ReadPipe (unsafe { File::from_raw_handle(read.cast()) });
    let write = WritePipe(unsafe { File::from_raw_handle(write.cast()) });
    Ok((read, write))
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-createnamedpipea)\] <strike>CreateNamedPipeA</strike>
#[cfg(doc)] pub fn create_named_pipe_a() { unimplemented!() }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createnamedpipew)\] <strike>CreateNamedPipeW</strike>
#[cfg(doc)] pub fn create_named_pipe_w() { unimplemented!() }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-disconnectnamedpipe)\] DisconnectNamedPipe
#[cfg(doc)] pub fn disconnect_named_pipe(handle: ()) -> Result<(), Error> { unimplemented!() }



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
