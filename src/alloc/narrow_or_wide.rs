use crate::prelude::*;



/// A `\0`-terminated, non-null, narrow or (preferrably) wide string.
/// Typically only used as an implementation detail when a Win32 function with `*{A,W}` variants takes a single string parameter.
///
#[derive(Clone, Copy, Debug)] pub enum NarrowOrWideCStrNonNull<'a> { Narrow(abistr::CStrNonNull<'a, u8>), Wide(abistr::CStrNonNull<'a, u16>) }

impl<'a>                                                    From<abistr::CStrNonNull<'a, u8 >   > for NarrowOrWideCStrNonNull<'a> { #[inline(always)] fn from(cstr: abistr::CStrNonNull<'a, u8 >  ) -> Self { Self::Narrow(cstr) }                                                                            }
impl<'a>                                                    From<abistr::CStrNonNull<'a, u16>   > for NarrowOrWideCStrNonNull<'a> { #[inline(always)] fn from(cstr: abistr::CStrNonNull<'a, u16>  ) -> Self { Self::Wide(cstr) }                                                                              }
impl<'a>                                                    From<&'a core::ffi::CStr            > for NarrowOrWideCStrNonNull<'a> { #[inline(always)] fn from(cstr: &'a core::ffi::CStr           ) -> Self { Self::Narrow(unsafe { abistr::CStrNonNull::from_ptr_unchecked_unbounded(cstr.as_ptr()) }) }     }
#[cfg(alloc)] impl<'a>                                      From<&'a alloc::ffi::CString        > for NarrowOrWideCStrNonNull<'a> { #[inline(always)] fn from(cstr: &'a alloc::ffi::CString       ) -> Self { Self::Narrow(unsafe { abistr::CStrNonNull::from_ptr_unchecked_unbounded(cstr.as_ptr()) }) }     }
impl<'a, A: ialloc::thin::Free + ialloc::meta::Stateless>   From<&'a alloc::CString<u8,  A>     > for NarrowOrWideCStrNonNull<'a> { #[inline(always)] fn from(cstr: &'a alloc::CString<u8,  A>    ) -> Self { Self::Narrow(cstr.as_cstr_nn()) }                                                               }
impl<'a, A: ialloc::thin::Free + ialloc::meta::Stateless>   From<&'a alloc::CString<u16, A>     > for NarrowOrWideCStrNonNull<'a> { #[inline(always)] fn from(cstr: &'a alloc::CString<u16, A>    ) -> Self { Self::Wide(cstr.as_cstr_nn()) }                                                                 }



