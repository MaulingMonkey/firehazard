use crate::*;

use abistr::CStrPtr;

use ialloc::meta::Stateless;
use ialloc::thin::Free;

use core::fmt::{self, Debug, Display, Formatter};
use core::marker::PhantomData;



#[repr(transparent)] pub struct CString<C, A: Free + Stateless>(*mut C, PhantomData<A>);
impl<C, A: Free + Stateless> CString<C, A> {
    pub const unsafe fn from_raw(raw: *mut C) -> Self { Self(raw, PhantomData) }
}
impl<A: Free + Stateless> CString<u8, A> {
    #[cfg(    std )] pub fn to_string_lossy<'s>(&'s self) -> std::borrow::Cow<'s, str> { self.as_cstr_ptr().to_string_lossy() }
    #[cfg(not(std))]     fn to_string_lossy<'s>(&'s self) -> &'s str                   { self.as_cstr_ptr().to_str().unwrap() }
    fn as_cstr_ptr<'s>(&'s self) -> CStrPtr<'s, u8> { unsafe { CStrPtr::from_ptr_unbounded(self.0.cast()) } }
}
impl<C, A: Free + Stateless> Drop    for CString<C,  A> { fn drop(&mut self) { unsafe { A::default().free_nullable(self.0.cast()) } } }
impl<   A: Free + Stateless> Debug   for CString<u8, A> { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "{:?}", &*self.to_string_lossy()) } }
impl<   A: Free + Stateless> Display for CString<u8, A> { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "{}", self.to_string_lossy()) } }
