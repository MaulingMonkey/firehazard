use crate::prelude::*;
use crate::process::environment::*;

use winapi::shared::minwindef::{LPVOID, DWORD};
use winapi::shared::ntdef::{LPCSTR, LPSTR, HANDLE};
use winapi::um::minwinbase::LPSECURITY_ATTRIBUTES;
use winapi::um::processthreadsapi::*;
use winapi::um::synchapi::WaitForSingleObject;
use winapi::um::winbase::*;

#[cfg(std)] use std::ffi::OsStr;
#[cfg(std)] use std::os::windows::ffi::OsStrExt;
#[cfg(std)] use std::path::Path;
#[cfg(std)] use std::vec::Vec;



/// Escape executable path + arguments in the format expected by typical applications that use [`CommandLineToArgv`](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-commandlinetoargvw) or similar to feed their main/WinMain argv arrays.
/// Be aware that some applications - notably including `cmd.exe` - may have their own nonstandard freeform command line parsing logic.
/// There's not much a generic crate like this can do to help with those - you're on your own with that.
/// Good luck!
#[cfg(std)]
pub fn argv_to_command_line_0<A: AsRef<OsStr>>(exe: impl AsRef<Path>, args: impl IntoIterator<Item = A>) -> Vec<u16> {
    let mut cl = Vec::new();
    argv_to_command_line_0_inplace(exe, args, &mut cl);
    cl
}

