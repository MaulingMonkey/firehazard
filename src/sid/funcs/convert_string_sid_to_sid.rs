use crate::*;
use crate::sid::Box as SidBox;

use abistr::{AsCStr, TryIntoAsCStr};

use winapi::shared::sddl::{ConvertStringSidToSidW, ConvertStringSidToSidA};
use winapi::shared::winerror::*;

use core::ptr::null_mut;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/sddl/nf-sddl-convertstringsidtosida)\]
/// ConvertStringSidToSidA - convert e.g. `"S-1-0-0"` to [`sid::Box`]
///
/// ### Examples
/// ```
/// # use win32_security_playground::*;
/// # #[cfg(feature = "std")] {
/// let err = convert_string_sid_to_sid_a("XYZ").unwrap_err();
/// let sid = convert_string_sid_to_sid_a("S-1-0-0").unwrap();
/// let sid = convert_string_sid_to_sid_a("S-1-16-0").unwrap();
/// # }
/// let sid = convert_string_sid_to_sid_a(abistr::cstr!("S-1-16-0")).unwrap();
/// ```
///
/// ### See Also
/// *   [sid!] for compile-time validated error-free [`sid::Ptr`]s.
pub fn convert_string_sid_to_sid_a(s: impl TryIntoAsCStr) -> Result<SidBox<alloc::LocalAllocFree>, Error> {
    let s = s.try_into().map_err(|_| Error(E_STRING_NOT_NULL_TERMINATED as _))?;
    let mut sid = null_mut();
    Error::get_last_if(0 == unsafe { ConvertStringSidToSidA(s.as_cstr(), &mut sid) })?;
    unsafe { SidBox::from_raw(sid.cast()) }.ok_or(Error(ERROR_INVALID_SID))
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/sddl/nf-sddl-convertstringsidtosidw)\]
/// ConvertStringSidToSidW - convert e.g. `"S-1-0-0"` to [`sid::Box`]
///
/// ### Examples
/// ```
/// # use win32_security_playground::*;
/// use abistr::cstr16;
/// let err = convert_string_sid_to_sid_w(cstr16!("XYZ")).unwrap_err();
/// let sid = convert_string_sid_to_sid_w(cstr16!("S-1-0-0")).unwrap();
/// let sid = convert_string_sid_to_sid_w(cstr16!("S-1-16-0")).unwrap();
/// ```
///
/// ### See Also
/// *   [sid!] for compile-time validated error-free [`sid::Ptr`]s.
pub fn convert_string_sid_to_sid_w(s: impl TryIntoAsCStr<u16>) -> Result<SidBox<alloc::LocalAllocFree>, Error> {
    let s = s.try_into().map_err(|_| Error(E_STRING_NOT_NULL_TERMINATED as _))?;
    let mut sid = null_mut();
    Error::get_last_if(0 == unsafe { ConvertStringSidToSidW(s.as_cstr(), &mut sid) })?;
    unsafe { SidBox::from_raw(sid.cast()) }.ok_or(Error(ERROR_INVALID_SID))
}
