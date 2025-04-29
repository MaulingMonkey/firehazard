//! ntdll.dll

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::prelude::*;
use super::OptionalLibrary;
use bytemuck::*;
use winapi::shared::minwindef::ULONG;
use winapi::shared::ntdef::{UNICODE_STRING};



// C:\Program Files (x86)\Windows Kits\10\Include\10.0.22621.0\um\winternl.h
// line 395
#[derive(Clone, Copy, Pod, Debug, Default, Zeroable, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct OBJECT_INFORMATION_CLASS(u32);
pub const ObjectBasicInformation    : OBJECT_INFORMATION_CLASS = OBJECT_INFORMATION_CLASS(0);
pub const ObjectTypeInformation     : OBJECT_INFORMATION_CLASS = OBJECT_INFORMATION_CLASS(2);

pub unsafe trait OBJECT_INFORMATION : Default                       { const CLASS : OBJECT_INFORMATION_CLASS; }
unsafe impl OBJECT_INFORMATION for PUBLIC_OBJECT_BASIC_INFORMATION  { const CLASS : OBJECT_INFORMATION_CLASS = ObjectBasicInformation; }
unsafe impl OBJECT_INFORMATION for PUBLIC_OBJECT_TYPE_INFORMATION   { const CLASS : OBJECT_INFORMATION_CLASS = ObjectTypeInformation; }

unsafe impl bytemuck::Zeroable for PUBLIC_OBJECT_TYPE_INFORMATION {} // can't derive because of UNICODE_STRING
impl Default for PUBLIC_OBJECT_BASIC_INFORMATION { fn default() -> Self { Zeroable::zeroed() } }
impl Default for PUBLIC_OBJECT_TYPE_INFORMATION  { fn default() -> Self { Zeroable::zeroed() } }

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntifs/ns-ntifs-_public_object_basic_information)\]
/// PUBLIC_OBJECT_BASIC_INFORMATION
///
#[derive(Clone, Zeroable)]
#[repr(C)] pub struct PUBLIC_OBJECT_BASIC_INFORMATION {
    pub Attributes:     ULONG,
    pub GrantedAccess:  access::Mask,
    pub HandleCount:    ULONG,
    pub PointerCount:   ULONG,
    pub Reserved:       [MaybeUninit<ULONG>; 10],
}

impl core::fmt::Debug for PUBLIC_OBJECT_BASIC_INFORMATION {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        fmt.debug_struct("PUBLIC_OBJECT_BASIC_INFORMATION")
            .field("Attributes",    &self.Attributes    )
            .field("GrantedAccess", &self.GrantedAccess )
            .field("HandleCount",   &self.HandleCount   )
            .field("PointerCount",  &self.PointerCount  )
            .finish_non_exhaustive() // Reserved
    }
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntifs/ns-ntifs-__public_object_type_information)\]
/// PUBLIC_OBJECT_TYPE_INFORMATION
///
#[derive(Clone)]
#[repr(C)] pub struct PUBLIC_OBJECT_TYPE_INFORMATION {
    pub TypeName:       UNICODE_STRING,
    pub Reserved:       [MaybeUninit<ULONG>; 22],
}

impl PUBLIC_OBJECT_TYPE_INFORMATION {
    pub fn type_name(&self) -> &[u16] {
        // Why yes, `UNICODE_STRING::Length` *is* in bytes for some god forsaken reason.
        unsafe { core::slice::from_raw_parts(self.TypeName.Buffer, (self.TypeName.Length / 2).into()) }
    }
}

impl core::fmt::Debug for PUBLIC_OBJECT_TYPE_INFORMATION {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        fmt.debug_struct("PUBLIC_OBJECT_TYPE_INFORMATION")
            .field("TypeName", &self.type_name())
            .finish_non_exhaustive() // Reserved
    }
}



lazy_static::lazy_static! {
    static ref DLL : OptionalLibrary = unsafe { OptionalLibrary::load_w(cstr16!("ntdll")) };

    // XXX: DDK?

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntifs/nf-ntifs-ntqueryinformationfile)\]
    /// NtQueryInformationFile
    pub(crate) static ref NtQueryInformationFile : unsafe extern "system" fn (
        file_handle:                HANDLE,
        io_status_block:            *mut io::StatusBlock,
        file_information:           *mut c_void,
        length:                     ULONG,
        file_information_class:     file::InformationClass,
    ) -> NtStatus = unsafe { DLL.get_proc_address(cstr8!("NtQueryInformationFile")) }.unwrap_or(stubs::NtQueryInformationFile);

    // C:\Program Files (x86)\Windows Kits\10\Include\10.0.22621.0\um\winternl.h
    // line 674

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntifs/nf-ntifs-ntqueryobject)\]
    /// NtQueryObject
    pub(crate) static ref NtQueryObject : unsafe extern "system" fn (
        handle:                     HANDLE,
        object_information_class:   OBJECT_INFORMATION_CLASS,
        object_information:         Option<NonNull<c_void>>,
        object_information_length:  ULONG,
        return_length:              Option<&mut ULONG>,
    ) -> NtStatus = unsafe { DLL.get_proc_address(cstr8!("NtQueryObject")) }.unwrap_or(stubs::NtQueryObject);
}

/// fallback impls if unavailable from ntdll.dll
mod stubs {
    use super::*;

    pub(crate) unsafe extern "system" fn NtQueryInformationFile(
        _file_handle:               HANDLE,
        _io_status_block:           *mut io::StatusBlock,
        _file_information:          *mut c_void,
        _length:                    ULONG,
        _file_information_class:    file::InformationClass,
    ) -> NtStatus {
        STATUS::NOT_IMPLEMENTED
    }

    pub(crate) unsafe extern "system" fn NtQueryObject(
        _handle:                    HANDLE,
        _object_information_class:  OBJECT_INFORMATION_CLASS,
        _object_information:        Option<NonNull<c_void>>,
        _object_information_length: ULONG,
        _return_length:             Option<&mut ULONG>,
    ) -> NtStatus {
        STATUS::NOT_IMPLEMENTED
    }
}
