//! \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_basic_limit_information#members)\] JOB_OBJECT_LIMIT_*

#[allow(unused_imports)] use crate::prelude::*;
use core::fmt::{self, Debug, Formatter};



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_basic_limit_information#members)\]
/// DWORD/[u32]: [job::object::BasicLimitInformation] flags mask
///
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct FlagsMask(pub(super) u32);

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_basic_limit_information#members)\]
/// DWORD/[u32]: [job::object::BasicLimitInformation] flags
///
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Flags(u32);

flags!(impl .. for Flags(u32) - FlagsMask);

impl Flags {
    /// ### Safety
    /// *   Some APIs might theoretically assume flags are a valid?
    pub const unsafe fn from_unchecked(flags: u32) -> Self { Self(flags) }
}

impl Debug for Flags {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        use winapi::um::winnt::*;
        flags!(self.0, fmt, "0x{:X}", [
            JOB_OBJECT_LIMIT_ACTIVE_PROCESS,
            JOB_OBJECT_LIMIT_AFFINITY,
            JOB_OBJECT_LIMIT_BREAKAWAY_OK,
            JOB_OBJECT_LIMIT_DIE_ON_UNHANDLED_EXCEPTION,
            JOB_OBJECT_LIMIT_JOB_MEMORY,
            JOB_OBJECT_LIMIT_JOB_TIME,
            JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
            JOB_OBJECT_LIMIT_PRESERVE_JOB_TIME,
            JOB_OBJECT_LIMIT_PRIORITY_CLASS,
            JOB_OBJECT_LIMIT_PROCESS_MEMORY,
            JOB_OBJECT_LIMIT_PROCESS_TIME,
            JOB_OBJECT_LIMIT_SCHEDULING_CLASS,
            JOB_OBJECT_LIMIT_SILENT_BREAKAWAY_OK,
            JOB_OBJECT_LIMIT_SUBSET_AFFINITY,
            JOB_OBJECT_LIMIT_WORKINGSET,
        ])
    }
}

// Basic Limits - C:\Program Files (x86)\Windows Kits\10\Include\10.0.22621.0\um\winnt.h

#[doc(alias = "JOB_OBJECT_LIMIT_WORKINGSET")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_basic_limit_information#members)\]
/// JOB_OBJECT_LIMIT_WORKINGSET
///
pub const WORKINGSET                   : Flags = Flags(winapi::um::winnt::JOB_OBJECT_LIMIT_WORKINGSET);

#[doc(alias = "JOB_OBJECT_LIMIT_PROCESS_TIME")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_basic_limit_information#members)\]
/// JOB_OBJECT_LIMIT_PROCESS_TIME
///
pub const PROCESS_TIME                 : Flags = Flags(winapi::um::winnt::JOB_OBJECT_LIMIT_PROCESS_TIME);

#[doc(alias = "JOB_OBJECT_LIMIT_JOB_TIME")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_basic_limit_information#members)\]
/// JOB_OBJECT_LIMIT_JOB_TIME
///
pub const JOB_TIME                     : Flags = Flags(winapi::um::winnt::JOB_OBJECT_LIMIT_JOB_TIME);

#[doc(alias = "JOB_OBJECT_LIMIT_ACTIVE_PROCESS")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_basic_limit_information#members)\]
/// JOB_OBJECT_LIMIT_ACTIVE_PROCESS
///
pub const ACTIVE_PROCESS               : Flags = Flags(winapi::um::winnt::JOB_OBJECT_LIMIT_ACTIVE_PROCESS);

#[doc(alias = "JOB_OBJECT_LIMIT_AFFINITY")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_basic_limit_information#members)\]
/// JOB_OBJECT_LIMIT_AFFINITY
///
pub const AFFINITY                     : Flags = Flags(winapi::um::winnt::JOB_OBJECT_LIMIT_AFFINITY);

#[doc(alias = "JOB_OBJECT_LIMIT_PRIORITY_CLASS")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_basic_limit_information#members)\]
/// JOB_OBJECT_LIMIT_PRIORITY_CLASS
///
pub const PRIORITY_CLASS               : Flags = Flags(winapi::um::winnt::JOB_OBJECT_LIMIT_PRIORITY_CLASS);

#[doc(alias = "JOB_OBJECT_LIMIT_PRESERVE_JOB_TIME")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_basic_limit_information#members)\]
/// JOB_OBJECT_LIMIT_PRESERVE_JOB_TIME
///
pub const PRESERVE_JOB_TIME            : Flags = Flags(winapi::um::winnt::JOB_OBJECT_LIMIT_PRESERVE_JOB_TIME);

#[doc(alias = "JOB_OBJECT_LIMIT_SCHEDULING_CLASS")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_basic_limit_information#members)\]
/// JOB_OBJECT_LIMIT_SCHEDULING_CLASS
///
pub const SCHEDULING_CLASS             : Flags = Flags(winapi::um::winnt::JOB_OBJECT_LIMIT_SCHEDULING_CLASS);



// Extended Limits - C:\Program Files (x86)\Windows Kits\10\Include\10.0.22621.0\um\winnt.h

#[doc(alias = "JOB_OBJECT_LIMIT_PROCESS_MEMORY")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_basic_limit_information#members)\]
/// JOB_OBJECT_LIMIT_PROCESS_MEMORY
///
pub const PROCESS_MEMORY               : Flags = Flags(winapi::um::winnt::JOB_OBJECT_LIMIT_PROCESS_MEMORY);

