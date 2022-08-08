//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-settokeninformation)\]
//! SetTokenInformation

use crate::*;
use crate::error::LastError;

use winapi::shared::winerror::ERROR_INVALID_PARAMETER;
use winapi::um::securitybaseapi::SetTokenInformation;
use winapi::um::winnt::{TOKEN_INFORMATION_CLASS, TokenIntegrityLevel};


/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-settokeninformation)\] `SetTokenInformation(self, TokenIntegrityLevel, ...)`
pub fn integrity_level(token: &token::Handle, saa: sid::AndAttributes) -> Result<(), LastError> { unsafe { raw_fixed(token, TokenIntegrityLevel, &saa) } }

impl token::Handle {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-settokeninformation)\] `SetTokenInformation(self, TokenIntegrityLevel, ...)`
    pub fn set_integrity_level(&self, saa: sid::AndAttributes) -> Result<(), LastError> { integrity_level(self, saa) }
}



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-settokeninformation)\]
/// `SetTokenInformation(self, class, slice, size_of_val(&slice))`
///
/// Set the token information as a raw byte buffer.
///
/// ### Safety
/// *   `class` might need to be a valid token information class?
/// *   `slice` might have alignment requirements
/// *   `slice` might be expected to contain valid pointers and other fields, depending on `class`
unsafe fn raw_slice<E>(token: &token::Handle, class: TOKEN_INFORMATION_CLASS, slice: &[E]) -> Result<(), LastError> {
    let len32 = u32::try_from(std::mem::size_of_val(slice)).map_err(|_| LastError(ERROR_INVALID_PARAMETER))?;
    let success = 0 != unsafe { SetTokenInformation(token.as_handle(), class, slice.as_ptr() as *mut _, len32) };
    if success { Ok(()) } else { Err(LastError::get()) }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)\]
/// `SetTokenInformation(self, class, value, size_of_val(value))`
///
/// Set the token information as a raw byte buffer.
///
/// ### Safety
/// *   `class` might need to be a valid token information class?
/// *   `value` might have alignment requirements
/// *   `value` might be expected to contain valid pointers and other fields, depending on `class`
unsafe fn raw_fixed<E>(token: &token::Handle, class: TOKEN_INFORMATION_CLASS, value: &E) -> Result<(), LastError> {
    unsafe { raw_slice(token, class, std::slice::from_ref(value)) }
}
