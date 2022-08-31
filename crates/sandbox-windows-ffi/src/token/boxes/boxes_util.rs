use crate::*;
use crate::alloc::{Allocator, CBoxSized};
use winapi::um::winnt::{SID, PSID, SID_AND_ATTRIBUTES};
use core::mem::{size_of, align_of};



#[track_caller] pub fn assert_valid_sid<T, A: Allocator>(cbs: &CBoxSized<T, A>, sid: PSID) {
    let p = cbs.as_ptr() as usize;
    let pend = p + cbs.bytes(); // shouldn't be possible for this to overflow since p .. p+bytes is a contiguous allocation
    let psid = sid as usize;
    assert!(psid % align_of::<SID>() == 0,  "sid was expected to have proper alignment");
    assert!(p + size_of::<T>() <= psid,     "sid was expected to trail TOKEN_USER in the same buffer");
    assert!(psid <= pend,                   "sid was expected to trail TOKEN_USER in the same buffer");
    let sid_bytes = pend - psid; // shouldn't be possible for this to underflow as pend >= psid
    let _validate_sid = unsafe { sid::Ptr::from_raw(sid.cast(), sid_bytes) }.expect("sid was truncated or otherwise invalid");
}

#[track_caller] pub fn assert_valid_saa<T, A: Allocator>(cbs: &CBoxSized<T, A>, saa: SID_AND_ATTRIBUTES) {
    assert_valid_sid(cbs, saa.Sid);
    let _ = saa.Attributes; // presumed valid
}

// TODO: assert/acquire valid slices
