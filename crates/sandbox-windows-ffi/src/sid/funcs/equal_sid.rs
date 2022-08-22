/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-equalsid)\]
/// EqualSid
pub fn equal_sid(sid1: crate::sid::Ptr, sid2: crate::sid::Ptr) -> bool {
    0 != unsafe { winapi::um::securitybaseapi::EqualSid(sid1.as_psid(), sid2.as_psid()) }
}
