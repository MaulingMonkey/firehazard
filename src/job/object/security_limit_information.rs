//! Deprecated and ignorable:
//!
//! >   Starting with Windows Vista, you must set security limitations individually for each process associated with a job object,
//! >   rather than setting them for the job object by using SetInformationJobObject. For information, see Process Security and Access Rights.
//! >
//! >   <https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_security_limit_information#remarks>

#![allow(dead_code)]

use crate::*;
use winapi::um::winnt::JOBOBJECT_SECURITY_LIMIT_INFORMATION;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_security_limit_information)\] JOBOBJECT_SECURITY_LIMIT_INFORMATION
#[derive(Debug, Default)]
#[repr(C)] struct SecurityLimitInformation {
    pub security_limit_flags:   u32, // typeable

    // XXX: we probably want a non-owned variants of everything bellow?
    // what I have here might "work" for setting job information, but probably not getting
    pub job_token:              Option<token::OwnedHandle>,
    pub sids_to_disable:        Option<token::BoxTokenGroups>,
    pub privileges_to_delete:   Option<token::BoxTokenPrivileges>,
    pub restricted_sids:        Option<token::BoxTokenGroups>,
}

structure!(@assert layout SecurityLimitInformation => JOBOBJECT_SECURITY_LIMIT_INFORMATION {
    security_limit_flags    == SecurityLimitFlags,
    job_token               == JobToken,
    sids_to_disable         == SidsToDisable,
    privileges_to_delete    == PrivilegesToDelete,
    restricted_sids         == RestrictedSids,
});

//impl job::QueryInformationJobObject for JOBOBJECT_SECURITY_LIMIT_INFORMATION   { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { query_header(job, JobObjectSecurityLimitInformation) } } } // self-referential pointers? (sids)
//impl job::SetInformationJobObject for JOBOBJECT_SECURITY_LIMIT_INFORMATION     { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { set(job, JobObjectSecurityLimitInformation, &self) } } } // interior pointers
