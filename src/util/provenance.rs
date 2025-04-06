/// Use to fix up pointer provenance.
///
/// If `addr` was derived from a trailing `[T; ANYSIZE_ARRAY]` such as found in `TOKEN_GROUPS`, `TOKEN_PRIVILEGES`, etc.
/// it may have an inappropriately narrowed spatial provenance to `[T; 1]` instead of `[T; count]`.  By contrast,
/// [`CBoxSized::as_ptr`] should return the pointer originally acquired from the underlying OS allocation function,
/// meaning it *should* have the entire CBoxSized allocation within its spatial provenance, so you might pass it to
/// `provenance` like so:
///
/// ```
/// # #[cfg(nope)]
/// # fn example<A: Alloc>(cbs: CBoxSized<TOKEN_GROUPS, A>) {
/// let cbs : CBoxSized<TOKEN_GROUPS, _> = ..;
/// let groups = provenance_addr(cbs.as_ptr(), cbs.Groups);
/// let groups = unsafe { core::slice::from_raw_parts(groups, cbs.GroupCount as _) };
/// # }
/// ```
///
/// See <https://doc.rust-lang.org/std/ptr/index.html#provenance>
pub(crate) fn provenance_addr<P, A>(provenance: *const P, addr: *const A) -> *const A {
    let self_addr = provenance as usize as isize;
    let dest_addr = addr as usize as isize;
    let offset = dest_addr.wrapping_sub(self_addr);
    provenance.cast::<u8>().wrapping_offset(offset).cast()
}

pub(crate) fn provenance_addr_mut<P, A>(provenance: *mut P, addr: *mut A) -> *mut A {
    let self_addr = provenance as usize as isize;
    let dest_addr = addr as usize as isize;
    let offset = dest_addr.wrapping_sub(self_addr);
    provenance.cast::<u8>().wrapping_offset(offset).cast()
}
