use crate::*;
use crate::alloc::CBoxSized;
use ialloc::meta::Stateless;
use ialloc::thin::{Alloc, Free};
use winapi::um::winnt::{SID, PSID, SID_AND_ATTRIBUTES};
use core::mem::{size_of, align_of};



/// ### Arguments
/// *   `cbs`       the allocation to validate against
/// *   `ptr`       a ptr into `cbs` (can be null if min_count == 0)
/// *   `min_count` the minimum number of `U` *elements* that `ptr` should be valid for
/// *   Returns:    the number of *bytes* available after `ptr`
#[track_caller] pub fn assert_valid_ptr_bytes<T, U, A: Alloc + Free + Stateless>(cbs: &CBoxSized<T, A>, ptr: *const U, min_count: usize) -> usize {
    if ptr.is_null() && min_count == 0 { return 0 }
    let pbegin = cbs.as_ptr() as usize;
    let pend = pbegin + cbs.bytes(); // shouldn't overflow: contiguous alloc
    let pbegin = pbegin + size_of::<T>().min(cbs.bytes()); // ensure `ptr` trails after the header
    let ptr = ptr as usize;
    assert!(ptr % align_of::<U>() == 0, "ptr was not aligned");
    assert!(pbegin <= ptr, "ptr not part of `cbs` allocation");
    assert!(ptr < pend, "ptr not part of `cbs` allocation");
    let bytes_avail = pend - ptr; // shouldn't underflow: pend > ptr
    let count_avail = bytes_avail / size_of::<U>();
    assert!(min_count <= count_avail, "ptr valid for at most {count_avail} elements, but was required to be valid for {min_count} elements");
    bytes_avail
}

#[track_caller] pub fn assert_valid_sid_or_null<T, A: Alloc + Free + Stateless>(cbs: &CBoxSized<T, A>, sid: PSID) {
    if sid.is_null() { return }
    assert_valid_sid(cbs, sid)
}

#[track_caller] pub fn assert_valid_sid<T, A: Alloc + Free + Stateless>(cbs: &CBoxSized<T, A>, sid: PSID) {
    let p = cbs.as_ptr() as usize;
    let pend = p + cbs.bytes(); // shouldn't be possible for this to overflow since p .. p+bytes is a contiguous allocation
    let psid = sid as usize;
    assert!(psid % align_of::<SID>() == 0,  "sid was expected to have proper alignment");
    assert!(p + size_of::<T>() <= psid,     "sid was expected to trail the header in the same buffer");
    assert!(psid <= pend,                   "sid was expected to trail the header in the same buffer");
    let sid_bytes = pend - psid; // shouldn't be possible for this to underflow as pend >= psid
    let _validate_sid = unsafe { sid::Ptr::from_raw(sid.cast(), sid_bytes) }.expect("sid was truncated or otherwise invalid");
}

#[track_caller] pub fn assert_valid_saa<T, A: Alloc + Free + Stateless>(cbs: &CBoxSized<T, A>, saa: SID_AND_ATTRIBUTES) {
    assert_valid_sid(cbs, saa.Sid);
    let _ = saa.Attributes; // presumed valid
}

/// ### Safety
/// `ptr`/`count` are validated against `cbs` with provenance fixing - however, this is undefined behavior
/// unless `U` is valid for the present bit patterns, and the relevant bytes of `cbs` are initialized.
#[track_caller] pub unsafe fn assert_valid_after_header_slice<T, U, A: Alloc + Free + Stateless>(cbs: &CBoxSized<T, A>, ptr: *const U, count: impl TryInto<usize>, allow_overlap_header: bool) -> &[U] {
    let count = count.try_into().map_err(|_| ()).expect("slice count was too large to fit into same buffer");
    if ptr.is_null() {
        assert!(count == 0, "slice pointer was null with nonzero count");
        return &[]
    }

    let ptr = provenance_addr(cbs.as_ptr(), ptr); // `ptr` was likely derived from a trailing `[T; ANYSIZE_ARRAY]` such as found in `TOKEN_GROUPS`, `TOKEN_PRIVILEGES`, etc.
    let p = cbs.as_ptr() as usize;
    let pend = p + cbs.bytes(); // shouldn't be possible for this to overflow since p .. p+bytes is a contiguous allocation
    let pslice = ptr as usize;
    assert!(pslice % align_of::<U>() == 0,                          "slice was expected to have proper alignment");
    assert!(allow_overlap_header || p + size_of::<T>() <= pslice,   "slice was expected to trail header in the same buffer, but it started in the header");
    assert!(pslice <= pend,                                         "slice was expected to trail header in the same buffer, but it started after the end of the buffer");
    let slice_bytes_needed = count.checked_mul(size_of::<U>()).expect("slice size cannot fit in memory");
    let slice_bytes_available = pend - pslice; // shouldn't be possible for this to underflow as pend >= pslice
    assert!(slice_bytes_available >= slice_bytes_needed, "slice was expected to trail header in the same buffer");

    unsafe { core::slice::from_raw_parts(ptr, count) }
}
