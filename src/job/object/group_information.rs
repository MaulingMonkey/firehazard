use crate::*;
use winapi::shared::ntdef;
use winapi::um::winnt::{self, JobObjectGroupInformation, JobObjectGroupInformationEx};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/jobapi2/nf-jobapi2-setinformationjobobject)\]
/// ~ WORD / USHORT referencing a processor group
///
/// get/set via `JobObjectGroupInformationEx`
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Group(u16);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-group_affinity)\]
/// GROUP_AFFINITY referencing a processor group and processor/core mask
///
/// get/set via `JobObjectGroupInformationEx`
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)] pub struct GroupAffinity {
    pub mask:   usize,
    pub group:  u16,
    #[doc(hidden)] pub _reserved: [u16; 3],
}

structure!(@assert layout GroupAffinity => ntdef::GROUP_AFFINITY {
    mask        == Mask,
    group       == Group,
    _reserved   == Reserved,
});

structure!(@assert layout GroupAffinity => winnt::GROUP_AFFINITY {
    mask        == Mask,
    group       == Group,
    _reserved   == Reserved,
});

#[cfg(std)] impl job::QueryInformationJobObject for std::vec::Vec<Group>                   { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { job::query_vec(job, JobObjectGroupInformation  ) } } }
#[cfg(std)] impl job::QueryInformationJobObject for std::vec::Vec<GroupAffinity>           { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { job::query_vec(job, JobObjectGroupInformationEx) } } }
#[cfg(std)] impl job::QueryInformationJobObject for std::vec::Vec<ntdef::GROUP_AFFINITY>   { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { job::query_vec(job, JobObjectGroupInformationEx) } } }
#[cfg(std)] impl job::QueryInformationJobObject for std::vec::Vec<winnt::GROUP_AFFINITY>   { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { job::query_vec(job, JobObjectGroupInformationEx) } } }

//impl job::SetInformationJobObject for &'_ [u16]                      { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { job::set(job, JobObjectGroupInformation, self) } } }
impl job::SetInformationJobObject for &'_ [Group]                    { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { job::set(job, JobObjectGroupInformation, self) } } }
impl job::SetInformationJobObject for &'_ [GroupAffinity]            { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { job::set(job, JobObjectGroupInformationEx, self) } } }
impl job::SetInformationJobObject for &'_ [ntdef::GROUP_AFFINITY]    { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { job::set(job, JobObjectGroupInformationEx, self) } } }
impl job::SetInformationJobObject for &'_ [winnt::GROUP_AFFINITY]    { fn set_on(self, job: &job::OwnedHandle) -> Result<(), Error> { unsafe { job::set(job, JobObjectGroupInformationEx, self) } } }

#[test] fn group_invalid() {
    use winapi::shared::winerror::ERROR_INVALID_PARAMETER;

    let job = create_job_object_a(None, ()).unwrap();
    let err = set_information_job_object(&job, &[Group(0xFFFF)][..]).unwrap_err();
    assert_eq!(ERROR_INVALID_PARAMETER, err);
}

#[cfg(std)] #[test] fn group_0() {
    let job = create_job_object_a(None, ()).unwrap();
    set_information_job_object(&job, &[Group(0)][..]).unwrap();

    let groups : std::vec::Vec<Group> = query_information_job_object(&job).unwrap();
    assert_eq!(1, groups.len());
    assert_eq!(groups[0].0, 0);
}

#[cfg(std)] #[test] fn group_0_affinity() {
    let job = create_job_object_a(None, ()).unwrap();
    set_information_job_object(&job, &[GroupAffinity { group: 0, mask: 0b1, ..Default::default() }][..]).unwrap();

    let groups : std::vec::Vec<GroupAffinity> = query_information_job_object(&job).unwrap();
    assert_eq!(1, groups.len());
    assert!(groups[0].mask == 0b1);
}

#[cfg(std)] #[test] fn default_job_groups() {
    let job = create_job_object_a(None, ()).unwrap();

    let groups : std::vec::Vec<Group> = query_information_job_object(&job).unwrap();
    assert_eq!(0, groups.len());

    let groups : std::vec::Vec<GroupAffinity> = query_information_job_object(&job).unwrap();
    assert!(!groups.is_empty());
    assert!(groups[0].mask != 0);
}
