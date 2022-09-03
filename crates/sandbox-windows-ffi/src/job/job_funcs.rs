use crate::*;

use abistr::*;

use winapi::shared::minwindef::FALSE;
use winapi::shared::winerror::*;
use winapi::um::jobapi::*;
use winapi::um::jobapi2::*;
use winapi::um::winbase::*;

use core::ptr::null_mut;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/jobapi2/nf-jobapi2-assignprocesstojobobject)\]
/// AssignProcessToJobObject
///
/// ### Example
/// ```
/// # use sandbox_windows_ffi::*;
/// # use abistr::*;
/// let anon = create_job_object_a(None, ()).unwrap();
/// assign_process_to_job_object(&anon, get_current_process()).unwrap();
/// ```
pub fn assign_process_to_job_object<'a>(job: &job::OwnedHandle, process: impl AsRef<process::PsuedoHandle<'a>>) -> Result<(), Error> {
    Error::get_last_if(FALSE == unsafe { AssignProcessToJobObject(job.as_handle(), process.as_ref().as_handle()) })
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-createjobobjecta)\]
/// CreateJobObjectA
///
/// ### Example
/// ```
/// # use sandbox_windows_ffi::*;
/// # use abistr::*;
/// let anon = create_job_object_a(None, ()).unwrap();
/// let named = create_job_object_a(None, cstr!("Local/win32_security_playground/tests/create_job_object_a")).unwrap();
/// ```
pub fn create_job_object_a(job_attributes: Option<core::convert::Infallible>, name: impl TryIntoAsOptCStr) -> Result<job::OwnedHandle, Error> {
    let name = name.try_into().map_err(|_| Error(E_STRING_NOT_NULL_TERMINATED as _))?;
    let h = unsafe { CreateJobObjectA(none2null(job_attributes), name.as_opt_cstr()) };
    Error::get_last_if(h.is_null())?;
    unsafe { job::OwnedHandle::from_raw(h) }
}


/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/jobapi2/nf-jobapi2-createjobobjectw)\]
/// CreateJobObjectW
///
/// ### Example
/// ```
/// # use sandbox_windows_ffi::*;
/// # use abistr::*;
/// let anon = create_job_object_w(None, ()).unwrap();
/// let named = create_job_object_w(None, cstr16!("Local/win32_security_playground/tests/create_job_object_a")).unwrap();
/// ```
pub fn create_job_object_w(job_attributes: Option<core::convert::Infallible>, name: impl TryIntoAsOptCStr<u16>) -> Result<job::OwnedHandle, Error> {
    let name = name.try_into().map_err(|_| Error(E_STRING_NOT_NULL_TERMINATED as _))?;
    let h = unsafe { CreateJobObjectW(none2null(job_attributes), name.as_opt_cstr()) };
    Error::get_last_if(h.is_null())?;
    unsafe { job::OwnedHandle::from_raw(h) }
}

// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/jobapi2/nf-jobapi2-freememoryjobobject)\]
// FreeMemoryJobObject

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/jobapi/nf-jobapi-isprocessinjob)\]
/// IsProcessInJob
///
/// ### Example
/// ```
/// # use sandbox_windows_ffi::*;
/// # use abistr::*;
/// let job = create_job_object_w(None, ()).unwrap();
/// assert_eq!(Ok(false), is_process_in_job(get_current_process(), Some(&job)));
/// assert_eq!(Ok(true),  is_process_in_job(get_current_process(), None));
/// ```
pub fn is_process_in_job<'a>(process: impl AsRef<process::PsuedoHandle<'a>>, job: Option<&job::OwnedHandle>) -> Result<bool, Error> {
    let mut r = 0;
    Error::get_last_if(FALSE == unsafe { IsProcessInJob(process.as_ref().as_handle(), job.map_or(null_mut(), |j| j.as_handle()), &mut r) })?;
    Ok(r != FALSE)
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-openjobobjecta)\]
/// OpenJobObjectA
///
/// ### Example
/// ```
/// # use sandbox_windows_ffi::*;
/// # use sandbox_windows_ffi::access::*;
/// # use abistr::*;
/// let job1 = create_job_object_a(None, cstr!("Local/win32_security_playground/tests/open_job_object_w")).unwrap();
/// let job2 = open_job_object_a(GENERIC_ALL, false, cstr!("Local/win32_security_playground/tests/open_job_object_w")).unwrap();
/// let err  = open_job_object_a(GENERIC_ALL, false, cstr!("Local/nope")).unwrap_err();
/// ```
pub fn open_job_object_a(desired_access: impl Into<access::Mask>, inherit_handle: bool, name: impl TryIntoAsCStr) -> Result<job::OwnedHandle, Error> {
    let name = name.try_into().map_err(|_| Error(E_STRING_NOT_NULL_TERMINATED as _))?;
    let h = unsafe { OpenJobObjectA(desired_access.into().into(), inherit_handle as _, name.as_cstr()) };
    Error::get_last_if(h.is_null())?;
    unsafe { job::OwnedHandle::from_raw(h) }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/jobapi2/nf-jobapi2-openjobobjectw)\]
