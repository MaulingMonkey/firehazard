use crate::*;
use winapi::um::winnt::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_limit_violation_information)\]
/// JOBOBJECT_LIMIT_VIOLATION_INFORMATION
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)] pub struct LimitViolationInformation {
    pub limit_flags:                    u32,
    pub violation_limit_flags:          u32,
    pub io_read_bytes:                  u64,
    pub io_read_bytes_limit:            u64,
    pub io_write_bytes:                 u64,
    pub io_write_bytes_limit:           u64,
    pub per_job_user_time:              u64,
    pub per_job_user_time_limit:        u64,
    pub job_memory:                     u64,
    pub job_memory_limit:               u64,
    pub rate_control_tolerance:         job::object::RateControlTolerance,
    pub rate_control_tolerance_limit:   job::object::RateControlTolerance,
}

structure!(@assert layout LimitViolationInformation => JOBOBJECT_LIMIT_VIOLATION_INFORMATION {
    limit_flags                     == LimitFlags,
    violation_limit_flags           == ViolationLimitFlags,
    io_read_bytes                   == IoReadBytes,
    io_read_bytes_limit             == IoReadBytesLimit,
    io_write_bytes                  == IoWriteBytes,
    io_write_bytes_limit            == IoWriteBytesLimit,
    per_job_user_time               == PerJobUserTime,
    per_job_user_time_limit         == PerJobUserTimeLimit,
    job_memory                      == JobMemory,
    job_memory_limit                == JobMemoryLimit,
    rate_control_tolerance          == RateControlTolerance,
    rate_control_tolerance_limit    == RateControlToleranceLimit,
});



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_limit_violation_information_2)\]
/// JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)] pub struct LimitViolationInformation2 {
    pub limit_flags:                        job::object::limit::Flags,
    pub violation_limit_flags:              job::object::limit::Flags,
    pub io_read_bytes:                      u64,
    pub io_read_bytes_limit:                u64,
    pub io_write_bytes:                     u64,
    pub io_write_bytes_limit:               u64,
    pub per_job_user_time:                  u64,
    pub per_job_user_time_limit:            u64,
    pub job_memory:                         u64,
    pub job_high_memory_limit:              u64,
    pub cpu_rate_control_tolerance:         job::object::RateControlTolerance,
    pub cpu_rate_control_tolerance_limit:   job::object::RateControlTolerance,
    pub job_low_memory_limit:               u64,
    pub io_rate_control_tolerance:          job::object::RateControlTolerance,
    pub io_rate_control_tolerance_limit:    job::object::RateControlTolerance,
    pub net_rate_control_tolerance:         job::object::RateControlTolerance,
    pub net_rate_control_tolerance_limit:   job::object::RateControlTolerance,
}

structure!(@assert layout LimitViolationInformation2 => JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2 {
    limit_flags                         == LimitFlags,
    violation_limit_flags               == ViolationLimitFlags,
    io_read_bytes                       == IoReadBytes,
    io_read_bytes_limit                 == IoReadBytesLimit,
    io_write_bytes                      == IoWriteBytes,
    io_write_bytes_limit                == IoWriteBytesLimit,
    per_job_user_time                   == PerJobUserTime,
    per_job_user_time_limit             == PerJobUserTimeLimit,
    job_memory                          == JobMemory,
    job_high_memory_limit               == u1,
    cpu_rate_control_tolerance          == u2,
    cpu_rate_control_tolerance_limit    == u3,
    job_low_memory_limit                == JobLowMemoryLimit,
    io_rate_control_tolerance           == IoRateControlTolerance,
    io_rate_control_tolerance_limit     == IoRateControlToleranceLimit,
    net_rate_control_tolerance          == NetRateControlTolerance,
    net_rate_control_tolerance_limit    == NetRateControlToleranceLimit,
});



impl job::QueryInformationJobObject for JOBOBJECT_LIMIT_VIOLATION_INFORMATION     { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { job::query_fixed(job, JobObjectLimitViolationInformation) } } }
impl job::QueryInformationJobObject for job::object::LimitViolationInformation    { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { job::query_fixed(job, JobObjectLimitViolationInformation) } } }
impl job::QueryInformationJobObject for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2   { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { job::query_fixed(job, JobObjectLimitViolationInformation2) } } }
impl job::QueryInformationJobObject for job::object::LimitViolationInformation2   { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { job::query_fixed(job, JobObjectLimitViolationInformation2) } } }

impl job::SetInformationJobObject for JOBOBJECT_LIMIT_VIOLATION_INFORMATION       { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { job::set(job, JobObjectLimitViolationInformation, &self) } } }
impl job::SetInformationJobObject for job::object::LimitViolationInformation      { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { job::set(job, JobObjectLimitViolationInformation, &self) } } }
impl job::SetInformationJobObject for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2     { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { job::set(job, JobObjectLimitViolationInformation2, &self) } } }
impl job::SetInformationJobObject for job::object::LimitViolationInformation2     { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { job::set(job, JobObjectLimitViolationInformation2, &self) } } }
