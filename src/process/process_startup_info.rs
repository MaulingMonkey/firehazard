use crate::*;

use abistr::CStrNonNull;

use winapi::shared::winerror::ERROR_INCORRECT_SIZE;
use winapi::um::processthreadsapi::{STARTUPINFOA, STARTUPINFOW};
use winapi::um::winbase::{STARTUPINFOEXA, STARTUPINFOEXW};

use core::mem::{align_of, size_of, transmute};



#[derive(Clone, Debug, Default)] #[repr(transparent)] #[doc(hidden)] pub struct DefaultOnly<T>(T);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/ns-processthreadsapi-startupinfoa)\]
/// STARTUPINFOA param for the CreateProcess family of functions
pub unsafe trait AsStartupInfoA { fn as_winapi(&self) -> Result<*mut STARTUPINFOA, Error>; }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/ns-processthreadsapi-startupinfow)\]
/// STARTUPINFOW param for the CreateProcess family of functions
pub unsafe trait AsStartupInfoW { fn as_winapi(&self) -> Result<*mut STARTUPINFOW, Error>; }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/ns-processthreadsapi-startupinfoa)\]
/// STARTUPINFOA
pub type StartupInfoA<'s> = StartupInfo<'s, u8>;

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/ns-processthreadsapi-startupinfow)\]
/// STARTUPINFOW
pub type StartupInfoW<'s> = StartupInfo<'s, u16>;

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winbase/ns-winbase-startupinfoexa)\]
/// STARTUPINFOEXA
pub type StartupInfoExA<'s> = StartupInfoEx<'s, u8>;

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winbase/ns-winbase-startupinfoexw)\]
/// STARTUPINFOEXW
pub type StartupInfoExW<'s> = StartupInfoEx<'s, u16>;

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/ns-processthreadsapi-startupinfoa)\]
/// STARTUPINFO
#[derive(Debug)]
#[repr(C)] pub struct StartupInfo<'s, U: abistr::Unit> where U::CChar : Clone {
    #[doc(hidden)] pub cb: u32,
    #[doc(hidden)] pub _reserved: DefaultOnly<Option<CStrNonNull<'s, U>>>,
    pub desktop:        Option<CStrNonNull<'s, U>>,
    pub title:          Option<CStrNonNull<'s, U>>,
    pub x:              u32,
    pub y:              u32,
    pub x_size:         u32,
    pub y_size:         u32,
    pub x_count_chars:  u32,
    pub y_count_chars:  u32,
    pub fill_attribute: u32, // XXX
    pub flags:          u32, // XXX
    pub show_window:    u16,
    #[doc(hidden)] pub _cb_reserved_2: DefaultOnly<u16>,
    #[doc(hidden)] pub _lp_reserved_2: DefaultOnly<usize>,
    pub std_input:      Option<io::ReadHandle<'s>>,
    pub std_output:     Option<io::WriteHandle<'s>>,
    pub std_error:      Option<io::WriteHandle<'s>>,
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winbase/ns-winbase-startupinfoexa)\]
/// STARTUPINFOEX
#[derive(Debug)]
#[repr(C)] pub struct StartupInfoEx<'s, U: abistr::Unit> where U::CChar : Clone {
    pub startup_info:   StartupInfo<'s, U>,
    pub attribute_list: Option<process::ThreadAttributeList<'s>>, // XXX: some borrowing option might be nicer, would re-enable Clone
}

const _A_ALIGN : () = assert!(align_of::<process::StartupInfoA>() == align_of::<STARTUPINFOA>());
const _A_SIZE  : () = assert!(size_of ::<process::StartupInfoA>() == size_of ::<STARTUPINFOA>());
const _W_ALIGN : () = assert!(align_of::<process::StartupInfoW>() == align_of::<STARTUPINFOW>());
const _W_SIZE  : () = assert!(size_of ::<process::StartupInfoW>() == size_of ::<STARTUPINFOW>());

const _A_EX_ALIGN : () = assert!(align_of::<process::StartupInfoExA>() == align_of::<STARTUPINFOEXA>());
const _A_EX_SIZE  : () = assert!(size_of ::<process::StartupInfoExA>() == size_of ::<STARTUPINFOEXA>());
const _W_EX_ALIGN : () = assert!(align_of::<process::StartupInfoExW>() == align_of::<STARTUPINFOEXW>());
const _W_EX_SIZE  : () = assert!(size_of ::<process::StartupInfoExW>() == size_of ::<STARTUPINFOEXW>());

impl AsRef<STARTUPINFOA> for StartupInfoA<'_> { fn as_ref(&self) -> &STARTUPINFOA { unsafe { transmute(self) } } }
impl AsRef<STARTUPINFOW> for StartupInfoW<'_> { fn as_ref(&self) -> &STARTUPINFOW { unsafe { transmute(self) } } }
impl AsRef<STARTUPINFOEXA> for StartupInfoExA<'_> { fn as_ref(&self) -> &STARTUPINFOEXA { unsafe { transmute(self) } } }
impl AsRef<STARTUPINFOEXW> for StartupInfoExW<'_> { fn as_ref(&self) -> &STARTUPINFOEXW { unsafe { transmute(self) } } }