/// OpenJobObjectW
///
/// ### Example
/// ```
/// # use sandbox_windows_ffi::*;
/// # use sandbox_windows_ffi::access::*;
/// # use abistr::*;
/// let job1 = create_job_object_w(None, cstr16!("Local/win32_security_playground/tests/open_job_object_a")).unwrap();
/// let job2 = open_job_object_w(GENERIC_ALL, false, cstr16!("Local/win32_security_playground/tests/open_job_object_a")).unwrap();
/// let err  = open_job_object_w(GENERIC_ALL, false, cstr16!("Local/nope")).unwrap_err();
/// ```
pub fn open_job_object_w(desired_access: impl Into<access::Mask>, inherit_handle: bool, name: impl TryIntoAsCStr<u16>) -> Result<job::OwnedHandle, Error> {
    let name = name.try_into().map_err(|_| Error(E_STRING_NOT_NULL_TERMINATED as _))?;
    let h = unsafe { OpenJobObjectW(desired_access.into().into(), inherit_handle as _, name.as_cstr()) };
    Error::get_last_if(h.is_null())?;
    unsafe { job::OwnedHandle::from_raw(h) }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/jobapi2/nf-jobapi2-queryinformationjobobject)\]
/// QueryInformationJobObject
///
/// ### Example
/// ```
/// # use sandbox_windows_ffi::*;
/// # use winapi::shared::winerror::*;
/// # use winapi::um::winnt::*;
/// let job = create_job_object_w(None, ()).unwrap();
/// let restrictions : JOBOBJECT_BASIC_UI_RESTRICTIONS = query_information_job_object(&job).unwrap();
/// ```
pub fn query_information_job_object<Info: job::QueryInformation>(job: &job::OwnedHandle) -> Result<Info, Error> { Info::query_from(job) }

// "Starting with Windows 10, version 1607, this function is no longer supported."
// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/jobapi2/nf-jobapi2-queryioratecontrolinformationjobobject)\]
// QueryIoRateControlInformationJobObject

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/jobapi2/nf-jobapi2-setinformationjobobject)\]
/// SetInformationJobObject
///
/// ### Example
/// ```
/// # use sandbox_windows_ffi::*;
/// # use winapi::shared::winerror::*;
/// let mut job = create_job_object_w(None, ()).unwrap();
///
/// let ui_restrictions_class = unsafe { job::object::uilimit::Flags::from_unchecked(!0) };
/// assert_eq!(ERROR_INVALID_PARAMETER, set_information_job_object(&mut job, job::object::BasicUiRestrictions { ui_restrictions_class }).unwrap_err());
///
/// set_information_job_object(&mut job, job::object::BasicUiRestrictions { ui_restrictions_class: ()
///     | job::object::uilimit::DESKTOP
///     | job::object::uilimit::DISPLAYSETTINGS
///     | job::object::uilimit::EXITWINDOWS
///     | job::object::uilimit::GLOBALATOMS
///     | job::object::uilimit::HANDLES
///     | job::object::uilimit::READCLIPBOARD
///     | job::object::uilimit::SYSTEMPARAMETERS
///     | job::object::uilimit::WRITECLIPBOARD
/// }).unwrap();
/// ```
pub fn set_information_job_object(job: &job::OwnedHandle, information: impl job::SetInformation) -> Result<(), Error> { information.set_on(job) }

// "Starting with Windows 10, version 1607, this function is no longer supported."
// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/jobapi2/nf-jobapi2-setioratecontrolinformationjobobject)\]
// SetIoRateControlInformationJobObject

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/jobapi2/nf-jobapi2-terminatejobobject)\]
/// TerminateJobObject
///
/// ### Example
/// ```
/// # use sandbox_windows_ffi::*;
/// # use winapi::um::winnt::GENERIC_ALL;
/// let job = create_job_object_w(None, ()).unwrap();
/// terminate_job_object(&job, 0).unwrap();
/// ```
pub fn terminate_job_object(job: &job::OwnedHandle, exit_code: u32) -> Result<(), Error> {
    Error::get_last_if(FALSE == unsafe { TerminateJobObject(job.as_handle(), exit_code) })
}
