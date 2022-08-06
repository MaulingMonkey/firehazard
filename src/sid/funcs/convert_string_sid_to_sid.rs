use crate::*;
use crate::error::LastError;
use crate::sid::Box as SidBox;

use abistr::{AsCStr, TryIntoAsCStr};

use winapi::shared::sddl::{ConvertStringSidToSidW, ConvertStringSidToSidA};
use winapi::shared::winerror::{ERROR_INVALID_PARAMETER, ERROR_INVALID_SID};

use std::ptr::null_mut;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/sddl/nf-sddl-convertstringsidtosida)\]
/// ConvertStringSidToSidA - convert e.g. `"S-1-0-0"` to [`sid::Box`]
pub fn convert_string_sid_to_sid_a(s: impl TryIntoAsCStr) -> Result<SidBox<alloc::LocalAllocFree>, LastError> {
    let s = s.try_into().map_err(|_| LastError(ERROR_INVALID_PARAMETER))?;
    let mut sid = null_mut();
    let success = 0 != unsafe { ConvertStringSidToSidA(s.as_cstr(), &mut sid) };
    let sid = unsafe { SidBox::from_raw(sid.cast()) };

    if !success { Err(LastError::get()) }
    else        { sid.ok_or(LastError(ERROR_INVALID_SID)) }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/sddl/nf-sddl-convertstringsidtosidw)\]
/// ConvertStringSidToSidW - convert e.g. `"S-1-0-0"` to [`sid::Box`]
pub fn convert_string_sid_to_sid_w(s: impl TryIntoAsCStr<u16>) -> Result<SidBox<alloc::LocalAllocFree>, LastError> {
    let s = s.try_into().map_err(|_| LastError(ERROR_INVALID_PARAMETER))?;
    let mut sid = null_mut();
    let success = 0 != unsafe { ConvertStringSidToSidW(s.as_cstr(), &mut sid) };
    let sid = unsafe { SidBox::from_raw(sid.cast()) };

    if !success { Err(LastError::get()) }
    else        { sid.ok_or(LastError(ERROR_INVALID_SID)) }
}
