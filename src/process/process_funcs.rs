use crate::error::LastError;
use crate::process;

use abistr::{TryIntoAsOptCStr, AsOptCStr};
use winapi::shared::winerror::ERROR_INVALID_PARAMETER;
use winapi::um::processthreadsapi::*;

use std::convert::Infallible;
use std::mem::zeroed;
use std::ptr::{null_mut, null};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessa)\] CreateProcessA
fn _create_process_a() -> Result<process::Information, LastError> { unimplemented!() }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessw)\] CreateProcessW
fn _create_process_w() -> Result<process::Information, LastError> { unimplemented!() }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessasusera)\] CreateProcessAsUserA
fn _create_process_as_user_a() -> Result<process::Information, LastError> { unimplemented!() }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessasuserw)\] CreateProcessAsUserW
///
/// ### Safety
/// *   `creation_flags`    Is unvalidated as heck
/// *   `startup_info`      Is unvalidated as heck
pub unsafe fn create_process_as_user_w(
    token:                  &crate::token::Handle,
    application_name:       impl TryIntoAsOptCStr<u16>,
    // "The Unicode version of this function, CreateProcessAsUserW, can modify the contents of this string.
    // Therefore, this parameter cannot be a pointer to read-only memory (such as a const variable or a literal string).
    // If this parameter is a constant string, the function may cause an access violation."
    mut command_line:       Option<&mut [u16]>,
    process_attributes:     Option<Infallible>,         // TODO: type
    thread_attributes:      Option<Infallible>,         // TODO: type
    inherit_handles:        bool,
    creation_flags:         u32,                        // TODO: type
    environment:            Option<&[u16]>,             // TODO: type to reduce validation needs (expected to be NUL separated, 2xNUL terminated: "key=value\0key=value\0\0")
    current_directory:      impl TryIntoAsOptCStr<u16>,
    startup_info:           &STARTUPINFOW,              // TODO: type via trait (could be STARTUPINFOW, STARTUPINFOEXW, etc.)
) -> Result<process::Information, LastError> {
    if !command_line.as_ref().map_or(false, |c| c.ends_with(&[0]))  { return Err(LastError(ERROR_INVALID_PARAMETER)) } // must be NUL terminated
    if !environment.unwrap_or(&[0, 0]).ends_with(&[0, 0])           { return Err(LastError(ERROR_INVALID_PARAMETER)) } // must be 2xNUL terminated
    let startup_info : *const STARTUPINFOW = startup_info;
    let mut process_information = unsafe { zeroed() };

    fn map_inconv<T>(_: Option<Infallible>) -> Option<T> { None }

    let success = 0 != unsafe { CreateProcessAsUserW(
        token.as_handle(),
        application_name.try_into().map_err(|_| LastError(ERROR_INVALID_PARAMETER))?.as_opt_cstr(),
        command_line.as_mut().map_or(null_mut(), |c| c.as_mut_ptr()),
        map_inconv(process_attributes).map_or(null_mut(), |a| a),
        map_inconv(thread_attributes).map_or(null_mut(), |a| a),
        inherit_handles as _,
        creation_flags,
        environment.map_or(null(), |e| e.as_ptr()) as *mut _,
        current_directory.try_into().map_err(|_| LastError(ERROR_INVALID_PARAMETER))?.as_opt_cstr(),
        startup_info as *mut _,
        &mut process_information
    )};
    if success { Ok(unsafe { process::Information::from_raw(process_information) }) } else { Err(LastError::get()) }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-createprocesswithlogonw)\] CreateProcessWithLogonW
fn _create_process_with_logon_w() -> Result<process::Information, LastError> { unimplemented!() }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-createprocesswithtokenw)\] CreateProcessWithTokenW
fn _create_process_with_token_w() -> Result<process::Information, LastError> { unimplemented!() }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocess)\] GetCurrentProcess
pub fn get_current_process() -> process::PsuedoHandle { unsafe { process::PsuedoHandle::from_raw_unchecked(GetCurrentProcess()) } }

// get/set process afinity masks, etc.
