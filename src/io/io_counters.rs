use crate::*;
use winapi::um::winnt::IO_COUNTERS;
use core::mem::{align_of, size_of};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-io_counters)\]
/// IO_COUNTERS
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)] pub struct Counters {
    pub read_operation_count:   u64,
    pub write_operation_count:  u64,
    pub other_operation_count:  u64,
    pub read_transfer_count:    u64,
    pub write_transfer_count:   u64,
    pub other_transfer_count:   u64,
}

const _ALIGN_IO : () = assert!(align_of::<io::Counters>() == align_of::<IO_COUNTERS>());
const _SIZE_IO  : () = assert!(size_of ::<io::Counters>() == size_of ::<IO_COUNTERS>());
