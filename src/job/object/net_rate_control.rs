use crate::*;
use winapi::um::winnt::{JOBOBJECT_NET_RATE_CONTROL_INFORMATION, JobObjectNetRateControlInformation};
use core::fmt::{self, Debug, Formatter};



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_net_rate_control_information)\]
/// JOBOBJECT_NET_RATE_CONTROL_INFORMATION
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)] pub struct NetRateControlInformation {
    /// Limit the maximum number of *outgoing* network traffic bytes for the job.
    ///
    /// Ignored unless [`control_flags`](Self::control_flags) has [`NET_RATE_CONTROL_ENABLE`] and [`NET_RATE_CONTROL_MAX_BANDWIDTH`] set?
    pub max_bandwidth:  u64,

    /// Control which field(s) of this [`NetRateControlInformation`] to use.
    pub control_flags:  NetRateControlFlags,

    /// Control the [differentiated services code point](https://en.wikipedia.org/wiki/Differentiated_services) tag.
    ///
    /// Ignored unless [`control_flags`](Self::control_flags) has [`NET_RATE_CONTROL_ENABLE`] and [`NET_RATE_CONTROL_DSCP_TAG`] set?
    pub dscp_tag:       u8,
}

structure!(@assert layout NetRateControlInformation => JOBOBJECT_NET_RATE_CONTROL_INFORMATION {
    max_bandwidth   == MaxBandwidth,
    control_flags   == ControlFlags,
    dscp_tag        == DscpTag,
});

impl NetRateControlInformation {
    /// control_flags: 0
    pub fn disabled() -> Self { Self { control_flags: None.into(), max_bandwidth: 0, dscp_tag: 0 } }

    /// control_flags: [`NET_RATE_CONTROL_ENABLE`] | [`NET_RATE_CONTROL_MAX_BANDWIDTH`]
    pub fn enable_max_bandwidth(max_bandwidth: u64) -> Self { Self { control_flags: NET_RATE_CONTROL_ENABLE | NET_RATE_CONTROL_MAX_BANDWIDTH, max_bandwidth, dscp_tag: 0 } }

    /// control_flags: [`NET_RATE_CONTROL_ENABLE`] | [`NET_RATE_CONTROL_DSCP_TAG`]
    pub fn enable_dscp_tag(dscp_tag: u8)            -> Self { Self { control_flags: NET_RATE_CONTROL_ENABLE | NET_RATE_CONTROL_DSCP_TAG, max_bandwidth: 0, dscp_tag } }

    /// control_flags: [`NET_RATE_CONTROL_ENABLE`] | [`NET_RATE_CONTROL_MAX_BANDWIDTH`] | [`NET_RATE_CONTROL_DSCP_TAG`]
    pub fn enable_max_bandwidth_dscp_tag(max_bandwidth: u64, dscp_tag: u8) -> Self { Self {
        max_bandwidth,
        control_flags: NET_RATE_CONTROL_ENABLE | NET_RATE_CONTROL_MAX_BANDWIDTH | NET_RATE_CONTROL_DSCP_TAG,
        dscp_tag
    }}
}



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ne-winnt-job_object_net_rate_control_flags)\] JOB_OBJECT_NET_RATE_CONTROL_FLAGS mask
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct NetRateControlFlagsMask(u32);

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ne-winnt-job_object_net_rate_control_flags)\] JOB_OBJECT_NET_RATE_CONTROL_FLAGS
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct NetRateControlFlags(u32);

flags!(impl .. for NetRateControlFlags(u32) - NetRateControlFlagsMask);

impl NetRateControlFlags {
    /// ### Safety
    /// *   Some APIs might theoretically assume flags are a valid?
    pub const unsafe fn from_unchecked(flags: u32) -> Self { Self(flags) }
}

impl Debug for NetRateControlFlags {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        use winapi::um::winnt::*;
        flags!(self.0, fmt, "0x{:X}", [
            JOB_OBJECT_NET_RATE_CONTROL_ENABLE,
            JOB_OBJECT_NET_RATE_CONTROL_MAX_BANDWIDTH,
            JOB_OBJECT_NET_RATE_CONTROL_DSCP_TAG,
        ])
    }
}

/// Turns on the control of the network traffic.
/// You must set this value if you also set either [job::object::NET_RATE_CONTROL_MAX_BANDWIDTH] or [job::object::NET_RATE_CONTROL_DSCP_TAG].
pub const NET_RATE_CONTROL_ENABLE           : NetRateControlFlags       = NetRateControlFlags(winapi::um::winnt::JOB_OBJECT_NET_RATE_CONTROL_ENABLE);

/// Use the [`NetRateControlInformation::max_bandwidth`] field to limit the total *outgoing* bandwidth used over the lifetime of the job.
pub const NET_RATE_CONTROL_MAX_BANDWIDTH    : NetRateControlFlags       = NetRateControlFlags(winapi::um::winnt::JOB_OBJECT_NET_RATE_CONTROL_MAX_BANDWIDTH);

/// Use the [`NetRateControlInformation::dscp_tag`] field to set the [differentiated services code point](https://en.wikipedia.org/wiki/Differentiated_services) tag.
pub const NET_RATE_CONTROL_DSCP_TAG         : NetRateControlFlags       = NetRateControlFlags(winapi::um::winnt::JOB_OBJECT_NET_RATE_CONTROL_DSCP_TAG);

pub const NET_RATE_CONTROL_VALID_FLAGS      : NetRateControlFlagsMask   = NetRateControlFlagsMask(winapi::um::winnt::JOB_OBJECT_NET_RATE_CONTROL_VALID_FLAGS);



impl job::QueryInformationJobObject for JOBOBJECT_NET_RATE_CONTROL_INFORMATION   { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { job::query_fixed(job, JobObjectNetRateControlInformation) } } }
impl job::QueryInformationJobObject for job::object::NetRateControlInformation   { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { job::query_fixed(job, JobObjectNetRateControlInformation) } } }

impl job::SetInformationJobObject for JOBOBJECT_NET_RATE_CONTROL_INFORMATION     { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { job::set(job, JobObjectNetRateControlInformation, &self) } } }
impl job::SetInformationJobObject for job::object::NetRateControlInformation     { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { job::set(job, JobObjectNetRateControlInformation, &self) } } }
