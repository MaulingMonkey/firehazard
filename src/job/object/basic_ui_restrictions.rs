use crate::*;
use winapi::um::winnt::{JOBOBJECT_BASIC_UI_RESTRICTIONS, JobObjectBasicUIRestrictions};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_basic_ui_restrictions)\] JOBOBJECT_BASIC_UI_RESTRICTIONS
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)] pub struct BasicUiRestrictions {
    pub ui_restrictions_class: job::object::uilimit::Flags,
}

impl job::QueryInformation for JOBOBJECT_BASIC_UI_RESTRICTIONS  { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { job::query_fixed(job, JobObjectBasicUIRestrictions) } } }
impl job::QueryInformation for job::object::BasicUiRestrictions { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { job::query_fixed(job, JobObjectBasicUIRestrictions) } } }
impl job::QueryInformation for job::object::uilimit::Flags      { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { job::query_fixed(job, JobObjectBasicUIRestrictions) } } }

impl job::SetInformation   for JOBOBJECT_BASIC_UI_RESTRICTIONS  { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { job::set(job, JobObjectBasicUIRestrictions, &self) } } }
impl job::SetInformation   for job::object::BasicUiRestrictions { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { job::set(job, JobObjectBasicUIRestrictions, &self) } } }
impl job::SetInformation   for job::object::uilimit::Flags      { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { job::set(job, JobObjectBasicUIRestrictions, &self) } } }

// TODO: impl From spam?
