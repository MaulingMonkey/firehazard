use crate::*;
use crate::process::environment::*;

use abistr::{TryIntoAsOptCStr, AsOptCStr};

use winapi::shared::minwindef::{BOOL, LPVOID, DWORD};
use winapi::shared::ntdef::{LPCSTR, LPSTR, HANDLE};
use winapi::shared::winerror::*;
use winapi::um::minwinbase::LPSECURITY_ATTRIBUTES;
use winapi::um::processthreadsapi::*;
use winapi::um::synchapi::WaitForSingleObject;
use winapi::um::winbase::*;

use core::mem::zeroed;
use core::ptr::{null_mut, null};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessa)\] CreateProcessA
pub fn create_process_a(
    application_name:       impl TryIntoAsOptCStr,
    command_line:           Option<&[u8]>,
    process_attributes:     Option<&security::Attributes>,
    thread_attributes:      Option<&security::Attributes>,
    inherit_handles:        bool,
    creation_flags:         impl Into<process::CreationFlags>,
    environment:            impl TryIntoEnvironment,
    current_directory:      impl TryIntoAsOptCStr,
    startup_info:           &impl process::AsStartupInfoA,
) -> Result<process::Information, Error> {
    if !command_line.as_ref().map_or(false, |c| c.ends_with(&[0]))  { return Err(Error(E_STRING_NOT_NULL_TERMINATED as _)) } // must be NUL terminated
    let creation_flags = creation_flags.into().into();
    let mut process_information = unsafe { zeroed() };

    Error::get_last_if(0 == unsafe { CreateProcessA(
        application_name.try_into().map_err(|_| Error(E_STRING_NOT_NULL_TERMINATED as _))?.as_opt_cstr(),
        command_line.as_ref().map_or(null(), |c| c.as_ptr()) as *mut _,
        process_attributes.map_or(null(), |a| a) as *mut _,
        thread_attributes.map_or(null(), |a| a) as *mut _,
        inherit_handles as _,
        creation_flags,
        environment.as_env_ptr(creation_flags & CREATE_UNICODE_ENVIRONMENT != 0)?,
        current_directory.try_into().map_err(|_| Error(E_STRING_NOT_NULL_TERMINATED as _))?.as_opt_cstr(),
        startup_info.as_winapi()?,
        &mut process_information
    )})?;
    Ok(unsafe { process::Information::from_raw(process_information) })
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessw)\] CreateProcessW
pub fn create_process_w(
    application_name:       impl TryIntoAsOptCStr<u16>,
    // "The Unicode version of this function, CreateProcessW, can modify the contents of this string.
    // Therefore, this parameter cannot be a pointer to read-only memory (such as a const variable or a literal string).
    // If this parameter is a constant string, the function may cause an access violation."
    mut command_line:       Option<&mut [u16]>,
    process_attributes:     Option<&security::Attributes>,
    thread_attributes:      Option<&security::Attributes>,
    inherit_handles:        bool,
    creation_flags:         impl Into<process::CreationFlags>,
    environment:            impl TryIntoEnvironment,
    current_directory:      impl TryIntoAsOptCStr<u16>,
    startup_info:           &impl process::AsStartupInfoW,
) -> Result<process::Information, Error> {
    if !command_line.as_ref().map_or(false, |c| c.ends_with(&[0]))  { return Err(Error(E_STRING_NOT_NULL_TERMINATED as _)) } // must be NUL terminated
    let creation_flags = creation_flags.into().into();
    let mut process_information = unsafe { zeroed() };

    Error::get_last_if(0 == unsafe { CreateProcessW(
        application_name.try_into().map_err(|_| Error(E_STRING_NOT_NULL_TERMINATED as _))?.as_opt_cstr(),
        command_line.as_mut().map_or(null_mut(), |c| c.as_mut_ptr()),
        process_attributes.map_or(null(), |a| a) as *mut _,
        thread_attributes.map_or(null(), |a| a) as *mut _,
        inherit_handles as _,
        creation_flags,
        environment.as_env_ptr(creation_flags & CREATE_UNICODE_ENVIRONMENT != 0)?,
        current_directory.try_into().map_err(|_| Error(E_STRING_NOT_NULL_TERMINATED as _))?.as_opt_cstr(),
        startup_info.as_winapi()?,
        &mut process_information
    )})?;
    Ok(unsafe { process::Information::from_raw(process_information) })
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessasusera)\] CreateProcessAsUserA
///
/// | Error                         | Condition |
/// | ----------------------------- | --------- |
/// | ERROR_INCORRECT_SIZE          | If using [process::EXTENDED_STARTUPINFO_PRESENT] with [process::StartupInfoW] instead of [process::StartupInfoExW]
/// | ERROR_INVALID_PARAMETER       | Various, including various issues with the [process::ThreadAttributeList] of [process::StartupInfoExW]
/// | ERROR_INVALID_PARAMETER       | [process::ThreadAttributeRef::handle_list] used but `inherit_handles` is false
/// | ERROR_INVALID_PARAMETER       | [process::creation::mitigation_policy2::xtended_control_flow_guard::ALWAYS_ON] on non-XFG-enabled binary?
/// | ERROR_INVALID_PARAMETER       | [process::creation::mitigation_policy2::pointer_auth_user_ip::ALWAYS_ON] on a non-ARM64 system?
/// | ERROR_FILE_NOT_FOUND          | Executable specified by `command_line` not found
/// | ERROR_ACCESS_DENIED           | Various path access errors
/// | ERROR_ACCESS_DENIED           | [process::creation::mitigation_policy2::block_non_cet_binaries::ALWAYS_ON] on a non-CET binrary<br>Various path access errors?
/// | ERROR_STRICT_CFG_VIOLATION    | [process::creation::mitigation_policy2::strict_control_flow_guard::ALWAYS_ON] on a partially CFG-enabled binary?
/// | ERROR_BAD_ENVIRONMENT         | `environment` was ANSI despite `creation_flags` containing `CREATE_UNICODE_ENVIRONMENT`
/// | ERROR_BAD_ENVIRONMENT         | `environment` was UTF16 despite `creation_flags` lacking `CREATE_UNICODE_ENVIRONMENT`
/// | E_STRING_NOT_NULL_TERMINATED  | `environment` was missing `\0\0` terminator
/// | E_STRING_NOT_NULL_TERMINATED  | `application_name`, `command_line`, or `current_directory` was missing `\0` terminator
pub fn create_process_as_user_a(
    token:                  &crate::token::OwnedHandle,
    application_name:       impl TryIntoAsOptCStr,
    command_line:           Option<&[u8]>,
    process_attributes:     Option<&security::Attributes>,
    thread_attributes:      Option<&security::Attributes>,
    inherit_handles:        bool,
    creation_flags:         impl Into<process::CreationFlags>,
    environment:            impl TryIntoEnvironment,
    current_directory:      impl TryIntoAsOptCStr,
    startup_info:           &impl process::AsStartupInfoA,
) -> Result<process::Information, Error> {
    if !command_line.as_ref().map_or(false, |c| c.ends_with(&[0]))  { return Err(Error(E_STRING_NOT_NULL_TERMINATED as _)) } // must be NUL terminated
    let creation_flags = creation_flags.into().into();
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
        application_name.try_into().map_err(|_| Error(E_STRING_NOT_NULL_TERMINATED as _))?.as_opt_cstr(),
        command_line.as_ref().map_or(null(), |c| c.as_ptr()) as *mut _,
        process_attributes.map_or(null(), |a| a) as *mut _,
        thread_attributes.map_or(null(), |a| a) as *mut _,
        inherit_handles as _,
        creation_flags,
        environment.as_env_ptr(creation_flags & CREATE_UNICODE_ENVIRONMENT != 0)?,
        current_directory.try_into().map_err(|_| Error(E_STRING_NOT_NULL_TERMINATED as _))?.as_opt_cstr(),
        startup_info.as_winapi()?,
        &mut process_information
    )})?;
    Ok(unsafe { process::Information::from_raw(process_information) })
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessasuserw)\] CreateProcessAsUserW
///
/// | Error                         | Condition |
/// | ----------------------------- | --------- |
/// | ERROR_INCORRECT_SIZE          | If using [process::EXTENDED_STARTUPINFO_PRESENT] with [process::StartupInfoW] instead of [process::StartupInfoExW]
/// | ERROR_INVALID_PARAMETER       | Various, including various issues with the [process::ThreadAttributeList] of [process::StartupInfoExW]
/// | ERROR_INVALID_PARAMETER       | [process::ThreadAttributeRef::handle_list] used but `inherit_handles` is false
/// | ERROR_INVALID_PARAMETER       | [process::creation::mitigation_policy2::xtended_control_flow_guard::ALWAYS_ON] on non-XFG-enabled binary?
/// | ERROR_INVALID_PARAMETER       | [process::creation::mitigation_policy2::pointer_auth_user_ip::ALWAYS_ON] on a non-ARM64 system?
/// | ERROR_FILE_NOT_FOUND          | Executable specified by `command_line` not found
/// | ERROR_ACCESS_DENIED           | Various path access errors
/// | ERROR_ACCESS_DENIED           | [process::creation::mitigation_policy2::block_non_cet_binaries::ALWAYS_ON] on a non-CET binrary<br>Various path access errors?
/// | ERROR_STRICT_CFG_VIOLATION    | [process::creation::mitigation_policy2::strict_control_flow_guard::ALWAYS_ON] on a partially CFG-enabled binary?
/// | ERROR_BAD_ENVIRONMENT         | `environment` was ANSI despite `creation_flags` containing `CREATE_UNICODE_ENVIRONMENT`
/// | ERROR_BAD_ENVIRONMENT         | `environment` was UTF16 despite `creation_flags` lacking `CREATE_UNICODE_ENVIRONMENT`
/// | E_STRING_NOT_NULL_TERMINATED  | `environment` was missing `\0\0` terminator
/// | E_STRING_NOT_NULL_TERMINATED  | `application_name`, `command_line`, or `current_directory` was missing `\0` terminator
pub fn create_process_as_user_w(
    token:                  &crate::token::OwnedHandle,
    application_name:       impl TryIntoAsOptCStr<u16>,
    // "The Unicode version of this function, CreateProcessAsUserW, can modify the contents of this string.
    // Therefore, this parameter cannot be a pointer to read-only memory (such as a const variable or a literal string).
    // If this parameter is a constant string, the function may cause an access violation."
    mut command_line:       Option<&mut [u16]>,
    process_attributes:     Option<&security::Attributes>,
    thread_attributes:      Option<&security::Attributes>,
    inherit_handles:        bool,
    creation_flags:         impl Into<process::CreationFlags>,
    environment:            impl TryIntoEnvironment,
    current_directory:      impl TryIntoAsOptCStr<u16>,
    startup_info:           &impl process::AsStartupInfoW,
) -> Result<process::Information, Error> {
    if !command_line.as_ref().map_or(false, |c| c.ends_with(&[0]))  { return Err(Error(E_STRING_NOT_NULL_TERMINATED as _)) } // must be NUL terminated
    let creation_flags = creation_flags.into().into();
    let mut process_information = unsafe { zeroed() };

    Error::get_last_if(0 == unsafe { CreateProcessAsUserW(
        token.as_handle(),
        application_name.try_into().map_err(|_| Error(E_STRING_NOT_NULL_TERMINATED as _))?.as_opt_cstr(),
        command_line.as_mut().map_or(null_mut(), |c| c.as_mut_ptr()),
        process_attributes.map_or(null(), |a| a) as *mut _,
        thread_attributes.map_or(null(), |a| a) as *mut _,
        inherit_handles as _,
        creation_flags,
        environment.as_env_ptr(creation_flags & CREATE_UNICODE_ENVIRONMENT != 0)?,
        current_directory.try_into().map_err(|_| Error(E_STRING_NOT_NULL_TERMINATED as _))?.as_opt_cstr(),
        startup_info.as_winapi()?,
        &mut process_information
    )})?;
    Ok(unsafe { process::Information::from_raw(process_information) })
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-createprocesswithlogonw)\] CreateProcessWithLogonW
fn _create_process_with_logon_w() -> Result<process::Information, Error> { unimplemented!() }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-createprocesswithtokenw)\] CreateProcessWithTokenW
fn _create_process_with_token_w() -> Result<process::Information, Error> { unimplemented!() }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-exitprocess)\] ExitProcess
pub fn exit_process(exit_code: u32) -> ! { unsafe { ExitProcess(exit_code); core::hint::unreachable_unchecked() } }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocess)\] GetCurrentProcess
pub fn get_current_process() -> process::PsuedoHandle<'static> { unsafe { process::PsuedoHandle::from_raw(GetCurrentProcess()).unwrap() } }

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
pub fn get_exit_code_process<'a>(process: impl AsRef<process::Handle<'a>>) -> Result<u32, Error> {
    let mut exit_code = 0;
    Error::get_last_if(0 == unsafe { GetExitCodeProcess(process.as_ref().as_handle(), &mut exit_code) })?;
    Ok(exit_code)
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-waitforsingleobject)\] WaitForSingleObject(process, 0) == WAIT_TIMEOUT
pub fn is_process_running<'a>(process: impl AsRef<process::Handle<'a>>) -> bool {
    WAIT_TIMEOUT == unsafe { WaitForSingleObject(process.as_ref().as_handle(), 0) }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-waitforsingleobject)\] WaitForSingleObject(process, INFINITE) +<br>
/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getexitcodeprocess)\] GetExitCodeProcess
pub fn wait_for_process<'a>(process: impl AsRef<process::Handle<'a>>) -> Result<u32, Error> {
    match unsafe { WaitForSingleObject(process.as_ref().as_handle(), INFINITE) } {
        WAIT_OBJECT_0       => {},
        WAIT_ABANDONED_0    => return Err(Error(ERROR_ABANDONED_WAIT_0)),   // shouldn't happen as `process` isn't a mutex, right?
        WAIT_TIMEOUT        => return Err(Error(ERROR_ABANDONED_WAIT_63)),  // shouldn't happen - hopefully the `63` hints that something is funky?
        WAIT_FAILED         => return Err(Error::get_last()),
        _                   => return Err(Error(ERROR_ABANDONED_WAIT_63)),  // shouldn't happen - hopefully the `63` hints that something is funky?
    }
    get_exit_code_process(process)
}

#[cfg(std)] #[test] fn test_wait_exit() {
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