/// A type which can be borrowed as a [`NarrowOrWideCStrNonNull`]
///
pub trait                                               AsNarrowOrWideCStrNonNull                                   {                   fn as_narrow_or_wide(&self) -> NarrowOrWideCStrNonNull;                                           }
impl<T: AsNarrowOrWideCStrNonNull>                      AsNarrowOrWideCStrNonNull for &'_ T                         { #[inline(always)] fn as_narrow_or_wide(&self) -> NarrowOrWideCStrNonNull { (&**self).as_narrow_or_wide() }  }
impl                                                    AsNarrowOrWideCStrNonNull for abistr::CStrNonNull<'_, u8 >  { #[inline(always)] fn as_narrow_or_wide(&self) -> NarrowOrWideCStrNonNull { (*self).into() }                         }
impl                                                    AsNarrowOrWideCStrNonNull for abistr::CStrNonNull<'_, u16>  { #[inline(always)] fn as_narrow_or_wide(&self) -> NarrowOrWideCStrNonNull { (*self).into() }                         }
impl                                                    AsNarrowOrWideCStrNonNull for core::ffi::CStr               { #[inline(always)] fn as_narrow_or_wide(&self) -> NarrowOrWideCStrNonNull { self.into() }                            }
#[cfg(alloc)] impl                                      AsNarrowOrWideCStrNonNull for alloc::ffi::CString           { #[inline(always)] fn as_narrow_or_wide(&self) -> NarrowOrWideCStrNonNull { self.into() }                            }
impl<A: ialloc::thin::Free + ialloc::meta::Stateless>   AsNarrowOrWideCStrNonNull for alloc::CString<u8,  A>        { #[inline(always)] fn as_narrow_or_wide(&self) -> NarrowOrWideCStrNonNull { self.into() }                            }
impl<A: ialloc::thin::Free + ialloc::meta::Stateless>   AsNarrowOrWideCStrNonNull for alloc::CString<u16, A>        { #[inline(always)] fn as_narrow_or_wide(&self) -> NarrowOrWideCStrNonNull { self.into() }                            }



/// <code>abistr::[CStrNonNull]&lt;[u8]|[u16]&gt;</code> |
/// <code>&amp;[str]</code> |
/// <code>&amp;[String](alloc::string::String)</code> |
/// <code>&amp;[std::ffi::CStr]\[[ing](std::ffi::CString)\]</code> |
/// <code>&amp;[std::ffi::OsStr]\[[ing](std::ffi::OsString)\]</code> |
/// <code>&amp[std::path::Path]\[[Buf](std::path::PathBuf)\]</code>
///
/// Accepts something that's convertable to a `\0`-terminated, non-null, narrow or (preferrably) wide string.
///
/// Typically used as an `impl` argument for wrappers around Win32 functions that accept a single string parameter, and have `*{A,W}` variants.
///
/// ### Wide Infallible Conversions
/// *   <code>abistr::[CStrNonNull]&lt;u16&gt;</code> (see e.g. <code>abistr::[cstr16\!](abistr::cstr16)("...")</code>)
///
/// ### Wide Fallible Conversions (must allocate, will reject interior `\0`s)
/// *   <code>&amp;[str]</code>
/// *   <code>&amp;[String]</code>
/// *   <code>&amp;[std::ffi::OsStr]\[[ing](std::ffi::OsString)\]</code>
/// *   <code>&amp[std::path::Path]\[[Buf](std::path::PathBuf)\]</code>
///
/// ### Narrow Infallible Conversions (but beware of [Mojibake](https://en.wikipedia.org/wiki/Mojibake)!)
/// *   <code>abistr::[CStrNonNull]&lt;u8&gt;</code> (see e.g. <code>abistr::[cstr8\!](abistr::cstr8)("...")</code>)
/// *   <code>&amp;[std::ffi::CStr]\[[ing](std::ffi::CString)\]</code>
///
pub trait TryIntoAsNarrowOrWideCStrNonNull {
    type Target : AsNarrowOrWideCStrNonNull;
    fn try_into(self) -> firehazard::Result<Self::Target>;
}

impl<'a, T: AsNarrowOrWideCStrNonNull> TryIntoAsNarrowOrWideCStrNonNull for T {
    type Target = Self;
    fn try_into(self) -> firehazard::Result<Self::Target> { Ok(self) }
}

impl                TryIntoAsNarrowOrWideCStrNonNull for &'_ str                    { type Target = CString<u16>; fn try_into(self) -> firehazard::Result<Self::Target> { to_cstring(self.encode_utf16()) } }
#[cfg(alloc)] impl  TryIntoAsNarrowOrWideCStrNonNull for &'_ alloc::string::String  { type Target = CString<u16>; fn try_into(self) -> firehazard::Result<Self::Target> { to_cstring(self.encode_utf16()) } }
#[cfg(alloc)] impl  TryIntoAsNarrowOrWideCStrNonNull for     alloc::string::String  { type Target = CString<u16>; fn try_into(self) -> firehazard::Result<Self::Target> { to_cstring(self.encode_utf16()) } }
#[cfg(std)] impl    TryIntoAsNarrowOrWideCStrNonNull for &'_ std::ffi::OsStr        { type Target = CString<u16>; fn try_into(self) -> firehazard::Result<Self::Target> { to_cstring(std::os::windows::ffi::OsStrExt::encode_wide(   self)) } }
#[cfg(std)] impl    TryIntoAsNarrowOrWideCStrNonNull for &'_ std::ffi::OsString     { type Target = CString<u16>; fn try_into(self) -> firehazard::Result<Self::Target> { to_cstring(std::os::windows::ffi::OsStrExt::encode_wide(&**self)) } }
#[cfg(std)] impl    TryIntoAsNarrowOrWideCStrNonNull for     std::ffi::OsString     { type Target = CString<u16>; fn try_into(self) -> firehazard::Result<Self::Target> { to_cstring(std::os::windows::ffi::OsStrExt::encode_wide(&* self)) } }
#[cfg(std)] impl    TryIntoAsNarrowOrWideCStrNonNull for &'_ std::path::Path        { type Target = CString<u16>; fn try_into(self) -> firehazard::Result<Self::Target> { to_cstring(std::os::windows::ffi::OsStrExt::encode_wide(self.as_os_str())) } }
#[cfg(std)] impl    TryIntoAsNarrowOrWideCStrNonNull for &'_ std::path::PathBuf     { type Target = CString<u16>; fn try_into(self) -> firehazard::Result<Self::Target> { to_cstring(std::os::windows::ffi::OsStrExt::encode_wide(self.as_os_str())) } }
#[cfg(std)] impl    TryIntoAsNarrowOrWideCStrNonNull for     std::path::PathBuf     { type Target = CString<u16>; fn try_into(self) -> firehazard::Result<Self::Target> { to_cstring(std::os::windows::ffi::OsStrExt::encode_wide(self.as_os_str())) } }

// XXX: ZSTs shouldn't actually be possible in practice as `\0`-terminated strings always have at least 1 byte... should I introduce PanicZst?
type Allocator = ialloc::allocator::adapt::DangleZst<ialloc::allocator::win32::ProcessHeap>;
type CString<C> = alloc::CString<C, Allocator>;
fn to_cstring<C: Copy + bytemuck::Zeroable + PartialEq>(iter: impl Iterator<Item = C>) -> firehazard::Result<CString<C>> {
    let nul = C::zeroed();

    // XXX: am I sure I don't want to enable feature = "global_oom_handling"?
    let mut buffer = ialloc::vec::AVec::<C, Allocator>::new();
    buffer.try_reserve(iter.size_hint().0 + 1).map_err(|_| ERROR_NOT_ENOUGH_MEMORY)?; // XXX
    for ch in iter.chain(Some(nul)) {
        if buffer.len() == buffer.capacity() {
            buffer.try_reserve(1).map_err(|_| ERROR_NOT_ENOUGH_MEMORY)?; // XXX
        }
        buffer.push_within_capacity(ch).map_err(|_| ERROR_NOT_ENOUGH_MEMORY)?; // XXX
    }

    if buffer[..buffer.len()-1].contains(&nul) { return Err(firehazard::Error(ERROR_ILLEGAL_CHARACTER)) }
    let (ptr, _len, _cap) = buffer.into_raw_parts();
    Ok(unsafe { CString::from_raw_nn(ptr) })
}
