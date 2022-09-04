use crate::*;
use winapi::um::winnt::*;
use core::fmt::{self, Debug, Formatter};
use core::mem::{align_of, size_of};



const _ALIGN : () = assert!(align_of::<CpuRateControlInformation>() == align_of::<JOBOBJECT_CPU_RATE_CONTROL_INFORMATION>());
const _SIZE  : () = assert!(size_of ::<CpuRateControlInformation>() == size_of ::<JOBOBJECT_CPU_RATE_CONTROL_INFORMATION>());

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_cpu_rate_control_information)\]
/// JOBOBJECT_CPU_RATE_CONTROL_INFORMATION
#[derive(Clone, Copy, Default)]
#[repr(C)] pub struct CpuRateControlInformation {
    control_flags:  CpuRateControlFlags,
    value:          u32, // interpretation varies wildly based on `control_flags`
}

impl CpuRateControlInformation {
    pub fn disabled() -> Self { Self {
        control_flags:  CpuRateControlFlags::default(),
        value:          0,
    }}

    /// Specifies the portion of processor cycles that the threads in a job object can use during each scheduling interval, as the number of cycles per 10,000 cycles.
    ///
    /// Set CpuRate to a percentage times 100.
    /// For example, to let the job use 20% of the CPU, set CpuRate to 20 times 100, or 2,000.
    ///
    /// Do not set `cpu_rate` to 0.
    /// If `cpu_rate` is 0, `set_information_job_object` returns `INVALID_ARGS`.
    pub fn from_cpu_rate(cpu_rate: u32, hard_cap: bool, notify: bool) -> Self { Self {
        control_flags:  CPU_RATE_CONTROL_ENABLE | (hard_cap * CPU_RATE_CONTROL_HARD_CAP) | (notify * CPU_RATE_CONTROL_NOTIFY),
        value:          cpu_rate,
    }}

    /// Specifies the scheduling weight of the job object, which determines the share of processor time given to the job relative to other workloads on the processor.
    ///
    /// `weight` can be a value from 1 through 9, where 1 is the smallest share and 9 is the largest share.
    /// The default is 5, which should be used for most workloads.
    pub fn from_weight(weight: u32, hard_cap: bool, notify: bool) -> Self { Self {
        control_flags:  CPU_RATE_CONTROL_ENABLE | (hard_cap * CPU_RATE_CONTROL_HARD_CAP) | (notify * CPU_RATE_CONTROL_NOTIFY) | CPU_RATE_CONTROL_WEIGHT_BASED,
        value:          weight
    }}

    /// Specifies the minimum and maximum portion of the processor cycles that the threads in a job object can reserve during each scheduling interval.
    /// Specify this rate as a percentage times 100. For example, to set a minimum or maximum rate of 50%, specify 50 times 100, or 5,000.
    ///
    /// For the minimum rates to work correctly, the sum of the minimum rates for all of the job objects in the system cannot exceed 10,000, which is the equivalent of 100%.
    ///
    /// After the job reaches this limit for a scheduling interval, no threads associated with the job can run until the next scheduling interval.
    pub fn from_min_max_rate(min_rate: u16, max_rate: u16, notify: bool) -> Self { Self {
        control_flags:  CPU_RATE_CONTROL_ENABLE | CPU_RATE_CONTROL_MIN_MAX_RATE | (notify * CPU_RATE_CONTROL_NOTIFY),
        value:          unsafe { core::mem::transmute([min_rate, max_rate]) }
    }}

    pub fn control_flags(&self) -> CpuRateControlFlags { self.control_flags }

    pub fn is_enabled   (&self) -> bool { self.control_flags & CPU_RATE_CONTROL_ENABLE   != CpuRateControlFlags::default() }

    /// The job's CPU rate is a hard limit.
    /// After the job reaches its CPU cycle limit for the current scheduling interval, no threads associated with the job will run until the next interval.
    pub fn is_hard_cap  (&self) -> bool { self.control_flags & CPU_RATE_CONTROL_HARD_CAP != CpuRateControlFlags::default() }

    /// Sends messages when the CPU rate for the job exceeds the rate limits for the job during the tolerance interval.
    pub fn is_notify    (&self) -> bool { self.control_flags & CPU_RATE_CONTROL_NOTIFY   != CpuRateControlFlags::default() }

    pub fn cpu_rate     (&self) -> Option<u32>      { ((self.control_flags & !CPU_RATE_CONTROL_HARD_CAP & !CPU_RATE_CONTROL_NOTIFY) == CPU_RATE_CONTROL_ENABLE                                   ).then_some(self.value) }
    pub fn weight       (&self) -> Option<u32>      { ((self.control_flags & !CPU_RATE_CONTROL_HARD_CAP & !CPU_RATE_CONTROL_NOTIFY) == CPU_RATE_CONTROL_ENABLE | CPU_RATE_CONTROL_WEIGHT_BASED   ).then_some(self.value) }
    pub fn min_max_rate (&self) -> Option<[u16; 2]> { ((self.control_flags & !CPU_RATE_CONTROL_HARD_CAP & !CPU_RATE_CONTROL_NOTIFY) == CPU_RATE_CONTROL_ENABLE | CPU_RATE_CONTROL_MIN_MAX_RATE   ).then_some(unsafe { core::mem::transmute(self.value) }) }
}

