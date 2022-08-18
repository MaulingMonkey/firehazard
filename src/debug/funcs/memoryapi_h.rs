use crate::*;

use winapi::shared::minwindef::FALSE;
use winapi::shared::winerror::ERROR_BUFFER_OVERFLOW;
use winapi::um::memoryapi::ReadProcessMemory;

use std::mem::{MaybeUninit, size_of_val, size_of};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-readprocessmemory)\]
/// ReadProcessMemory
pub fn read_process_memory<'a, T>(
    process:        impl AsRef<process::Handle>,
    base_address:   *const T,
    buffer:         &'a mut [MaybeUninit<T>],
) -> Result<&'a [T], LastError> {
    let size = size_of_val(buffer);
    let mut read = 0;
    LastError::get_if(FALSE == unsafe { ReadProcessMemory(process.as_ref().as_handle(), base_address.cast(), buffer.as_mut_ptr().cast(), size, &mut read) })?;
    if read > size { return Err(LastError(ERROR_BUFFER_OVERFLOW)) }
    let n = read/size_of::<T>();
    Ok(unsafe { slice_assume_init_ref(&buffer[..n]) })
}

/// replace if/when `#![feature(maybe_uninit_slice)]` stabilizes
/// <https://github.com/rust-lang/rust/issues/63569>
/// <https://doc.rust-lang.org/core/mem/union.MaybeUninit.html#method.slice_assume_init_ref>
const unsafe fn slice_assume_init_ref<T>(slice: &[MaybeUninit<T>]) -> &[T] {
    unsafe { &*(slice as *const [MaybeUninit<T>] as *const [T]) }
}
