use crate::*;

use abistr::CStrPtr;

use core::fmt::{self, Debug, Display, Formatter};
use core::marker::PhantomData;



#[repr(transparent)] pub struct CString<C, D: alloc::Deallocator>(*mut C, PhantomData<D>);
impl<C, D: alloc::Deallocator> CString<C, D> {
    pub const unsafe fn from_raw(raw: *mut C) -> Self { Self(raw, PhantomData) }
}
impl<D: alloc::Deallocator> CString<u8, D> {
    #[cfg(    std )] pub fn to_string_lossy<'s>(&'s self) -> std::borrow::Cow<'s, str> { self.as_cstr_ptr().to_string_lossy() }
    #[cfg(not(std))]     fn to_string_lossy<'s>(&'s self) -> &'s str                   { self.as_cstr_ptr().to_str().unwrap() }
    fn as_cstr_ptr<'s>(&'s self) -> CStrPtr<'s, u8> { unsafe { CStrPtr::from_ptr_unbounded(self.0.cast()) } }
}
impl<C, D: alloc::Deallocator> Drop    for CString<C,  D> { fn drop(&mut self) { unsafe { D::free(self.0) } } }
impl<   D: alloc::Deallocator> Debug   for CString<u8, D> { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "{:?}", &*self.to_string_lossy()) } }
impl<   D: alloc::Deallocator> Display for CString<u8, D> { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "{}", self.to_string_lossy()) } }
