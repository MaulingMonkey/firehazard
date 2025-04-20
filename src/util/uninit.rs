// replace if/when `#![feature(maybe_uninit_slice)]` stabilizes
// <https://github.com/rust-lang/rust/issues/63569>
// <https://doc.rust-lang.org/core/mem/union.MaybeUninit.html#method.slice_assume_init_ref>

pub(crate) unsafe fn slice_assume_init_ref<T>(slice: &[core::mem::MaybeUninit<T>]) -> &[T] {
    let len = slice.len();
    unsafe { core::slice::from_raw_parts(slice.as_ptr().cast(), len) }
}

pub(crate) unsafe fn slice_assume_init_mut<T>(slice: &mut [core::mem::MaybeUninit<T>]) -> &mut [T] {
    let len = slice.len();
    unsafe { core::slice::from_raw_parts_mut(slice.as_mut_ptr().cast(), len) }
}
