use crate::*;

use abistr::{TryIntoAsOptCStr, AsOptCStr};

use winapi::shared::minwindef::{BOOL, LPVOID, DWORD};
use winapi::shared::ntdef::{LPCSTR, LPSTR, HANDLE};
use winapi::shared::winerror::*;
use winapi::um::minwinbase::LPSECURITY_ATTRIBUTES;
use winapi::um::processthreadsapi::*;
use winapi::um::synchapi::WaitForSingleObject;
use winapi::um::winbase::*;

use std::convert::Infallible;
use std::mem::zeroed;
use std::ptr::{null_mut, null};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessa)\] CreateProcessA
fn _create_process_a() -> Result<process::Information, Error> { unimplemented!() }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessw)\] CreateProcessW
fn _create_process_w() -> Result<process::Information, Error> { unimplemented!() }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessasusera)\] CreateProcessAsUserA
///
/// ### Safety
/// *   `creation_flags`    Is unvalidated as heck
/// *   `startup_info`      Is unvalidated as heck
pub unsafe fn create_process_as_user_a(
    token:                  &crate::token::OwnedHandle,
    application_name:       impl TryIntoAsOptCStr,
    command_line:           Option<&[u8]>,
    process_attributes:     Option<Infallible>,         // TODO: type
    thread_attributes:      Option<Infallible>,         // TODO: type
    inherit_handles:        bool,
    creation_flags:         u32,                        // TODO: type
    environment:            Option<&[u8]>,              // TODO: type to reduce validation needs (expected to be NUL separated, 2xNUL terminated: "key=value\0key=value\0\0")
    current_directory:      impl TryIntoAsOptCStr,
    startup_info:           &STARTUPINFOA,              // TODO: type via trait (could be STARTUPINFOW, STARTUPINFOEXW, etc.)
) -> Result<process::Information, Error> {
    if !command_line.as_ref().map_or(false, |c| c.ends_with(&[0]))  { return Err(Error(ERROR_INVALID_PARAMETER)) } // must be NUL terminated
    if !environment.unwrap_or(&[0, 0]).ends_with(&[0, 0])           { return Err(Error(ERROR_INVALID_PARAMETER)) } // must be 2xNUL terminated
    let startup_info : *const STARTUPINFOA = startup_info;
    let mut process_information = unsafe { zeroed() };

    extern "system" { fn CreateProcessAsUserA(
        hToken: HANDLE,
        lpApplicationName: LPCSTR,
        lpCommandLine: LPSTR,
        lpProcessAttributes: LPSECURITY_ATTRIBUTES,
        lpThreadAttributes: LPSECURITY_ATTRIBUTES,
        bInheritHandles: BOOL,
        dwCreationFlags: DWORD,
        lpEnvironment: LPVOID,
        lpCurrentDirectory: LPCSTR,
        lpStartupInfo: LPSTARTUPINFOA,
        lpProcessInformation: LPPROCESS_INFORMATION,
    ) -> BOOL;}

    Error::get_last_if(0 == unsafe { CreateProcessAsUserA(
        token.as_handle(),
        application_name.try_into().map_err(|_| Error(ERROR_INVALID_PARAMETER))?.as_opt_cstr(),
        command_line.as_ref().map_or(null(), |c| c.as_ptr()) as *mut _,
        map_inconv(process_attributes).map_or(null_mut(), |a| a),
        map_inconv(thread_attributes).map_or(null_mut(), |a| a),
        inherit_handles as _,
        creation_flags,
        environment.map_or(null(), |e| e.as_ptr()) as *mut _,
        current_directory.try_into().map_err(|_| Error(ERROR_INVALID_PARAMETER))?.as_opt_cstr(),
        startup_info as *mut _,
        &mut process_information
    )})?;
    Ok(unsafe { process::Information::from_raw(process_information) })
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessasuserw)\] CreateProcessAsUserW
///
/// ### Safety
/// *   `creation_flags`    Is unvalidated as heck
/// *   `startup_info`      Is unvalidated as heck
pub unsafe fn create_process_as_user_w(
    token:                  &crate::token::OwnedHandle,
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
) -> Result<process::Information, Error> {
    if !command_line.as_ref().map_or(false, |c| c.ends_with(&[0]))  { return Err(Error(ERROR_INVALID_PARAMETER)) } // must be NUL terminated
    if !environment.unwrap_or(&[0, 0]).ends_with(&[0, 0])           { return Err(Error(ERROR_INVALID_PARAMETER)) } // must be 2xNUL terminated
    let startup_info : *const STARTUPINFOW = startup_info;
    let mut process_information = unsafe { zeroed() };

    Error::get_last_if(0 == unsafe { CreateProcessAsUserW(
        token.as_handle(),
        application_name.try_into().map_err(|_| Error(ERROR_INVALID_PARAMETER))?.as_opt_cstr(),
        command_line.as_mut().map_or(null_mut(), |c| c.as_mut_ptr()),
        map_inconv(process_attributes).map_or(null_mut(), |a| a),
        map_inconv(thread_attributes).map_or(null_mut(), |a| a),
        inherit_handles as _,
        creation_flags,
        environment.map_or(null(), |e| e.as_ptr()) as *mut _,
        current_directory.try_into().map_err(|_| Error(ERROR_INVALID_PARAMETER))?.as_opt_cstr(),
        startup_info as *mut _,
        &mut process_information
    )})?;
    Ok(unsafe { process::Information::from_raw(process_information) })
}

