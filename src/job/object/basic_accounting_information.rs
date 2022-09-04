use crate::*;
use winapi::um::winnt::*;
use core::mem::{align_of, size_of};



const _ : () = assert!(align_of::<BasicAccountingInformation>() == align_of::<JOBOBJECT_BASIC_ACCOUNTING_INFORMATION>());
const _ : () = assert!(size_of ::<BasicAccountingInformation>() == size_of ::<JOBOBJECT_BASIC_ACCOUNTING_INFORMATION>());

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_basic_accounting_information)\]
/// JOBOBJECT_BASIC_ACCOUNTING_INFORMATION
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)] pub struct BasicAccountingInformation {
    pub total_user_time:                u64,
    pub total_kernel_time:              u64,
    pub this_period_total_user_time:    u64,
    pub this_period_total_kernel_time:  u64,
    pub total_page_fault_count:         u32,
    pub total_processes:                u32,
    pub active_processes:               u32,
    pub total_terminated_processes:     u32,
}



const _ : () = assert!(align_of::<BasicAndIoAccountingInformation>() == align_of::<JOBOBJECT_BASIC_AND_IO_ACCOUNTING_INFORMATION>());
const _ : () = assert!(size_of ::<BasicAndIoAccountingInformation>() == size_of ::<JOBOBJECT_BASIC_AND_IO_ACCOUNTING_INFORMATION>());

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_basic_and_io_accounting_information)\]
/// JOBOBJECT_BASIC_AND_IO_ACCOUNTING_INFORMATION
#[repr(C)] pub struct BasicAndIoAccountingInformation {
    pub basic_info: BasicAccountingInformation,
    pub io_info:    io::Counters,
}



impl job::QueryInformation for JOBOBJECT_BASIC_ACCOUNTING_INFORMATION           { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { job::query_fixed(job, JobObjectBasicAccountingInformation) } } }
impl job::QueryInformation for job::object::BasicAccountingInformation          { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { job::query_fixed(job, JobObjectBasicAccountingInformation) } } }
impl job::QueryInformation for JOBOBJECT_BASIC_AND_IO_ACCOUNTING_INFORMATION    { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { job::query_fixed(job, JobObjectBasicAndIoAccountingInformation) } } }
impl job::QueryInformation for job::object::BasicAndIoAccountingInformation     { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> { unsafe { job::query_fixed(job, JobObjectBasicAndIoAccountingInformation) } } }

// not implemented: job::SetInformation
