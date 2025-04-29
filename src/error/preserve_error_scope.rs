use crate::prelude::*;
use winapi::um::errhandlingapi::*;



/// Saves the value of `GetLastError()` for restoration on [`Drop`].
pub(crate) struct PreserveErrorScope {
    previous: u32,
}

impl PreserveErrorScope {
    /// Saves the value of `GetLastError()` for restoration on [`Drop`].
    pub fn new() -> Self {
        Self { previous: unsafe { GetLastError() } }
    }
}

impl Drop for PreserveErrorScope {
    fn drop(&mut self) {
        unsafe { SetLastError(self.previous) };
    }
}