#[doc(alias = "JOB_OBJECT_LIMIT_JOB_MEMORY")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_basic_limit_information#members)\]
/// JOB_OBJECT_LIMIT_JOB_MEMORY
///
pub const JOB_MEMORY                   : Flags = Flags(winapi::um::winnt::JOB_OBJECT_LIMIT_JOB_MEMORY);

#[doc(alias = "JOB_OBJECT_LIMIT_JOB_MEMORY_HIGH")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_limit_violation_information_2#members)\]
/// JOB_OBJECT_LIMIT_JOB_MEMORY_HIGH
///
pub const JOB_MEMORY_HIGH              : Flags = Flags(winapi::um::winnt::JOB_OBJECT_LIMIT_JOB_MEMORY_HIGH);

#[doc(alias = "JOB_OBJECT_LIMIT_DIE_ON_UNHANDLED_EXCEPTION")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_basic_limit_information#members)\]
/// JOB_OBJECT_LIMIT_DIE_ON_UNHANDLED_EXCEPTION
///
pub const DIE_ON_UNHANDLED_EXCEPTION   : Flags = Flags(winapi::um::winnt::JOB_OBJECT_LIMIT_DIE_ON_UNHANDLED_EXCEPTION);

#[doc(alias = "JOB_OBJECT_LIMIT_BREAKAWAY_OK")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_basic_limit_information#members)\]
/// JOB_OBJECT_LIMIT_BREAKAWAY_OK
///
pub const BREAKAWAY_OK                 : Flags = Flags(winapi::um::winnt::JOB_OBJECT_LIMIT_BREAKAWAY_OK);

#[doc(alias = "JOB_OBJECT_LIMIT_SILENT_BREAKAWAY_OK")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_basic_limit_information#members)\]
/// JOB_OBJECT_LIMIT_SILENT_BREAKAWAY_OK
///
pub const SILENT_BREAKAWAY_OK          : Flags = Flags(winapi::um::winnt::JOB_OBJECT_LIMIT_SILENT_BREAKAWAY_OK);

#[doc(alias = "JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_basic_limit_information#members)\]
/// JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE
///
pub const KILL_ON_JOB_CLOSE            : Flags = Flags(winapi::um::winnt::JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE);

#[doc(alias = "JOB_OBJECT_LIMIT_SUBSET_AFFINITY")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_basic_limit_information#members)\]
/// JOB_OBJECT_LIMIT_SUBSET_AFFINITY
///
pub const SUBSET_AFFINITY              : Flags = Flags(winapi::um::winnt::JOB_OBJECT_LIMIT_SUBSET_AFFINITY);

#[doc(alias = "JOB_OBJECT_LIMIT_JOB_MEMORY_LOW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_limit_violation_information_2#members)\]
/// JOB_OBJECT_LIMIT_JOB_MEMORY_LOW
///
pub const JOB_MEMORY_LOW               : Flags = Flags(winapi::um::winnt::JOB_OBJECT_LIMIT_JOB_MEMORY_LOW);



// Notification Limits - C:\Program Files (x86)\Windows Kits\10\Include\10.0.22621.0\um\winnt.h

#[doc(alias = "JOB_OBJECT_LIMIT_JOB_READ_BYTES")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_limit_violation_information_2#members)\]
/// JOB_OBJECT_LIMIT_JOB_READ_BYTES
///
pub const JOB_READ_BYTES               : Flags = Flags(winapi::um::winnt::JOB_OBJECT_LIMIT_JOB_READ_BYTES);

#[doc(alias = "JOB_OBJECT_LIMIT_JOB_WRITE_BYTES")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_limit_violation_information_2#members)\]
/// JOB_OBJECT_LIMIT_JOB_WRITE_BYTES
///
pub const JOB_WRITE_BYTES              : Flags = Flags(winapi::um::winnt::JOB_OBJECT_LIMIT_JOB_WRITE_BYTES);

#[doc(alias = "JOB_OBJECT_LIMIT_RATE_CONTROL")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_limit_violation_information_2#members)\]
/// JOB_OBJECT_LIMIT_RATE_CONTROL
///
pub const RATE_CONTROL                 : Flags = Flags(winapi::um::winnt::JOB_OBJECT_LIMIT_RATE_CONTROL);

#[doc(alias = "JOB_OBJECT_LIMIT_CPU_RATE_CONTROL")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_limit_violation_information_2#members)\]
/// JOB_OBJECT_LIMIT_CPU_RATE_CONTROL
///
pub const CPU_RATE_CONTROL             : Flags = Flags(winapi::um::winnt::JOB_OBJECT_LIMIT_CPU_RATE_CONTROL);

#[doc(alias = "JOB_OBJECT_LIMIT_IO_RATE_CONTROL")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_limit_violation_information_2#members)\]
/// JOB_OBJECT_LIMIT_IO_RATE_CONTROL
///
pub const IO_RATE_CONTROL              : Flags = Flags(winapi::um::winnt::JOB_OBJECT_LIMIT_IO_RATE_CONTROL);

#[doc(alias = "JOB_OBJECT_LIMIT_NET_RATE_CONTROL")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_limit_violation_information_2#members)\]
/// JOB_OBJECT_LIMIT_NET_RATE_CONTROL
///
pub const NET_RATE_CONTROL             : Flags = Flags(winapi::um::winnt::JOB_OBJECT_LIMIT_NET_RATE_CONTROL);
