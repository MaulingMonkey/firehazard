#![doc(alias = "SetTokenInformation")]
//! \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-settokeninformation)\]
//! SetTokenInformation
//!
//! ### Errors
//! *   `ERROR_ACCESS_DENIED`   - if the [`token::OwnedHandle`] wasn't opened with at least [`token::ADJUST_DEFAULT`]

use crate::*;

use winapi::shared::winerror::ERROR_INVALID_PARAMETER;
use winapi::um::securitybaseapi::SetTokenInformation;
use winapi::um::winnt::*;



#[doc(alias = "SetTokenInformation")]
#[doc(alias = "TokenDefaultDacl")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-settokeninformation)\]
/// `SetTokenInformation(self, TokenDefaultDacl, ...)`
///
pub fn default_dacl<'acl>(token: &token::OwnedHandle, dacl: impl Into<acl::Ptr<'acl>>) -> Result<(), Error> { unsafe { raw_fixed(token, TokenDefaultDacl, &TOKEN_DEFAULT_DACL { DefaultDacl: dacl.into().as_pacl() }) } }



#[doc(alias = "SetTokenInformation")]
#[doc(alias = "TokenIntegrityLevel")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-settokeninformation)\]
/// `SetTokenInformation(self, TokenIntegrityLevel, ...)`
///
pub fn integrity_level(token: &token::OwnedHandle, saa: sid::AndAttributes) -> Result<(), Error> { unsafe { raw_fixed(token, TokenIntegrityLevel, &saa) } }



impl token::OwnedHandle {
    #[doc(alias = "SetTokenInformation")]
    #[doc(alias = "TokenDefaultDacl")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-settokeninformation)\]
    /// `SetTokenInformation(self, TokenDefaultDacl, ...)`
    ///
    pub fn set_default_dacl<'acl>(&self, dacl: impl Into<acl::Ptr<'acl>>) -> Result<(), Error> { default_dacl(self, dacl) }



    #[doc(alias = "SetTokenInformation")]
    #[doc(alias = "TokenIntegrityLevel")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-settokeninformation)\]
    /// `SetTokenInformation(self, TokenIntegrityLevel, ...)`
    ///
    pub fn set_integrity_level(&self, saa: sid::AndAttributes) -> Result<(), Error> { integrity_level(self, saa) }
}



#[doc(alias = "SetTokenInformation")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-settokeninformation)\]
/// `SetTokenInformation(self, class, slice, size_of_val(&slice))`
///
/// Set the token information as a raw byte buffer.
///
/// ### Safety
/// *   `class` might need to be a valid token information class?
/// *   `slice` might have alignment requirements
/// *   `slice` might be expected to contain valid pointers and other fields, depending on `class`
///
unsafe fn raw_slice<E>(token: &token::OwnedHandle, class: TOKEN_INFORMATION_CLASS, slice: &[E]) -> Result<(), Error> {
    let len32 = u32::try_from(core::mem::size_of_val(slice)).map_err(|_| ERROR_INVALID_PARAMETER)?;
    Error::get_last_if(0 == unsafe { SetTokenInformation(token.as_handle(), class, slice.as_ptr() as *mut _, len32) })
}



#[doc(alias = "SetTokenInformation")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-settokeninformation)\]
/// `SetTokenInformation(self, class, value, size_of_val(value))`
///
/// Set the token information as a raw byte buffer.
///
/// ### Safety
/// *   `class` might need to be a valid token information class?
/// *   `value` might have alignment requirements
/// *   `value` might be expected to contain valid pointers and other fields, depending on `class`
///
unsafe fn raw_fixed<E>(token: &token::OwnedHandle, class: TOKEN_INFORMATION_CLASS, value: &E) -> Result<(), Error> {
    unsafe { raw_slice(token, class, core::slice::from_ref(value)) }
}
