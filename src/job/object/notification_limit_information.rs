use crate::*;
use winapi::um::winnt::*;
use core::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_notification_limit_information)\]
/// JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION
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



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_notification_limit_information_2)\]
/// JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2
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



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_notification_limit_information#members)\]
/// JOBOBJECT_RATE_CONTROL_TOLERANCE
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct RateControlTolerance(u32);

#[allow(non_upper_case_globals)] // enum-like
impl RateControlTolerance {
    /// The job can exceed its CPU rate control limits for 20% of the tolerance interval.
    pub const Low       : Self = Self(ToleranceLow);

    /// The job can exceed its CPU rate control limits for 40% of the tolerance interval.
    pub const Medium    : Self = Self(ToleranceMedium);

    /// The job can exceed its CPU rate control limits for 60% of the tolerance interval.
    pub const High      : Self = Self(ToleranceHigh);
}

impl Debug for RateControlTolerance {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        let f = match *self {
            RateControlTolerance::Low       => "RateControlTolerance::Low",
            RateControlTolerance::Medium    => "RateControlTolerance::Medium",
            RateControlTolerance::High      => "RateControlTolerance::High",
            _ => return write!(fmt, "RateControlTolerance({})", self.0),
        };
        fmt.write_str(f)
    }
}



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_notification_limit_information#members)\]
/// JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct RateControlToleranceInterval(u32);

#[allow(non_upper_case_globals)] // enum-like
impl RateControlToleranceInterval {
    /// The tolerance interval is 10 seconds.
    pub const Short     : Self = Self(ToleranceIntervalShort);

    /// The tolerance interval is one minute.
    pub const Medium    : Self = Self(ToleranceIntervalMedium);

    /// The tolerance interval is 10 minutes.
    pub const Long      : Self = Self(ToleranceIntervalLong);
}

impl Debug for RateControlToleranceInterval {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        let f = match *self {
            RateControlToleranceInterval::Short     => "RateControlToleranceInterval::Short",
            RateControlToleranceInterval::Medium    => "RateControlToleranceInterval::Medium",
            RateControlToleranceInterval::Long      => "RateControlToleranceInterval::Long",
            _ => return write!(fmt, "RateControlToleranceInterval({})", self.0),
        };
        fmt.write_str(f)
    }
}



impl job::QueryInformation for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION     { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { job::query_fixed(job, JobObjectNotificationLimitInformation) } } }
impl job::QueryInformation for job::object::NotificationLimitInformation    { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { job::query_fixed(job, JobObjectNotificationLimitInformation) } } }
impl job::QueryInformation for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2   { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { job::query_fixed(job, JobObjectNotificationLimitInformation2) } } }
impl job::QueryInformation for job::object::NotificationLimitInformation2   { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { job::query_fixed(job, JobObjectNotificationLimitInformation2) } } }

impl job::SetInformation for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION       { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { job::set(job, JobObjectNotificationLimitInformation, &self) } } }
impl job::SetInformation for job::object::NotificationLimitInformation      { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { job::set(job, JobObjectNotificationLimitInformation, &self) } } }
impl job::SetInformation for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2     { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { job::set(job, JobObjectNotificationLimitInformation2, &self) } } }
impl job::SetInformation for job::object::NotificationLimitInformation2     { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { job::set(job, JobObjectNotificationLimitInformation2, &self) } } }
