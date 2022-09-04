use crate::*;
use winapi::um::winnt::{JOBOBJECT_NET_RATE_CONTROL_INFORMATION, JobObjectNetRateControlInformation};
use core::fmt::{self, Debug, Formatter};
use core::mem::{align_of, size_of};



const _ALIGN : () = assert!(align_of::<NetRateControlInformation>() == align_of::<JOBOBJECT_NET_RATE_CONTROL_INFORMATION>());
const _SIZE  : () = assert!(size_of ::<NetRateControlInformation>() == size_of ::<JOBOBJECT_NET_RATE_CONTROL_INFORMATION>());

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_net_rate_control_information)\]
/// JOBOBJECT_NET_RATE_CONTROL_INFORMATION
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)] pub struct NetRateControlInformation {
    /// Limit the maximum number of *outgoing* network traffic bytes for the job.
    ///
    /// Ignored unless [`control_flags`] has [`NET_RATE_CONTROL_ENABLE`] and [`NET_RATE_CONTROL_MAX_BANDWIDTH`] set?
    pub max_bandwidth:  u64,

    /// Control which field(s) of this [`NetRateControlInformation`] to use.
    pub control_flags:  NetRateControlFlags,

    /// Control the [differentiated services code point](https://en.wikipedia.org/wiki/Differentiated_services) tag.
    ///
    /// Ignored unless [`control_flags`] has [`NET_RATE_CONTROL_ENABLE`] and [`NET_RATE_CONTROL_DSCP_TAG`] set?
    pub dscp_tag:       u8,
}

// TODO: utility constructor/builder methods?



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ne-winnt-job_object_net_rate_control_flags)\] JOB_OBJECT_NET_RATE_CONTROL_FLAGS mask
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct NetRateControlFlagsMask(u32);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ne-winnt-job_object_net_rate_control_flags)\] JOB_OBJECT_NET_RATE_CONTROL_FLAGS
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



impl job::QueryInformation for JOBOBJECT_NET_RATE_CONTROL_INFORMATION   { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { job::query_fixed(job, JobObjectNetRateControlInformation) } } }
impl job::QueryInformation for job::object::NetRateControlInformation   { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { job::query_fixed(job, JobObjectNetRateControlInformation) } } }

impl job::SetInformation for JOBOBJECT_NET_RATE_CONTROL_INFORMATION     { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { job::set(job, JobObjectNetRateControlInformation, &self) } } }
impl job::SetInformation for job::object::NetRateControlInformation     { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { job::set(job, JobObjectNetRateControlInformation, &self) } } }
