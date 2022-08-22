use crate::*;

use abistr::CStrPtr;

use winapi::um::winnt::LPSTR;

use core::fmt::{self, Debug, Display, Formatter};
use core::marker::PhantomData;



#[repr(transparent)] pub struct CString<D: alloc::Deallocator>(LPSTR, PhantomData<D>);
impl<D: alloc::Deallocator> CString<D> {
    pub const unsafe fn from_raw(raw: LPSTR) -> Self { Self(raw, PhantomData) }
    #[cfg(std)] pub fn to_string_lossy<'s>(&'s self) -> std::borrow::Cow<'s, str> { self.as_cstr_ptr().to_string_lossy() }
    #[cfg(not(std))] fn to_string_lossy<'s>(&'s self) -> &'s str { self.as_cstr_ptr().to_str().unwrap() }
}
impl<D: alloc::Deallocator> CString<D> {
    fn as_cstr_ptr<'s>(&'s self) -> CStrPtr<'s> { unsafe { CStrPtr::from_ptr_unbounded(self.0) } }
}
impl<D: alloc::Deallocator> Drop    for CString<D> { fn drop(&mut self) { unsafe { D::free(self.0) } } }
impl<D: alloc::Deallocator> Debug   for CString<D> { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "{:?}", &*self.to_string_lossy()) } }
impl<D: alloc::Deallocator> Display for CString<D> { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "{}", self.to_string_lossy()) } }
