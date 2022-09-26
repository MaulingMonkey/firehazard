use crate::alloc::{Allocator, Deallocator};

use core::fmt::{self, Debug, Formatter};
use core::marker::PhantomData;
use core::mem::forget;
use core::ops::*;
use core::ptr::NonNull;
use core::slice::SliceIndex;



pub struct CVec<T, D: Deallocator> {
    ptr:        NonNull<T>, // dangling if capacity == 0
    length:     usize,
    capacity:   usize,
    phantom:    PhantomData<D>,
}

unsafe impl<T: Send, D: Deallocator> Send for CVec<T, D> {}
unsafe impl<T: Sync, D: Deallocator> Sync for CVec<T, D> {}

impl<T, D: Deallocator> CVec<T, D> {
    pub fn new() -> Self { Self { ptr: NonNull::dangling(), length: 0, capacity: 0, phantom: PhantomData } }

    pub unsafe fn from_raw_parts(ptr: *mut T, length: usize, capacity: usize) -> Self {
        assert!(length <= capacity);
        assert!(!ptr.is_null() || capacity == 0);
        let ptr = NonNull::new(ptr).unwrap_or(NonNull::dangling());
        Self { ptr, length, capacity, phantom: PhantomData }
    }

    pub fn into_raw_parts(mut self) -> (*mut T, usize, usize) {
        let r = (self.as_mut_ptr(), self.length, self.capacity);
        forget(self);
        r
    }

    //pub fn as_non_null(&self) -> NonNull<T> { self.ptr }
    pub fn as_mut_ptr(&mut self) -> *mut   T { self.ptr.as_ptr() }
    pub fn as_ptr    (&    self) -> *const T { self.ptr.as_ptr() }
    pub fn as_mut_slice(&mut self) -> &mut [T] { unsafe { core::slice::from_raw_parts_mut(self.as_mut_ptr(), self.len()) } }
    pub fn as_slice    (&    self) -> &    [T] { unsafe { core::slice::from_raw_parts    (self.as_ptr(),     self.len()) } }

    pub fn is_empty(&self) -> bool { self.length == 0 }
    pub fn len(&self) -> usize { self.length }
    pub fn capacity(&self) -> usize { self.capacity }

    pub fn pop(&mut self) -> Option<T> {
        let length = self.length.checked_sub(1)?;
        self.length = length;
        Some(unsafe { core::ptr::read(self.as_ptr().add(length)) })
    }

    pub fn clear(&mut self) { while self.pop().is_some() {} }
    pub unsafe fn set_len(&mut self, new_len: usize) { debug_assert!(new_len <= self.capacity); self.length = new_len; }

    // TODO: methods like:
    // drain
    // drain_filter
    // leak
    // remove
    // retain
    // retain_mut
    // shrink_to
    // shrink_to_fit
    // spare_capacity_mut
    // split_at_spare_mut
    // swap_remove
    pub fn truncate(&mut self, len: usize) { while self.len() > len { self.pop(); } }
}

impl<T, A: Allocator> CVec<T, A> {
    // TODO: allocation reliant methods like:
    // append
    // push
    // extend_from_*
    // insert
    // reserve
    // reserve_exact
    // resize
    // resize_with
    // splice
    // split_off
    // try_reserve
    // try_reserve_exact
    // with_capacity
}

impl<T,        D: Deallocator> Default  for CVec<T, D> { fn default() -> Self { Self::new() } }
impl<T: Debug, D: Deallocator> Debug    for CVec<T, D> { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Debug::fmt(&**self, fmt) } }
impl<T       , D: Deallocator> Drop     for CVec<T, D> { fn drop(&mut self) { if self.capacity > 0 { unsafe { D::free(self.ptr.as_ptr()) } } } }
impl<T       , D: Deallocator> Deref    for CVec<T, D> { fn deref    (&    self) -> &    [T] { self.as_slice() } type Target = [T]; }
impl<T       , D: Deallocator> DerefMut for CVec<T, D> { fn deref_mut(&mut self) -> &mut [T] { self.as_mut_slice() } }
// TODO: many traits

impl<T, D: Deallocator, I: SliceIndex<[T]>> Index<I>    for CVec<T, D> { fn index    (&    self, index: I) -> &    Self::Output { Index   ::index    (self.as_slice(),     index) } type Output = I::Output; }
impl<T, D: Deallocator, I: SliceIndex<[T]>> IndexMut<I> for CVec<T, D> { fn index_mut(&mut self, index: I) -> &mut Self::Output { IndexMut::index_mut(self.as_mut_slice(), index) } }
