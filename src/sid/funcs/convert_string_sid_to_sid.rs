#[doc(alias = "ConvertStringSidToSidA")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/sddl/nf-sddl-convertstringsidtosida)\]
/// ConvertStringSidToSidA - convert e.g. `"S-1-0-0"` to [`sid::Box`]
///
/// ### Examples
/// ```
/// # use firehazard::*;
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
///
pub fn convert_string_sid_to_sid_a(s: impl TryIntoAsCStr) -> firehazard::Result<sid::Box<alloc::LocalAllocFree>> {
    let s = s.try_into()?;
    let mut sid = null_mut();
    firehazard::Error::get_last_if(0 == unsafe { winapi::shared::sddl::ConvertStringSidToSidA(s.as_cstr(), &mut sid) })?;
    Ok(unsafe { sid::Box::from_raw(sid.cast()) }.ok_or(ERROR_INVALID_SID)?)
}



#[doc(alias = "ConvertStringSidToSid")]
#[doc(alias = "ConvertStringSidToSidW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/sddl/nf-sddl-convertstringsidtosidw)\]
/// ConvertStringSidToSidW - convert e.g. `"S-1-0-0"` to [`sid::Box`]
///
/// ### Examples
/// ```
/// # use firehazard::*;
/// use abistr::cstr16;
/// let err = convert_string_sid_to_sid_w(cstr16!("XYZ")).unwrap_err();
/// let sid = convert_string_sid_to_sid_w(cstr16!("S-1-0-0")).unwrap();
/// let sid = convert_string_sid_to_sid_w(cstr16!("S-1-16-0")).unwrap();
/// ```
///
/// ### See Also
/// *   [sid!] for compile-time validated error-free [`sid::Ptr`]s.
///
pub fn convert_string_sid_to_sid_w(s: impl TryIntoAsCStr<u16>) -> firehazard::Result<sid::Box<alloc::LocalAllocFree>> {
    let s = s.try_into()?;
    let mut sid = null_mut();
    firehazard::Error::get_last_if(0 == unsafe { winapi::shared::sddl::ConvertStringSidToSidW(s.as_cstr(), &mut sid) })?;
    Ok(unsafe { sid::Box::from_raw(sid.cast()) }.ok_or(ERROR_INVALID_SID)?)
}
