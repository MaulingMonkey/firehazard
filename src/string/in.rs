#[doc(alias = "LPCSTR")]
/// ≈ \[in\] LPCSTR ← [abistr::CStrNonNull]\<[u8]\> | [CStr](core::ffi::CStr)\[[ing](alloc::ffi::CString)\]
///
pub trait InNarrow : In<u8> + NonNull {}
impl<T: In<u8> + NonNull + ?Sized> InNarrow for T {}



#[doc(alias = "LPCWSTR")]
/// ≈ \[in\] LPCWSTR ← [abistr::CStrNonNull]\<[u16]\> | [str] | [String](alloc::string::String) | [OsStr](std::ffi::OsStr)\[[ing](std::ffi::OsString)\] | [Path](std::path::Path)\[[Buf](std::path::PathBuf)\]
///
pub trait InWide : In<u16> + NonNull {}
impl<T: In<u16> + NonNull + ?Sized> InWide for T {}



#[doc(alias = "LPCSTR")]
/// ≈ \[in, optional\] LPCSTR ← [abistr::CStrNonNull]\<[u8]\> | [CStr](core::ffi::CStr)\[[ing](alloc::ffi::CString)\]
///
pub trait InOptionalNarrow : In<u8> {}
impl<T: In<u8> + ?Sized> InOptionalNarrow for T {}



#[doc(alias = "LPCWSTR")]
/// ≈ \[in, optional\] LPCWSTR ← [abistr::CStrNonNull]\<[u16]\> | [str] | [String](alloc::string::String) | [OsStr](std::ffi::OsStr)\[[ing](std::ffi::OsString)\] | [Path](std::path::Path)\[[Buf](std::path::PathBuf)\]
///
pub trait InOptionalWide : In<u16> {}
impl<T: In<u16> + ?Sized> InOptionalWide for T {}



pub trait NonNull {}
#[cfg(all())]   impl<T: NonNull + ?Sized> NonNull for &'_ T     {}
//cfg(all())]   impl NonNull for abistr::CStrPtr<'_, u8 >       {}
//cfg(all())]   impl NonNull for abistr::CStrPtr<'_, u16>       {}
#[cfg(all())]   impl NonNull for abistr::CStrNonNull<'_, u8 >   {}
#[cfg(all())]   impl NonNull for abistr::CStrNonNull<'_, u16>   {}
#[cfg(all())]   impl NonNull for core::ffi::CStr                {}
#[cfg(alloc)]   impl NonNull for alloc::ffi::CString            {}
#[cfg(all())]   impl NonNull for str                            {}
#[cfg(alloc)]   impl NonNull for alloc::string::String          {}
#[cfg(std)]     impl NonNull for std::ffi::OsStr                {}
#[cfg(std)]     impl NonNull for std::ffi::OsString             {}
#[cfg(std)]     impl NonNull for std::path::Path                {}
#[cfg(std)]     impl NonNull for std::path::PathBuf             {}



/// [InNarrow] | [InWide]
pub trait In<U: Unit> {
    /// Attempt to borrow `self` as a `\0`-terminated, non-null, UTF16ish C-string.
    /// Likely to return [`Err`] if the original string type isn't UTF16ish, or isn't `\0`-terminated.
    #[inline(always)] fn try_as_cstr(&self) -> Result<abistr::CStrPtr<U>, ()> {
        Ok(string::cstrnn_to_ptr(self.try_as_cstrnn()?))
    }

    /// Attempt to borrow `self` as a `\0`-terminated, non-null, UTF16ish C-string.
    /// Likely to return [`Err`] if the original string type isn't UTF16ish, or isn't `\0`-terminated.
    #[inline(always)] fn try_as_cstrnn(&self) -> Result<abistr::CStrNonNull<U>, ()> {
        Err(())
    }

    /// Likely to return [`Err`] if the original string type isn't UTF16ish, or isn't `\0`-terminated.
    #[inline(always)] fn try_as_units(&self) -> Result<&[U], ()> {
        Ok(self.try_as_cstr()?.to_units())
    }

    /// Get the length in units (not including terminal `\0`)
    #[inline(always)] fn len(&self) -> usize {
        self.try_as_units().map_or(!0, |u| u.len())
    }

    /// Write out to `buffer` (not including terminal `\0`)
    fn try_copy_to_buffer(&self, buffer: &mut [U]) -> firehazard::Result<usize> {
        let src = self.try_as_units().map_err(|_| ERROR::CALL_NOT_IMPLEMENTED)?;
        let n = src.len();
        let dst = buffer.get_mut(..n).ok_or(ERROR::BUFFER_TOO_SMALL)?;
        dst.copy_from_slice(src);
        Ok(n)
    }
}

