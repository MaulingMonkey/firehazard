//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_basic_limit_information#members)\] JOB_OBJECT_LIMIT_*

#[allow(unused_imports)] use crate::*;
use core::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_basic_limit_information#members)\] DWORD/[u32]: [job::object::BasicLimitInformation] flags mask
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct FlagsMask(u32);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_basic_limit_information#members)\] DWORD/[u32]: [job::object::BasicLimitInformation] flags
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

pub const ACTIVE_PROCESS                : Flags = Flags(winapi::um::winnt::JOB_OBJECT_LIMIT_ACTIVE_PROCESS);
pub const AFFINITY                      : Flags = Flags(winapi::um::winnt::JOB_OBJECT_LIMIT_AFFINITY);
pub const BREAKAWAY_OK                  : Flags = Flags(winapi::um::winnt::JOB_OBJECT_LIMIT_BREAKAWAY_OK);
pub const DIE_ON_UNHANDLED_EXCEPTION    : Flags = Flags(winapi::um::winnt::JOB_OBJECT_LIMIT_DIE_ON_UNHANDLED_EXCEPTION);
pub const JOB_MEMORY                    : Flags = Flags(winapi::um::winnt::JOB_OBJECT_LIMIT_JOB_MEMORY);
pub const JOB_TIME                      : Flags = Flags(winapi::um::winnt::JOB_OBJECT_LIMIT_JOB_TIME);
pub const KILL_ON_JOB_CLOSE             : Flags = Flags(winapi::um::winnt::JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE);
pub const PRESERVE_JOB_TIME             : Flags = Flags(winapi::um::winnt::JOB_OBJECT_LIMIT_PRESERVE_JOB_TIME);
pub const PRIORITY_CLASS                : Flags = Flags(winapi::um::winnt::JOB_OBJECT_LIMIT_PRIORITY_CLASS);
pub const PROCESS_MEMORY                : Flags = Flags(winapi::um::winnt::JOB_OBJECT_LIMIT_PROCESS_MEMORY);
pub const PROCESS_TIME                  : Flags = Flags(winapi::um::winnt::JOB_OBJECT_LIMIT_PROCESS_TIME);
pub const SCHEDULING_CLASS              : Flags = Flags(winapi::um::winnt::JOB_OBJECT_LIMIT_SCHEDULING_CLASS);
pub const SILENT_BREAKAWAY_OK           : Flags = Flags(winapi::um::winnt::JOB_OBJECT_LIMIT_SILENT_BREAKAWAY_OK);
pub const SUBSET_AFFINITY               : Flags = Flags(winapi::um::winnt::JOB_OBJECT_LIMIT_SUBSET_AFFINITY);
pub const WORKINGSET                    : Flags = Flags(winapi::um::winnt::JOB_OBJECT_LIMIT_WORKINGSET);
