use crate::prelude::*;

use winapi::um::jobapi::*;
use winapi::um::jobapi2::*;
use winapi::um::winbase::*;



#[doc(alias = "AssignProcessToJobObject")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/jobapi2/nf-jobapi2-assignprocesstojobobject)\]
/// AssignProcessToJobObject
///
/// ### Example
/// ```
/// # use firehazard::*;
/// # use abistr::*;
/// let anon = create_job_object_a(None, ()).unwrap();
/// assign_process_to_job_object(&anon, get_current_process()).unwrap();
/// ```
///
pub fn assign_process_to_job_object<'a>(
    job:            &job::OwnedHandle,
    process:        impl Into<process::PseudoHandle<'a>>,
) -> firehazard::Result<()> {
    firehazard::Error::get_last_if(FALSE == unsafe { AssignProcessToJobObject(
        job.as_handle(),
        process.into().as_handle()
    )})
}



#[doc(no_inline)] pub use create_job_object_w as create_job_object;



#[doc(alias = "CreateJobObjectA")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-createjobobjecta)\]
/// CreateJobObjectA
///
/// ### Example
/// ```
/// # use firehazard::*;
/// # use abistr::*;
/// let anon = create_job_object_a(None, ()).unwrap();
/// let named = create_job_object_a(None, cstr!("Local/win32_security_playground/tests/create_job_object_a")).unwrap();
/// ```
///
pub fn create_job_object_a(
    job_attributes:     Option<core::convert::Infallible>,
    name:               impl string::InOptionalNarrow,
) -> firehazard::Result<job::OwnedHandle> {
    string::convert_to_cstr::<{limit::stack::JOB_NAME}, _, _>(name, |name| {
        let h = unsafe { CreateJobObjectA(none2null(job_attributes), name.as_opt_cstr()) };
        firehazard::Error::get_last_if(h.is_null())?;
        unsafe { job::OwnedHandle::from_raw(h) }
    })?
}



#[doc(alias = "CreateJobObject")]
#[doc(alias = "CreateJobObjectW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/jobapi2/nf-jobapi2-createjobobjectw)\]
/// CreateJobObjectW
///
/// ### Example
/// ```
/// # use firehazard::*;
/// # use abistr::*;
/// let anon = create_job_object_w(None, ()).unwrap();
/// let named = create_job_object_w(None, cstr16!("Local/win32_security_playground/tests/create_job_object_w")).unwrap();
/// ```
///
pub fn create_job_object_w(
    job_attributes:     Option<core::convert::Infallible>,
    name:               impl string::InOptionalWide,
) -> firehazard::Result<job::OwnedHandle> {
    string::convert_to_cstr::<{limit::stack::JOB_NAME}, _, _>(name, |name| {
        let h = unsafe { CreateJobObjectW(none2null(job_attributes), name.as_opt_cstr()) };
        firehazard::Error::get_last_if(h.is_null())?;
        unsafe { job::OwnedHandle::from_raw(h) }
    })?
}



// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/jobapi2/nf-jobapi2-freememoryjobobject)\]
// FreeMemoryJobObject



#[doc(alias = "IsProcessInJob")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/jobapi/nf-jobapi-isprocessinjob)\]
/// IsProcessInJob
///
/// ### Example
/// ```
/// # use firehazard::*;
/// # use abistr::*;
/// let job = create_job_object_w(None, ()).unwrap();
/// assert_eq!(Ok(false), is_process_in_job(get_current_process(), Some(&job)));
/// assert_eq!(Ok(true),  is_process_in_job(get_current_process(), None));
/// ```
///
pub fn is_process_in_job<'a>(
    process:        impl Into<process::PseudoHandle<'a>>,
    job:            Option<&job::OwnedHandle>,
) -> firehazard::Result<bool> {
    let mut r = 0;
    firehazard::Error::get_last_if(FALSE == unsafe { IsProcessInJob(
        process.into().as_handle(),
        job.map_or(null_mut(), |j| j.as_handle()),
        &mut r,
    )})?;
    Ok(r != FALSE)
}



#[doc(no_inline)] pub use open_job_object_w as open_job_object;



#[doc(alias = "OpenJobObjectA")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-openjobobjecta)\]
/// OpenJobObjectA
///
/// ### Example
/// ```
/// # use firehazard::*;
/// # use firehazard::access::*;
/// # use abistr::*;
/// let job1 = create_job_object_a(None, cstr!("Local/win32_security_playground/tests/open_job_object_a")).unwrap();
/// let job2 = open_job_object_a(GENERIC_ALL, false, cstr!("Local/win32_security_playground/tests/open_job_object_a")).unwrap();
/// let err  = open_job_object_a(GENERIC_ALL, false, cstr!("Local/nope")).unwrap_err();
/// ```
///
pub fn open_job_object_a(
    desired_access:     impl Into<access::Mask>,
    inherit_handle:     bool,
    name:               impl string::InNarrow,
) -> firehazard::Result<job::OwnedHandle> {
    let desired_access  = desired_access.into().into();
    let inherit_handle  = inherit_handle as _;

    string::convert_to_cstr::<{limit::stack::JOB_NAME}, _, _>(name, |name| {
        let h = unsafe { OpenJobObjectA(desired_access, inherit_handle, name.as_opt_cstr()) };
        firehazard::Error::get_last_if(h.is_null())?;
        unsafe { job::OwnedHandle::from_raw(h) }
    })?
}



