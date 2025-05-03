use crate::prelude::*;

use abistr::{CStrPtr, Unit};

use ialloc::meta::Stateless;
use ialloc::thin::Free;

use core::fmt::{self, Debug, Display, Formatter};



#[repr(transparent)] pub struct CString<C, A: Free + Stateless>(NonNull<C>, PhantomData<A>);

impl<C, A: Free + Stateless> CString<C, A> {
    #[deprecated = "use `from_raw_{ptr,nn}` instead"] #[doc(hidden)] pub const unsafe fn from_raw(raw: *mut C) -> Self {
        match unsafe { transmute::<*mut C, Option<NonNull<C>>>(raw) } { // XXX: NonNull::new is not yet const-friendly
            Some(nn)    => Self(nn, PhantomData),
            None        => panic!("`CString::from_raw`: pointer cannot be null"),
        }
    }

    pub const unsafe fn from_raw_ptr(raw: *mut C) -> Option<Self> {
        match unsafe { transmute::<*mut C, Option<NonNull<C>>>(raw) } { // XXX: NonNull::new is not yet const-friendly
            Some(nn)    => Some(Self(nn, PhantomData)),
            None        => None,
        }
    }

    pub const unsafe fn from_raw_nn(raw: NonNull<C>) -> Self {
        Self(raw, PhantomData)
    }
}

impl<A: Free + Stateless> CString<u8, A> {
    #[cfg(    std )] pub fn to_string_lossy<'s>(&'s self) -> std::borrow::Cow<'s, str> { self.as_cstr_ptr().to_string_lossy() }
    #[cfg(not(std))]     fn to_string_lossy<'s>(&'s self) -> &'s str                   { self.as_cstr_ptr().to_str().unwrap() }
}

impl<C: Unit, A: Free + Stateless> CString<C, A> {
    pub(crate) const fn as_cstr_nn <'s>(&'s self) -> CStrNonNull<'s, C> { unsafe { CStrNonNull::from_ptr_unchecked_unbounded(self.0.as_ptr().cast()) } }
    pub(crate) const fn as_cstr_ptr<'s>(&'s self) -> CStrPtr<'s, C>     { unsafe { CStrPtr    ::from_ptr_unbounded          (self.0.as_ptr().cast()) } }
}

impl<C, A: Free + Stateless> Drop    for CString<C,  A> { fn drop(&mut self) { unsafe { A::default().free(self.0.cast()) } } }
impl<   A: Free + Stateless> Debug   for CString<u8, A> { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Debug::fmt(&self.as_cstr_ptr(), fmt) } }
impl<   A: Free + Stateless> Debug   for CString<u16,A> { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Debug::fmt(&self.as_cstr_ptr(), fmt) } }
impl<   A: Free + Stateless> Display for CString<u8, A> { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "{}", self.to_string_lossy()) } }
