pub(crate) fn convert_to_cstrnn<const LIMIT : usize, S: string::In, R>(
    s: S,
    with_cstrnn: impl FnOnce(abistr::CStrNonNull<S::Unit>) -> R
) -> firehazard::Result<R> {
    convert_to_cstrnn_impl::<LIMIT, false, S, R>(s, with_cstrnn)
}

pub(crate) fn convert_to_cstrnn_lossy<const LIMIT : usize, S: string::In, R>(
    s: S,
    with_cstrnn: impl FnOnce(abistr::CStrNonNull<S::Unit>) -> R
) -> firehazard::Result<R> {
    convert_to_cstrnn_impl::<LIMIT, true, S, R>(s, with_cstrnn)
}

fn convert_to_cstrnn_impl<const LIMIT : usize, const LOSSY : bool, S: string::In, R>(
    s: S,
    with_cstrnn: impl FnOnce(abistr::CStrNonNull<S::Unit>) -> R
) -> firehazard::Result<R> {
    if let Ok(cstrnn) = s.try_as_cstrnn() { return Ok(with_cstrnn(cstrnn)) }
    // else `s` is not a native string type

    let s_len = s.len();
    let Some(s_len_nul) = s_len.checked_add(1) else { return Err(ERROR::BUFFER_OVERFLOW.into()) };

    if s_len_nul <= LIMIT {
        return stack::<LIMIT, LOSSY, S, R>(s, with_cstrnn);
    } else {
        return heap::<LOSSY, S, R>(s, s_len_nul, with_cstrnn);
    }

    #[inline(never)] // I do not want to pay the stack cost unless we're using the stack
    fn stack<const LIMIT : usize, const LOSSY: bool, S: string::In, R>(
        s: S,
        with_cstrnn: impl FnOnce(abistr::CStrNonNull<S::Unit>) -> R
    ) -> firehazard::Result<R> {
        let mut stack : [S::Unit; LIMIT] = bytemuck::Zeroable::zeroed();
        common::<LOSSY, S, R>(s, &mut stack, with_cstrnn)
    }

    fn heap<const LOSSY: bool, S: string::In, R>(
        s:              S,
        s_len_nul:      usize,
        with_cstrnn:    impl FnOnce(abistr::CStrNonNull<S::Unit>) -> R
    ) -> firehazard::Result<R> {
        // ialloc::allocator::alloc::Global requires cfg(alloc), and has extra logic for handling extra alignment, which we don't need
        type Allocator = ialloc::allocator::adapt::DangleZst<ialloc::allocator::win32::ProcessHeap>;
        let mut heap = ialloc::vec::AVec::<S::Unit, Allocator>::new();
        heap.try_reserve(s_len_nul).map_err(|_| ERROR::NOT_ENOUGH_MEMORY)?;
        let nul = S::Unit::default();
        while heap.push_within_capacity(nul).is_ok() {}
        common::<LOSSY, S, R>(s, &mut heap, with_cstrnn)
    }

    fn common<const LOSSY: bool, S: string::In, R>(
        s:              S,
        buffer:         &mut [S::Unit],
        with_cstrnn:    impl FnOnce(abistr::CStrNonNull<S::Unit>) -> R,
    ) -> firehazard::Result<R> {
        if let [buffer @ .., _nul] = buffer {
            s.try_copy_to_buffer(buffer)?;
            let nul = S::Unit::default();
            if let Some(replacement) = LOSSY.then(|| S::Unit::try_from(SYMBOL_FOR_NULL_CHAR).ok()).flatten() {
                for b in buffer.iter_mut() {
                    if *b == nul { *b = replacement; }
                }
            } else if buffer.contains(&nul) {
                Err(ERROR::ILLEGAL_CHARACTER)?
            }
        } else {
            Err(ERROR::NOT_ENOUGH_MEMORY)?;
        }
        Ok(with_cstrnn(unsafe { CStrNonNull::from_units_with_nul_unchecked(buffer) }))
    }
}