/// Escape executable path - like [`argv_to_command_line_0`] - without any arguments.
#[cfg(std)]
pub fn exe_to_command_line_0(exe: impl AsRef<Path>) -> Vec<u16> {
    let args : [&'static str; 0] = [];
    argv_to_command_line_0(exe, args)
}

#[cfg(std)]
fn argv_to_command_line_0_inplace<A: AsRef<OsStr>>(exe: impl AsRef<Path>, args: impl IntoIterator<Item = A>, command_line: &mut Vec<u16>) {
    const QUOTE     : u16 = b'\"' as u16;
    const BACKSLASH : u16 = b'\\' as u16;
    command_line.clear();
    command_line.push(QUOTE);
    command_line.extend(exe.as_ref().as_os_str().encode_wide());
    command_line.push(QUOTE);
    for arg in args {
        let arg = arg.as_ref();
        let quote = arg.is_empty() || arg.encode_wide().any(|ch| ch == b' ' as u16 || ch == b'\t' as u16);
        command_line.push(b' ' as _);
        if quote { command_line.push(QUOTE) }
        let mut arg = arg.encode_wide();
        while let Some(ch) = arg.next() {
            match ch {
                BACKSLASH => {
                    let mut backslashes = 1;
                    loop {
                        match arg.next() {
                            Some(BACKSLASH) => backslashes += 1,
                            Some(QUOTE) => {
                                for _ in 0 .. backslashes {
                                    command_line.push(BACKSLASH);
                                    command_line.push(BACKSLASH);
                                }
                                command_line.push(BACKSLASH);
                                command_line.push(QUOTE);
                                break;
                            },
                            Some(ch) => {
                                for _ in 0 .. backslashes { command_line.push(BACKSLASH) }
                                command_line.push(ch);
                                break;
                            },
                            None => {
                                for _ in 0 .. backslashes { command_line.push(BACKSLASH) }
                                break;
                            },
                        }
                    }
                },
                QUOTE => {
                    command_line.push(BACKSLASH);
                    command_line.push(ch);
                },
                _ => command_line.push(ch),
            }
        }
        if quote { command_line.push(QUOTE) }
    }
    command_line.push(0);
}



#[doc(no_inline)] pub use create_process_w as create_process;



#[doc(alias = "CreateProcessA")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessa)\]
/// CreateProcessA
///
pub fn create_process_a(
    application_name:       impl string::InOptionalNarrow,
    command_line:           Option<&[u8]>,
    process_attributes:     Option<&security::Attributes>,
    thread_attributes:      Option<&security::Attributes>,
    inherit_handles:        bool,
    creation_flags:         impl Into<process::CreationFlags>,
    environment:            impl TryIntoEnvironment,
    current_directory:      impl string::InOptionalNarrow,
    startup_info:           &impl process::AsStartupInfoA,
) -> firehazard::Result<process::Information> {
    if let Some(command_line) = command_line.as_ref() {
        let interior = command_line.strip_suffix(&[0]).ok_or(Error(E_STRING_NOT_NULL_TERMINATED as _))?; // must be NUL terminated
        if interior.contains(&0) { return Err(Error(ERROR_ILLEGAL_CHARACTER)) }
    }
    let command_line            = command_line.as_ref().map_or(null(), |c| c.as_ptr()) as *mut _;
    let process_attributes      = process_attributes.map_or(null(), |a| a) as *mut _;
    let thread_attributes       = thread_attributes.map_or(null(), |a| a) as *mut _;
    let inherit_handles         = inherit_handles as _;
    let creation_flags          = creation_flags.into().into();
    let environment             = environment.as_env_ptr(creation_flags & CREATE_UNICODE_ENVIRONMENT != 0)?;
    let startup_info            = startup_info.as_winapi()?;
    let mut process_information = Default::default();

    string::convert_to_cstr::<{limit::stack::PATH}, _, _>(application_name, |application_name| string::convert_to_cstr::<{limit::stack::PATH}, _, _>(current_directory, |current_directory| {
        firehazard::Error::get_last_if(0 == unsafe { CreateProcessA(
            application_name.as_opt_cstr(),
            command_line,
            process_attributes,
            thread_attributes,
            inherit_handles,
            creation_flags,
            environment,
            current_directory.as_opt_cstr(),
            startup_info,
            &mut process_information,
        )})?;
        Ok(unsafe { process::Information::from_raw(process_information) })
    }))??
}



#[doc(alias = "CreateProcess")]
#[doc(alias = "CreateProcessW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessw)\]
/// CreateProcessW
///
pub fn create_process_w(
    application_name:       impl string::InOptionalWide,
    // "The Unicode version of this function, CreateProcessW, can modify the contents of this string.
    // Therefore, this parameter cannot be a pointer to read-only memory (such as a const variable or a literal string).
    // If this parameter is a constant string, the function may cause an access violation."
    mut command_line:       Option<&mut [u16]>,
    process_attributes:     Option<&security::Attributes>,
    thread_attributes:      Option<&security::Attributes>,
    inherit_handles:        bool,
    creation_flags:         impl Into<process::CreationFlags>,
    environment:            impl TryIntoEnvironment,
    current_directory:      impl string::InOptionalWide,
    startup_info:           &impl process::AsStartupInfoW,
) -> firehazard::Result<process::Information> {
    if let Some(command_line) = command_line.as_ref() {
        let interior = command_line.strip_suffix(&[0]).ok_or(Error(E_STRING_NOT_NULL_TERMINATED as _))?; // must be NUL terminated
        if interior.contains(&0) { return Err(Error(ERROR_ILLEGAL_CHARACTER)) }
    }
    let command_line            = command_line.as_mut().map_or(null_mut(), |c| c.as_mut_ptr());
    let process_attributes      = process_attributes.map_or(null(), |a| a) as *mut _;
    let thread_attributes       = thread_attributes.map_or(null(), |a| a) as *mut _;
    let inherit_handles         = inherit_handles as _;
    let creation_flags          = creation_flags.into().into();
    let environment             = environment.as_env_ptr(creation_flags & CREATE_UNICODE_ENVIRONMENT != 0)?;
    let startup_info            = startup_info.as_winapi()?;
    let mut process_information = Default::default();

    string::convert_to_cstr::<{limit::stack::PATH}, _, _>(application_name, |application_name| string::convert_to_cstr::<{limit::stack::PATH}, _, _>(current_directory, |current_directory| {
        firehazard::Error::get_last_if(0 == unsafe { CreateProcessW(
            application_name.as_opt_cstr(),
            command_line,
            process_attributes,
            thread_attributes,
            inherit_handles,
            creation_flags,
            environment,
            current_directory.as_opt_cstr(),
            startup_info,
            &mut process_information
        )})?;
        Ok(unsafe { process::Information::from_raw(process_information) })
    }))??
}



#[doc(no_inline)] pub use create_process_as_user_w as create_process_as_user;



#[doc(alias = "CreateProcessAsUserA")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessasusera)\]
/// CreateProcessAsUserA
///
/// ### Errors
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
/// | ERROR_ILLEGAL_CHARACTER       | `application_name` contains interior `\0`s
/// | ERROR_ILLEGAL_CHARACTER       | `command_line` contains interior `\0`s
/// | ERROR_ILLEGAL_CHARACTER       | `current_directory` contains interior `\0`s
/// | ERROR_BAD_ENVIRONMENT         | `environment` was ANSI despite `creation_flags` containing `CREATE_UNICODE_ENVIRONMENT`
/// | ERROR_BAD_ENVIRONMENT         | `environment` was UTF16 despite `creation_flags` lacking `CREATE_UNICODE_ENVIRONMENT`
/// | E_STRING_NOT_NULL_TERMINATED  | `environment` was missing `\0\0` terminator
/// | E_STRING_NOT_NULL_TERMINATED  | `command_line` was missing a `\0` terminator
///
pub fn create_process_as_user_a(
    token:                  &crate::token::OwnedHandle,
    application_name:       impl string::InOptionalNarrow,
    command_line:           Option<&[u8]>,
    process_attributes:     Option<&security::Attributes>,
    thread_attributes:      Option<&security::Attributes>,
    inherit_handles:        bool,
    creation_flags:         impl Into<process::CreationFlags>,
    environment:            impl TryIntoEnvironment,
    current_directory:      impl string::InOptionalNarrow,
    startup_info:           &impl process::AsStartupInfoA,
) -> firehazard::Result<process::Information> {
    if let Some(command_line) = command_line.as_ref() {
        let interior = command_line.strip_suffix(&[0]).ok_or(Error(E_STRING_NOT_NULL_TERMINATED as _))?; // must be NUL terminated
        if interior.contains(&0) { return Err(Error(ERROR_ILLEGAL_CHARACTER)) }
    }
    let token               = token.as_handle();
    let command_line        = command_line.as_ref().map_or(null(), |c| c.as_ptr()) as *mut _;
    let process_attributes  = process_attributes.map_or(null(), |a| a) as *mut _;
    let thread_attributes   = thread_attributes.map_or(null(), |a| a) as *mut _;
    let inherit_handles     = inherit_handles as _;
    let creation_flags      = creation_flags.into().into();
    let environment         = environment.as_env_ptr(creation_flags & CREATE_UNICODE_ENVIRONMENT != 0)?;
    let startup_info        = startup_info.as_winapi()?;
    let mut process_information = Default::default();

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

    string::convert_to_cstr::<{limit::stack::PATH}, _, _>(application_name, |application_name| string::convert_to_cstr::<{limit::stack::PATH}, _, _>(current_directory, |current_directory| {
        firehazard::Error::get_last_if(0 == unsafe { CreateProcessAsUserA(
            token,
            application_name.as_opt_cstr(),
            command_line,
            process_attributes,
            thread_attributes,
            inherit_handles,
            creation_flags,
            environment,
            current_directory.as_opt_cstr(),
            startup_info,
            &mut process_information
        )})?;
        Ok(unsafe { process::Information::from_raw(process_information) })
    }))??
}



#[doc(alias = "CreateProcessAsUser")]
#[doc(alias = "CreateProcessAsUserW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessasuserw)\] CreateProcessAsUserW
///
/// ### Errors
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
/// | ERROR_ILLEGAL_CHARACTER       | `application_name` contains interior `\0`s
/// | ERROR_ILLEGAL_CHARACTER       | `command_line` contains interior `\0`s
/// | ERROR_ILLEGAL_CHARACTER       | `current_directory` contains interior `\0`s
/// | ERROR_BAD_ENVIRONMENT         | `environment` was ANSI despite `creation_flags` containing `CREATE_UNICODE_ENVIRONMENT`
/// | ERROR_BAD_ENVIRONMENT         | `environment` was UTF16 despite `creation_flags` lacking `CREATE_UNICODE_ENVIRONMENT`
/// | E_STRING_NOT_NULL_TERMINATED  | `environment` was missing `\0\0` terminator
/// | E_STRING_NOT_NULL_TERMINATED  | `command_line` was missing a `\0` terminator
///
pub fn create_process_as_user_w(
    token:                  &crate::token::OwnedHandle,
    application_name:       impl string::InOptionalWide,
    // "The Unicode version of this function, CreateProcessAsUserW, can modify the contents of this string.
    // Therefore, this parameter cannot be a pointer to read-only memory (such as a const variable or a literal string).
    // If this parameter is a constant string, the function may cause an access violation."
    mut command_line:       Option<&mut [u16]>,
    process_attributes:     Option<&security::Attributes>,
    thread_attributes:      Option<&security::Attributes>,
    inherit_handles:        bool,
    creation_flags:         impl Into<process::CreationFlags>,
    environment:            impl TryIntoEnvironment,
    current_directory:      impl string::InOptionalWide,
    startup_info:           &impl process::AsStartupInfoW,
) -> firehazard::Result<process::Information> {
    if let Some(command_line) = command_line.as_ref() {
        let interior = command_line.strip_suffix(&[0]).ok_or(Error(E_STRING_NOT_NULL_TERMINATED as _))?; // must be NUL terminated
        if interior.contains(&0) { return Err(Error(ERROR_ILLEGAL_CHARACTER)) }
    }
    let token               = token.as_handle();
    let command_line        = command_line.as_mut().map_or(null_mut(), |c| c.as_mut_ptr());
    let process_attributes  = process_attributes.map_or(null(), |a| a) as *mut _;
    let thread_attributes   = thread_attributes.map_or(null(), |a| a) as *mut _;
    let inherit_handles     = inherit_handles as _;
    let creation_flags      = creation_flags.into().into();
    let environment         = environment.as_env_ptr(creation_flags & CREATE_UNICODE_ENVIRONMENT != 0)?;
    let startup_info        = startup_info.as_winapi()?;
    let mut process_information = Default::default();

    string::convert_to_cstr::<{limit::stack::PATH}, _, _>(application_name, |application_name| string::convert_to_cstr::<{limit::stack::PATH}, _, _>(current_directory, |current_directory| {
        firehazard::Error::get_last_if(0 == unsafe { CreateProcessAsUserW(
            token,
            application_name.as_opt_cstr(),
            command_line,
            process_attributes,
            thread_attributes,
            inherit_handles,
            creation_flags,
            environment,
            current_directory.as_opt_cstr(),
            startup_info,
            &mut process_information
        )})?;
        Ok(unsafe { process::Information::from_raw(process_information) })
    }))??
}



#[doc(alias = "CreateProcessWithLogon")]
#[doc(alias = "CreateProcessWithLogonW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-createprocesswithlogonw)\]
/// CreateProcessWithLogonW
///
fn _create_process_with_logon_w() -> firehazard::Result<process::Information> { unimplemented!() }



#[doc(alias = "CreateProcessWithToken")]
#[doc(alias = "CreateProcessWithTokenW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-createprocesswithtokenw)\]
/// CreateProcessWithTokenW
///
fn _create_process_with_token_w() -> firehazard::Result<process::Information> { unimplemented!() }



#[doc(alias = "ExitProcess")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-exitprocess)\]
/// ExitProcess
///
/// Exit the current process with a specific exit code (typically `0` to indicate success, nonzero to indicate failure.)
///
/// ### Alternatives
///
/// *   <code>std::process::[exit](std::process::exit)(exit_code)</code> &mdash; cross platform, expects a signed value, requires [`std`].
///
/// ### Examples
///
/// ```
/// # use firehazard::*;
/// # if false {
/// exit_process(0); // success
/// // unreachable
///
/// exit_process(42); // error
/// // unreachable
/// # }
/// ```
///
pub fn exit_process(exit_code: u32) -> ! { unsafe { ExitProcess(exit_code); core::hint::unreachable_unchecked() } }



#[doc(alias = "GetCurrentProcess")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocess)\]
/// GetCurrentProcess
///
/// Get a pseudo-handle (currently `-1`) to the current process.
///
/// ### Example
///
/// ```
/// # use firehazard::*;
/// let pseudo_handle   : process::PseudoHandle = get_current_process();
/// let real_handle     : process::OwnedHandle  = pseudo_handle.try_clone_to_owned().unwrap();
/// ```
///
pub fn get_current_process() -> process::PseudoHandle<'static> { unsafe { process::PseudoHandle::from_raw(GetCurrentProcess()).unwrap() } }



#[doc(alias = "GetCurrentProcessId")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocessid)\]
/// GetCurrentProcessId
///
/// Gets the caller's process ID.
/// For a more durable reference to a process, consider using a handle instead.
///
/// ### Alternatives
///
/// *   <code>std::process::[id](std::process::id)()</code> &mdash; cross platform, requires [`std`].
///
/// ### Example
///
/// ```
/// # use firehazard::*;
/// let pid = get_current_process_id();
/// dbg!(pid); // display some process ID
/// ```
///
pub fn get_current_process_id() -> process::Id { unsafe { GetCurrentProcessId() } }



// get/set process afinity masks, etc.



#[doc(alias = "GetExitCodeProcess")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getexitcodeprocess)\]
/// GetExitCodeProcess
///
/// ### Alternatives
///
/// *   <code>std::process::[ExitStatus](std::process::ExitStatus)::[code](std::process::ExitStatus::code)()</code> &mdash; cross platform, requires [`std`], child processes only.
/// *   [`wait_for_process`] &mdash; waits for the process to exit before calling `GetExitCodeProcess`.
///
/// ### Returns
///
/// *   `Ok(259)` (`STILL_ACTIVE` / `STATUS_PENDING`)   &mdash; if `process` is still running
/// *   `Ok(0)`                                         &mdash; if `process` exited "successfully"
/// *   `Ok(exit_code)`                                 &mdash; if `process` exited otherwise
/// *   `Err(...)`                                      &mdash; if `process` lacks `PROCESS_QUERY_[LIMITED_]INFORMATION ` permission?
/// *   `Err(...)`                                      &mdash; if `process` is an invalid handle?
///
/// ### Example
///
/// ```
/// # use firehazard::*;
/// # #[cfg(std)] {
/// #
/// let mut process = std::process::Command::new("ping.exe")
///     .args("localhost -n 2".split(' '))
///     .spawn().unwrap();
///
/// // while (probably) running
/// assert_eq!(Ok(259), get_exit_code_process(&process)); // STILL_ACTIVE
///
/// std::thread::sleep(std::time::Duration::from_secs(2));
///
/// // after (probably) running
/// assert_eq!(Ok(0), get_exit_code_process(&process));
/// assert_eq!(Some(0), process.wait().unwrap().code());
/// #
/// # }
/// ```
///
pub fn get_exit_code_process<'a>(process: impl Into<process::Handle<'a>>) -> firehazard::Result<u32> {
    let mut exit_code = 0;
    firehazard::Error::get_last_if(0 == unsafe { GetExitCodeProcess(
        process.into().as_handle(),
        &mut exit_code,
    )})?;
    Ok(exit_code)
}



