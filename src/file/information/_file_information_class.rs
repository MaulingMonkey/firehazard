// NOTE: Everything here is at most `pub(crate)`
//  • Based on this being specific to winternl.h etc., this doesn't appear to be part of Microsoft's intended public API.
//  • Constants are typically based on documentation, rather than an actual header.  Sketchy, as docs lie.



#[doc(alias = "FILE_INFORMATION_CLASS")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/wdm/ne-wdm-_file_information_class)\]
/// FILE_INFORMATION_CLASS
///
#[derive(Clone, Copy, bytemuck::Pod, Default, bytemuck::Zeroable, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub(crate) struct InformationClass(pub(crate) u32);

impl From<u32> for InformationClass { fn from(class: u32                ) -> Self { Self(class) } }
impl From<InformationClass> for u32 { fn from(class: InformationClass   ) -> Self { class.0 } }

// Constants omitted: their naming overlaps with rustified struct names.  See associated constants instead, e.g.:
//
// | C++                    | Rust                          |
// | -----------------------| ------------------------------|
// | FILE_MODE_INFORMATION  | file::ModeInformation         |
// | FileModeInformation    | file::ModeInformation::CLASS  |

impl core::fmt::Debug for InformationClass {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        let id = match self.0 {
            // https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/wdm/ne-wdm-_file_information_class
            0  => "0",
            1  => "FileDirectoryInformation",
            2  => "FileFullDirectoryInformation",
            3  => "FileBothDirectoryInformation",
            4  => "FileBasicInformation",
            5  => "FileStandardInformation",
            6  => "FileInternalInformation",
            7  => "FileEaInformation",
            8  => "FileAccessInformation",
            9  => "FileNameInformation",
            10 => "FileRenameInformation",
            11 => "FileLinkInformation",
            12 => "FileNamesInformation",
            13 => "FileDispositionInformation",
            14 => "FilePositionInformation",
            15 => "FileFullEaInformation",
            16 => "FileModeInformation",
            17 => "FileAlignmentInformation",
            18 => "FileAllInformation",
            19 => "FileAllocationInformation",
            20 => "FileEndOfFileInformation",
            21 => "FileAlternateNameInformation",
            22 => "FileStreamInformation",
            23 => "FilePipeInformation",
            24 => "FilePipeLocalInformation",
            25 => "FilePipeRemoteInformation",
            26 => "FileMailslotQueryInformation",
            27 => "FileMailslotSetInformation",
            28 => "FileCompressionInformation",
            29 => "FileObjectIdInformation",
            30 => "FileCompletionInformation",
            31 => "FileMoveClusterInformation",
            32 => "FileQuotaInformation",
            33 => "FileReparsePointInformation",
            34 => "FileNetworkOpenInformation",
            35 => "FileAttributeTagInformation",
            36 => "FileTrackingInformation",
            37 => "FileIdBothDirectoryInformation",
            38 => "FileIdFullDirectoryInformation",
            39 => "FileValidDataLengthInformation",
            40 => "FileShortNameInformation",
            41 => "FileIoCompletionNotificationInformation",
            42 => "FileIoStatusBlockRangeInformation",
            43 => "FileIoPriorityHintInformation",
            44 => "FileSfioReserveInformation",
            45 => "FileSfioVolumeInformation",
            46 => "FileHardLinkInformation",
            47 => "FileProcessIdsUsingFileInformation",
            48 => "FileNormalizedNameInformation",
            49 => "FileNetworkPhysicalNameInformation",
            50 => "FileIdGlobalTxDirectoryInformation",
            51 => "FileIsRemoteDeviceInformation",
            52 => "FileUnusedInformation",
            53 => "FileNumaNodeInformation",
            54 => "FileStandardLinkInformation",
            55 => "FileRemoteProtocolInformation",
            56 => "FileRenameInformationBypassAccessCheck",
            57 => "FileLinkInformationBypassAccessCheck",
            58 => "FileVolumeNameInformation",
            59 => "FileIdInformation",
            60 => "FileIdExtdDirectoryInformation",
            61 => "FileReplaceCompletionInformation",
            62 => "FileHardLinkFullIdInformation",
            63 => "FileIdExtdBothDirectoryInformation",
            64 => "FileDispositionInformationEx",
            65 => "FileRenameInformationEx",
            66 => "FileRenameInformationExBypassAccessCheck",
            67 => "FileDesiredStorageClassInformation",
            68 => "FileStatInformation",
            69 => "FileMemoryPartitionInformation",
            70 => "FileStatLxInformation",
            71 => "FileCaseSensitiveInformation",
            72 => "FileLinkInformationEx",
            73 => "FileLinkInformationExBypassAccessCheck",
            74 => "FileStorageReserveIdInformation",
            75 => "FileCaseSensitiveInformationForceAccessCheck",
            76 => "FileKnownFolderInformation",
            77 => "FileStatBasicInformation",
            78 => "FileId64ExtdDirectoryInformation",
            79 => "FileId64ExtdBothDirectoryInformation",
            80 => "FileIdAllExtdDirectoryInformation",
            81 => "FileIdAllExtdBothDirectoryInformation",
            82 => "FileStreamReservationInformation", // XXX: value inferred
            83 => "FileMupProviderInfo", // XXX: value inferred
            // => "FileMaximumInformation", // XXX: moving target
            unknown => return write!(fmt, "File???Information ({unknown})"),
        };
        write!(fmt, "{id}")
    }
}
