use crate::alloc::{Allocator, Deallocator, ProcessHeapAllocFree};

use core::alloc::Layout;
use core::fmt::{self, Debug, Formatter};
use core::marker::PhantomData;
use core::mem::{align_of, size_of};
use core::ops::*;
use core::ptr::NonNull;



pub struct CBoxSized<T, D: Deallocator = ProcessHeapAllocFree>(NonNull<T>, usize, PhantomData<D>);
unsafe impl<T: Send, D: Deallocator> Send for CBoxSized<T, D> {}
unsafe impl<T: Sync, D: Deallocator> Sync for CBoxSized<T, D> {}

impl<T, A: Allocator> CBoxSized<T, A> {
    pub unsafe fn from_raw_ptr(value: *mut T, total_bytes: usize) -> Option<Self> { Some(Self(NonNull::new(value)?, total_bytes, PhantomData)) }
    pub unsafe fn from_raw_nn(value: NonNull<T>, total_bytes: usize) -> Self { Self(value, total_bytes, PhantomData) }

    pub fn new(value: T) -> Self { Self::new_oversized(value, 0) }

    pub fn new_oversized(value: T, total_bytes: usize) -> Self {
        let total_bytes = size_of::<T>().max(total_bytes);
        let layout = Layout::from_size_align(total_bytes, align_of::<T>()).unwrap();
        let alloc = A::try_alloc_zeroed(layout).unwrap();
        unsafe { core::ptr::write(alloc.as_ptr(), value) };
        Self(alloc, total_bytes, PhantomData)
    }
}

impl<T, D: Deallocator> CBoxSized<T, D> {
    pub fn as_non_null(&self) -> NonNull<T> { self.0 }
    pub fn as_mut_ptr(&mut self) -> *mut   T { self.0.as_ptr() }
    pub fn as_ptr    (&    self) -> *const T { self.0.as_ptr() }

    pub fn bytes(&self) -> usize { self.1 }
}

impl<T: Debug, D: Deallocator> Debug    for CBoxSized<T, D> { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { fmt.debug_tuple("CBoxSized").field(&**self).finish() } }
impl<T       , D: Deallocator> Drop     for CBoxSized<T, D> { fn drop(&mut self) { unsafe { D::free(self.0.as_ptr()) } } }
impl<T       , D: Deallocator> Deref    for CBoxSized<T, D> { fn deref    (&    self) -> &    T { unsafe { self.0.as_ref() } } type Target = T; }
impl<T       , D: Deallocator> DerefMut for CBoxSized<T, D> { fn deref_mut(&mut self) -> &mut T { unsafe { self.0.as_mut() } } }
