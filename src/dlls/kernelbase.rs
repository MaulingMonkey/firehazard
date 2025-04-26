//! kernelbase.dll

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::prelude::*;
use super::OptionalLibrary;

use winapi::shared::minwindef::DWORD;
use winapi::shared::ntdef::LPCWSTR;
use winapi::um::winnt::SECURITY_CAPABILITIES;
use winapi::um::winnt::PSID;



lazy_static::lazy_static! {
    static ref DLL : OptionalLibrary = unsafe { OptionalLibrary::load_w(cstr16!("kernelbase")) };

    pub static ref CreateAppContainerToken : unsafe extern "system" fn (
        token_handle:               HANDLE,
        security_capabilities:      *const SECURITY_CAPABILITIES,
        out_token:                  *mut HANDLE,
    ) -> BOOL = unsafe { DLL.get_proc_address(cstr8!("CreateAppContainerToken")) }.unwrap_or(stubs::CreateAppContainerToken);

    pub static ref DeriveCapabilitySidsFromName : unsafe extern "system" fn(
        CapName:                    LPCWSTR,
        CapabilityGroupSids:        *mut *mut PSID,
        CapabilityGroupSidCount:    *mut DWORD,
        CapabilitySids:             *mut *mut PSID,
        CapabilitySidCount:         *mut DWORD,
    ) -> BOOL = unsafe { DLL.get_proc_address(cstr8!("DeriveCapabilitySidsFromName")) }.unwrap_or(stubs::DeriveCapabilitySidsFromName);
}

mod stubs {
    #![allow(unused_variables)]
    use super::*;
    use winapi::um::errhandlingapi::SetLastError;

    pub unsafe extern "system" fn CreateAppContainerToken(
        token_handle:               HANDLE,
        security_capabilities:      *const SECURITY_CAPABILITIES,
        out_token:                  *mut HANDLE,
    ) -> BOOL {
        unsafe { SetLastError(ERROR_CALL_NOT_IMPLEMENTED) }
        FALSE
    }

    pub unsafe extern "system" fn DeriveCapabilitySidsFromName(
        CapName:                    LPCWSTR,
        CapabilityGroupSids:        *mut *mut PSID,
        CapabilityGroupSidCount:    *mut DWORD,
        CapabilitySids:             *mut *mut PSID,
        CapabilitySidCount:         *mut DWORD,
    ) -> BOOL {
        unsafe { SetLastError(ERROR_CALL_NOT_IMPLEMENTED) }
        FALSE
    }
}
