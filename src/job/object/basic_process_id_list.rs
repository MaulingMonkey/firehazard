use crate::*;
use bytemuck::*;
use winapi::um::winnt::*;
use core::mem::size_of_val;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_basic_process_id_list)\]
/// JOBOBJECT_BASIC_PROCESS_ID_LIST
///
/// ### Errata
/// *   Not directly ABI compatible
#[derive(Clone, Debug, Default)]
pub struct BasicProcessIdList {
    pub number_of_assigned_processes:   u32,
    pub number_of_process_ids_in_list:  u32,
    pub process_id_list:                std::vec::Vec<usize>,
}

impl job::QueryInformationJobObject for job::object::BasicProcessIdList { fn query_from(job: &job::OwnedHandle) -> Result<Self, Error> {
    let mut process_id_list = unsafe { job::query_vec::<usize>(job, JobObjectBasicProcessIdList) }?;
    let mut header = Header::zeroed();
    let header_bytes = size_of_val(&header);
    bytes_of_mut(&mut header).copy_from_slice(&cast_slice(&process_id_list[..])[..header_bytes]);
    let to_remove = header_bytes / size_of_val(&process_id_list[0]);
    let _ = process_id_list.drain(..to_remove); // remove Header
    let Header { number_of_assigned_processes, number_of_process_ids_in_list } = header;
    Ok(Self {
        number_of_assigned_processes,
        number_of_process_ids_in_list,
        process_id_list,
    })
}}



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_basic_process_id_list)\]
/// JOBOBJECT_BASIC_PROCESS_ID_LIST minus the trailing "any length" `ULONG_PTR ProcessIdList[1];`
#[derive(Clone, Copy, Debug)]
#[derive(Pod, Zeroable)]
#[repr(C)] struct Header {
    pub number_of_assigned_processes:   u32,
    pub number_of_process_ids_in_list:  u32,
}

#[allow(dead_code)] // only for layout validation of `Header`
#[repr(C)] struct HeaderPlusOneProcess {
    header:             Header,
    process_id_list:    [usize; 1]
}

structure!(@assert layout HeaderPlusOneProcess => JOBOBJECT_BASIC_PROCESS_ID_LIST {
    //h.number_of_assigned_processes  == NumberOfAssignedProcesses,
    //h.number_of_process_ids_in_list == NumberOfProcessIdsInList,
    process_id_list                 == ProcessIdList,
});
