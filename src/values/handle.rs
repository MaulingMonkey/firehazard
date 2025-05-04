#[doc(alias = "HANDLE")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/sysinfo/kernel-objects)\]
/// A raw **N**on **N**ull handle
///
pub type HANDLENN = core::ptr::NonNull<winapi::ctypes::c_void>;

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/sysinfo/kernel-objects)\]
/// A raw nullable handle
///
pub use winapi::shared::ntdef::HANDLE;
