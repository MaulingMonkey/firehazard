use abistr::CStrPtr;

use winapi::shared::sddl::ConvertSidToStringSidA;
use winapi::um::winbase::LocalFree;
use winapi::um::winnt::{SID, LPSTR, SID_AND_ATTRIBUTES};

use std::borrow::Cow;
use std::fmt::{self, Debug, Display, Formatter};
use std::marker::PhantomData;
use std::mem::{align_of, size_of};
use std::ptr::null_mut;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-sid)\] ~ PSID
#[repr(transparent)] pub struct SidPtr<'a>(*mut SID, PhantomData<&'a SID>);

impl SidPtr<'_> {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/sddl/nf-sddl-convertsidtostringsida)\] ConvertSidToStringSidA
    pub fn to_string_sid_a(&self) -> LocalString {
        let mut local_string = null_mut();
        let succeeded = 0 != unsafe { ConvertSidToStringSidA(self.0.cast(), &mut local_string) };
        let local_string = unsafe { LocalString::from_raw(local_string) };
        assert!(succeeded, "ConvertSidToStringSidA");
        local_string
    }
}

impl<'a> Debug for SidPtr<'a> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "PSID({})", self.to_string_sid_a())
    }
}



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-sid_and_attributes)\] SID_AND_ATTRIBUTES
#[repr(C)] pub struct SidAndAttributes<'a> {
    pub sid:        SidPtr<'a>,
    pub attributes: u32,
}

impl Debug for SidAndAttributes<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        struct AsHex(u32);
        impl Debug for AsHex { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "0x{:08x}", self.0) } }
        fmt.debug_struct("SidAndAttributes")
            .field("sid", &self.sid)
            .field("attributes", &AsHex(self.attributes))
            .finish()
    }
}

const _SID_AND_ATTRIBUTES_STATIC_ASSERTS : () = {
    assert!(align_of::<SID_AND_ATTRIBUTES>() == align_of::<SidAndAttributes>());
    assert!(size_of ::<SID_AND_ATTRIBUTES>() == size_of ::<SidAndAttributes>());
};



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