#[doc(alias = "GetProcessMitigationPolicy")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocessmitigationpolicy)\]
/// GetProcessMitigationPolicy
///
pub fn get_process_mitigation_policy<'a, P: process::mitigation::GetPolicy>(process: impl Into<process::PseudoHandle<'a>>) -> firehazard::Result<P> {
    let mut p = P::Raw::default();
    firehazard::Error::get_last_if(0 == unsafe { GetProcessMitigationPolicy(
        process.into().as_handle(),
        P::ty() as u32,
        &mut p as *mut P::Raw as *mut _,
        size_of::<P::Raw>(),
    )})?;
    Ok(P::from_policy(p))
}



#[doc(alias = "WaitForSingleObject")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-waitforsingleobject)\]
/// WaitForSingleObject(process, 0) == WAIT_TIMEOUT
///
/// | Process/Handle State              | Returns   |
/// | ----------------------------------| ----------|
/// | *no `SYNCHRONIZE` permissions*    | false?    |
/// | Running                           | true      |
/// | Blocked                           | true      |
/// | Suspended                         | true      |
/// | Exited                            | false     |
/// | Killed                            | false     |
///
/// Since [`process::Handle`](crate::process::Handle) should be a valid handle,
/// it's "impossible" to pass a dangling/invalid handle.  If you do anyways,
/// the return value is indeterminite, `STATUS_INVALID_HANDLE` may be thrown if
/// [strict handle checks](crate::process::mitigation::StrictHandleCheckPolicy)
/// are enabled.
///
/// ### Example
///
/// ```
/// # use firehazard::*;
/// # #[cfg(std)] {
/// #
/// let mut process = std::process::Command::new("ping.exe")
///     .args("localhost -n 2".split(' '))
///     .spawn().unwrap();
///
/// // while (probably) running
/// assert_eq!(true, is_process_alive(&process));
///
/// std::thread::sleep(std::time::Duration::from_secs(2));
///
/// // after (probably) running
/// assert_eq!(false, is_process_alive(&process));
/// #
/// # }
/// ```
///
pub fn is_process_alive<'a>(process: impl Into<process::Handle<'a>>) -> bool {
    WAIT_TIMEOUT == unsafe { WaitForSingleObject(process.into().as_handle(), 0) }
}

