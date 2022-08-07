/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-reverttoself)\] RevertToSelf
///
/// ### Example
/// ```
/// use win32_security_playground::*;
/// // TODO: set/reset/verify thread token
/// revert_to_self().unwrap();
/// ```
pub fn revert_to_self() -> Result<(), crate::error::LastError> {
    let success = 0 != unsafe { winapi::um::securitybaseapi::RevertToSelf() };
    if success { Ok(()) } else { Err(crate::error::LastError::get()) }
}
