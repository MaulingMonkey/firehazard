pub(crate) fn convert_to_cstrnn<const LIMIT : usize, U: string::Unit, R>(
    s:      impl string::In<U> + string::NonNull,
    with_s: impl FnOnce(abistr::CStrNonNull<U>) -> R
) -> firehazard::Result<R> {
    match s.try_as_cstrnn() {
        Ok(s)   => Ok(with_s(s)),
        Err(()) => convert_to_impl::<LIMIT, false, U, R>(s, with_s),
    }
}

pub(crate) fn convert_to_cstrnn_lossy<const LIMIT : usize, U: string::Unit, R>(
    s:      impl string::In<U> + string::NonNull,
    with_s: impl FnOnce(abistr::CStrNonNull<U>) -> R
) -> firehazard::Result<R> {
    match s.try_as_cstrnn() {
        Ok(s)   => Ok(with_s(s)),
        Err(()) => convert_to_impl::<LIMIT, true, U, R>(s, with_s)
    }
}

pub(crate) fn convert_to_cstr<const LIMIT : usize, U: string::Unit, R>(
    s:      impl string::In<U>,
    with_s: impl FnOnce(abistr::CStrPtr<U>) -> R
) -> firehazard::Result<R> {
    match s.try_as_cstr() {
        Ok(s)   => Ok(with_s(s)),
        Err(()) => convert_to_impl::<LIMIT, false, U, R>(s, |s| with_s(cstrnn_to_ptr(s))),
    }
}

#[allow(dead_code)]
pub(crate) fn convert_to_cstr_lossy<const LIMIT : usize, U: string::Unit, R>(
    s:      impl string::In<U>,
    with_s: impl FnOnce(abistr::CStrPtr<U>) -> R
) -> firehazard::Result<R> {
    match s.try_as_cstr() {
        Ok(s)   => Ok(with_s(s)),
        Err(()) => convert_to_impl::<LIMIT, true, U, R>(s, |s| with_s(cstrnn_to_ptr(s))),
    }
}



fn cstrnn_to_ptr<U: abistr::Unit>(s: abistr::CStrNonNull<U>) -> abistr::CStrPtr<U> {
    // TODO: upstream as a From conversion into abistr
    unsafe { abistr::CStrPtr::from_units_with_nul_unchecked(s.to_units_with_nul()) }
}

fn convert_to_impl<const LIMIT : usize, const LOSSY : bool, U: string::Unit, R>(
    s:              impl string::In<U>,
    with_s:         impl FnOnce(abistr::CStrNonNull<U>) -> R
) -> firehazard::Result<R> {
    let s_len = s.len();
    let Some(s_len_nul) = s_len.checked_add(1) else { return Err(ERROR::BUFFER_OVERFLOW.into()) };

    if s_len_nul <= LIMIT {
        return stack::<LIMIT, LOSSY, U, R>(s, s_len_nul, with_s);
    } else {
        return heap::<LOSSY, U, R>(s, s_len_nul, with_s);
    }

    #[inline(never)] // I do not want to pay the stack cost unless we're using the stack
    fn stack<const LIMIT : usize, const LOSSY: bool, U: string::Unit, R>(
        s:              impl string::In<U>,
        s_len_nul:      usize,
        with_s:         impl FnOnce(abistr::CStrNonNull<U>) -> R
    ) -> firehazard::Result<R> {
        let mut stack : [U; LIMIT] = bytemuck::Zeroable::zeroed();
        common::<LOSSY, U, R>(s, &mut stack[..s_len_nul], with_s)
    }

    fn heap<const LOSSY: bool, U: string::Unit, R>(
        s:              impl string::In<U>,
        s_len_nul:      usize,
        with_s:         impl FnOnce(abistr::CStrNonNull<U>) -> R
    ) -> firehazard::Result<R> {
        // ialloc::allocator::alloc::Global requires cfg(alloc), and has extra logic for handling extra alignment, which we don't need
        type Allocator = ialloc::allocator::adapt::DangleZst<ialloc::allocator::win32::ProcessHeap>;
        let mut heap = ialloc::vec::AVec::<U, Allocator>::new();
        heap.try_reserve(s_len_nul).map_err(|_| ERROR::NOT_ENOUGH_MEMORY)?;
        let nul = U::default();
        while heap.push_within_capacity(nul).is_ok() {}
        common::<LOSSY, U, R>(s, &mut heap, with_s)
    }

    fn common<const LOSSY: bool, U: string::Unit, R>(
        s:              impl string::In<U>,
        buffer:         &mut [U],
        with_s:         impl FnOnce(abistr::CStrNonNull<U>) -> R,
    ) -> firehazard::Result<R> {
        if let [buffer @ .., _nul] = buffer {
            s.try_copy_to_buffer(buffer)?;
            let nul = U::default();
            if let Some(replacement) = LOSSY.then(|| U::try_from(SYMBOL_FOR_NULL_CHAR).ok()).flatten() {
                for b in buffer.iter_mut() {
                    if *b == nul { *b = replacement; }
                }
            } else if buffer.contains(&nul) {
                Err(ERROR::ILLEGAL_CHARACTER)?
            }
        } else {
            Err(ERROR::NOT_ENOUGH_MEMORY)?;
        }
        Ok(with_s(unsafe { CStrNonNull::from_units_with_nul_unchecked(buffer) }))
    }
}