#[doc(hidden)] #[deprecated = "renamed to is_process_alive: will return `true` for suspended processes, which isn't \"running\" per se"] pub use is_process_alive as is_process_running;



#[doc(alias = "SetProcessMitigationPolicy")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessmitigationpolicy)\]
/// SetProcessMitigationPolicy
///
pub fn set_process_mitigation_policy<P: process::mitigation::SetPolicy>(policy: P) -> firehazard::Result<()> {
    firehazard::Error::get_last_if(0 == unsafe { SetProcessMitigationPolicy(
        P::ty() as u32,
        &policy.into_policy() as *const P::Raw as *mut _,
        size_of::<P::Raw>(),
    )})
}



#[doc(alias = "GetExitCodeProcess")]
#[doc(alias = "WaitForSingleObject")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-waitforsingleobject)\] WaitForSingleObject(process, INFINITE) +<br>
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getexitcodeprocess)\] GetExitCodeProcess
///
/// ### Alternatives
///
/// *   <code>std::process::[Child](std::process::Child)::[wait](std::process::Child)()?.[code](std::process::ExitStatus::code)()?</code> &mdash; cross platform, requires [`std`], child processes only, consumes handle.
/// *   [`get_exit_code_process`] &mdash; works on running processes without waiting for them
///
/// ### Returns
///
/// *   `Ok(0)`         &mdash; if `process` exited "successfully"
/// *   `Ok(exit_code)` &mdash; if `process` exited otherwise
/// *   `Err(...)`      &mdash; if `process` lacks `PROCESS_QUERY_[LIMITED_]INFORMATION ` permission?
/// *   `Err(...)`      &mdash; if `process` lacks `SYNCHRONIZE` permission?
/// *   `Err(...)`      &mdash; if `process` is an invalid handle?
///
/// ### Example
///
/// ```
/// # use firehazard::*;
/// # #[cfg(std)] {
/// #
/// let mut process = std::process::Command::new("ping.exe")
///     .args("localhost -n 2".split(' '))
///     .spawn().unwrap();
///
/// assert_eq!(Ok(0), wait_for_process(&process));
/// assert_eq!(Some(0), process.wait().unwrap().code());
/// #
/// # }
/// ```
///
pub fn wait_for_process<'a>(process: impl Into<process::Handle<'a>>) -> firehazard::Result<u32> {
    let process = process.into();
    match unsafe { WaitForSingleObject(process.as_handle(), INFINITE) } {
        WAIT_OBJECT_0       => {},
        WAIT_ABANDONED_0    => return Err(firehazard::Error(ERROR_ABANDONED_WAIT_0)),   // shouldn't happen as `process` isn't a mutex, right?
        WAIT_TIMEOUT        => return Err(firehazard::Error(ERROR_ABANDONED_WAIT_63)),  // shouldn't happen - hopefully the `63` hints that something is funky?
        WAIT_FAILED         => return Err(firehazard::Error::get_last()),
        _                   => return Err(firehazard::Error(ERROR_ABANDONED_WAIT_63)),  // shouldn't happen - hopefully the `63` hints that something is funky?
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
