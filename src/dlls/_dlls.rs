use crate::prelude::*;
use winapi::shared::minwindef::HMODULE;

pub(crate) mod kernelbase;
pub(crate) mod ntdll;



#[derive(Clone, Copy)] struct OptionalLibrary(HMODULE);
unsafe impl Send for OptionalLibrary {}
unsafe impl Sync for OptionalLibrary {}

impl OptionalLibrary {
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibraryw)\]
    /// LoadLibraryW
    ///
    pub(crate) unsafe fn load_w(lib_file_name: CStrNonNull<u16>) -> Self {
        let hmodule = unsafe { winapi::um::libloaderapi::LoadLibraryW(
            lib_file_name.as_cstr(),
        )};
        Self(hmodule)
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getprocaddress)\]
    /// GetProcAddress
    ///
    pub(crate) unsafe fn get_proc_address<F>(&self, proc_name: CStrNonNull) -> Option<F> {
        use winapi::shared::minwindef::FARPROC;
        const { assert!(size_of::<F>() == size_of::<FARPROC>()) };

        if self.0.is_null() { return None }
        let farproc = unsafe { winapi::um::libloaderapi::GetProcAddress(
            self.0,
            proc_name.as_cstr(),
        )};
        if farproc.is_null() { return None }
        Some(unsafe { core::mem::transmute_copy::<FARPROC, F>(&farproc) })
    }
}
