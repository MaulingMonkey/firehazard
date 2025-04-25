#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::prelude::*;
use bytemuck::*;
use winapi::shared::minwindef::ULONG;
use winapi::shared::ntdef::{NTSTATUS, UNICODE_STRING};
use winapi::um::winnt::ACCESS_MASK;



// C:\Program Files (x86)\Windows Kits\10\Include\10.0.22621.0\um\winternl.h
// line 395
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)] pub enum OBJECT_INFORMATION_CLASS {
    ObjectBasicInformation = 0,
    ObjectTypeInformation = 2,
}

pub unsafe trait OBJECT_INFORMATION : Default                       { const CLASS : OBJECT_INFORMATION_CLASS; }
unsafe impl OBJECT_INFORMATION for PUBLIC_OBJECT_BASIC_INFORMATION  { const CLASS : OBJECT_INFORMATION_CLASS = OBJECT_INFORMATION_CLASS::ObjectBasicInformation; }
unsafe impl OBJECT_INFORMATION for PUBLIC_OBJECT_TYPE_INFORMATION   { const CLASS : OBJECT_INFORMATION_CLASS = OBJECT_INFORMATION_CLASS::ObjectTypeInformation; }

unsafe impl bytemuck::Zeroable for PUBLIC_OBJECT_TYPE_INFORMATION {} // can't derive because of UNICODE_STRING
impl Default for PUBLIC_OBJECT_BASIC_INFORMATION { fn default() -> Self { Zeroable::zeroed() } }
impl Default for PUBLIC_OBJECT_TYPE_INFORMATION  { fn default() -> Self { Zeroable::zeroed() } }

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntifs/ns-ntifs-_public_object_basic_information)\]
/// PUBLIC_OBJECT_BASIC_INFORMATION
///
#[derive(Clone, Debug, Zeroable)]
#[repr(C)] pub struct PUBLIC_OBJECT_BASIC_INFORMATION {
    pub Attributes:     ULONG,
    pub GrantedAccess:  ACCESS_MASK,
    pub HandleCount:    ULONG,
    pub PointerCount:   ULONG,
    pub Reserved:       [MaybeUninit<ULONG>; 10],
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



unsafe fn func<F>(name: &'static str) -> firehazard::Result<F> { unsafe { (*DLL)?.sym_opt(name) }.ok_or(firehazard::Error(ERROR_API_UNAVAILABLE)) }

lazy_static::lazy_static! {
    static ref DLL : firehazard::Result<minidl::Library> = minidl::Library::load("ntdll").map_err(|_| firehazard::Error(ERROR_API_UNAVAILABLE));

    // C:\Program Files (x86)\Windows Kits\10\Include\10.0.22621.0\um\winternl.h
    // line 674

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntifs/nf-ntifs-ntqueryobject)\]
    /// NtQueryObject
    pub static ref NtQueryObject : firehazard::Result<unsafe extern "system" fn (
        handle:                     HANDLE,
        object_information_class:   OBJECT_INFORMATION_CLASS,
        object_information:         Option<NonNull<c_void>>,
        object_information_length:  ULONG,
        return_length:              Option<&mut ULONG>,
    ) -> NTSTATUS> = unsafe { func("NtQueryObject\0") };
}
