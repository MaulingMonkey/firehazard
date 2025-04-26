use crate::prelude::*;

use ialloc::allocator::{adapt::PanicOverAlign, win32::ProcessHeap};
use ialloc::fat;
use ialloc::meta::Stateless;
use ialloc::thin::Free;

use core::alloc::Layout;
use core::fmt::{self, Debug, Formatter};
use core::ops::*;



pub struct CBoxSized<T, A: Free + Stateless = ProcessHeap>(NonNull<T>, usize, PhantomData<A>);
unsafe impl<T: Send, A: Free + Stateless> Send for CBoxSized<T, A> {}
unsafe impl<T: Sync, A: Free + Stateless> Sync for CBoxSized<T, A> {}

impl<T, A: Free + Stateless> CBoxSized<T, A> {
    pub unsafe fn from_raw_ptr(value: *mut T, total_bytes: usize) -> Option<Self> { Some(Self(NonNull::new(value)?, total_bytes, PhantomData)) }
    pub unsafe fn from_raw_nn(value: NonNull<T>, total_bytes: usize) -> Self { Self(value, total_bytes, PhantomData) }
}

impl<T, A: fat::Alloc + Free + Stateless> CBoxSized<T, A> {
    /// Allocates `size_of::<T>()` bytes exactly
    pub fn new(value: T) -> Self { Self::new_oversized(value, 0) }

    /// Allocates `size_of::<T>().max(total_bytes)` bytes
    pub fn new_oversized(value: T, total_bytes: usize) -> Self {
        use ialloc::fat::Alloc as _;
        let total_bytes = size_of::<T>().max(total_bytes);
        let layout = Layout::from_size_align(total_bytes, align_of::<T>()).unwrap();
        let alloc = PanicOverAlign(A::default()).alloc_zeroed(layout).unwrap().cast();
        unsafe { core::ptr::write(alloc.as_ptr(), value) };
        Self(alloc, total_bytes, PhantomData)
    }
}

impl<T, A: Free + Stateless> CBoxSized<T, A> {
    pub fn as_non_null(&self) -> NonNull<T> { self.0 }
    pub fn as_mut_ptr(&mut self) -> *mut   T { self.0.as_ptr() }
    pub fn as_ptr    (&    self) -> *const T { self.0.as_ptr() }

    pub fn bytes(&self) -> usize { self.1 }
}

impl<T: Debug, A: Free + Stateless> Debug    for CBoxSized<T, A> { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { fmt.debug_tuple("CBoxSized").field(&**self).finish() } }
impl<T       , A: Free + Stateless> Drop     for CBoxSized<T, A> { fn drop(&mut self) { unsafe { A::default().free(self.0.cast()) } } }
impl<T       , A: Free + Stateless> Deref    for CBoxSized<T, A> { fn deref    (&    self) -> &    T { unsafe { self.0.as_ref() } } type Target = T; }
impl<T       , A: Free + Stateless> DerefMut for CBoxSized<T, A> { fn deref_mut(&mut self) -> &mut T { unsafe { self.0.as_mut() } } }
