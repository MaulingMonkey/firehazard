use core::ptr::NonNull;



/// ≈ [`core::slice::from_raw_parts`], but takes [`NonNull`]
pub(crate) const unsafe fn from_nn_len_ref<'a, T>(data: NonNull<T>, len: usize) -> &'a [T] {
    unsafe { core::slice::from_raw_parts(data.as_ptr(), len) }
}

/// ≈ [`core::slice::from_raw_parts_mut`], but takes [`NonNull`]
pub(crate) const unsafe fn from_nn_len_mut<'a, T>(data: NonNull<T>, len: usize) -> &'a mut [T] {
    unsafe { core::slice::from_raw_parts_mut(data.as_ptr(), len) }
}

/// ≈ [`core::slice::from_raw_parts`], but tolerates `(data, len)` == `(nullptr, 0)`
pub(crate) const unsafe fn from_nullable_len_ref<'a, T>(data: *const T, len: usize) -> &'a [T] {
    match NonNull::new(data.cast_mut()) {
        Some(data)          => unsafe { from_nn_len_ref(data, len) },
        None if len == 0    => &[],
        None                => panic!("firehazard::slice::from_nullable_len_ref(null, 1 ..): undefined behavior"),
    }
}

/// ≈ [`core::slice::from_raw_parts_mut`], but tolerates `(data, len)` == `(nullptr, 0)`
pub(crate) const unsafe fn from_nullable_len_mut<'a, T>(data: *mut T, len: usize) -> &'a mut [T] {
    match NonNull::new(data) {
        Some(data)          => unsafe { from_nn_len_mut(data, len) },
        None if len == 0    => &mut [],
        None                => panic!("firehazard::slice::from_nullable_len_mut(null, 1 ..): undefined behavior"),
    }
}



/// Properly acquiring a slice from a C-style flexible array member requires a fairly careful dance:
/// -   Reading the length via a reference to the structure owning the array would presumably invalidate any `&mut [T; _]`
/// -   The spatial provenance of the array likely only extends to 0 or 1 elements, not `len` elements, at least in stacked borrows mode.
///
/// This fn attempts to carefully avoid these problems:
/// -   `select_len` encourages going through an appropriate reborrow of the mut pointer instead of a parallel, invalidating borrow
/// -   Provenance fixups to expand the spatial provenance to `parent` is automatically provided
///
pub(crate) unsafe fn from_flexible_array_mut<'a, P, T, const N : usize>(
    parent:         *mut P,
    select_len:     impl FnOnce(&P) -> usize,
    select_array:   impl FnOnce(*mut P) -> *mut [T; N],
) -> &'a mut [T] {
    const { assert!(N <= 1, "slice::from_flexible_array_mut: array was really expected to have a length of 0 or 1") };
    if parent.is_null() { return &mut [] }
    let len = select_len(unsafe { &*parent });
    let array = unsafe { NonNull::new_unchecked(crate::provenance_addr_mut(parent, select_array(parent).cast::<T>())) };
    unsafe { from_nn_len_mut(array, len) }
}

pub(crate) unsafe fn from_flexible_array_ref<'a, P, T, const N : usize>(
    parent:         *const P,
    select_len:     impl FnOnce(&P) -> usize,
    select_array:   impl FnOnce(*const P) -> *const [T; N],
) -> &'a [T] {
    const { assert!(N <= 1, "slice::from_flexible_array_ref: array was really expected to have a length of 0 or 1") };
    if parent.is_null() { return &[] }
    let len = select_len(unsafe { &*parent });
    let array = unsafe { NonNull::new_unchecked(crate::provenance_addr(parent, select_array(parent).cast::<T>()).cast_mut()) };
    unsafe { from_nn_len_ref(array, len) }
}



// replace if/when `#![feature(maybe_uninit_slice)]` stabilizes
// <https://github.com/rust-lang/rust/issues/63569>
// <https://doc.rust-lang.org/core/mem/union.MaybeUninit.html#method.slice::assume_init_ref>

pub(crate) unsafe fn assume_init_ref<T>(slice: &[core::mem::MaybeUninit<T>]) -> &[T] {
    let len = slice.len();
    unsafe { core::slice::from_raw_parts(slice.as_ptr().cast(), len) }
}

pub(crate) unsafe fn assume_init_mut<T>(slice: &mut [core::mem::MaybeUninit<T>]) -> &mut [T] {
    let len = slice.len();
    unsafe { core::slice::from_raw_parts_mut(slice.as_mut_ptr().cast(), len) }
}
