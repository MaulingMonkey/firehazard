#[doc(alias = "Sleep")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-sleep)\]
/// Sleep
///
pub fn sleep_ms(milliseconds: u32) { unsafe { winapi::um::synchapi::Sleep(milliseconds) } }



#[doc(alias = "SleepEx")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-sleepex)\]
/// SleepEx
///
pub fn sleep_ms_ex(milliseconds: u32, alertable: bool) -> u32 { unsafe { winapi::um::synchapi::SleepEx(milliseconds, alertable as _) } }
