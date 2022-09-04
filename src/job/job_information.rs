use crate::*;

use winapi::shared::minwindef::FALSE;
use winapi::shared::winerror::*;
use winapi::um::jobapi2::*;
use winapi::um::winnt::*;

use core::mem::{size_of, MaybeUninit};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/jobapi2/nf-jobapi2-queryinformationjobobject)\]
/// QueryInformationJobObject parameters
pub trait QueryInformation : Sized                                          { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error>; }
impl QueryInformation for JOBOBJECT_BASIC_ACCOUNTING_INFORMATION            { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { query_fixed(job, JobObjectBasicAccountingInformation) } } }
impl QueryInformation for JOBOBJECT_BASIC_AND_IO_ACCOUNTING_INFORMATION     { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { query_fixed(job, JobObjectBasicAndIoAccountingInformation) } } }
//impl QueryInformation for JOBOBJECT_BASIC_PROCESS_ID_LIST                   { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { query_header(job, JobObjectBasicProcessIdList) } } } // trailing array
impl QueryInformation for JOBOBJECT_LIMIT_VIOLATION_INFORMATION             { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { query_fixed(job, JobObjectLimitViolationInformation) } } }
impl QueryInformation for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2           { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { query_fixed(job, JobObjectLimitViolationInformation2) } } }
impl QueryInformation for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION          { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { query_fixed(job, JobObjectNotificationLimitInformation) } } }
impl QueryInformation for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2        { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { query_fixed(job, JobObjectNotificationLimitInformation2) } } }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/jobapi2/nf-jobapi2-setinformationjobobject)\]
/// SetInformationJobObject parameters
pub trait SetInformation                                            { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error>; }

impl SetInformation for JOBOBJECT_ASSOCIATE_COMPLETION_PORT         { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { set(job, JobObjectAssociateCompletionPortInformation, &self) } } }
impl SetInformation for JOBOBJECT_LIMIT_VIOLATION_INFORMATION       { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { set(job, JobObjectLimitViolationInformation, &self) } } }
impl SetInformation for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2     { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { set(job, JobObjectLimitViolationInformation2, &self) } } }
impl SetInformation for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION    { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { set(job, JobObjectNotificationLimitInformation, &self) } } }
impl SetInformation for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2  { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { set(job, JobObjectNotificationLimitInformation2, &self) } } }


/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/jobapi2/nf-jobapi2-queryinformationjobobject)\]
/// QueryInformationJobObject
pub(super) unsafe fn query_fixed<T>(job: &job::OwnedHandle, class: JOBOBJECTINFOCLASS) -> Result<T, Error> {
    let mut info = MaybeUninit::<T>::zeroed();
    let size = u32::try_from(core::mem::size_of_val(&info)).map_err(|_| Error(ERROR_INVALID_PARAMETER))?;
    let pinfo : *mut MaybeUninit<T> = &mut info;
    let mut ret_size = 0;
    Error::get_last_if(FALSE == unsafe { QueryInformationJobObject(job.as_handle(), class, pinfo as *mut _, size, &mut ret_size) })?;
    if ret_size > size { return Err(Error(ERROR_BUFFER_OVERFLOW)) }
    if ret_size < size { return Err(Error(ERROR_INVALID_PARAMETER)) }
    Ok(unsafe { info.assume_init() })
}

#[cfg(std)]
/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/jobapi2/nf-jobapi2-queryinformationjobobject)\]
/// QueryInformationJobObject
pub(super) unsafe fn query_vec<T>(job: &job::OwnedHandle, class: JOBOBJECTINFOCLASS) -> Result<Vec<T>, Error> {
    let mut bytes = 0;
    match Error::get_last_if(FALSE == unsafe { QueryInformationJobObject(job.as_handle(), class, core::ptr::null_mut(), 0, &mut bytes) }) {
        Ok(()) if bytes == 0                    => return Ok(Vec::new()),
        Ok(())                                  => {},
        Err(Error(ERROR_INSUFFICIENT_BUFFER))   => {}, // seen for e.g. JobObjectGroupInformationEx (set bytes)
        Err(Error(ERROR_BAD_LENGTH))            => {}, // seen for e.g. JobObjectGroupInformation (doesn't set bytes)
        Err(error)                              => return Err(error),
    }

    let capacity = 1.max(usize::from32(bytes) / size_of::<T>());
    let mut vec = Vec::<T>::new();
    loop {
        vec.reserve(vec.capacity().max(capacity));
        let capacity = u32::try_from(vec.capacity() * size_of::<T>()).unwrap_or(!0u32);

        let mut bytes = 0;
        match Error::get_last_if(FALSE == unsafe { QueryInformationJobObject(job.as_handle(), class, vec.as_mut_ptr().cast(), capacity, &mut bytes) }) {
            Ok(())                                                      => {},
            Err(Error(ERROR_INSUFFICIENT_BUFFER)) if capacity != !0u32  => continue, // seen for e.g. JobObjectGroupInformationEx (set bytes)
            Err(Error(ERROR_BAD_LENGTH))          if capacity != !0u32  => continue, // seen for e.g. JobObjectGroupInformation (doesn't set bytes)
            Err(error)                                                  => return Err(error),
        }

        let bytes = usize::from32(bytes);
        let count = bytes / size_of::<T>();
        assert_eq!(count * size_of::<T>(), bytes, "query_vec: job object supposedly returned fractions of a T");
        unsafe { vec.set_len(count) };
        vec.shrink_to_fit();
        return Ok(vec)
    }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/jobapi2/nf-jobapi2-setinformationjobobject)\]
/// SetInformationJobObject
pub(super) unsafe fn set<T: ?Sized>(job: &job::OwnedHandle, class: JOBOBJECTINFOCLASS, information: &T) -> Result<(), Error> {
    let size = u32::try_from(core::mem::size_of_val(information)).map_err(|_| Error(ERROR_INVALID_PARAMETER))?;
    let info : *const T = information;
    Error::get_last_if(FALSE == unsafe { SetInformationJobObject(job.as_handle(), class, info as *mut _, size) })
}
