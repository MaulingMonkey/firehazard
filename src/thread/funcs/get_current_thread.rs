#[doc(alias = "GetCurrentThread")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentthread)\]
/// GetCurrentThread
///
pub fn get_current_thread() -> firehazard::thread::PseudoHandle<'static> {
    use firehazard::FromLocalHandle;
    unsafe { firehazard::thread::PseudoHandle::from_raw(winapi::um::processthreadsapi::GetCurrentThread()).unwrap() }
}
