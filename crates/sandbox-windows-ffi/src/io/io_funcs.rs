use crate::*;

use winapi::shared::minwindef::FALSE;
use winapi::shared::winerror::ERROR_INVALID_HANDLE;
use winapi::um::namedpipeapi::*;

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
