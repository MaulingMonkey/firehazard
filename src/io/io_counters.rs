use crate::*;
use winapi::um::winnt::IO_COUNTERS;



#[doc(alias = "IO_COUNTERS")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-io_counters)\]
/// IO_COUNTERS
///
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)] pub struct Counters {
    pub read_operation_count:   u64,
    pub write_operation_count:  u64,
    pub other_operation_count:  u64,
    pub read_transfer_count:    u64,
    pub write_transfer_count:   u64,
    pub other_transfer_count:   u64,
}

structure!(@assert layout io::Counters => IO_COUNTERS {
    read_operation_count    == ReadOperationCount,
    write_operation_count   == WriteOperationCount,
    other_operation_count   == OtherOperationCount,
    read_transfer_count     == ReadTransferCount,
    write_transfer_count    == WriteTransferCount,
    other_transfer_count    == OtherTransferCount,
});
