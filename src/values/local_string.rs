use abistr::CStrPtr;

use winapi::um::winbase::LocalFree;
use winapi::um::winnt::LPSTR;

use std::borrow::Cow;
use std::fmt::{self, Debug, Display, Formatter};



pub struct LocalString(LPSTR);
impl LocalString {
    pub const unsafe fn from_raw(raw: LPSTR) -> Self { Self(raw) }
    pub fn to_string_lossy<'s>(&'s self) -> Cow<'s, str> { self.as_cstr_ptr().to_string_lossy() }
}
impl LocalString {
    fn as_cstr_ptr<'s>(&'s self) -> CStrPtr<'s> { unsafe { CStrPtr::from_ptr_unbounded(self.0) } }
}
impl Drop       for LocalString { fn drop(&mut self) { assert!(unsafe { LocalFree(self.0.cast()) }.is_null()) } }
impl Debug      for LocalString { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "{:?}", &*self.to_string_lossy()) } }
impl Display    for LocalString { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "{}", self.to_string_lossy()) } }
