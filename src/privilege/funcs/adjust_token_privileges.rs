use crate::*;

use winapi::um::securitybaseapi::AdjustTokenPrivileges;

use core::convert::Infallible;
use core::ptr::null_mut;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-adjusttokenprivileges)\] `AdjustTokenPrivileges(self, FALSE, ...)`<br>
/// Enable only the specified privileges of the token.<br>
/// **Disabled privileges can be re-enabled** - this guards against accidents, not malware!<br>
/// &nbsp;
pub fn adjust_token_privileges_enable_if(token: &token::OwnedHandle, mut cond: impl FnMut(privilege::Luid) -> bool) -> Result<(), Error> {
    adjust_token_privileges_each(token, move |p| {
        let prev = p.attributes & privilege::ENABLED;
        p.attributes = if cond(p.luid) { privilege::ENABLED } else { privilege::Attributes::default() };
        p.attributes != prev
    })
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-adjusttokenprivileges)\] `AdjustTokenPrivileges(self, FALSE, ...)`<br>
/// Disable the specified privileges of the token.<br>
/// **Disabled privileges can be re-enabled** - this guards against accidents, not malware!<br>
/// &nbsp;
pub fn adjust_token_privileges_disable_if(token: &token::OwnedHandle, mut cond: impl FnMut(privilege::Luid) -> bool) -> Result<(), Error> {
    adjust_token_privileges_each(token, move |p| {
        let prev = p.attributes & privilege::ENABLED;
        p.attributes = if cond(p.luid) { privilege::Attributes::default() } else { privilege::ENABLED };
        p.attributes != prev
    })
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-adjusttokenprivileges)\] `AdjustTokenPrivileges(self, FALSE, ...)`<br>
/// Keep only the specified privileges of the token.<br>
/// Discarded privileges cannot be reapplied.<br>
/// &nbsp;
pub fn adjust_token_privileges_retain_if(token: &token::OwnedHandle, mut cond: impl FnMut(privilege::Luid) -> bool) -> Result<(), Error> {
    adjust_token_privileges_each(token, move |p| {
        let prev = p.attributes & privilege::ENABLED;
        p.attributes = if cond(p.luid) { prev } else { privilege::REMOVED };
        p.attributes != prev
    })
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-adjusttokenprivileges)\] `AdjustTokenPrivileges(self, FALSE, ...)`<br>
/// Remove the specified privileges of the token.<br>
/// Discarded privileges cannot be reapplied.<br>
/// &nbsp;
pub fn adjust_token_privileges_remove_if(token: &token::OwnedHandle, mut cond: impl FnMut(privilege::Luid) -> bool) -> Result<(), Error> {
    adjust_token_privileges_each(token, move |p| {
        let prev = p.attributes & privilege::ENABLED;
        p.attributes = if cond(p.luid) { privilege::REMOVED } else { prev };
        p.attributes != prev
    })
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-adjusttokenprivileges)\] `GetTokenInformation(self, TokenPrivileges, ...)` + `AdjustTokenPrivileges(self, ...)`
fn adjust_token_privileges_each(token: &token::OwnedHandle, mut adjust_attributes: impl FnMut(&mut privilege::LuidAndAttributes) -> bool) -> Result<(), Error> {
    let mut privileges = get_token_information::privileges(token)?;
    let mut changes = false;
    for privilege in privileges.privileges_mut() {
        changes |= adjust_attributes(privilege);
    }
    if changes {
        unsafe { adjust_token_privileges(token, false, Some(&mut privileges), None, None)? };
    }
    Ok(())
}

impl token::OwnedHandle {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-adjusttokenprivileges)\] `AdjustTokenPrivileges(self, FALSE, ...)`<br>
    /// Enable only the specified privileges of the token.<br>
    /// **Disabled privileges can be re-enabled** - this guards against accidents, not malware!<br>
    pub fn privileges_enable_if(&self, cond: impl FnMut(privilege::Luid) -> bool) -> Result<(), Error> { adjust_token_privileges_enable_if(self, cond) }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-adjusttokenprivileges)\] `AdjustTokenPrivileges(self, FALSE, ...)`<br>
    /// Disable the specified privileges of the token.<br>
    /// **Disabled privileges can be re-enabled** - this guards against accidents, not malware!<br>
    pub fn privileges_disable_if(&self, cond: impl FnMut(privilege::Luid) -> bool) -> Result<(), Error> { adjust_token_privileges_disable_if(self, cond) }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-adjusttokenprivileges)\] `AdjustTokenPrivileges(self, FALSE, ...)`<br>
    /// Keep only the specified privileges of the token.<br>
    /// Discarded privileges cannot be reapplied.<br>
    pub fn privileges_retain_if(&self, cond: impl FnMut(privilege::Luid) -> bool) -> Result<(), Error> { adjust_token_privileges_retain_if(self, cond) }
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-adjusttokenprivileges)\] `AdjustTokenPrivileges(self, FALSE, ...)`<br>
    /// Remove the specified privileges of the token.<br>
    /// Discarded privileges cannot be reapplied.<br>
    pub fn privileges_remove_if(&self, cond: impl FnMut(privilege::Luid) -> bool) -> Result<(), Error> { adjust_token_privileges_remove_if(self, cond) }
}



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-adjusttokenprivileges)\] `AdjustTokenPrivileges(self, TRUE, ...)`
///
/// Remove all privileges (except `"SeChangeNotifyPrivilege"`? (allows file/dir change notifications, bypasses traversal checking))
fn _adjust_token_privileges_disable_all(token: &token::OwnedHandle) -> Result<(), Error> {
    unsafe { adjust_token_privileges(token, true, None, None, None) }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-adjusttokenprivileges)\] `AdjustTokenPrivileges(self, ...)`
unsafe fn adjust_token_privileges(token: &token::OwnedHandle, disable_all_privileges: bool, new_state: Option<&mut token::BoxTokenPrivileges>, _previous_state: Option<Infallible>, _return_length: Option<Infallible>) -> Result<(), Error> {
    let new_state = new_state.map_or(null_mut(), |s| s.as_token_privileges_mut_ptr()).cast();
    Error::get_last_if(0 == unsafe { AdjustTokenPrivileges(token.as_handle(), disable_all_privileges as _, new_state, 0, none2null(_previous_state), none2null(_return_length)) })
}

#[test] fn test() {
    let t = open_process_token(get_current_process(), token::ALL_ACCESS).unwrap();
    let t = duplicate_token_ex(&t, token::ALL_ACCESS, None, security::Delegation, token::Primary).unwrap();
    t.privileges_disable_if(|_| true).unwrap();
    t.privileges_remove_if(|_| true).unwrap();
}
