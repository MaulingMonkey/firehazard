#[doc(alias = "HeapSetInformation")]
#[doc(alias = "HeapEnableTerminationOnCorruption")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapsetinformation)\]
/// HeapSetInformation(0, HeapEnableTerminationOnCorruption, 0, 0)
///
/// ### Example
/// ```
/// # use firehazard::*;
/// heap_enable_termination_on_corruption().unwrap();
/// ```
///
/// ### Notes
/// Per [rust#56054](https://github.com/rust-lang/rust/issues/56054) and [The Old New Thing](https://devblogs.microsoft.com/oldnewthing/20131227-00/?p=2243), this is probably already enabled for you.
///
pub fn heap_enable_termination_on_corruption() -> firehazard::Result<()> {
    firehazard::Error::get_last_if(0 == unsafe { winapi::um::heapapi::HeapSetInformation(
        null_mut(),
        winapi::um::winnt::HeapEnableTerminationOnCorruption,
        null_mut(),
        0,
    )})
}
