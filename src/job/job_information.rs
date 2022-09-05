use crate::*;

use winapi::shared::minwindef::FALSE;
use winapi::shared::winerror::*;
use winapi::um::jobapi2::*;
use winapi::um::winnt::*;

use core::mem::{size_of, MaybeUninit};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/jobapi2/nf-jobapi2-queryinformationjobobject)\]
/// [`query_information_job_object`] result
pub trait QueryInformationJobObject : Sized { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error>; }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/jobapi2/nf-jobapi2-setinformationjobobject)\]
/// [`set_information_job_object`] parameter
pub trait SetInformationJobObject { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error>; }

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
