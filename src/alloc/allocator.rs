use ialloc::Alignment;

use core::mem::MaybeUninit;



pub use ialloc::thin::Free as Deallocator;

#[doc(no_inline)] pub use ialloc::allocator::win32::CoTaskMem;
#[doc(no_inline)] pub use ialloc::allocator::win32::ProcessHeap as ProcessHeapAllocFree;
#[doc(no_inline)] pub use ialloc::allocator::win32::Local as LocalAllocFree;

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-freesid)
/// FreeSid
///
#[derive(Clone, Copy, Debug, Default)] pub struct FreeSid;

impl ialloc::meta::Meta for FreeSid {
    type Error = ialloc::allocator::win32::Error;

    const MAX_ALIGN     : Alignment = Alignment::of::<winapi::um::winnt::SID>();
    const MAX_SIZE      : usize     = usize::MAX;
    const ZST_SUPPORTED : bool      = false;
}

unsafe impl ialloc::meta::Stateless for FreeSid {}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-freesid)
/// FreeSid
///
unsafe impl ialloc::thin::Free for FreeSid {
    unsafe fn free_nullable(&self, mem: *mut MaybeUninit<u8>) {
        assert!(unsafe { winapi::um::securitybaseapi::FreeSid(mem.cast()) }.is_null())
    }
}

ialloc::impls! {
    unsafe impl ialloc::fat::Free for FreeSid => ialloc::thin::Free;
}
