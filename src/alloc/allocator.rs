use crate::Error;

use winapi::shared::minwindef::FALSE;
use winapi::shared::ntdef::*;
use winapi::shared::winerror::*;

use winapi::um::combaseapi::*;
use winapi::um::heapapi::{HeapAlloc, HeapFree, GetProcessHeap};
use winapi::um::minwinbase::LMEM_ZEROINIT;
use winapi::um::winbase::{LocalAlloc, LocalFree};
use winapi::um::winnt::HEAP_ZERO_MEMORY;

use core::alloc::Layout;
use core::mem::MaybeUninit;
use core::mem::align_of;
use core::ptr::NonNull;



/// [`CoTaskMemAlloc`](https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-cotaskmemalloc) /
/// [`CoTaskMemFree`](https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-cotaskmemfree)
pub struct CoTaskMem;

/// [`HeapAlloc`](https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapalloc) /
/// [`HeapFree`](https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapfree) on
/// [`GetProcessHeap`](https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-getprocessheap)
pub struct ProcessHeapAllocFree;

/// [`LocalAlloc`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localalloc) /
/// [`LocalFree`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localfree)
pub struct LocalAllocFree;

/// [`FreeSid`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-freesid)
pub struct FreeSid;

/// Deallocates memory
pub unsafe trait Deallocator                        { unsafe fn free<T>(mem: *mut T); }
unsafe impl Deallocator for CoTaskMem               { unsafe fn free<T>(mem: *mut T) { unsafe { CoTaskMemFree(mem.cast()) } } }
unsafe impl Deallocator for ProcessHeapAllocFree    { unsafe fn free<T>(mem: *mut T) { assert!(unsafe { HeapFree(GetProcessHeap(), 0, mem.cast()) } != FALSE) } }
unsafe impl Deallocator for LocalAllocFree          { unsafe fn free<T>(mem: *mut T) { assert!(unsafe { LocalFree(mem.cast()) }.is_null()) } }
unsafe impl Deallocator for FreeSid                 { unsafe fn free<T>(mem: *mut T) { assert!(unsafe { winapi::um::securitybaseapi::FreeSid(mem.cast()) }.is_null()) } }



/// Allocates memory
pub unsafe trait Allocator : Deallocator {
    fn try_alloc       <T>(layout: Layout) -> Result<NonNull<T>, Error>;
    fn try_alloc_zeroed<T>(layout: Layout) -> Result<NonNull<T>, Error> {
        let r = Self::try_alloc::<T>(layout)?;
        unsafe { core::ptr::write_bytes(r.as_ptr().cast::<MaybeUninit<u8>>(), 0, layout.size()) };
        Ok(r)
    }
}

unsafe impl Allocator for CoTaskMem {
    fn try_alloc<T>(layout: Layout) -> Result<NonNull<T>, Error> {
        // I can't find documentation on CoTaskMemAlloc's resulting alignment.
        // I suspect it's MEMORY_ALLOCATION_ALIGNMENT ?
        // Out of an abundance of caution I'm instead only assuming it's MAX_NATURAL_ALIGNMENT.

        let layout = layout.align_to(align_of::<T>()).map_err(|_| ERROR_OFFSET_ALIGNMENT_VIOLATION)?;
        if layout.align() > MAX_NATURAL_ALIGNMENT { return Err(Error(ERROR_OFFSET_ALIGNMENT_VIOLATION)) }
        let alloc = unsafe { CoTaskMemAlloc(layout.size()) };
        NonNull::new(alloc.cast()).ok_or_else(|| Error::get_last())
    }
}

unsafe impl Allocator for ProcessHeapAllocFree {
    fn try_alloc       <T>(layout: Layout) -> Result<NonNull<T>, Error> { Self::try_alloc_impl(layout, 0                ) }
    fn try_alloc_zeroed<T>(layout: Layout) -> Result<NonNull<T>, Error> { Self::try_alloc_impl(layout, HEAP_ZERO_MEMORY ) }
}

unsafe impl Allocator for LocalAllocFree {
    fn try_alloc       <T>(layout: Layout) -> Result<NonNull<T>, Error> { Self::try_alloc_impl(layout, 0                ) }
    fn try_alloc_zeroed<T>(layout: Layout) -> Result<NonNull<T>, Error> { Self::try_alloc_impl(layout, LMEM_ZEROINIT    ) }
}

impl ProcessHeapAllocFree {
    fn try_alloc_impl<T>(layout: Layout, flags: u32) -> Result<NonNull<T>, Error> {
        let layout = layout.align_to(align_of::<T>()).map_err(|_| ERROR_OFFSET_ALIGNMENT_VIOLATION)?;
        if layout.align() > MEMORY_ALLOCATION_ALIGNMENT { return Err(Error(ERROR_OFFSET_ALIGNMENT_VIOLATION)) }
        let heap = unsafe { GetProcessHeap() };
        let alloc = unsafe { HeapAlloc(heap, flags, layout.size().max(1)) };
        NonNull::new(alloc.cast()).ok_or_else(|| Error::get_last())
    }
}

impl LocalAllocFree {
    fn try_alloc_impl<T>(layout: Layout, flags: u32) -> Result<NonNull<T>, Error> {
        let layout = layout.align_to(align_of::<T>()).map_err(|_| ERROR_OFFSET_ALIGNMENT_VIOLATION)?;
        if layout.align() > MEMORY_ALLOCATION_ALIGNMENT { return Err(Error(ERROR_OFFSET_ALIGNMENT_VIOLATION)) }
        let alloc = unsafe { LocalAlloc(flags, layout.size().max(1)) };
        NonNull::new(alloc.cast()).ok_or_else(|| Error::get_last())
    }
}
