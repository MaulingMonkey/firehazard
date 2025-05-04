#[doc(alias = "SetLastError")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror)\]
/// Saves the value of `GetLastError()` for restoration on [`Drop`].
///
pub(crate) struct PreserveErrorScope {
    previous: u32,
}

impl Drop for PreserveErrorScope {
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-setlasterror)\]
    /// SetLastError(self.previous)
    ///
    fn drop(&mut self) {
        unsafe { winapi::um::errhandlingapi::SetLastError(self.previous) };
    }
}

impl PreserveErrorScope {
    #[doc(alias = "GetLastError")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror)\]
    /// Saves the value of `GetLastError()` for restoration on [`Drop`].
    ///
    pub fn new() -> Self {
        Self { previous: unsafe { winapi::um::errhandlingapi::GetLastError() } }
    }
}
