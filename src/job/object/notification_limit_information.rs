use crate::prelude::*;
use winapi::um::winnt::*;



#[doc(alias = "JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_notification_limit_information)\]
/// JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION
///
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)] pub struct NotificationLimitInformation {
    pub io_read_bytes_limit:                u64,
    pub io_write_bytes_limit:               u64,
    pub per_job_user_time_limit:            u64,
    pub job_memory_limit:                   u64,
    pub rate_control_tolerance:             job::object::RateControlTolerance,
    pub rate_control_tolerance_interval:    job::object::RateControlToleranceInterval,
    pub limit_flags:                        job::object::limit::Flags,
}

structure!(@assert layout NotificationLimitInformation => JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION {
    io_read_bytes_limit             == IoReadBytesLimit,
    io_write_bytes_limit            == IoWriteBytesLimit,
    per_job_user_time_limit         == PerJobUserTimeLimit,
    job_memory_limit                == JobMemoryLimit,
    rate_control_tolerance          == RateControlTolerance,
    rate_control_tolerance_interval == RateControlToleranceInterval,
    limit_flags                     == LimitFlags,
});



#[doc(alias = "JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_notification_limit_information_2)\]
/// JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2
///
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)] pub struct NotificationLimitInformation2 {
    pub io_read_bytes_limit:                    u64,
    pub io_write_bytes_limit:                   u64,
    pub per_job_user_time_limit:                u64,
    pub job_high_memory_limit:                  u64,                                        // aka job_memory_limit
    pub cpu_rate_control_tolerance:             job::object::RateControlTolerance,          // aka rate_control_tolerance
    pub cpu_rate_control_tolerance_interval:    job::object::RateControlToleranceInterval,  // aka rate_control_tolerance_interval
    pub limit_flags:                            job::object::limit::Flags,
    pub io_rate_control_tolerance:              job::object::RateControlTolerance,
    pub job_low_memory_limit:                   u64,
    pub io_rate_control_tolerance_interval:     job::object::RateControlToleranceInterval,
    pub net_rate_control_tolerance:             job::object::RateControlTolerance,
    pub net_rate_control_tolerance_interval:    job::object::RateControlToleranceInterval,
}

structure!(@assert layout NotificationLimitInformation2 => JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2 {
    io_read_bytes_limit                 == IoReadBytesLimit,
    io_write_bytes_limit                == IoWriteBytesLimit,
    per_job_user_time_limit             == PerJobUserTimeLimit,
    job_high_memory_limit               == u1,
    cpu_rate_control_tolerance          == u2,
    cpu_rate_control_tolerance_interval == u3,
    limit_flags                         == LimitFlags,
    io_rate_control_tolerance           == IoRateControlTolerance,
    job_low_memory_limit                == JobLowMemoryLimit,
    io_rate_control_tolerance_interval  == IoRateControlToleranceInterval,
    net_rate_control_tolerance          == NetRateControlTolerance,
    net_rate_control_tolerance_interval == NetRateControlToleranceInterval,
});



impl job::QueryInformationJobObject for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION     { fn query_from(job: &job::OwnedHandle) -> firehazard::Result<Self> { unsafe { job::query_fixed(job, JobObjectNotificationLimitInformation) } } }
impl job::QueryInformationJobObject for job::object::NotificationLimitInformation    { fn query_from(job: &job::OwnedHandle) -> firehazard::Result<Self> { unsafe { job::query_fixed(job, JobObjectNotificationLimitInformation) } } }
impl job::QueryInformationJobObject for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2   { fn query_from(job: &job::OwnedHandle) -> firehazard::Result<Self> { unsafe { job::query_fixed(job, JobObjectNotificationLimitInformation2) } } }
impl job::QueryInformationJobObject for job::object::NotificationLimitInformation2   { fn query_from(job: &job::OwnedHandle) -> firehazard::Result<Self> { unsafe { job::query_fixed(job, JobObjectNotificationLimitInformation2) } } }

impl job::SetInformationJobObject for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION       { fn set_on(self, job: &job::OwnedHandle) -> firehazard::Result<()> { unsafe { job::set(job, JobObjectNotificationLimitInformation, &self) } } }
impl job::SetInformationJobObject for job::object::NotificationLimitInformation      { fn set_on(self, job: &job::OwnedHandle) -> firehazard::Result<()> { unsafe { job::set(job, JobObjectNotificationLimitInformation, &self) } } }
impl job::SetInformationJobObject for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2     { fn set_on(self, job: &job::OwnedHandle) -> firehazard::Result<()> { unsafe { job::set(job, JobObjectNotificationLimitInformation2, &self) } } }
impl job::SetInformationJobObject for job::object::NotificationLimitInformation2     { fn set_on(self, job: &job::OwnedHandle) -> firehazard::Result<()> { unsafe { job::set(job, JobObjectNotificationLimitInformation2, &self) } } }
