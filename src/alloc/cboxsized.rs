use crate::alloc::{Allocator, Deallocator, ProcessHeapAllocFree};

use core::alloc::Layout;
use core::fmt::{self, Debug, Formatter};
use core::marker::PhantomData;
use core::mem::{align_of, size_of};
use core::ops::*;



pub struct CBoxSized<T, D: Deallocator = ProcessHeapAllocFree>(*mut T, usize, PhantomData<D>);
unsafe impl<T: Send, D: Deallocator> Send for CBoxSized<T, D> {}
unsafe impl<T: Sync, D: Deallocator> Sync for CBoxSized<T, D> {}

impl<T, A: Allocator> CBoxSized<T, A> {
    pub fn new(value: T) -> Self { Self::new_oversized(value, 0) }

    pub fn new_oversized(value: T, total_bytes: usize) -> Self {
        let total_bytes = size_of::<T>().max(total_bytes);
        let layout = Layout::from_size_align(total_bytes, align_of::<T>()).unwrap();
        let alloc = A::try_alloc_zeroed(layout).unwrap();
        unsafe { core::ptr::write(alloc, value) };
        Self(alloc, total_bytes, PhantomData)
    }
}

impl<T, D: Deallocator> CBoxSized<T, D> {
    pub fn as_mut_ptr(&mut self) -> *mut   T { self.0 }
    pub fn as_ptr    (&    self) -> *const T { self.0 }

    pub fn bytes(&self) -> usize { self.1 }
}

impl<T: Debug, D: Deallocator> Debug    for CBoxSized<T, D> { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { fmt.debug_tuple("CBoxSized").field(&**self).finish() } }
impl<T       , D: Deallocator> Drop     for CBoxSized<T, D> { fn drop(&mut self) { unsafe { D::free(self.0) } } }
impl<T       , D: Deallocator> Deref    for CBoxSized<T, D> { fn deref    (&    self) -> &    T { unsafe { &    *self.0 } } type Target = T; }
impl<T       , D: Deallocator> DerefMut for CBoxSized<T, D> { fn deref_mut(&mut self) -> &mut T { unsafe { &mut *self.0 } } }
