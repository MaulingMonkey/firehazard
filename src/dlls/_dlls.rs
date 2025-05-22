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
    /// ### Safety
    /// Libraries may have unsound preconditions about the context they're expected to be loaded into,
    /// and loading said library may immediate execute code (via DllMain, constructors, etc.) which relies on said unsound preconditions.
    /// Even worse, tricking programs into loading the wrong library as a means of executing malware is a whole thing.
    /// Use with caution!
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
    /// ### Safety
    /// This assumes the symbol referred to by `proc_name` is a `fn` of type `F`.
    /// This assumption is utterly unchecked, and could be trivially undermined by anyone wanting to get up to *shennanigans*.
    ///
    pub(crate) unsafe fn get_proc_address<F>(&self, proc_name: &core::ffi::CStr) -> Option<F> {
        use winapi::shared::minwindef::FARPROC;
        const { assert!(size_of::<F>() == size_of::<FARPROC>()) };

        if self.0.is_null() { return None }
        let farproc = unsafe { winapi::um::libloaderapi::GetProcAddress(
            self.0,
            proc_name.as_ptr().cast(),
        )};
        if farproc.is_null() { return None }
        Some(unsafe { core::mem::transmute_copy::<FARPROC, F>(&farproc) })
    }
}
