#[doc(alias = "LookupPrivilegeValueA")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-lookupprivilegevaluea)\]
/// LookupPrivilegeValueA
///
/// Lookup a [`privilege::Luid`](crate::privilege::Luid) by it's string identifier.
///
/// ### Examples
/// ```
/// # use abistr::*;
/// # use firehazard::*;
/// # use winapi::shared::winerror::*;
/// // unit tests aren't run in any kind of sandbox, so this should work:
/// let p = lookup_privilege_value_a(cstr!("SeChangeNotifyPrivilege"));
/// p.expect("SeChangeNotifyPrivilege should be a valid privilege");
///
/// let r = lookup_privilege_value_a(cstr!("not a valid privilege name"));
/// assert_eq!(r.unwrap_err(), ERROR_NO_SUCH_PRIVILEGE);
/// ```
///
/// ### Errors
/// *   `ERROR_NO_SUCH_PRIVILEGE`   if `name` doesn't name a known privilege in this version of Windows
/// *   `ERROR_INVALID_HANDLE`      on some permission lookup errors (e.g. if the current process's token was restricted, and excluded [`sid::builtin::alias::USERS`](crate::sid::builtin::alias::USERS))
///
pub fn lookup_privilege_value_a(name: impl abistr::AsCStr) -> Result<crate::privilege::Luid, crate::Error> {
    use crate::*;
    let name = name.as_cstr();
    let mut luid = Luid::from(0u64);
    Error::get_last_if(0 == unsafe { winapi::um::winbase::LookupPrivilegeValueA(core::ptr::null_mut(), name, &mut luid.0) })?;
    Ok(privilege::Luid(luid))
}



#[doc(alias = "LookupPrivilegeNameA")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-lookupprivilegenamea)\]
/// LookupPrivilegeNameA
///
#[cfg(std)] pub fn lookup_privilege_name_a(mut luid: crate::privilege::Luid) -> Result<std::string::String, crate::Error> {
    use crate::*;
    use winapi::shared::winerror::*;
    use winapi::um::winbase::LookupPrivilegeNameA;
    use std::ptr::null_mut;

    let system_name = null_mut();
    let luid = &mut luid.0.0;
    let mut len = 0;
    Error::get_last_if(0 == unsafe { LookupPrivilegeNameA(system_name, luid, null_mut(), &mut len) }).unerr(ERROR_INSUFFICIENT_BUFFER, ())?;
    let mut buf = std::vec![0u8; usize::from32(len)];
    Error::get_last_if(0 == unsafe { LookupPrivilegeNameA(system_name, luid, buf.as_mut_ptr().cast(), &mut len) })?;
    buf.shrink_to(usize::from32(len)); // on the off chance that len shrunk (if it grew, we would've already returned `Error(ERROR_INSUFFICIENT_BUFFER)`)
    assert!(buf.pop() == Some(b'\0'), "BUG: privilege name was expected to be null terminated");
    Ok(std::string::String::from_utf8(buf).map_err(|_| ERROR_INVALID_DATA)?)
}

#[cfg(not(std))] pub(crate) fn lookup_privilege_name_a(_luid: crate::privilege::Luid) -> Result<&'static str, crate::Error> { Err(crate::Error(winapi::shared::winerror::ERROR_OUTOFMEMORY)) }