#[doc(alias = "OpenJobObject")]
#[doc(alias = "OpenJobObjectW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/jobapi2/nf-jobapi2-openjobobjectw)\]
/// OpenJobObjectW
///
/// ### Example
/// ```
/// # use firehazard::*;
/// # use firehazard::access::*;
/// # use abistr::*;
/// let job1 = create_job_object_w(None, cstr16!("Local/win32_security_playground/tests/open_job_object_w")).unwrap();
/// let job2 = open_job_object_w(GENERIC_ALL, false, cstr16!("Local/win32_security_playground/tests/open_job_object_w")).unwrap();
/// let err  = open_job_object_w(GENERIC_ALL, false, cstr16!("Local/nope")).unwrap_err();
/// ```
///
pub fn open_job_object_w(
    desired_access:     impl Into<access::Mask>,
    inherit_handle:     bool,
    name:               impl string::InWide,
) -> firehazard::Result<job::OwnedHandle> {
    let desired_access  = desired_access.into().into();
    let inherit_handle  = inherit_handle as _;

    string::convert_to_cstr::<{limit::stack::JOB_NAME}, _, _>(name, |name| {
        let h = unsafe { OpenJobObjectW(desired_access, inherit_handle, name.as_opt_cstr()) };
        firehazard::Error::get_last_if(h.is_null())?;
        unsafe { job::OwnedHandle::from_raw(h) }
    })?
}



#[doc(alias = "QueryInformationJobObject")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/jobapi2/nf-jobapi2-queryinformationjobobject)\]
/// QueryInformationJobObject
///
/// ### Example
/// ```
/// use firehazard::{*, job::object::BasicUiRestrictions};
///
/// let job = create_job_object_w(None, ()).unwrap();
/// let restrictions : BasicUiRestrictions = query_information_job_object(&job).unwrap();
/// ```
///
pub fn query_information_job_object<Info: job::QueryInformationJobObject>(job: &job::OwnedHandle) -> firehazard::Result<Info> { Info::query_from(job) }



// "Starting with Windows 10, version 1607, this function is no longer supported."
// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/jobapi2/nf-jobapi2-queryioratecontrolinformationjobobject)\]
// QueryIoRateControlInformationJobObject



#[doc(alias = "SetInformationJobObject")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/jobapi2/nf-jobapi2-setinformationjobobject)\]
/// SetInformationJobObject
///
/// ### Examples
/// ```
/// # use winapi::shared::winerror::*;
/// use firehazard::{*, job::object::{uilimit, BasicUiRestrictions}};
/// let mut job = create_job_object_w(None, ()).unwrap();
///
/// set_information_job_object(&mut job, BasicUiRestrictions {
///     ui_restrictions_class: ()
///         | uilimit::DESKTOP
///         | uilimit::DISPLAYSETTINGS
///         | uilimit::EXITWINDOWS
///         | uilimit::GLOBALATOMS
///         | uilimit::HANDLES
///         | uilimit::READCLIPBOARD
///         | uilimit::SYSTEMPARAMETERS
///         | uilimit::WRITECLIPBOARD
/// }).unwrap();
///
/// let err = set_information_job_object(&mut job, BasicUiRestrictions {
///     ui_restrictions_class: unsafe { uilimit::Flags::from_unchecked(!0) }
/// }).unwrap_err();
/// assert_eq!(ERROR_INVALID_PARAMETER, err);
/// ```
///
pub fn set_information_job_object(job: &job::OwnedHandle, information: impl job::SetInformationJobObject) -> firehazard::Result<()> { information.set_on(job) }



// "Starting with Windows 10, version 1607, this function is no longer supported."
// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/jobapi2/nf-jobapi2-setioratecontrolinformationjobobject)\]
// SetIoRateControlInformationJobObject



#[doc(alias = "TerminateJobObject")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/jobapi2/nf-jobapi2-terminatejobobject)\]
/// TerminateJobObject
///
/// ### Example
/// ```
/// # use firehazard::*;
/// # use winapi::um::winnt::GENERIC_ALL;
/// let job = create_job_object_w(None, ()).unwrap();
/// terminate_job_object(&job, 0).unwrap();
/// ```
///
pub fn terminate_job_object(
    job:            &job::OwnedHandle,
    exit_code:      u32,
) -> firehazard::Result<()> {
    firehazard::Error::get_last_if(FALSE == unsafe { TerminateJobObject(
        job.as_handle(),
        exit_code,
    )})
}
