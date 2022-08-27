use core::mem::MaybeUninit;

use crate::*;

use winapi::shared::minwindef::FALSE;
use winapi::shared::winerror::*;
use winapi::um::jobapi2::*;
use winapi::um::winnt::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/jobapi2/nf-jobapi2-queryinformationjobobject)\]
/// QueryInformationJobObject parameters
pub trait QueryInformation : Sized                                          { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error>; }
impl QueryInformation for JOBOBJECT_BASIC_ACCOUNTING_INFORMATION            { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { query_fixed(job, JobObjectBasicAccountingInformation) } } }
impl QueryInformation for JOBOBJECT_BASIC_AND_IO_ACCOUNTING_INFORMATION     { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { query_fixed(job, JobObjectBasicAndIoAccountingInformation) } } }
impl QueryInformation for JOBOBJECT_BASIC_LIMIT_INFORMATION                 { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { query_fixed(job, JobObjectBasicLimitInformation) } } }
//impl QueryInformation for JOBOBJECT_BASIC_PROCESS_ID_LIST                   { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { query_header(job, JobObjectBasicProcessIdList) } } } // trailing array
impl QueryInformation for JOBOBJECT_BASIC_UI_RESTRICTIONS                   { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { query_fixed(job, JobObjectBasicUIRestrictions) } } }
impl QueryInformation for JOBOBJECT_CPU_RATE_CONTROL_INFORMATION            { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { query_fixed(job, JobObjectCpuRateControlInformation) } } }
impl QueryInformation for JOBOBJECT_END_OF_JOB_TIME_INFORMATION             { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { query_fixed(job, JobObjectEndOfJobTimeInformation) } } }
impl QueryInformation for JOBOBJECT_EXTENDED_LIMIT_INFORMATION              { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { query_fixed(job, JobObjectExtendedLimitInformation) } } }
//impl QueryInformation for Vec<u16>                                          { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { query_???(job, JobObjectGroupInformation) } } }
//impl QueryInformation for Vec<GROUP_AFFINITY>                               { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { query_???(job, JobObjectGroupInformationEx) } } }
impl QueryInformation for JOBOBJECT_LIMIT_VIOLATION_INFORMATION             { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { query_fixed(job, JobObjectLimitViolationInformation) } } }
impl QueryInformation for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2           { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { query_fixed(job, JobObjectLimitViolationInformation2) } } }
impl QueryInformation for JOBOBJECT_NET_RATE_CONTROL_INFORMATION            { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { query_fixed(job, JobObjectNetRateControlInformation) } } }
impl QueryInformation for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION          { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { query_fixed(job, JobObjectNotificationLimitInformation) } } }
impl QueryInformation for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2        { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { query_fixed(job, JobObjectNotificationLimitInformation2) } } }
//impl QueryInformation for  JOBOBJECT_SECURITY_LIMIT_INFORMATION             { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { query_header(job, JobObjectSecurityLimitInformation) } } } // self-referential pointers? (sids)

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/jobapi2/nf-jobapi2-setinformationjobobject)\]
/// SetInformationJobObject parameters
pub trait SetInformation                                            { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error>; }

impl SetInformation for JOBOBJECT_ASSOCIATE_COMPLETION_PORT         { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { set(job, JobObjectAssociateCompletionPortInformation, &self) } } }
impl SetInformation for JOBOBJECT_BASIC_LIMIT_INFORMATION           { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { set(job, JobObjectBasicLimitInformation, &self) } } }
impl SetInformation for JOBOBJECT_BASIC_UI_RESTRICTIONS             { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { set(job, JobObjectBasicUIRestrictions, &self) } } }
impl SetInformation for JOBOBJECT_CPU_RATE_CONTROL_INFORMATION      { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { set(job, JobObjectCpuRateControlInformation, &self) } } }
impl SetInformation for JOBOBJECT_END_OF_JOB_TIME_INFORMATION       { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { set(job, JobObjectEndOfJobTimeInformation, &self) } } }
impl SetInformation for JOBOBJECT_EXTENDED_LIMIT_INFORMATION        { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { set(job, JobObjectExtendedLimitInformation, &self) } } }
impl SetInformation for &'_ [u16]                                   { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { set(job, JobObjectGroupInformation, self) } } }
impl SetInformation for &'_ [GROUP_AFFINITY]                        { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { set(job, JobObjectGroupInformationEx, self) } } }
impl SetInformation for JOBOBJECT_LIMIT_VIOLATION_INFORMATION       { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { set(job, JobObjectLimitViolationInformation, &self) } } }
impl SetInformation for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2     { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { set(job, JobObjectLimitViolationInformation2, &self) } } }
impl SetInformation for JOBOBJECT_NET_RATE_CONTROL_INFORMATION      { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { set(job, JobObjectNetRateControlInformation, &self) } } }
impl SetInformation for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION    { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { set(job, JobObjectNotificationLimitInformation, &self) } } }
impl SetInformation for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2  { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { set(job, JobObjectNotificationLimitInformation2, &self) } } }
//impl SetInformation for JOBOBJECT_SECURITY_LIMIT_INFORMATION        { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { set(job, JobObjectSecurityLimitInformation, &self) } } } // interior pointers


/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/jobapi2/nf-jobapi2-queryinformationjobobject)\]
/// QueryInformationJobObject
unsafe fn query_fixed<T>(job: &job::OwnedHandle, class: JOBOBJECTINFOCLASS) -> Result<T, Error> {
    let mut info = MaybeUninit::<T>::zeroed();
    let size = u32::try_from(core::mem::size_of_val(&info)).map_err(|_| Error(ERROR_INVALID_PARAMETER))?;
    let pinfo : *mut MaybeUninit<T> = &mut info;
    let mut ret_size = 0;
    Error::get_last_if(FALSE == unsafe { QueryInformationJobObject(job.as_handle(), class, pinfo as *mut _, size, &mut ret_size) })?;
    if ret_size > size { return Err(Error(ERROR_BUFFER_OVERFLOW)) }
    if ret_size < size { return Err(Error(ERROR_INVALID_PARAMETER)) }
    Ok(unsafe { info.assume_init() })
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/jobapi2/nf-jobapi2-setinformationjobobject)\]
/// SetInformationJobObject
unsafe fn set<T: ?Sized>(job: &job::OwnedHandle, class: JOBOBJECTINFOCLASS, information: &T) -> Result<(), Error> {
    let size = u32::try_from(core::mem::size_of_val(information)).map_err(|_| Error(ERROR_INVALID_PARAMETER))?;
    let info : *const T = information;
    Error::get_last_if(FALSE == unsafe { SetInformationJobObject(job.as_handle(), class, info as *mut _, size) })
}
