//! Allocators, deallocators, etc.



/// [`LocalAlloc`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localalloc) /
/// [`LocalFree`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localfree)
pub struct LocalAllocFree;

/// [`FreeSid`](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-freesid)
pub struct FreeSid;

/// Deallocates [`SID`](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-sid)s
pub unsafe trait Deallocator                { unsafe fn free<T>(sid: *mut T); }
unsafe impl Deallocator for LocalAllocFree  { unsafe fn free<T>(sid: *mut T) { assert!(unsafe { winapi::um::winbase::LocalFree(sid.cast()) }.is_null()) } }
unsafe impl Deallocator for FreeSid         { unsafe fn free<T>(sid: *mut T) { assert!(unsafe { winapi::um::securitybaseapi::FreeSid(sid.cast()) }.is_null()) } }