impl<U: Unit, T: In<U> + ?Sized> In<U> for &'_ T {
    #[inline(always)] fn try_as_cstr(&self) -> Result<abistr::CStrPtr<U>, ()> { (&**self).try_as_cstr() }
    #[inline(always)] fn try_as_cstrnn(&self) -> Result<abistr::CStrNonNull<U>, ()> { (&**self).try_as_cstrnn() }
    #[inline(always)] fn try_as_units(&self) -> Result<&[U], ()> { (&**self).try_as_units() }
    #[inline(always)] fn len(&self) -> usize { (&**self).len() }
    #[inline(always)] fn try_copy_to_buffer(&self, buffer: &mut [U]) -> firehazard::Result<usize> { (&**self).try_copy_to_buffer(buffer) }
}


impl In<u8> for () {
    #[inline(always)] fn try_as_cstr(&self) -> Result<abistr::CStrPtr<u8>, ()> { Ok(abistr::CStrPtr::NULL) }
}

impl In<u16> for () {
    #[inline(always)] fn try_as_cstr(&self) -> Result<abistr::CStrPtr<u16>, ()> { Ok(abistr::CStrPtr::NULL) }
}

impl In<u8> for abistr::CStrPtr<'_, u8> {
    #[inline(always)] fn try_as_cstr(&self) -> Result<abistr::CStrPtr<u8>, ()> { Ok(*self) }
}

impl In<u16> for abistr::CStrPtr<'_, u16> {
    #[inline(always)] fn try_as_cstr(&self) -> Result<abistr::CStrPtr<u16>, ()> { Ok(*self) }
}

impl In<u8> for abistr::CStrNonNull<'_, u8> {
    #[inline(always)] fn try_as_cstrnn(&self) -> Result<abistr::CStrNonNull<u8>, ()> { Ok(*self) }
}

impl In<u16> for abistr::CStrNonNull<'_, u16> {
    #[inline(always)] fn try_as_cstrnn(&self) -> Result<abistr::CStrNonNull<u16>, ()> { Ok(*self) }
}

impl In<u8> for core::ffi::CStr {
    #[inline(always)] fn try_as_cstrnn(&self) -> Result<abistr::CStrNonNull<u8>, ()> { Ok(unsafe { abistr::CStrNonNull::<u8>::from_ptr_unchecked_unbounded(self.as_ptr().cast()) }) }
    // optimizations to take advantage of the fact that std::ffi::CStr[ing] is currently a fat pointer:
    #[inline(always)] fn try_as_units(&self) -> Result<&[u8], ()> { Ok(self.to_bytes()) }
    #[inline(always)] fn len(&self) -> usize { self.count_bytes() }
}

#[cfg(alloc)] impl In<u8> for alloc::ffi::CString {
    #[inline(always)] fn try_as_cstrnn(&self) -> Result<abistr::CStrNonNull<u8>, ()> { Ok(unsafe { abistr::CStrNonNull::<u8>::from_ptr_unchecked_unbounded(self.as_ptr().cast()) }) }
    // optimizations to take advantage of the fact that std::ffi::CStr[ing] is currently a fat pointer:
    #[inline(always)] fn try_as_units(&self) -> Result<&[u8], ()> { Ok(self.to_bytes()) } // likely O(1) instead of O(n), but subject to change
    #[inline(always)] fn len(&self) -> usize { self.count_bytes() } // likely O(1) instead of O(n), but subject to change
}

impl In<u16> for str {
    fn len(&self) -> usize { self.encode_utf16().count() }
    fn try_copy_to_buffer(&self, buffer: &mut [u16]) -> firehazard::Result<usize> { try_copy_to_buffer_from_units(self.encode_utf16(), buffer) }
}

#[cfg(alloc)] impl In<u16> for alloc::string::String {
    fn len(&self) -> usize { self.encode_utf16().count() }
    fn try_copy_to_buffer(&self, buffer: &mut [u16]) -> firehazard::Result<usize> { try_copy_to_buffer_from_units(self.encode_utf16(), buffer) }
}

#[cfg(std)] impl In<u16> for std::ffi::OsStr {
    fn len(&self) -> usize { std::os::windows::ffi::OsStrExt::encode_wide(self).count() }
    fn try_copy_to_buffer(&self, buffer: &mut [u16]) -> firehazard::Result<usize> { try_copy_to_buffer_from_units(std::os::windows::ffi::OsStrExt::encode_wide(self), buffer) }
}

#[cfg(std)] impl In<u16> for std::ffi::OsString {
    fn len(&self) -> usize { std::os::windows::ffi::OsStrExt::encode_wide(&**self).count() }
    fn try_copy_to_buffer(&self, buffer: &mut [u16]) -> firehazard::Result<usize> { try_copy_to_buffer_from_units(std::os::windows::ffi::OsStrExt::encode_wide(&**self), buffer) }
}

#[cfg(std)] impl In<u16> for std::path::Path {
    fn len(&self) -> usize { In::len(self.as_os_str()) }
    fn try_copy_to_buffer(&self, buffer: &mut [u16]) -> firehazard::Result<usize> { In::try_copy_to_buffer(self.as_os_str(), buffer) }
}

#[cfg(std)] impl In<u16> for std::path::PathBuf {
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
