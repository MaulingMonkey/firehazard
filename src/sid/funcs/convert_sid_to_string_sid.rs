/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/sddl/nf-sddl-convertsidtostringsida)\] ConvertSidToStringSidA
pub fn convert_sid_to_string_sid_a(sid: &crate::sid::Value) -> Option<crate::alloc::CString<u8, crate::alloc::LocalAllocFree>> {
    if sid.as_psid().is_null() { return None }
    let mut local_string = core::ptr::null_mut();
    let succeeded = 0 != unsafe { winapi::shared::sddl::ConvertSidToStringSidA(sid.as_psid(), &mut local_string) };
    let local_string = unsafe { crate::alloc::CString::from_raw(local_string.cast()) };
    assert!(succeeded, "ConvertSidToStringSidA");
    Some(local_string)
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/sddl/nf-sddl-convertsidtostringsidw)\] ConvertSidToStringSidW
pub fn convert_sid_to_string_sid_w(sid: &crate::sid::Value) -> Option<crate::alloc::CString<u16, crate::alloc::LocalAllocFree>> {
    if sid.as_psid().is_null() { return None }
    let mut local_string = core::ptr::null_mut();
    let succeeded = 0 != unsafe { winapi::shared::sddl::ConvertSidToStringSidA(sid.as_psid(), &mut local_string) };
    let local_string = unsafe { crate::alloc::CString::from_raw(local_string.cast()) };
    assert!(succeeded, "ConvertSidToStringSidW");
    Some(local_string)
}
