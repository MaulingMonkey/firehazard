use crate::*;

use winapi::shared::minwindef::FALSE;
use winapi::shared::winerror::*;
use winapi::um::jobapi2::SetInformationJobObject;
use winapi::um::winnt::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/jobapi2/nf-jobapi2-setinformationjobobject)\]
/// SetInformationJobObject parameters
pub trait SetInformation                                            { fn set_on(self, job: &mut job::OwnedHandle) -> Result<(), Error>; }

impl SetInformation for JOBOBJECT_ASSOCIATE_COMPLETION_PORT         { fn set_on(self, job: &mut job::OwnedHandle) -> Result<(), Error> { unsafe { set(job, JobObjectAssociateCompletionPortInformation, &self) } } }
impl SetInformation for JOBOBJECT_BASIC_LIMIT_INFORMATION           { fn set_on(self, job: &mut job::OwnedHandle) -> Result<(), Error> { unsafe { set(job, JobObjectBasicLimitInformation, &self) } } }
impl SetInformation for JOBOBJECT_BASIC_UI_RESTRICTIONS             { fn set_on(self, job: &mut job::OwnedHandle) -> Result<(), Error> { unsafe { set(job, JobObjectBasicUIRestrictions, &self) } } }
impl SetInformation for JOBOBJECT_CPU_RATE_CONTROL_INFORMATION      { fn set_on(self, job: &mut job::OwnedHandle) -> Result<(), Error> { unsafe { set(job, JobObjectCpuRateControlInformation, &self) } } }
impl SetInformation for JOBOBJECT_END_OF_JOB_TIME_INFORMATION       { fn set_on(self, job: &mut job::OwnedHandle) -> Result<(), Error> { unsafe { set(job, JobObjectEndOfJobTimeInformation, &self) } } }
impl SetInformation for JOBOBJECT_EXTENDED_LIMIT_INFORMATION        { fn set_on(self, job: &mut job::OwnedHandle) -> Result<(), Error> { unsafe { set(job, JobObjectExtendedLimitInformation, &self) } } }
impl SetInformation for &'_ [u16]                                   { fn set_on(self, job: &mut job::OwnedHandle) -> Result<(), Error> { unsafe { set(job, JobObjectGroupInformation, self) } } }
impl SetInformation for &'_ [GROUP_AFFINITY]                        { fn set_on(self, job: &mut job::OwnedHandle) -> Result<(), Error> { unsafe { set(job, JobObjectGroupInformationEx, self) } } }
impl SetInformation for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2     { fn set_on(self, job: &mut job::OwnedHandle) -> Result<(), Error> { unsafe { set(job, JobObjectLimitViolationInformation2, &self) } } }
impl SetInformation for JOBOBJECT_NET_RATE_CONTROL_INFORMATION      { fn set_on(self, job: &mut job::OwnedHandle) -> Result<(), Error> { unsafe { set(job, JobObjectNetRateControlInformation, &self) } } }
impl SetInformation for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION    { fn set_on(self, job: &mut job::OwnedHandle) -> Result<(), Error> { unsafe { set(job, JobObjectNotificationLimitInformation, &self) } } }
impl SetInformation for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2  { fn set_on(self, job: &mut job::OwnedHandle) -> Result<(), Error> { unsafe { set(job, JobObjectNotificationLimitInformation2, &self) } } }
impl SetInformation for JOBOBJECT_SECURITY_LIMIT_INFORMATION        { fn set_on(self, job: &mut job::OwnedHandle) -> Result<(), Error> { unsafe { set(job, JobObjectSecurityLimitInformation, &self) } } }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/jobapi2/nf-jobapi2-setinformationjobobject)\]
/// SetInformationJobObject
unsafe fn set<T: ?Sized>(job: &mut job::OwnedHandle, class: JOBOBJECTINFOCLASS, information: &T) -> Result<(), Error> {
    let size = u32::try_from(std::mem::size_of_val(information)).map_err(|_| Error(ERROR_INVALID_PARAMETER))?;
    let info : *const T = information;
    Error::get_last_if(FALSE == unsafe { SetInformationJobObject(job.as_handle(), class, info as *mut _, size) })
}
