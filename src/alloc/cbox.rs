use crate::alloc::CBoxSized;

use ialloc::allocator::win32::ProcessHeap;
use ialloc::fat;
use ialloc::meta::Stateless;
use ialloc::thin::Free;

use core::alloc::Layout;
use core::fmt::{self, Debug, Formatter};
use core::marker::PhantomData;
use core::mem::{align_of, size_of, forget};
use core::ops::*;
use core::ptr::NonNull;



#[repr(transparent)] pub struct CBox<T, A: Free + Stateless = ProcessHeap>(NonNull<T>, PhantomData<A>);
unsafe impl<T: Send, A: Free + Stateless> Send for CBox<T, A> {}
unsafe impl<T: Sync, A: Free + Stateless> Sync for CBox<T, A> {}

impl<T, A: Free + Stateless> CBox<T, A> {
    pub unsafe fn from_raw_ptr(value: *mut T) -> Option<Self> { Some(Self(NonNull::new(value)?, PhantomData)) }
    pub unsafe fn from_raw_nn(value: NonNull<T>) -> Self { Self(value, PhantomData) }
}

impl<T, A: fat::Alloc + Free + Stateless> CBox<T, A> {
    pub fn new(value: T) -> Self { Self::new_oversized(value, 0) }

    pub fn new_oversized(value: T, total_bytes: usize) -> Self {
        let total_bytes = size_of::<T>().max(total_bytes);
        let layout = Layout::from_size_align(total_bytes, align_of::<T>()).unwrap();
        let alloc = A::default().alloc_zeroed(layout).unwrap().cast();
        unsafe { core::ptr::write(alloc.as_ptr(), value) };
        Self(alloc, PhantomData)
    }
}

impl<T, A: Free + Stateless> CBox<T, A> {
    pub fn as_non_null(&self) -> NonNull<T> { self.0 }
    pub fn as_mut_ptr(&mut self) -> *mut   T { self.0.as_ptr() }
    pub fn as_ptr    (&    self) -> *const T { self.0.as_ptr() }
}

impl<T: Debug, A: Free + Stateless> Debug    for CBox<T, A> { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { fmt.debug_tuple("CBox").field(&**self).finish() } }
impl<T       , A: Free + Stateless> Drop     for CBox<T, A> { fn drop(&mut self) { unsafe { A::default().free(self.0.cast()) } } }
impl<T       , A: Free + Stateless> Deref    for CBox<T, A> { fn deref    (&    self) -> &    T { unsafe { self.0.as_ref() } } type Target = T; }
impl<T       , A: Free + Stateless> DerefMut for CBox<T, A> { fn deref_mut(&mut self) -> &mut T { unsafe { self.0.as_mut() } } }
impl<T       , A: Free + Stateless> From<CBoxSized<T, A>> for CBox<T, A> { fn from(cbs: CBoxSized<T, A>) -> Self { let r = Self(cbs.as_non_null(), PhantomData); forget(cbs); r } }
