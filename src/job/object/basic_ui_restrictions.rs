use crate::prelude::*;
use winapi::um::winnt::{JOBOBJECT_BASIC_UI_RESTRICTIONS, JobObjectBasicUIRestrictions};



#[doc(alias = "JOBOBJECT_BASIC_UI_RESTRICTIONS")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_basic_ui_restrictions)\]
/// JOBOBJECT_BASIC_UI_RESTRICTIONS
///
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)] pub struct BasicUiRestrictions {
    pub ui_restrictions_class: job::object::uilimit::Flags,
}

structure!(@assert layout BasicUiRestrictions => JOBOBJECT_BASIC_UI_RESTRICTIONS {
    ui_restrictions_class   == UIRestrictionsClass,
});

impl job::QueryInformationJobObject for JOBOBJECT_BASIC_UI_RESTRICTIONS  { fn query_from(job: &job::OwnedHandle) -> firehazard::Result<Self> { unsafe { job::query_fixed(job, JobObjectBasicUIRestrictions) } } }
impl job::QueryInformationJobObject for job::object::BasicUiRestrictions { fn query_from(job: &job::OwnedHandle) -> firehazard::Result<Self> { unsafe { job::query_fixed(job, JobObjectBasicUIRestrictions) } } }
impl job::QueryInformationJobObject for job::object::uilimit::Flags      { fn query_from(job: &job::OwnedHandle) -> firehazard::Result<Self> { unsafe { job::query_fixed(job, JobObjectBasicUIRestrictions) } } }

impl job::SetInformationJobObject   for JOBOBJECT_BASIC_UI_RESTRICTIONS  { fn set_on(self, job: &job::OwnedHandle) -> firehazard::Result<()> { unsafe { job::set(job, JobObjectBasicUIRestrictions, &self) } } }
impl job::SetInformationJobObject   for job::object::BasicUiRestrictions { fn set_on(self, job: &job::OwnedHandle) -> firehazard::Result<()> { unsafe { job::set(job, JobObjectBasicUIRestrictions, &self) } } }
impl job::SetInformationJobObject   for job::object::uilimit::Flags      { fn set_on(self, job: &job::OwnedHandle) -> firehazard::Result<()> { unsafe { job::set(job, JobObjectBasicUIRestrictions, &self) } } }
