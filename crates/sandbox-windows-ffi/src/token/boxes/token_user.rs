use crate::*;
use crate::alloc::*;

use winapi::um::winnt::{TOKEN_USER, SID};

use core::fmt::{self, Debug, Formatter};
use core::mem::{align_of, size_of};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_user)\] ~ `Box<(TOKEN_USER, ..)>`
pub struct BoxTokenUser(CBox<TOKEN_USER>);

impl BoxTokenUser {
    pub fn from_raw(cbs: CBoxSized<TOKEN_USER>) -> Self {
        // SAFETY: We must validate the pointer TOKEN_USER::User::Sid
        // TOKEN_USER::User::Attributes (u32) is presumed valid for all bit patterns
        let p = cbs.as_ptr() as usize;
        let pend = p + cbs.bytes(); // shouldn't be possible for this to overflow since p .. p+bytes is a contiguous allocation
        let psid = cbs.User.Sid as usize;
        assert!(psid % align_of::<SID>() == 0,          "TOKEN_USER::User::Sid was expected to have proper alignment");
        assert!(p + size_of::<TOKEN_USER>() <= psid,    "TOKEN_USER::User::Sid was expected to trail TOKEN_USER in the same buffer");
        assert!(psid <= pend,                           "TOKEN_USER::User::Sid was expected to trail TOKEN_USER in the same buffer");
        let sid_bytes = pend - psid; // shouldn't be possible for this to underflow as pend >= psid
        let _validate_sid = unsafe { sid::Ptr::from_raw(cbs.User.Sid.cast(), sid_bytes) }.expect("TOKEN_USER::User::Sid was truncated or otherwise invalid");
        Self(cbs.into())
    }

    /// User
    pub fn user<'s>(&'s self) -> &'s sid::AndAttributes<'s> {
        unsafe { &*(&self.0.User as *const _ as *const sid::AndAttributes) }
    }
}

impl Debug for BoxTokenUser {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("BoxTokenUser").field("user", self.user()).finish()
    }
}
