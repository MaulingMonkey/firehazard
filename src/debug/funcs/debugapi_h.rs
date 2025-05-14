use crate::prelude::*;

use abistr::AsCStr;

use winapi::shared::minwindef::FALSE;
use winapi::um::debugapi::*;
use winapi::um::winbase::INFINITE;

use core::time::Duration;



#[doc(alias = "CheckRemoteDebuggerPresent")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/debugapi/nf-debugapi-checkremotedebuggerpresent)\]
/// CheckRemoteDebuggerPresent
///
/// ### Example
/// ```
/// # use firehazard::*;
/// let a = is_debugger_present();
/// let b = check_remote_debugger_present(get_current_process()).unwrap();
/// assert_eq!(a, b);
/// ```
///
pub fn check_remote_debugger_present<'a>(process: impl Into<process::PseudoHandle<'a>>) -> firehazard::Result<bool> {
    let mut result = 0;
    firehazard::Error::get_last_if(FALSE == unsafe { CheckRemoteDebuggerPresent(process.into().as_handle(), &mut result) })?;
    Ok(result != FALSE)
}



#[doc(alias = "ContinueDebugEvent")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/debugapi/nf-debugapi-continuedebugevent)\]
/// ContinueDebugEvent
///
pub fn continue_debug_event(process_id: process::Id, thread_id: thread::Id, continue_status: u32) -> firehazard::Result<()> {
    firehazard::Error::get_last_if(FALSE == unsafe { ContinueDebugEvent(process_id, thread_id, continue_status) })
}



#[doc(alias = "DebugActiveProcess")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/debugapi/nf-debugapi-debugactiveprocess)\]
/// DebugActiveProcess
///
/// ### Example
/// ```
/// # use firehazard::*;
/// # return;
/// # let pid = 0;
/// # #[cfg(nope)]
/// let pid = ..;
/// debug_active_process(pid).unwrap();
/// ```
///
pub fn debug_active_process(process_id: process::Id) -> firehazard::Result<()> {
    firehazard::Error::get_last_if(FALSE == unsafe { DebugActiveProcess(process_id) })
}



#[doc(alias = "DebugActiveProcessStop")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/debugapi/nf-debugapi-debugactiveprocessstop)\]
/// DebugActiveProcessStop
///
/// ### Example
/// ```
/// # use firehazard::*;
/// # return;
/// # let pid = 0;
/// # #[cfg(nope)]
/// let pid = ..;
/// debug_active_process(pid).expect("start debugging self");
/// debug_active_process_stop(pid).expect("stop debugging self");
/// ```
///
pub fn debug_active_process_stop(process_id: process::Id) -> firehazard::Result<()> {
    firehazard::Error::get_last_if(FALSE == unsafe { DebugActiveProcessStop(process_id) })
}



#[doc(alias = "DebugBreak")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/debugapi/nf-debugapi-debugbreak)\]
/// DebugBreak
///
/// ### Example
/// ```
/// # use firehazard::*;
/// # return;
/// eprintln!("BUG: something bad happened");
/// debug_break(); // trigegr the debugger
/// ```
///
pub fn debug_break() {
    unsafe { DebugBreak() }
}




#[doc(alias = "IsDebuggerPresent")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/debugapi/nf-debugapi-isdebuggerpresent)\]
/// IsDebuggerPresent
///
/// ### Example
/// ```
/// # use firehazard::*;
/// if is_debugger_present() { println!("Hello, debugger!"); }
/// ```
///
pub fn is_debugger_present() -> bool {
    FALSE != unsafe { IsDebuggerPresent() }
}



#[doc(alias = "OutputDebugStringA")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/debugapi/nf-debugapi-outputdebugstringa)\]
/// OutputDebugStringA
/// &mdash; Outputs an arbitrary string to a debugger-specific text channel.
///
/// Caveats:
/// *   Messages should terminate in a newline (`\r\n`).  Failure to do so will result in inconsistent behavior between debug message viewers - see Example.
/// *   Messages may be truncated (to as little as 512 bytes?) on some platforms.
///
/// ### Alternatives
/// *   <code>bugsalot::{[debugln!](https://docs.rs/bugsalot/latest/bugsalot/macro.debugln.html), [debug!](https://docs.rs/bugsalot/latest/bugsalot/macro.debug.html)}</code>
///     &mdash; cross platform, formatting
///
/// ### Example
/// ```
/// # use firehazard::*;
/// # use abistr::*;
/// output_debug_string_a(cstr!("A"));
/// output_debug_string_a(cstr!("B"));
/// ```
///
/// In [DebugView] on Windows, this will display something like:
///
/// ```text
/// 33  616.02484131 [12584] A
/// 34  616.02490234 [12584] B
/// ```
///
/// But in [Visual Studio]'s Output tab, or [Visual Studio Code]'s Debug Console, it will instead display something like:
///
/// ```text
/// AB
/// ```
///
/// <!-- References -->
/// [DebugView]:            https://docs.microsoft.com/en-us/sysinternals/downloads/debugview
/// [Visual Studio]:        https://visualstudio.microsoft.com/
/// [Visual Studio Code]:   https://code.visualstudio.com/
///
pub fn output_debug_string_a(output_string: impl AsCStr) {
    unsafe { OutputDebugStringA(output_string.as_cstr()) }
}



