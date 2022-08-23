use crate::alloc::{Allocator, Deallocator, CBoxSized, ProcessHeapAllocFree};

use core::alloc::Layout;
use core::fmt::{self, Debug, Formatter};
use core::marker::PhantomData;
use core::mem::{align_of, size_of, forget};
use core::ops::*;
use core::ptr::NonNull;



#[repr(transparent)] pub struct CBox<T, D: Deallocator = ProcessHeapAllocFree>(NonNull<T>, PhantomData<D>);
unsafe impl<T: Send, D: Deallocator> Send for CBox<T, D> {}
unsafe impl<T: Sync, D: Deallocator> Sync for CBox<T, D> {}

impl<T, A: Allocator> CBox<T, A> {
    pub fn new(value: T) -> Self { Self::new_oversized(value, 0) }

    pub fn new_oversized(value: T, total_bytes: usize) -> Self {
        let total_bytes = size_of::<T>().max(total_bytes);
        let layout = Layout::from_size_align(total_bytes, align_of::<T>()).unwrap();
        let alloc = A::try_alloc_zeroed(layout).unwrap();
        unsafe { core::ptr::write(alloc.as_ptr(), value) };
        Self(alloc, PhantomData)
    }
}

impl<T, D: Deallocator> CBox<T, D> {
    pub fn as_non_null(&self) -> NonNull<T> { self.0 }
    pub fn as_mut_ptr(&mut self) -> *mut   T { self.0.as_ptr() }
    pub fn as_ptr    (&    self) -> *const T { self.0.as_ptr() }
}

impl<T: Debug, D: Deallocator> Debug    for CBox<T, D> { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { fmt.debug_tuple("CBox").field(&**self).finish() } }
impl<T       , D: Deallocator> Drop     for CBox<T, D> { fn drop(&mut self) { unsafe { D::free(self.0.as_ptr()) } } }
impl<T       , D: Deallocator> Deref    for CBox<T, D> { fn deref    (&    self) -> &    T { unsafe { self.0.as_ref() } } type Target = T; }
impl<T       , D: Deallocator> DerefMut for CBox<T, D> { fn deref_mut(&mut self) -> &mut T { unsafe { self.0.as_mut() } } }
impl<T       , D: Deallocator> From<CBoxSized<T, D>> for CBox<T, D> { fn from(cbs: CBoxSized<T, D>) -> Self { let r = Self(cbs.as_non_null(), PhantomData); forget(cbs); r } }
