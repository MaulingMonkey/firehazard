//! Allocators, deallocators, etc.

use winapi::um::winnt::SID;



/// [`LocalAlloc`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localalloc) /
/// [`LocalFree`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localfree)
pub struct LocalAllocFree;

/// [`FreeSid`](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-freesid)
pub struct FreeSid;

/// Deallocates [`SID`](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-sid)s
pub unsafe trait SidDeallocator                 { unsafe fn free(sid: *mut SID); }
unsafe impl SidDeallocator for LocalAllocFree   { unsafe fn free(sid: *mut SID) { assert!(unsafe { winapi::um::winbase::LocalFree(sid.cast()) }.is_null()) } }
unsafe impl SidDeallocator for FreeSid          { unsafe fn free(sid: *mut SID) { assert!(unsafe { winapi::um::securitybaseapi::FreeSid(sid.cast()) }.is_null()) } }
