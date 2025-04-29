// C:\Program Files (x86)\Windows Kits\10\Include\10.0.22621.0\um\winternl.h
// lines 223-233

use crate::prelude::*;



#[doc(alias = "IO_STATUS_BLOCK")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/wdm/ns-wdm-_io_status_block)\]
/// IO_STATUS_BLOCK
///
#[allow(non_snake_case)]
#[derive(Clone, bytemuck::Zeroable)]
#[repr(C)] pub(crate) struct StatusBlock {
    pub Pointer:        *mut c_void, // or NTSTATUS by union
    pub Information:    usize,       // ≈ ULONG_PTR
}

#[allow(non_snake_case)]
impl io::StatusBlock {
    pub fn Status(&self) -> winresult::NtStatus { unsafe { core::mem::transmute_copy(&self.Pointer) } }
}

impl Default for io::StatusBlock {
    fn default() -> Self { bytemuck::Zeroable::zeroed() }
}

impl core::fmt::Debug for io::StatusBlock {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(fmt, "io::StatusBlock {{ Status: ")?;
        write!(fmt, "{:?}", self.Status())?;
        write!(fmt, ", Information: ")?;
        if self.Information < 0x10000 {
            write!(fmt, "{}", self.Information)?;
        } else {
            write!(fmt, "0x{:08X}", self.Information)?;
        }
        write!(fmt, " }}")
    }
}
