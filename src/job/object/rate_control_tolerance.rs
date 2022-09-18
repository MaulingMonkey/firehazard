use crate::*;
use winapi::um::winnt::*;
use core::fmt::{self, Debug, Formatter};



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_notification_limit_information#members)\]
/// JOBOBJECT_RATE_CONTROL_TOLERANCE
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct RateControlTolerance(u32);

#[allow(non_upper_case_globals)] // enum-like
impl RateControlTolerance {
    /// The job can or has exceeded its CPU, I/O, or network rate control limits for 20% of the tolerance interval.
    pub const Low       : Self = Self(ToleranceLow);

    /// The job can or has exceeded its CPU, I/O, or network rate control limits for 40% of the tolerance interval.
    pub const Medium    : Self = Self(ToleranceMedium);

    /// The job can or has exceeded its CPU, I/O, or network rate control limits for 60% of the tolerance interval.
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



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_notification_limit_information#members)\]
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
