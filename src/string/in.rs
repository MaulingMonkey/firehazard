#[doc(alias = "LPCSTR")]
/// ≈ \[in\] LPCSTR ← [abistr::CStrNonNull]\<[u8]\> | [CStr](core::ffi::CStr)\[[ing](alloc::ffi::CString)\]
///
pub trait InNarrow : In<Unit = u8> {}
impl<T: In<Unit = u8> + ?Sized> InNarrow for T {}



#[doc(alias = "LPCWSTR")]
/// ≈ \[in\] LPCWSTR ← [abistr::CStrNonNull]\<[u16]\> | [str] | [String](alloc::string::String) | [OsStr](std::ffi::OsStr)\[[ing](std::ffi::OsString)\] | [Path](std::path::Path)\[[Buf](std::path::PathBuf)\]
///
pub trait InWide : In<Unit = u16> {}
impl<T: In<Unit = u16> + ?Sized> InWide for T {}



/// [InNarrow] | [InWide]
pub trait In {
    type Unit : Unit;

    /// Attempt to borrow `self` as a `\0`-terminated, non-null, UTF16ish C-string.
    /// Likely to return [`Err`] if the original string type isn't UTF16ish, or isn't `\0`-terminated.
    #[inline(always)] fn try_as_cstrnn(&self) -> Result<abistr::CStrNonNull<Self::Unit>, ()> {
        Err(())
    }

    /// Likely to return [`Err`] if the original string type isn't UTF16ish, or isn't `\0`-terminated.
    #[inline(always)] fn try_as_units(&self) -> Result<&[Self::Unit], ()> {
        Ok(self.try_as_cstrnn()?.to_units())
    }

    /// Get the length in units (not including terminal `\0`)
    #[inline(always)] fn len(&self) -> usize {
        self.try_as_units().map_or(!0, |u| u.len())
    }

    /// Write out to `buffer` (not including terminal `\0`)
    fn try_copy_to_buffer(&self, buffer: &mut [Self::Unit]) -> firehazard::Result<usize> {
        let src = self.try_as_units().map_err(|_| ERROR::CALL_NOT_IMPLEMENTED)?;
        let n = src.len();
        let dst = buffer.get_mut(..n).ok_or(ERROR::BUFFER_TOO_SMALL)?;
        dst.copy_from_slice(src);
        Ok(n)
    }
}

impl<T: In + ?Sized> In for &'_ T {
    type Unit = T::Unit;
    #[inline(always)] fn try_as_cstrnn(&self) -> Result<abistr::CStrNonNull<Self::Unit>, ()> { (&**self).try_as_cstrnn() }
    #[inline(always)] fn len(&self) -> usize { (&**self).len() }
    #[inline(always)] fn try_copy_to_buffer(&self, buffer: &mut [Self::Unit]) -> firehazard::Result<usize> { (&**self).try_copy_to_buffer(buffer) }
}

impl In for abistr::CStrNonNull<'_, u8> {
    type Unit = u8;
    #[inline(always)] fn try_as_cstrnn(&self) -> Result<abistr::CStrNonNull<u8>, ()> { Ok(*self) }
}

impl In for abistr::CStrNonNull<'_, u16> {
    type Unit = u16;
    #[inline(always)] fn try_as_cstrnn(&self) -> Result<abistr::CStrNonNull<u16>, ()> { Ok(*self) }
}

impl In for core::ffi::CStr {
    type Unit = u8;
    #[inline(always)] fn try_as_cstrnn(&self) -> Result<abistr::CStrNonNull<u8>, ()> { Ok(unsafe { abistr::CStrNonNull::<u8>::from_ptr_unchecked_unbounded(self.as_ptr().cast()) }) }
    // optimizations to take advantage of the fact that std::ffi::CStr[ing] is currently a fat pointer:
    #[inline(always)] fn try_as_units(&self) -> Result<&[u8], ()> { Ok(self.to_bytes()) }
    #[inline(always)] fn len(&self) -> usize { self.count_bytes() }
}

#[cfg(alloc)] impl In for alloc::ffi::CString {
    type Unit = u8;
    #[inline(always)] fn try_as_cstrnn(&self) -> Result<abistr::CStrNonNull<u8>, ()> { Ok(unsafe { abistr::CStrNonNull::<u8>::from_ptr_unchecked_unbounded(self.as_ptr().cast()) }) }
    // optimizations to take advantage of the fact that std::ffi::CStr[ing] is currently a fat pointer:
    #[inline(always)] fn try_as_units(&self) -> Result<&[u8], ()> { Ok(self.to_bytes()) } // likely O(1) instead of O(n), but subject to change
    #[inline(always)] fn len(&self) -> usize { self.count_bytes() } // likely O(1) instead of O(n), but subject to change
}

impl In for str {
    type Unit = u16;
    fn len(&self) -> usize { self.encode_utf16().count() }
    fn try_copy_to_buffer(&self, buffer: &mut [u16]) -> firehazard::Result<usize> { try_copy_to_buffer_from_units(self.encode_utf16(), buffer) }
}

#[cfg(alloc)] impl In for alloc::string::String {
    type Unit = u16;
    fn len(&self) -> usize { self.encode_utf16().count() }
    fn try_copy_to_buffer(&self, buffer: &mut [u16]) -> firehazard::Result<usize> { try_copy_to_buffer_from_units(self.encode_utf16(), buffer) }
}

#[cfg(std)] impl In for std::ffi::OsStr {
    type Unit = u16;
    fn len(&self) -> usize { std::os::windows::ffi::OsStrExt::encode_wide(self).count() }
    fn try_copy_to_buffer(&self, buffer: &mut [u16]) -> firehazard::Result<usize> { try_copy_to_buffer_from_units(std::os::windows::ffi::OsStrExt::encode_wide(self), buffer) }
}

#[cfg(std)] impl In for std::ffi::OsString {
    type Unit = u16;
    fn len(&self) -> usize { std::os::windows::ffi::OsStrExt::encode_wide(&**self).count() }
    fn try_copy_to_buffer(&self, buffer: &mut [u16]) -> firehazard::Result<usize> { try_copy_to_buffer_from_units(std::os::windows::ffi::OsStrExt::encode_wide(&**self), buffer) }
}

#[cfg(std)] impl In for std::path::Path {
    type Unit = u16;
    fn len(&self) -> usize { In::len(self.as_os_str()) }
    fn try_copy_to_buffer(&self, buffer: &mut [u16]) -> firehazard::Result<usize> { In::try_copy_to_buffer(self.as_os_str(), buffer) }
}

#[cfg(std)] impl In for std::path::PathBuf {
    type Unit = u16;
    fn len(&self) -> usize { In::len(self.as_os_str()) }
    fn try_copy_to_buffer(&self, buffer: &mut [u16]) -> firehazard::Result<usize> { In::try_copy_to_buffer(self.as_os_str(), buffer) }
}



fn try_copy_to_buffer_from_units<U: Unit>(mut src: (impl Clone + Iterator<Item = U>), dst: &mut [U]) -> firehazard::Result<usize> {
    let dst_len = dst.len();
    let mut dst = dst.iter_mut();
    loop {
        match (dst.next(), src.next()) {
            (Some(dst), Some(src)   ) => *dst = src,
            (None,      Some(_)     ) => return Err(ERROR::BUFFER_TOO_SMALL.into()),
            (None,      None        ) => return Ok(dst_len),
            (Some(_st), None        ) => return Ok(dst_len - dst.as_slice().len() - 1),
        }
    }
}
