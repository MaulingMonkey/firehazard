use crate::*;
use winapi::um::winnt::*;
use core::fmt::{self, Debug, Formatter};



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_end_of_job_time_information)\]
/// JOBOBJECT_END_OF_JOB_TIME_INFORMATION
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)] pub struct EndOfJobTimeInformation {
    pub end_of_job_time_action: EndOfJobTimeAction,
}

structure!(@assert layout EndOfJobTimeInformation => JOBOBJECT_END_OF_JOB_TIME_INFORMATION {
    end_of_job_time_action  == EndOfJobTimeAction,
});

structure!(@assert layout EndOfJobTimeAction => JOBOBJECT_END_OF_JOB_TIME_INFORMATION {});



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_end_of_job_time_information)\]
/// JOBOBJECT_END_OF_JOB_TIME_INFORMATION::EndOfJobTimeAction
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct EndOfJobTimeAction(u32);

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_end_of_job_time_information)\]
/// JOB_OBJECT_TERMINATE_AT_END_OF_JOB
pub const TERMINATE_AT_END_OF_JOB : EndOfJobTimeAction = EndOfJobTimeAction(winapi::um::winnt::JOB_OBJECT_TERMINATE_AT_END_OF_JOB);

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_end_of_job_time_information)\]
/// JOB_OBJECT_POST_AT_END_OF_JOB
pub const POST_AT_END_OF_JOB : EndOfJobTimeAction = EndOfJobTimeAction(winapi::um::winnt::JOB_OBJECT_POST_AT_END_OF_JOB);

impl Debug for EndOfJobTimeAction {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        let friendly = match *self {
            job::object::TERMINATE_AT_END_OF_JOB    => "JOB_OBJECT_TERMINATE_AT_END_OF_JOB",
            job::object::POST_AT_END_OF_JOB         => "JOB_OBJECT_POST_AT_END_OF_JOB",
            _                                       => return write!(fmt, "JOB_OBJECT_??? ({})", self.0),
        };
        fmt.write_str(friendly)
    }
}



impl job::QueryInformationJobObject for JOBOBJECT_END_OF_JOB_TIME_INFORMATION    { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { job::query_fixed(job, JobObjectEndOfJobTimeInformation) } } }
impl job::QueryInformationJobObject for job::object::EndOfJobTimeInformation     { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { job::query_fixed(job, JobObjectEndOfJobTimeInformation) } } }
impl job::QueryInformationJobObject for job::object::EndOfJobTimeAction          { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { job::query_fixed(job, JobObjectEndOfJobTimeInformation) } } }

impl job::SetInformationJobObject for JOBOBJECT_END_OF_JOB_TIME_INFORMATION      { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { job::set(job, JobObjectEndOfJobTimeInformation, &self) } } }
impl job::SetInformationJobObject for job::object::EndOfJobTimeInformation       { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { job::set(job, JobObjectEndOfJobTimeInformation, &self) } } }
impl job::SetInformationJobObject for job::object::EndOfJobTimeAction            { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { job::set(job, JobObjectEndOfJobTimeInformation, &self) } } }
