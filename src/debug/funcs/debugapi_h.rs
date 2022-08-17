use crate::*;
use crate::debug::DebugEvent;

use abistr::AsCStr;

use winapi::shared::minwindef::FALSE;
use winapi::um::debugapi::*;
use winapi::um::winbase::INFINITE;

use std::mem::zeroed;
use std::time::Duration;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/debugapi/nf-debugapi-checkremotedebuggerpresent)\]
/// CheckRemoteDebuggerPresent
pub fn check_remote_debugger_present(process: &process::OwnedHandle) -> Result<bool, LastError> {
    let mut result = 0;
    LastError::get_if(FALSE == unsafe { CheckRemoteDebuggerPresent(process.as_handle(), &mut result) })?;
    Ok(result != FALSE)
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/debugapi/nf-debugapi-continuedebugevent)\]
/// ContinueDebugEvent
pub fn continue_debug_event(process_id: process::Id, thread_id: thread::Id, continue_status: u32) -> Result<(), LastError> {
    LastError::get_if(FALSE == unsafe { ContinueDebugEvent(process_id, thread_id, continue_status) })
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/debugapi/nf-debugapi-debugactiveprocess)\]
/// DebugActiveProcess
pub fn debug_active_process(process_id: process::Id) -> Result<(), LastError> {
    LastError::get_if(FALSE == unsafe { DebugActiveProcess(process_id) })
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/debugapi/nf-debugapi-debugactiveprocessstop)\]
/// DebugActiveProcessStop
pub fn debug_active_process_stop(process_id: process::Id) -> Result<(), LastError> {
    LastError::get_if(FALSE == unsafe { DebugActiveProcessStop(process_id) })
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/debugapi/nf-debugapi-debugbreak)\]
/// DebugBreak
pub fn debug_break() {
    unsafe { DebugBreak() }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/debugapi/nf-debugapi-isdebuggerpresent)\]
/// IsDebuggerPresent
pub fn is_debugger_present() -> bool {
    FALSE != unsafe { IsDebuggerPresent() }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/debugapi/nf-debugapi-outputdebugstringa)\]
/// OutputDebugStringA
pub fn output_debug_string_a(output_string: impl AsCStr) {
    unsafe { OutputDebugStringA(output_string.as_cstr()) }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/debugapi/nf-debugapi-outputdebugstringw)\]
/// OutputDebugStringW
pub fn output_debug_string_w(output_string: impl AsCStr<u16>) {
    unsafe { OutputDebugStringW(output_string.as_cstr()) }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/debugapi/nf-debugapi-waitfordebugevent)\]
/// WaitForDebugEvent
pub fn wait_for_debug_event(timeout: impl Into<Option<Duration>>) -> Result<DebugEvent, LastError> {
    let timeout_ms = timeout.into().map_or(INFINITE, |d| u32::try_from(d.as_millis()).unwrap_or(INFINITE).max(INFINITE-1));
    let mut de = unsafe { zeroed() };
    LastError::get_if(FALSE == unsafe { WaitForDebugEvent(&mut de, timeout_ms) })?;
    Ok(unsafe { DebugEvent::from_raw(de) })
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/debugapi/nf-debugapi-waitfordebugeventex)\]
/// WaitForDebugEventEx
pub fn wait_for_debug_event_ex(timeout: impl Into<Option<Duration>>) -> Result<DebugEvent, LastError> {
    let timeout_ms = timeout.into().map_or(INFINITE, |d| u32::try_from(d.as_millis()).unwrap_or(INFINITE).max(INFINITE-1));
    let mut de = unsafe { zeroed() };
    LastError::get_if(FALSE == unsafe { WaitForDebugEventEx(&mut de, timeout_ms) })?;
    Ok(unsafe { DebugEvent::from_raw(de) })
}
