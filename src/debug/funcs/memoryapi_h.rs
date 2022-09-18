use crate::*;

use winapi::shared::minwindef::FALSE;
use winapi::shared::winerror::ERROR_BUFFER_OVERFLOW;
use winapi::um::memoryapi::ReadProcessMemory;

use core::mem::{MaybeUninit, size_of_val, size_of};



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-readprocessmemory)\]
/// ReadProcessMemory
///
/// ### Safety
/// This function can corrupt the target `process` by triggering a stack guard page exception without growing the stack,
/// if anything between `base_address .. base_address + buffer.len()` isn't a valid address/pointer.  Since `process`
/// could be the current process, this means we can corrupt our *own* stack with this function!
///
/// Ref: [ReadProcessMemory is not a preferred IPC mechanism](https://devblogs.microsoft.com/oldnewthing/20060117-14/?p=32633)
///
/// ### Example
/// ```
/// # use firehazard::*;
/// let original = [42u32];
/// let mut copy = [core::mem::MaybeUninit::<u32>::uninit(); 1];
/// let copy = unsafe { read_process_memory(get_current_process(), original.as_ptr(), &mut copy[..]) }.unwrap();
/// assert_eq!(copy[0], 42);
/// ```
pub unsafe fn read_process_memory<'a, 'p, T>(
    process:        impl AsRef<process::PsuedoHandle<'p>>,
    base_address:   *const T,
    buffer:         &'a mut [MaybeUninit<T>],
) -> Result<&'a [T], Error> {
    let size = size_of_val(buffer);
    let mut read = 0;
    Error::get_last_if(FALSE == unsafe { ReadProcessMemory(process.as_ref().as_handle(), base_address.cast(), buffer.as_mut_ptr().cast(), size, &mut read) })?;
    if read > size { return Err(Error(ERROR_BUFFER_OVERFLOW)) }
    let n = read/size_of::<T>();
    Ok(unsafe { slice_assume_init_ref(&buffer[..n]) })
}

/// replace if/when `#![feature(maybe_uninit_slice)]` stabilizes
/// <https://github.com/rust-lang/rust/issues/63569>
/// <https://doc.rust-lang.org/core/mem/union.MaybeUninit.html#method.slice_assume_init_ref>
const unsafe fn slice_assume_init_ref<T>(slice: &[MaybeUninit<T>]) -> &[T] {
    unsafe { &*(slice as *const [MaybeUninit<T>] as *const [T]) }
}
