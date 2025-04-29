// NOTE: Everything here is at most `pub(crate)`
//  • Not a public part of the Windows SDK, presumably an implementation detail of Windows NT
//  • Defined based on docs - sketchy, as docs lie.



#[doc(alias = "FILE_MODE_INFORMATION")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntifs/ns-ntifs-_file_mode_information)\]
/// FILE_MODE_INFORMATION
///
#[allow(non_snake_case)]
#[derive(Clone, Copy, bytemuck::Pod, Default, bytemuck::Zeroable)]
#[repr(C)] pub(crate) struct ModeInformation {
    pub Mode: winapi::shared::minwindef::ULONG,
}

impl ModeInformation {
    #[doc(alias = "FileModeInformation")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/wdm/ne-wdm-_file_information_class#constants)\]
    /// FileModeInformation
    ///
    pub const CLASS : file::InformationClass = file::InformationClass(16);
}

unsafe impl NtFileInformation for file::ModeInformation {
    const CLASS : file::InformationClass = Self::CLASS;
}

impl core::fmt::Debug for ModeInformation {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        let mut m = self.Mode;
        if m == 0 { return write!(fmt, "file::ModeInformation {{ Mode: 0 }}"); }

        write!(fmt, "file::ModeInformation {{ Mode: ")?;
        for (name, value) in [
            // C:\Program Files (x86)\Windows Kits\10\Include\10.0.22621.0\um\winternl.h
            // lines 985-1017

            ("FILE_DIRECTORY_FILE",                 0x00000001),
            ("FILE_WRITE_THROUGH",                  0x00000002),
            ("FILE_SEQUENTIAL_ONLY",                0x00000004),
            ("FILE_NO_INTERMEDIATE_BUFFERING",      0x00000008),

            ("FILE_SYNCHRONOUS_IO_ALERT",           0x00000010),
            ("FILE_SYNCHRONOUS_IO_NONALERT",        0x00000020),
            ("FILE_NON_DIRECTORY_FILE",             0x00000040),
            ("FILE_CREATE_TREE_CONNECTION",         0x00000080),

            ("FILE_COMPLETE_IF_OPLOCKED",           0x00000100),
            ("FILE_NO_EA_KNOWLEDGE",                0x00000200),
            ("FILE_OPEN_REMOTE_INSTANCE",           0x00000400),
            ("FILE_RANDOM_ACCESS",                  0x00000800),

            ("FILE_DELETE_ON_CLOSE",                0x00001000),
            ("FILE_OPEN_BY_FILE_ID",                0x00002000),
            ("FILE_OPEN_FOR_BACKUP_INTENT",         0x00004000),
            ("FILE_NO_COMPRESSION",                 0x00008000),

            // #if (_WIN32_WINNT >= _WIN32_WINNT_WIN7)
            ("FILE_OPEN_REQUIRING_OPLOCK",          0x00010000),

            ("FILE_RESERVE_OPFILTER",               0x00100000),
            ("FILE_OPEN_REPARSE_POINT",             0x00200000),
            ("FILE_OPEN_NO_RECALL",                 0x00400000),
            ("FILE_OPEN_FOR_FREE_SPACE_QUERY",      0x00800000),

            ("FILE_VALID_OPTION_FLAGS",             0x00ffffff),
            ("FILE_VALID_PIPE_OPTION_FLAGS",        0x00000032),
            ("FILE_VALID_MAILSLOT_OPTION_FLAGS",    0x00000032),
            ("FILE_VALID_SET_FLAGS",                0x00000036),
        ].iter().copied() {
            if (m & value) == 0 { continue }
            m &= !value;
            write!(fmt, "{name}")?;
            if m != 0 { write!(fmt, " | ")?; }
        }
        if m != 0 { write!(fmt, "0x{m:08x}")?; }
        write!(fmt, " }}")
    }
}
