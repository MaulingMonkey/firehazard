use crate::*;
use winapi::um::winnt::*;
use core::fmt::{self, Debug, Formatter};
use core::mem::{align_of, size_of};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_end_of_job_time_information)\]
/// JOBOBJECT_END_OF_JOB_TIME_INFORMATION
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)] pub struct EndOfJobTimeInformation {
    pub end_of_job_time_action: EndOfJobTimeAction,
}



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_end_of_job_time_information)\]
/// JOBOBJECT_END_OF_JOB_TIME_INFORMATION::EndOfJobTimeAction
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct EndOfJobTimeAction(u32);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_end_of_job_time_information)\]
/// JOB_OBJECT_TERMINATE_AT_END_OF_JOB
pub const TERMINATE_AT_END_OF_JOB : EndOfJobTimeAction = EndOfJobTimeAction(winapi::um::winnt::JOB_OBJECT_TERMINATE_AT_END_OF_JOB);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_end_of_job_time_information)\]
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



const _ : () = assert!(align_of::<EndOfJobTimeInformation>() == align_of::<JOBOBJECT_END_OF_JOB_TIME_INFORMATION>());
const _ : () = assert!(size_of ::<EndOfJobTimeInformation>() == size_of ::<JOBOBJECT_END_OF_JOB_TIME_INFORMATION>());

const _ : () = assert!(align_of::<EndOfJobTimeAction>() == align_of::<JOBOBJECT_END_OF_JOB_TIME_INFORMATION>());
const _ : () = assert!(size_of ::<EndOfJobTimeAction>() == size_of ::<JOBOBJECT_END_OF_JOB_TIME_INFORMATION>());

impl job::QueryInformation for JOBOBJECT_END_OF_JOB_TIME_INFORMATION    { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { job::query_fixed(job, JobObjectEndOfJobTimeInformation) } } }
impl job::QueryInformation for job::object::EndOfJobTimeInformation     { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { job::query_fixed(job, JobObjectEndOfJobTimeInformation) } } }
impl job::QueryInformation for job::object::EndOfJobTimeAction          { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { job::query_fixed(job, JobObjectEndOfJobTimeInformation) } } }

impl job::SetInformation for JOBOBJECT_END_OF_JOB_TIME_INFORMATION      { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { job::set(job, JobObjectEndOfJobTimeInformation, &self) } } }
impl job::SetInformation for job::object::EndOfJobTimeInformation       { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { job::set(job, JobObjectEndOfJobTimeInformation, &self) } } }
impl job::SetInformation for job::object::EndOfJobTimeAction            { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { job::set(job, JobObjectEndOfJobTimeInformation, &self) } } }
