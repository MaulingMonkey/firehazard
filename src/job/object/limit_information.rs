use crate::*;
use winapi::um::winnt::{JOBOBJECT_BASIC_LIMIT_INFORMATION, JOBOBJECT_EXTENDED_LIMIT_INFORMATION, JobObjectBasicLimitInformation, JobObjectExtendedLimitInformation};
use core::mem::{align_of, size_of};



const _ : () = assert!(align_of::<BasicLimitInformation>() == align_of::<JOBOBJECT_BASIC_LIMIT_INFORMATION>());
const _ : () = assert!(size_of ::<BasicLimitInformation>() == size_of ::<JOBOBJECT_BASIC_LIMIT_INFORMATION>());
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



const _ : () = assert!(align_of::<ExtendedLimitInformation>() == align_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>());
const _ : () = assert!(size_of ::<ExtendedLimitInformation>() == size_of ::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>());
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



impl job::QueryInformation for JOBOBJECT_BASIC_LIMIT_INFORMATION        { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { job::query_fixed(job, JobObjectBasicLimitInformation) } } }
impl job::QueryInformation for job::object::BasicLimitInformation       { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { job::query_fixed(job, JobObjectBasicLimitInformation) } } }
impl job::QueryInformation for JOBOBJECT_EXTENDED_LIMIT_INFORMATION     { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { job::query_fixed(job, JobObjectExtendedLimitInformation) } } }
impl job::QueryInformation for job::object::ExtendedLimitInformation    { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { job::query_fixed(job, JobObjectExtendedLimitInformation) } } }

impl job::SetInformation for JOBOBJECT_BASIC_LIMIT_INFORMATION          { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { job::set(job, JobObjectBasicLimitInformation, &self) } } }
impl job::SetInformation for job::object::BasicLimitInformation         { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { job::set(job, JobObjectBasicLimitInformation, &self) } } }
impl job::SetInformation for JOBOBJECT_EXTENDED_LIMIT_INFORMATION       { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { job::set(job, JobObjectExtendedLimitInformation, &self) } } }
impl job::SetInformation for job::object::ExtendedLimitInformation      { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { job::set(job, JobObjectExtendedLimitInformation, &self) } } }
