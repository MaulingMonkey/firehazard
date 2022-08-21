//! Allocators, deallocators, etc.

mod cstring;                    pub use cstring::*;



/// [`HeapAlloc`](https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapalloc) /
/// [`HeapFree`](https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapfree) on
/// [`GetProcessHeap`](https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-getprocessheap)
pub struct ProcessHeapAllocFree;

/// [`LocalAlloc`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localalloc) /
/// [`LocalFree`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localfree)
pub struct LocalAllocFree;

/// [`FreeSid`](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-freesid)
pub struct FreeSid;

/// Deallocates memory
pub unsafe trait Deallocator                        { unsafe fn free<T>(mem: *mut T); }
unsafe impl Deallocator for ProcessHeapAllocFree    { unsafe fn free<T>(mem: *mut T) { assert!(unsafe { winapi::um::heapapi::HeapFree(winapi::um::heapapi::GetProcessHeap(), 0, mem.cast()) } != 0) } }
unsafe impl Deallocator for LocalAllocFree          { unsafe fn free<T>(mem: *mut T) { assert!(unsafe { winapi::um::winbase::LocalFree(mem.cast()) }.is_null()) } }
unsafe impl Deallocator for FreeSid                 { unsafe fn free<T>(mem: *mut T) { assert!(unsafe { winapi::um::securitybaseapi::FreeSid(mem.cast()) }.is_null()) } }