impl Debug for CpuRateControlInformation {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        let mut s = fmt.debug_struct("CpuRateControlInformation");
        s.field("control_flags", &self.control_flags);
        if let Some(cpu_rate) = self.cpu_rate() {
            s.field("cpu_rate", &cpu_rate).finish()
        } else if let Some(weight) = self.weight() {
            s.field("weight", &weight).finish()
        } else if let Some(min_max_rate) = self.min_max_rate() {
            s.field("min_max_rate", &min_max_rate).finish()
        } else {
            s.finish_non_exhaustive()
        }
    }
}



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_cpu_rate_control_information#members)\] JOBOBJECT_CPU_RATE_CONTROL_INFORMATION::ControlFlags mask
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct CpuRateControlFlagsMask(u32);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_cpu_rate_control_information#members)\] JOBOBJECT_CPU_RATE_CONTROL_INFORMATION::ControlFlags
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct CpuRateControlFlags(u32);

flags!(impl .. for CpuRateControlFlags(u32) - CpuRateControlFlagsMask);

impl CpuRateControlFlags {
    /// ### Safety
    /// *   Some APIs might theoretically assume flags are a valid?
    pub const unsafe fn from_unchecked(flags: u32) -> Self { Self(flags) }
}

impl Debug for CpuRateControlFlags {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        use winapi::um::winnt::*;
        flags!(self.0, fmt, "0x{:X}", [
            JOB_OBJECT_CPU_RATE_CONTROL_ENABLE,
            JOB_OBJECT_CPU_RATE_CONTROL_WEIGHT_BASED,
            JOB_OBJECT_CPU_RATE_CONTROL_HARD_CAP,
            JOB_OBJECT_CPU_RATE_CONTROL_NOTIFY,
            JOB_OBJECT_CPU_RATE_CONTROL_MIN_MAX_RATE,
        ])
    }
}



/// This flag enables the job's CPU rate to be controlled based on weight or hard cap.
/// You must set this value if you also set JOB_OBJECT_CPU_RATE_CONTROL_WEIGHT_BASED, JOB_OBJECT_CPU_RATE_CONTROL_HARD_CAP, or JOB_OBJECT_CPU_RATE_CONTROL_MIN_MAX_RATE.
pub const CPU_RATE_CONTROL_ENABLE : CpuRateControlFlags = CpuRateControlFlags(JOB_OBJECT_CPU_RATE_CONTROL_ENABLE);

/// The job's CPU rate is calculated based on its relative weight to the weight of other jobs.
/// If this flag is set, the Weight member contains more information.
/// If this flag is clear, the CpuRate member contains more information.
/// If you set JOB_OBJECT_CPU_RATE_CONTROL_WEIGHT_BASED, you cannot also set JOB_OBJECT_CPU_RATE_CONTROL_MIN_MAX_RATE.
pub const CPU_RATE_CONTROL_WEIGHT_BASED : CpuRateControlFlags = CpuRateControlFlags(JOB_OBJECT_CPU_RATE_CONTROL_WEIGHT_BASED);

/// The job's CPU rate is a hard limit. After the job reaches its CPU cycle limit for the current scheduling interval, no threads associated with the job will run until the next interval.
/// If you set JOB_OBJECT_CPU_RATE_CONTROL_HARD_CAP, you cannot also set JOB_OBJECT_CPU_RATE_CONTROL_MIN_MAX_RATE.
pub const CPU_RATE_CONTROL_HARD_CAP : CpuRateControlFlags = CpuRateControlFlags(JOB_OBJECT_CPU_RATE_CONTROL_HARD_CAP);

/// Sends messages when the CPU rate for the job exceeds the rate limits for the job during the tolerance interval.
pub const CPU_RATE_CONTROL_NOTIFY : CpuRateControlFlags = CpuRateControlFlags(JOB_OBJECT_CPU_RATE_CONTROL_NOTIFY);

/// The CPU rate for the job is limited by minimum and maximum rates that you specify in the MinRate and MaxRate members.
/// If you set JOB_OBJECT_CPU_RATE_CONTROL_MIN_MAX_RATE, you can set neither JOB_OBJECT_CPU_RATE_CONTROL_WEIGHT_BASED nor JOB_OBJECT_CPU_RATE_CONTROL_HARD_CAP.
pub const CPU_RATE_CONTROL_MIN_MAX_RATE : CpuRateControlFlags = CpuRateControlFlags(JOB_OBJECT_CPU_RATE_CONTROL_MIN_MAX_RATE);



impl job::QueryInformation for JOBOBJECT_CPU_RATE_CONTROL_INFORMATION   { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { job::query_fixed(job, JobObjectCpuRateControlInformation) } } }
impl job::QueryInformation for job::object::CpuRateControlInformation   { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { job::query_fixed(job, JobObjectCpuRateControlInformation) } } }

impl job::SetInformation for JOBOBJECT_CPU_RATE_CONTROL_INFORMATION     { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { job::set(job, JobObjectCpuRateControlInformation, &self) } } }
impl job::SetInformation for job::object::CpuRateControlInformation     { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { job::set(job, JobObjectCpuRateControlInformation, &self) } } }
