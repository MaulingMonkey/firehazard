use crate::*;
use winapi::um::winnt::{JOBOBJECT_BASIC_LIMIT_INFORMATION, JOBOBJECT_EXTENDED_LIMIT_INFORMATION, JobObjectBasicLimitInformation, JobObjectExtendedLimitInformation};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_basic_limit_information)\]
/// JOBOBJECT_BASIC_LIMIT_INFORMATION
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)] pub struct BasicLimitInformation {
    pub per_process_user_time_limit:    u64,    // 100-nanosecond ticks
    pub per_job_user_time_limit:        u64,    // 100-nanosecond ticks
    pub limit_flags:                    job::object::limit::Flags,
    pub minimum_working_set_size:       usize,  // bytes
    pub maximum_working_set_size:       usize,  // bytes
    pub active_process_limit:           u32,    // processes
    pub affinity:                       usize,  // mask
    pub priority_class:                 u32,    // {NORMAL,IDLE,HIGH,...}_PRIORITY_CLASS
    pub scheduling_class:               u32,    // 0 ..= 9 (default: 5)
}

structure!(@assert layout BasicLimitInformation => JOBOBJECT_BASIC_LIMIT_INFORMATION {
    per_process_user_time_limit == PerProcessUserTimeLimit,
    per_job_user_time_limit     == PerJobUserTimeLimit,
    limit_flags                 == LimitFlags,
    minimum_working_set_size    == MinimumWorkingSetSize,
    maximum_working_set_size    == MaximumWorkingSetSize,
    active_process_limit        == ActiveProcessLimit,
    affinity                    == Affinity,
    priority_class              == PriorityClass,
    scheduling_class            == SchedulingClass,
});



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_extended_limit_information)\]
/// JOBOBJECT_EXTENDED_LIMIT_INFORMATION
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)] pub struct ExtendedLimitInformation {
    pub basic_limit_information:        BasicLimitInformation,
    pub io_info:                        io::Counters,
    pub process_memory_limit:           usize,  // bytes
    pub job_memory_limit:               usize,  // bytes
    pub peak_process_memory_used:       usize,  // bytes
    pub peak_job_memory_used:           usize,  // bytes
}

structure!(@assert layout ExtendedLimitInformation => JOBOBJECT_EXTENDED_LIMIT_INFORMATION {
    basic_limit_information     == BasicLimitInformation,
    io_info                     == IoInfo,
    process_memory_limit        == ProcessMemoryLimit,
    job_memory_limit            == JobMemoryLimit,
    peak_process_memory_used    == PeakProcessMemoryUsed,
    peak_job_memory_used        == PeakJobMemoryUsed,
});



impl job::QueryInformationJobObject for JOBOBJECT_BASIC_LIMIT_INFORMATION        { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { job::query_fixed(job, JobObjectBasicLimitInformation) } } }
impl job::QueryInformationJobObject for job::object::BasicLimitInformation       { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { job::query_fixed(job, JobObjectBasicLimitInformation) } } }
impl job::QueryInformationJobObject for JOBOBJECT_EXTENDED_LIMIT_INFORMATION     { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { job::query_fixed(job, JobObjectExtendedLimitInformation) } } }
impl job::QueryInformationJobObject for job::object::ExtendedLimitInformation    { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { job::query_fixed(job, JobObjectExtendedLimitInformation) } } }

impl job::SetInformationJobObject for JOBOBJECT_BASIC_LIMIT_INFORMATION          { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { job::set(job, JobObjectBasicLimitInformation, &self) } } }
impl job::SetInformationJobObject for job::object::BasicLimitInformation         { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { job::set(job, JobObjectBasicLimitInformation, &self) } } }
impl job::SetInformationJobObject for JOBOBJECT_EXTENDED_LIMIT_INFORMATION       { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { job::set(job, JobObjectExtendedLimitInformation, &self) } } }
impl job::SetInformationJobObject for job::object::ExtendedLimitInformation      { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { job::set(job, JobObjectExtendedLimitInformation, &self) } } }
