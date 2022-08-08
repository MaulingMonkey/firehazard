use crate::*;
use crate::error::LastError;

use winapi::um::securitybaseapi::AdjustTokenPrivileges;
use winapi::um::winnt::*;

use std::convert::Infallible;
use std::ptr::null_mut;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-adjusttokenprivileges)\] `AdjustTokenPrivileges(self, FALSE, ...)`<br>
/// Enable only the specified privileges of the token.<br>
/// **Disabled privileges can be re-enabled** - this guards against accidents, not malware!<br>
/// &nbsp;
pub fn adjust_token_privileges_enable_if(token: &token::Handle, mut cond: impl FnMut(privilege::Luid) -> bool) -> Result<(), LastError> {
    adjust_token_privileges_each(token, move |p| {
        let prev = p.attributes & SE_PRIVILEGE_ENABLED;
        p.attributes = if cond(p.luid) { SE_PRIVILEGE_ENABLED } else { 0 };
        p.attributes != prev
    })
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-adjusttokenprivileges)\] `AdjustTokenPrivileges(self, FALSE, ...)`<br>
/// Disable the specified privileges of the token.<br>
/// **Disabled privileges can be re-enabled** - this guards against accidents, not malware!<br>
/// &nbsp;
pub fn adjust_token_privileges_disable_if(token: &token::Handle, mut cond: impl FnMut(privilege::Luid) -> bool) -> Result<(), LastError> {
    adjust_token_privileges_each(token, move |p| {
        let prev = p.attributes & SE_PRIVILEGE_ENABLED;
        p.attributes = if cond(p.luid) { 0 } else { SE_PRIVILEGE_ENABLED };
        p.attributes != prev
    })
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-adjusttokenprivileges)\] `AdjustTokenPrivileges(self, FALSE, ...)`<br>
/// Keep only the specified privileges of the token.<br>
/// Discarded privileges cannot be reapplied.<br>
/// &nbsp;
pub fn adjust_token_privileges_retain_if(token: &token::Handle, mut cond: impl FnMut(privilege::Luid) -> bool) -> Result<(), LastError> {
    adjust_token_privileges_each(token, move |p| {
        let prev = p.attributes & SE_PRIVILEGE_ENABLED;
        p.attributes = if cond(p.luid) { prev } else { SE_PRIVILEGE_REMOVED };
        p.attributes != prev
    })
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-adjusttokenprivileges)\] `AdjustTokenPrivileges(self, FALSE, ...)`<br>
/// Remove the specified privileges of the token.<br>
/// Discarded privileges cannot be reapplied.<br>
/// &nbsp;
pub fn adjust_token_privileges_remove_if(token: &token::Handle, mut cond: impl FnMut(privilege::Luid) -> bool) -> Result<(), LastError> {
    adjust_token_privileges_each(token, move |p| {
        let prev = p.attributes & SE_PRIVILEGE_ENABLED;
        p.attributes = if cond(p.luid) { SE_PRIVILEGE_REMOVED } else { prev };
        p.attributes != prev
    })
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-adjusttokenprivileges)\] `GetTokenInformation(self, TokenPrivileges, ...)` + `AdjustTokenPrivileges(self, ...)`
fn adjust_token_privileges_each(token: &token::Handle, mut adjust_attributes: impl FnMut(&mut privilege::LuidAndAttributes) -> bool) -> Result<(), LastError> {
    let mut privileges = token.get_token_privileges()?;
    let mut changes = false;
    for privilege in privileges.privileges_mut() {
        changes |= adjust_attributes(privilege);
    }
    if changes {
        unsafe { adjust_token_privileges(token, false, Some(&mut privileges), None, None)? };
    }
    Ok(())
}

impl token::Handle {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-adjusttokenprivileges)\] `AdjustTokenPrivileges(self, FALSE, ...)`<br>
    /// Enable only the specified privileges of the token.<br>
    /// **Disabled privileges can be re-enabled** - this guards against accidents, not malware!<br>
    pub fn privileges_enable_if(&self, cond: impl FnMut(privilege::Luid) -> bool) -> Result<(), LastError> { adjust_token_privileges_enable_if(self, cond) }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-adjusttokenprivileges)\] `AdjustTokenPrivileges(self, FALSE, ...)`<br>
    /// Disable the specified privileges of the token.<br>
    /// **Disabled privileges can be re-enabled** - this guards against accidents, not malware!<br>
    pub fn privileges_disable_if(&self, cond: impl FnMut(privilege::Luid) -> bool) -> Result<(), LastError> { adjust_token_privileges_disable_if(self, cond) }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-adjusttokenprivileges)\] `AdjustTokenPrivileges(self, FALSE, ...)`<br>
    /// Keep only the specified privileges of the token.<br>
    /// Discarded privileges cannot be reapplied.<br>
    pub fn privileges_retain_if(&self, cond: impl FnMut(privilege::Luid) -> bool) -> Result<(), LastError> { adjust_token_privileges_retain_if(self, cond) }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-adjusttokenprivileges)\] `AdjustTokenPrivileges(self, FALSE, ...)`<br>
    /// Remove the specified privileges of the token.<br>
    /// Discarded privileges cannot be reapplied.<br>
    pub fn privileges_remove_if(&self, cond: impl FnMut(privilege::Luid) -> bool) -> Result<(), LastError> { adjust_token_privileges_remove_if(self, cond) }
}



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-adjusttokenprivileges)\] `AdjustTokenPrivileges(self, TRUE, ...)`
///
/// Remove all privileges (except `"SeChangeNotifyPrivilege"`? (allows file/dir change notifications, bypasses traversal checking))
fn _adjust_token_privileges_disable_all(token: &token::Handle) -> Result<(), LastError> {
    unsafe { adjust_token_privileges(token, true, None, None, None) }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-adjusttokenprivileges)\] `AdjustTokenPrivileges(self, ...)`
unsafe fn adjust_token_privileges(token: &token::Handle, disable_all_privileges: bool, new_state: Option<&mut token::BoxTokenPrivileges>, _previous_state: Option<Infallible>, _return_length: Option<Infallible>) -> Result<(), LastError> {
    let new_state = new_state.map_or(null_mut(), |s| s.as_token_privileges_mut_ptr()).cast();
    let success = 0 != unsafe { AdjustTokenPrivileges(token.as_handle(), disable_all_privileges as _, new_state, 0, null_mut(), null_mut()) };
    if success { Ok(()) } else { Err(LastError::get()) }
}