fn map_inconv<T>(_: Option<Infallible>) -> Option<T> { None }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-createprocesswithlogonw)\] CreateProcessWithLogonW
fn _create_process_with_logon_w() -> Result<process::Information, Error> { unimplemented!() }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-createprocesswithtokenw)\] CreateProcessWithTokenW
fn _create_process_with_token_w() -> Result<process::Information, Error> { unimplemented!() }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocess)\] GetCurrentProcess
pub fn get_current_process() -> process::PsuedoHandle { unsafe { process::PsuedoHandle::from_raw_unchecked(GetCurrentProcess()) } }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocessid)\] GetCurrentProcess
pub fn get_current_process_id() -> process::Id { unsafe { GetCurrentProcessId() } }

// get/set process afinity masks, etc.

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getexitcodeprocess)\] GetExitCodeProcess
///
/// ### Returns
/// *   `Ok(STILL_ACTIVE)` / `Ok(STATUS_PENDING)`   if `process` is still running
/// *   `Ok(0)`                                     if `process` exited "successfully"
/// *   `Ok(exit_code)`                             if `process` exited otherwise
/// *   `Err(...)`                                  if `process` lacks appropriate querying permissions?
/// *   `Err(...)`                                  if `process` is an invalid handle?
pub fn get_exit_code_process(process: impl AsRef<process::Handle>) -> Result<u32, Error> {
    let mut exit_code = 0;
    Error::get_last_if(0 == unsafe { GetExitCodeProcess(process.as_ref().as_handle(), &mut exit_code) })?;
    Ok(exit_code)
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-waitforsingleobject)\] `WaitForSingleObject(process, 0) == WAIT_TIMEOUT`
pub fn is_process_running(process: impl AsRef<process::Handle>) -> bool {
    WAIT_TIMEOUT == unsafe { WaitForSingleObject(process.as_ref().as_handle(), 0) }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-waitforsingleobject)\] `WaitForSingleObject(process, INFINITE)` + `GetExitCodeProcess`
pub fn wait_for_process(process: impl AsRef<process::Handle>) -> Result<u32, Error> {
    match unsafe { WaitForSingleObject(process.as_ref().as_handle(), INFINITE) } {
        WAIT_OBJECT_0       => {},
        WAIT_ABANDONED_0    => return Err(Error(ERROR_ABANDONED_WAIT_0)),   // shouldn't happen as `process` isn't a mutex, right?
        WAIT_TIMEOUT        => return Err(Error(ERROR_ABANDONED_WAIT_63)),  // shouldn't happen - hopefully the `63` hints that something is funky?
        WAIT_FAILED         => return Err(Error::get_last()),
        _                   => return Err(Error(ERROR_ABANDONED_WAIT_63)),  // shouldn't happen - hopefully the `63` hints that something is funky?
    }
    get_exit_code_process(process)
}

#[test] fn test_wait_exit() {
    use winapi::um::minwinbase::STILL_ACTIVE;
    use std::process::*;
    let child : Child = Command::new("cmd").args("/C ping localhost -n 2 && exit /B 3".split(' ')).stdout(Stdio::null()).spawn().unwrap();
    let child = process::OwnedHandle::from(child);

    assert!(is_process_running(&child));
    assert_eq!(STILL_ACTIVE, get_exit_code_process(&child).unwrap());

    assert_eq!(3, wait_for_process(&child).unwrap());

    assert!(!is_process_running(&child));
    assert_eq!(3, get_exit_code_process(&child).unwrap());
}
