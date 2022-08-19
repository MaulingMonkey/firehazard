use crate::*;

use abistr::CStrPtr;

use winapi::um::winnt::LPSTR;

use std::borrow::Cow;
use std::fmt::{self, Debug, Display, Formatter};
use std::marker::PhantomData;



#[repr(transparent)] pub struct CString<D: alloc::Deallocator>(LPSTR, PhantomData<D>);
impl<D: alloc::Deallocator> CString<D> {
    pub const unsafe fn from_raw(raw: LPSTR) -> Self { Self(raw, PhantomData) }
    pub fn to_string_lossy<'s>(&'s self) -> Cow<'s, str> { self.as_cstr_ptr().to_string_lossy() }
}
impl<D: alloc::Deallocator> CString<D> {
    fn as_cstr_ptr<'s>(&'s self) -> CStrPtr<'s> { unsafe { CStrPtr::from_ptr_unbounded(self.0) } }
}
impl<D: alloc::Deallocator> Drop    for CString<D> { fn drop(&mut self) { unsafe { D::free(self.0) } } }
impl<D: alloc::Deallocator> Debug   for CString<D> { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "{:?}", &*self.to_string_lossy()) } }
impl<D: alloc::Deallocator> Display for CString<D> { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "{}", self.to_string_lossy()) } }