#[doc(alias = "OutputDebugStringW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/debugapi/nf-debugapi-outputdebugstringw)\]
/// OutputDebugStringW
/// &mdash; Outputs an arbitrary string to a debugger-specific text channel.
///
/// Caveats:
/// *   Messages should terminate in a newline (`\r\n`).  Failure to do so will result in inconsistent behavior between debug message viewers - see Example.
/// *   Messages may be truncated (to as little as 512 bytes?) on some platforms.
///
/// ### Alternatives
/// *   <code>bugsalot::{[debugln!](https://docs.rs/bugsalot/latest/bugsalot/macro.debugln.html), [debug!](https://docs.rs/bugsalot/latest/bugsalot/macro.debug.html)}</code>
///     &mdash; cross platform, formatting
///
/// ### Example
/// ```
/// # use firehazard::*;
/// # use abistr::*;
/// output_debug_string_w(cstr16!("A"));
/// output_debug_string_w(cstr16!("B"));
/// ```
///
/// In [DebugView] on Windows, this will display something like:
///
/// ```text
/// 33  616.02484131 [12584] A
/// 34  616.02490234 [12584] B
/// ```
///
/// But in [Visual Studio]'s Output tab, or [Visual Studio Code]'s Debug Console, it will instead display something like:
///
/// ```text
/// AB
/// ```
///
/// <!-- References -->
/// [DebugView]:            https://docs.microsoft.com/en-us/sysinternals/downloads/debugview
/// [Visual Studio]:        https://visualstudio.microsoft.com/
/// [Visual Studio Code]:   https://code.visualstudio.com/
///
pub fn output_debug_string_w(output_string: impl AsCStr<u16>) {
    unsafe { OutputDebugStringW(output_string.as_cstr()) }
}



#[doc(alias = "OutputDebugString")]
#[doc(alias = "OutputDebugStringW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/debugapi/nf-debugapi-outputdebugstringw)\]
/// OutputDebugStringW
/// &mdash; Outputs an arbitrary string to a debugger-specific text channel.
///
/// Caveats:
/// *   Messages should terminate in a newline (`\r\n`).  Failure to do so will result in inconsistent behavior between debug message viewers - see Example.
/// *   Messages may be truncated (to as little as 512 bytes?) on some platforms.
/// *   This Rust wrapper may convert `\0` (U+0000) to `␀` (U+2400)
///
/// ### Alternatives
/// *   <code>bugsalot::{[debugln!](https://docs.rs/bugsalot/latest/bugsalot/macro.debugln.html), [debug!](https://docs.rs/bugsalot/latest/bugsalot/macro.debug.html)}</code>
///     &mdash; cross platform, formatting
///
/// ### Example
/// ```
/// # use firehazard::*;
/// # use abistr::*;
/// output_debug_string("A");
/// output_debug_string("B");
/// ```
///
/// In [DebugView] on Windows, this will display something like:
///
/// ```text
/// 33  616.02484131 [12584] A
/// 34  616.02490234 [12584] B
/// ```
///
/// But in [Visual Studio]'s Output tab, or [Visual Studio Code]'s Debug Console, it will instead display something like:
///
/// ```text
/// AB
/// ```
///
/// <!-- References -->
/// [DebugView]:            https://docs.microsoft.com/en-us/sysinternals/downloads/debugview
/// [Visual Studio]:        https://visualstudio.microsoft.com/
/// [Visual Studio Code]:   https://code.visualstudio.com/
///
pub fn output_debug_string(output_string: impl string::InWide) -> firehazard::Result<()> {
    string::convert_to_cstrnn_lossy::<{limit::stack::DEBUG_STRING}, _, _>(output_string, |output_string| {
        unsafe { OutputDebugStringW(output_string.as_cstr()) }
    }).map(|_| ())
}



#[doc(alias = "WaitForDebugEvent")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/debugapi/nf-debugapi-waitfordebugevent)\]
/// WaitForDebugEvent
///
pub fn wait_for_debug_event(timeout: impl Into<Option<Duration>>) -> firehazard::Result<debug::Event> {
    let timeout_ms = timeout.into().map_or(INFINITE, |d| u32::try_from(d.as_millis()).unwrap_or(INFINITE).max(INFINITE-1));
    let mut de = Default::default();
    firehazard::Error::get_last_if(FALSE == unsafe { WaitForDebugEvent(&mut de, timeout_ms) })?;
    Ok(unsafe { debug::Event::from_raw(de) })
}



#[doc(alias = "WaitForDebugEventEx")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/debugapi/nf-debugapi-waitfordebugeventex)\]
/// WaitForDebugEventEx
///
pub fn wait_for_debug_event_ex(timeout: impl Into<Option<Duration>>) -> firehazard::Result<debug::Event> {
    let timeout_ms = timeout.into().map_or(INFINITE, |d| u32::try_from(d.as_millis()).unwrap_or(INFINITE).max(INFINITE-1));
    let mut de = Default::default();
    firehazard::Error::get_last_if(FALSE == unsafe { WaitForDebugEventEx(&mut de, timeout_ms) })?;
    Ok(unsafe { debug::Event::from_raw(de) })
}