impl StartupInfoA<'_> { pub unsafe fn from_raw(pi: STARTUPINFOA) -> Self { unsafe { transmute(pi) } } }
impl StartupInfoW<'_> { pub unsafe fn from_raw(pi: STARTUPINFOW) -> Self { unsafe { transmute(pi) } } }
impl StartupInfoExA<'_> { pub unsafe fn from_raw(pi: STARTUPINFOEXA) -> Self { unsafe { transmute(pi) } } }
impl StartupInfoExW<'_> { pub unsafe fn from_raw(pi: STARTUPINFOEXW) -> Self { unsafe { transmute(pi) } } }

unsafe impl AsStartupInfoA for StartupInfoA<'_>   { fn as_winapi(&self) -> Result<*mut STARTUPINFOA, Error> { if usize::from32(self             .cb) != size_of::<Self>() { Err(Error(ERROR_INCORRECT_SIZE)) } else { Ok(self as *const _ as *mut _) } } }
unsafe impl AsStartupInfoW for StartupInfoW<'_>   { fn as_winapi(&self) -> Result<*mut STARTUPINFOW, Error> { if usize::from32(self             .cb) != size_of::<Self>() { Err(Error(ERROR_INCORRECT_SIZE)) } else { Ok(self as *const _ as *mut _) } } }
unsafe impl AsStartupInfoA for StartupInfoExA<'_> { fn as_winapi(&self) -> Result<*mut STARTUPINFOA, Error> { if usize::from32(self.startup_info.cb) != size_of::<Self>() { Err(Error(ERROR_INCORRECT_SIZE)) } else { Ok(&self.startup_info as *const _ as *mut _) } } }
unsafe impl AsStartupInfoW for StartupInfoExW<'_> { fn as_winapi(&self) -> Result<*mut STARTUPINFOW, Error> { if usize::from32(self.startup_info.cb) != size_of::<Self>() { Err(Error(ERROR_INCORRECT_SIZE)) } else { Ok(&self.startup_info as *const _ as *mut _) } } }

// XXX: startup info isn't optional IME (results in ERROR_INVALID_PARAMETER), so don't implement the trait for now
// unsafe impl AsOptStartupInfoA for Option<&'_ StartupInfoA<'_>>   { fn as_winapi(self) -> Result<*mut STARTUPINFOA, Error> { match self { None => Ok(null_mut()), Some(si) => si.as_winapi() } } }
// unsafe impl AsOptStartupInfoW for Option<&'_ StartupInfoW<'_>>   { fn as_winapi(self) -> Result<*mut STARTUPINFOW, Error> { match self { None => Ok(null_mut()), Some(si) => si.as_winapi() } } }
// unsafe impl AsOptStartupInfoA for Option<&'_ StartupInfoExA<'_>> { fn as_winapi(self) -> Result<*mut STARTUPINFOA, Error> { match self { None => Ok(null_mut()), Some(si) => si.as_winapi() } } }
// unsafe impl AsOptStartupInfoW for Option<&'_ StartupInfoExW<'_>> { fn as_winapi(self) -> Result<*mut STARTUPINFOW, Error> { match self { None => Ok(null_mut()), Some(si) => si.as_winapi() } } }
//
// // `None` is ambiguous so provide `()` as an alternative:
// unsafe impl AsOptStartupInfoA for () { fn as_winapi(self) -> Result<*mut STARTUPINFOA, Error> { Ok(null_mut()) } }
// unsafe impl AsOptStartupInfoW for () { fn as_winapi(self) -> Result<*mut STARTUPINFOW, Error> { Ok(null_mut()) } }

impl<'s, U: abistr::Unit> Default for StartupInfo<'s, U> where U::CChar : Clone {
    fn default() -> Self {
        Self {
            cb:             size_of::<Self>() as _,
            _reserved:      Default::default(),
            desktop:        None,
            title:          None,
            x:              0,
            y:              0,
            x_size:         0,
            y_size:         0,
            x_count_chars:  0,
            y_count_chars:  0,
            fill_attribute: 0,
            flags:          0,
            show_window:    0,
            _cb_reserved_2: Default::default(),
            _lp_reserved_2: Default::default(),
            std_input:      Default::default(),
            std_output:     Default::default(),
            std_error:      Default::default(),
        }
    }
}

impl<'s, U: abistr::Unit> Default for StartupInfoEx<'s, U> where U::CChar : Clone {
    fn default() -> Self {
        Self {
            startup_info: StartupInfo {
                cb: size_of::<Self>() as _,
                .. Default::default()
            },
            attribute_list: None,
        }
    }
}
