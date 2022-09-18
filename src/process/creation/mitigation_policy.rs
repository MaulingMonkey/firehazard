//! \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-updateprocthreadattribute)\] UpdateProcThreadAttribute value constants for use with<br>
//! PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY

use crate::*;
use process::creation::{MitigationPolicyFlags1, MitigationPolicyFlags1Mask};
use core::fmt::{self, Debug, Formatter};



/// "legacy" creation mitigation policy options
///
/// Windows 7+
pub mod legacy {
    use super::*;

    /// Enables data execution prevention (DEP) for the child process.
    /// For more information, see [Data Execution Prevention](https://learn.microsoft.com/en-us/windows/desktop/Memory/data-execution-prevention).
    pub const DEP_ENABLE            : MitigationPolicyFlags1 = MitigationPolicyFlags1(0x01);

    /// Enables DEP-ATL thunk emulation for the child process.
    /// DEP-ATL thunk emulation causes the system to intercept NX faults that originate from the Active Template Library (ATL) thunk layer.
    /// This value can be specified only with [process::creation::mitigation_policy::DEP_ENABLE].
    pub const DEP_ATL_THUNK_ENABLE  : MitigationPolicyFlags1 = MitigationPolicyFlags1(0x02);

    /// Enables structured exception handler overwrite protection (SEHOP) for the child process.
    /// SEHOP blocks exploits that use the structured exception handler (SEH) overwrite technique.
    pub const SEHOP_ENABLE          : MitigationPolicyFlags1 = MitigationPolicyFlags1(0x04);
}
#[doc(hidden)] pub use legacy::*;

/// Mandatory ASLR
///
/// Mandatory ASLR forcibly rebases images that are not dynamic base compatible
/// by acting as though there were an image base collision at load time.
///
/// Note that 'require relocations' mode refuses load of images
/// that do not have a base relocation section.
///
/// Windows 8+
pub mod force_relocate_images {
    use super::*;
    pub const MASK                  : MitigationPolicyFlags1Mask = MitigationPolicyFlags1Mask(PROCESS_CREATION_MITIGATION_POLICY_FORCE_RELOCATE_IMAGES_MASK);
    pub const DEFER                 : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_FORCE_RELOCATE_IMAGES_DEFER);
    pub const ALWAYS_ON             : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_FORCE_RELOCATE_IMAGES_ALWAYS_ON);
    pub const ALWAYS_OFF            : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_FORCE_RELOCATE_IMAGES_ALWAYS_OFF);
    pub const ALWAYS_ON_REQ_RELOCS  : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_FORCE_RELOCATE_IMAGES_ALWAYS_ON_REQ_RELOCS);
}

/// Heap terminate on Corruption
///
/// Note that 'always off' does not override the default opt-in for binaries with current subsystem versions set in the image header.
///
/// Heap terminate on corruption is user mode enforced.
///
/// Windows 8+
pub mod heap_terminate {
    use super::*;
    pub const MASK                  : MitigationPolicyFlags1Mask = MitigationPolicyFlags1Mask(PROCESS_CREATION_MITIGATION_POLICY_HEAP_TERMINATE_MASK);
    pub const DEFER                 : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_HEAP_TERMINATE_DEFER);
    pub const ALWAYS_ON             : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_HEAP_TERMINATE_ALWAYS_ON);
    pub const ALWAYS_OFF            : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_HEAP_TERMINATE_ALWAYS_OFF);
    pub const RESERVED              : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_HEAP_TERMINATE_RESERVED);
}

/// Bottom up randomization (includes stack randomization)
///
/// (e.g. randomization of the lowest user address.)
///
/// Windows 8+
pub mod bottom_up_aslr {
    use super::*;
    pub const MASK                  : MitigationPolicyFlags1Mask = MitigationPolicyFlags1Mask(PROCESS_CREATION_MITIGATION_POLICY_BOTTOM_UP_ASLR_MASK);
    pub const DEFER                 : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_BOTTOM_UP_ASLR_DEFER);
    pub const ALWAYS_ON             : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_BOTTOM_UP_ASLR_ALWAYS_ON);
    pub const ALWAYS_OFF            : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_BOTTOM_UP_ASLR_ALWAYS_OFF);
    pub const RESERVED              : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_BOTTOM_UP_ASLR_RESERVED);
}

/// High entropy bottom up randomization.
/// Note that high entropy bottom up randomization is effective if and only if bottom up ASLR is also enabled.
///
/// N.B.  High entropy mode is only meaningful for native 64-bit processes.
/// In high entropy mode, up to 1TB of bottom up variance is enabled.
///
/// Windows 8+
pub mod high_entropy_aslr {
    use super::*;
    pub const MASK                  : MitigationPolicyFlags1Mask = MitigationPolicyFlags1Mask(PROCESS_CREATION_MITIGATION_POLICY_HIGH_ENTROPY_ASLR_MASK);
    pub const DEFER                 : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_HIGH_ENTROPY_ASLR_DEFER);
    pub const ALWAYS_ON             : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_HIGH_ENTROPY_ASLR_ALWAYS_ON);
    pub const ALWAYS_OFF            : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_HIGH_ENTROPY_ASLR_ALWAYS_OFF);
    pub const RESERVED              : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_HIGH_ENTROPY_ASLR_RESERVED);
}

/// Handle checking enforcement options.
///
/// Handle checking enforcement causes an exception to be raised immediately on a
/// bad handle reference, versus simply returning a failure status from the handle reference.
///
/// Windows 8+
pub mod strict_handle_checks {
    use super::*;
    pub const MASK                  : MitigationPolicyFlags1Mask = MitigationPolicyFlags1Mask(PROCESS_CREATION_MITIGATION_POLICY_STRICT_HANDLE_CHECKS_MASK);
    pub const DEFER                 : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_STRICT_HANDLE_CHECKS_DEFER);
    pub const ALWAYS_ON             : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_STRICT_HANDLE_CHECKS_ALWAYS_ON);
    pub const ALWAYS_OFF            : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_STRICT_HANDLE_CHECKS_ALWAYS_OFF);
    pub const RESERVED              : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_STRICT_HANDLE_CHECKS_RESERVED);
}

/// Win32k system call disable prevents a process from making Win32k calls.
///
/// Windows 8+
pub mod win32k_system_call_disable {
    use super::*;
    pub const MASK                  : MitigationPolicyFlags1Mask = MitigationPolicyFlags1Mask(PROCESS_CREATION_MITIGATION_POLICY_WIN32K_SYSTEM_CALL_DISABLE_MASK);
    pub const DEFER                 : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_WIN32K_SYSTEM_CALL_DISABLE_DEFER);
    pub const ALWAYS_ON             : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_WIN32K_SYSTEM_CALL_DISABLE_ALWAYS_ON);
    pub const ALWAYS_OFF            : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_WIN32K_SYSTEM_CALL_DISABLE_ALWAYS_OFF);
    pub const RESERVED              : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_WIN32K_SYSTEM_CALL_DISABLE_RESERVED);
}

/// Extension point disable allows a process to opt-out of loading various arbitrary extension point DLLs.
///
/// Windows 8+
pub mod extension_point_disable {
    use super::*;
    pub const MASK                  : MitigationPolicyFlags1Mask = MitigationPolicyFlags1Mask(PROCESS_CREATION_MITIGATION_POLICY_EXTENSION_POINT_DISABLE_MASK);
    pub const DEFER                 : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_EXTENSION_POINT_DISABLE_DEFER);
    pub const ALWAYS_ON             : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_EXTENSION_POINT_DISABLE_ALWAYS_ON);
    pub const ALWAYS_OFF            : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_EXTENSION_POINT_DISABLE_ALWAYS_OFF);
    pub const RESERVED              : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_EXTENSION_POINT_DISABLE_RESERVED);
}

/// Dynamic code
///
/// Windows 8.1+
pub mod prohibit_dynamic_code {
    use super::*;
    pub const MASK                      : MitigationPolicyFlags1Mask = MitigationPolicyFlags1Mask(PROCESS_CREATION_MITIGATION_POLICY_PROHIBIT_DYNAMIC_CODE_MASK);
    pub const DEFER                     : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_PROHIBIT_DYNAMIC_CODE_DEFER);
    pub const ALWAYS_ON                 : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_PROHIBIT_DYNAMIC_CODE_ALWAYS_ON);
    pub const ALWAYS_OFF                : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_PROHIBIT_DYNAMIC_CODE_ALWAYS_OFF);
    pub const ALWAYS_ON_ALLOW_OPT_OUT   : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_PROHIBIT_DYNAMIC_CODE_ALWAYS_ON_ALLOW_OPT_OUT);
}

/// Control Flow Guard (CFG) allows indirect control transfers to be checked at runtime.
///
/// Windows 8.1+
pub mod control_flow_guard {
    use super::*;
    pub const MASK                  : MitigationPolicyFlags1Mask = MitigationPolicyFlags1Mask(PROCESS_CREATION_MITIGATION_POLICY_CONTROL_FLOW_GUARD_MASK);
    pub const DEFER                 : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_CONTROL_FLOW_GUARD_DEFER);
    pub const ALWAYS_ON             : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_CONTROL_FLOW_GUARD_ALWAYS_ON);
    pub const ALWAYS_OFF            : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_CONTROL_FLOW_GUARD_ALWAYS_OFF);
    pub const EXPORT_SUPPRESSION    : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_CONTROL_FLOW_GUARD_EXPORT_SUPPRESSION);
}

/// Module signature options.  When enabled, this option will block mapping of non-microsoft binaries.
///
/// Windows 8.1+
pub mod block_non_microsoft_binaries {
    use super::*;
    pub const MASK                  : MitigationPolicyFlags1Mask = MitigationPolicyFlags1Mask(PROCESS_CREATION_MITIGATION_POLICY_BLOCK_NON_MICROSOFT_BINARIES_MASK);
    pub const DEFER                 : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_BLOCK_NON_MICROSOFT_BINARIES_DEFER);
    pub const ALWAYS_ON             : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_BLOCK_NON_MICROSOFT_BINARIES_ALWAYS_ON);
    pub const ALWAYS_OFF            : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_BLOCK_NON_MICROSOFT_BINARIES_ALWAYS_OFF);
    pub const ALLOW_STORE           : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_BLOCK_NON_MICROSOFT_BINARIES_ALLOW_STORE);
}

/// Font Disable Policy: Blocks loading non-system fonts.
///
/// Windows 10+
pub mod font_disable {
    use super::*;
    pub const MASK                  : MitigationPolicyFlags1Mask = MitigationPolicyFlags1Mask(PROCESS_CREATION_MITIGATION_POLICY_FONT_DISABLE_MASK);
    pub const DEFER                 : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_FONT_DISABLE_DEFER);
    pub const ALWAYS_ON             : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_FONT_DISABLE_ALWAYS_ON);
    pub const ALWAYS_OFF            : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_FONT_DISABLE_ALWAYS_OFF);
    pub const AUDIT_NONSYSTEM_FONTS : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_AUDIT_NONSYSTEM_FONTS);
}
#[doc(hidden)] pub use font_disable::AUDIT_NONSYSTEM_FONTS; // not prefixed with "FONT_DISABLE_" like the rest of the font disabling constants

/// Block mapping of images from remote devices.
///
/// Windows 10+
pub mod image_load_no_remote {
    use super::*;
    pub const MASK                  : MitigationPolicyFlags1Mask = MitigationPolicyFlags1Mask(PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_NO_REMOTE_MASK);
    pub const DEFER                 : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_NO_REMOTE_DEFER);
    pub const ALWAYS_ON             : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_NO_REMOTE_ALWAYS_ON);
    pub const ALWAYS_OFF            : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_NO_REMOTE_ALWAYS_OFF);
    pub const RESERVED              : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_NO_REMOTE_RESERVED);
}

/// Block mapping of images that have the low mandatory label.
///
/// Windows 10+
pub mod image_load_no_low_label {
    use super::*;
    pub const MASK                  : MitigationPolicyFlags1Mask = MitigationPolicyFlags1Mask(PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_NO_LOW_LABEL_MASK);
    pub const DEFER                 : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_NO_LOW_LABEL_DEFER);
    pub const ALWAYS_ON             : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_NO_LOW_LABEL_ALWAYS_ON);
    pub const ALWAYS_OFF            : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_NO_LOW_LABEL_ALWAYS_OFF);
    pub const RESERVED              : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_NO_LOW_LABEL_RESERVED);
}

/// Prefer loading images from the System32 folder over images in the application directory.
///
/// Windows 10+
pub mod image_load_prefer_system32 {
    use super::*;
    pub const MASK                  : MitigationPolicyFlags1Mask = MitigationPolicyFlags1Mask(PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_PREFER_SYSTEM32_MASK);
    pub const DEFER                 : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_PREFER_SYSTEM32_DEFER);
    pub const ALWAYS_ON             : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_PREFER_SYSTEM32_ALWAYS_ON);
    pub const ALWAYS_OFF            : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_PREFER_SYSTEM32_ALWAYS_OFF);
    pub const RESERVED              : MitigationPolicyFlags1     = MitigationPolicyFlags1    (PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_PREFER_SYSTEM32_RESERVED);
}

impl Debug for MitigationPolicyFlags1 {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        flags!(self.0, fmt, "0x{:016X}", [
            PROCESS_CREATION_MITIGATION_POLICY_DEP_ENABLE,
            PROCESS_CREATION_MITIGATION_POLICY_DEP_ATL_THUNK_ENABLE,
            PROCESS_CREATION_MITIGATION_POLICY_SEHOP_ENABLE,

            PROCESS_CREATION_MITIGATION_POLICY_FORCE_RELOCATE_IMAGES_ALWAYS_ON_REQ_RELOCS,
            PROCESS_CREATION_MITIGATION_POLICY_FORCE_RELOCATE_IMAGES_ALWAYS_OFF,
            PROCESS_CREATION_MITIGATION_POLICY_FORCE_RELOCATE_IMAGES_ALWAYS_ON,
            //PROCESS_CREATION_MITIGATION_POLICY_FORCE_RELOCATE_IMAGES_DEFER,
            //PROCESS_CREATION_MITIGATION_POLICY_FORCE_RELOCATE_IMAGES_MASK,

            PROCESS_CREATION_MITIGATION_POLICY_HEAP_TERMINATE_RESERVED,
            PROCESS_CREATION_MITIGATION_POLICY_HEAP_TERMINATE_ALWAYS_OFF,
            PROCESS_CREATION_MITIGATION_POLICY_HEAP_TERMINATE_ALWAYS_ON,
            //PROCESS_CREATION_MITIGATION_POLICY_HEAP_TERMINATE_DEFER,
            //PROCESS_CREATION_MITIGATION_POLICY_HEAP_TERMINATE_MASK,

            PROCESS_CREATION_MITIGATION_POLICY_BOTTOM_UP_ASLR_RESERVED,
            PROCESS_CREATION_MITIGATION_POLICY_BOTTOM_UP_ASLR_ALWAYS_OFF,
            PROCESS_CREATION_MITIGATION_POLICY_BOTTOM_UP_ASLR_ALWAYS_ON,
            //PROCESS_CREATION_MITIGATION_POLICY_BOTTOM_UP_ASLR_DEFER,
            //PROCESS_CREATION_MITIGATION_POLICY_BOTTOM_UP_ASLR_MASK,

            //PROCESS_CREATION_MITIGATION_POLICY_HIGH_ENTROPY_ASLR_MASK,
            //PROCESS_CREATION_MITIGATION_POLICY_HIGH_ENTROPY_ASLR_DEFER,
            PROCESS_CREATION_MITIGATION_POLICY_HIGH_ENTROPY_ASLR_ALWAYS_ON,
            PROCESS_CREATION_MITIGATION_POLICY_HIGH_ENTROPY_ASLR_ALWAYS_OFF,
            PROCESS_CREATION_MITIGATION_POLICY_HIGH_ENTROPY_ASLR_RESERVED,

            PROCESS_CREATION_MITIGATION_POLICY_STRICT_HANDLE_CHECKS_RESERVED,
            PROCESS_CREATION_MITIGATION_POLICY_STRICT_HANDLE_CHECKS_ALWAYS_OFF,
            PROCESS_CREATION_MITIGATION_POLICY_STRICT_HANDLE_CHECKS_ALWAYS_ON,
            //PROCESS_CREATION_MITIGATION_POLICY_STRICT_HANDLE_CHECKS_DEFER,
            //PROCESS_CREATION_MITIGATION_POLICY_STRICT_HANDLE_CHECKS_MASK,

            PROCESS_CREATION_MITIGATION_POLICY_WIN32K_SYSTEM_CALL_DISABLE_RESERVED,
            PROCESS_CREATION_MITIGATION_POLICY_WIN32K_SYSTEM_CALL_DISABLE_ALWAYS_OFF,
            PROCESS_CREATION_MITIGATION_POLICY_WIN32K_SYSTEM_CALL_DISABLE_ALWAYS_ON,
            //PROCESS_CREATION_MITIGATION_POLICY_WIN32K_SYSTEM_CALL_DISABLE_DEFER,
            //PROCESS_CREATION_MITIGATION_POLICY_WIN32K_SYSTEM_CALL_DISABLE_MASK,

            PROCESS_CREATION_MITIGATION_POLICY_EXTENSION_POINT_DISABLE_RESERVED,
            PROCESS_CREATION_MITIGATION_POLICY_EXTENSION_POINT_DISABLE_ALWAYS_OFF,
            PROCESS_CREATION_MITIGATION_POLICY_EXTENSION_POINT_DISABLE_ALWAYS_ON,
            //PROCESS_CREATION_MITIGATION_POLICY_EXTENSION_POINT_DISABLE_DEFER,
            //PROCESS_CREATION_MITIGATION_POLICY_EXTENSION_POINT_DISABLE_MASK,

            PROCESS_CREATION_MITIGATION_POLICY_PROHIBIT_DYNAMIC_CODE_ALWAYS_ON_ALLOW_OPT_OUT,
            PROCESS_CREATION_MITIGATION_POLICY_PROHIBIT_DYNAMIC_CODE_ALWAYS_OFF,
            PROCESS_CREATION_MITIGATION_POLICY_PROHIBIT_DYNAMIC_CODE_ALWAYS_ON,
            //PROCESS_CREATION_MITIGATION_POLICY_PROHIBIT_DYNAMIC_CODE_DEFER,
            //PROCESS_CREATION_MITIGATION_POLICY_PROHIBIT_DYNAMIC_CODE_MASK,

            PROCESS_CREATION_MITIGATION_POLICY_CONTROL_FLOW_GUARD_EXPORT_SUPPRESSION,
            PROCESS_CREATION_MITIGATION_POLICY_CONTROL_FLOW_GUARD_ALWAYS_OFF,
            PROCESS_CREATION_MITIGATION_POLICY_CONTROL_FLOW_GUARD_ALWAYS_ON,
            //PROCESS_CREATION_MITIGATION_POLICY_CONTROL_FLOW_GUARD_DEFER,
            //PROCESS_CREATION_MITIGATION_POLICY_CONTROL_FLOW_GUARD_MASK,

            PROCESS_CREATION_MITIGATION_POLICY_BLOCK_NON_MICROSOFT_BINARIES_ALLOW_STORE,
            PROCESS_CREATION_MITIGATION_POLICY_BLOCK_NON_MICROSOFT_BINARIES_ALWAYS_OFF,
            PROCESS_CREATION_MITIGATION_POLICY_BLOCK_NON_MICROSOFT_BINARIES_ALWAYS_ON,
            //PROCESS_CREATION_MITIGATION_POLICY_BLOCK_NON_MICROSOFT_BINARIES_DEFER,
            //PROCESS_CREATION_MITIGATION_POLICY_BLOCK_NON_MICROSOFT_BINARIES_MASK,

            PROCESS_CREATION_MITIGATION_POLICY_AUDIT_NONSYSTEM_FONTS,
            PROCESS_CREATION_MITIGATION_POLICY_FONT_DISABLE_ALWAYS_OFF,
            PROCESS_CREATION_MITIGATION_POLICY_FONT_DISABLE_ALWAYS_ON,
            //PROCESS_CREATION_MITIGATION_POLICY_FONT_DISABLE_DEFER,
            //PROCESS_CREATION_MITIGATION_POLICY_FONT_DISABLE_MASK,

            PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_NO_REMOTE_RESERVED,
            PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_NO_REMOTE_ALWAYS_OFF,
            PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_NO_REMOTE_ALWAYS_ON,
            //PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_NO_REMOTE_DEFER,
            //PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_NO_REMOTE_MASK,

            PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_NO_LOW_LABEL_RESERVED,
            PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_NO_LOW_LABEL_ALWAYS_OFF,
            PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_NO_LOW_LABEL_ALWAYS_ON,
            //PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_NO_LOW_LABEL_DEFER,
            //PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_NO_LOW_LABEL_MASK,

            PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_PREFER_SYSTEM32_RESERVED,
            PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_PREFER_SYSTEM32_ALWAYS_OFF,
            PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_PREFER_SYSTEM32_ALWAYS_ON,
            //PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_PREFER_SYSTEM32_DEFER,
            //PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_PREFER_SYSTEM32_MASK,
        ])
    }
}

const PROCESS_CREATION_MITIGATION_POLICY_DEP_ENABLE                                     : u64 = 0x01;
const PROCESS_CREATION_MITIGATION_POLICY_DEP_ATL_THUNK_ENABLE                           : u64 = 0x02;
const PROCESS_CREATION_MITIGATION_POLICY_SEHOP_ENABLE                                   : u64 = 0x04;

const PROCESS_CREATION_MITIGATION_POLICY_FORCE_RELOCATE_IMAGES_MASK                     : u64 = 0x00000003 <<  8;
const PROCESS_CREATION_MITIGATION_POLICY_FORCE_RELOCATE_IMAGES_DEFER                    : u64 = 0x00000000 <<  8;
const PROCESS_CREATION_MITIGATION_POLICY_FORCE_RELOCATE_IMAGES_ALWAYS_ON                : u64 = 0x00000001 <<  8;
const PROCESS_CREATION_MITIGATION_POLICY_FORCE_RELOCATE_IMAGES_ALWAYS_OFF               : u64 = 0x00000002 <<  8;
const PROCESS_CREATION_MITIGATION_POLICY_FORCE_RELOCATE_IMAGES_ALWAYS_ON_REQ_RELOCS     : u64 = 0x00000003 <<  8;

const PROCESS_CREATION_MITIGATION_POLICY_HEAP_TERMINATE_MASK                            : u64 = 0x00000003 << 12;
const PROCESS_CREATION_MITIGATION_POLICY_HEAP_TERMINATE_DEFER                           : u64 = 0x00000000 << 12;
const PROCESS_CREATION_MITIGATION_POLICY_HEAP_TERMINATE_ALWAYS_ON                       : u64 = 0x00000001 << 12;
const PROCESS_CREATION_MITIGATION_POLICY_HEAP_TERMINATE_ALWAYS_OFF                      : u64 = 0x00000002 << 12;
const PROCESS_CREATION_MITIGATION_POLICY_HEAP_TERMINATE_RESERVED                        : u64 = 0x00000003 << 12;

const PROCESS_CREATION_MITIGATION_POLICY_BOTTOM_UP_ASLR_MASK                            : u64 = 0x00000003 << 16;
const PROCESS_CREATION_MITIGATION_POLICY_BOTTOM_UP_ASLR_DEFER                           : u64 = 0x00000000 << 16;
const PROCESS_CREATION_MITIGATION_POLICY_BOTTOM_UP_ASLR_ALWAYS_ON                       : u64 = 0x00000001 << 16;
const PROCESS_CREATION_MITIGATION_POLICY_BOTTOM_UP_ASLR_ALWAYS_OFF                      : u64 = 0x00000002 << 16;
const PROCESS_CREATION_MITIGATION_POLICY_BOTTOM_UP_ASLR_RESERVED                        : u64 = 0x00000003 << 16;

const PROCESS_CREATION_MITIGATION_POLICY_HIGH_ENTROPY_ASLR_MASK                         : u64 = 0x00000003 << 20;
const PROCESS_CREATION_MITIGATION_POLICY_HIGH_ENTROPY_ASLR_DEFER                        : u64 = 0x00000000 << 20;
const PROCESS_CREATION_MITIGATION_POLICY_HIGH_ENTROPY_ASLR_ALWAYS_ON                    : u64 = 0x00000001 << 20;
const PROCESS_CREATION_MITIGATION_POLICY_HIGH_ENTROPY_ASLR_ALWAYS_OFF                   : u64 = 0x00000002 << 20;
const PROCESS_CREATION_MITIGATION_POLICY_HIGH_ENTROPY_ASLR_RESERVED                     : u64 = 0x00000003 << 20;

const PROCESS_CREATION_MITIGATION_POLICY_STRICT_HANDLE_CHECKS_MASK                      : u64 = 0x00000003 << 24;
const PROCESS_CREATION_MITIGATION_POLICY_STRICT_HANDLE_CHECKS_DEFER                     : u64 = 0x00000000 << 24;
const PROCESS_CREATION_MITIGATION_POLICY_STRICT_HANDLE_CHECKS_ALWAYS_ON                 : u64 = 0x00000001 << 24;
const PROCESS_CREATION_MITIGATION_POLICY_STRICT_HANDLE_CHECKS_ALWAYS_OFF                : u64 = 0x00000002 << 24;
const PROCESS_CREATION_MITIGATION_POLICY_STRICT_HANDLE_CHECKS_RESERVED                  : u64 = 0x00000003 << 24;

const PROCESS_CREATION_MITIGATION_POLICY_WIN32K_SYSTEM_CALL_DISABLE_MASK                : u64 = 0x00000003 << 28;
const PROCESS_CREATION_MITIGATION_POLICY_WIN32K_SYSTEM_CALL_DISABLE_DEFER               : u64 = 0x00000000 << 28;
const PROCESS_CREATION_MITIGATION_POLICY_WIN32K_SYSTEM_CALL_DISABLE_ALWAYS_ON           : u64 = 0x00000001 << 28;
const PROCESS_CREATION_MITIGATION_POLICY_WIN32K_SYSTEM_CALL_DISABLE_ALWAYS_OFF          : u64 = 0x00000002 << 28;
const PROCESS_CREATION_MITIGATION_POLICY_WIN32K_SYSTEM_CALL_DISABLE_RESERVED            : u64 = 0x00000003 << 28;

const PROCESS_CREATION_MITIGATION_POLICY_EXTENSION_POINT_DISABLE_MASK                   : u64 = 0x00000003u64 << 32;
const PROCESS_CREATION_MITIGATION_POLICY_EXTENSION_POINT_DISABLE_DEFER                  : u64 = 0x00000000u64 << 32;
const PROCESS_CREATION_MITIGATION_POLICY_EXTENSION_POINT_DISABLE_ALWAYS_ON              : u64 = 0x00000001u64 << 32;
const PROCESS_CREATION_MITIGATION_POLICY_EXTENSION_POINT_DISABLE_ALWAYS_OFF             : u64 = 0x00000002u64 << 32;
const PROCESS_CREATION_MITIGATION_POLICY_EXTENSION_POINT_DISABLE_RESERVED               : u64 = 0x00000003u64 << 32;

const PROCESS_CREATION_MITIGATION_POLICY_PROHIBIT_DYNAMIC_CODE_MASK                     : u64 = 0x00000003u64 << 36;
const PROCESS_CREATION_MITIGATION_POLICY_PROHIBIT_DYNAMIC_CODE_DEFER                    : u64 = 0x00000000u64 << 36;
const PROCESS_CREATION_MITIGATION_POLICY_PROHIBIT_DYNAMIC_CODE_ALWAYS_ON                : u64 = 0x00000001u64 << 36;
const PROCESS_CREATION_MITIGATION_POLICY_PROHIBIT_DYNAMIC_CODE_ALWAYS_OFF               : u64 = 0x00000002u64 << 36;
const PROCESS_CREATION_MITIGATION_POLICY_PROHIBIT_DYNAMIC_CODE_ALWAYS_ON_ALLOW_OPT_OUT  : u64 = 0x00000003u64 << 36;

const PROCESS_CREATION_MITIGATION_POLICY_CONTROL_FLOW_GUARD_MASK                        : u64 = 0x00000003u64 << 40;
const PROCESS_CREATION_MITIGATION_POLICY_CONTROL_FLOW_GUARD_DEFER                       : u64 = 0x00000000u64 << 40;
const PROCESS_CREATION_MITIGATION_POLICY_CONTROL_FLOW_GUARD_ALWAYS_ON                   : u64 = 0x00000001u64 << 40;
const PROCESS_CREATION_MITIGATION_POLICY_CONTROL_FLOW_GUARD_ALWAYS_OFF                  : u64 = 0x00000002u64 << 40;
const PROCESS_CREATION_MITIGATION_POLICY_CONTROL_FLOW_GUARD_EXPORT_SUPPRESSION          : u64 = 0x00000003u64 << 40;

const PROCESS_CREATION_MITIGATION_POLICY_BLOCK_NON_MICROSOFT_BINARIES_MASK              : u64 = 0x00000003u64 << 44;
const PROCESS_CREATION_MITIGATION_POLICY_BLOCK_NON_MICROSOFT_BINARIES_DEFER             : u64 = 0x00000000u64 << 44;
const PROCESS_CREATION_MITIGATION_POLICY_BLOCK_NON_MICROSOFT_BINARIES_ALWAYS_ON         : u64 = 0x00000001u64 << 44;
const PROCESS_CREATION_MITIGATION_POLICY_BLOCK_NON_MICROSOFT_BINARIES_ALWAYS_OFF        : u64 = 0x00000002u64 << 44;
const PROCESS_CREATION_MITIGATION_POLICY_BLOCK_NON_MICROSOFT_BINARIES_ALLOW_STORE       : u64 = 0x00000003u64 << 44;

const PROCESS_CREATION_MITIGATION_POLICY_FONT_DISABLE_MASK                              : u64 = 0x00000003u64 << 48;
const PROCESS_CREATION_MITIGATION_POLICY_FONT_DISABLE_DEFER                             : u64 = 0x00000000u64 << 48;
const PROCESS_CREATION_MITIGATION_POLICY_FONT_DISABLE_ALWAYS_ON                         : u64 = 0x00000001u64 << 48;
const PROCESS_CREATION_MITIGATION_POLICY_FONT_DISABLE_ALWAYS_OFF                        : u64 = 0x00000002u64 << 48;
const PROCESS_CREATION_MITIGATION_POLICY_AUDIT_NONSYSTEM_FONTS                          : u64 = 0x00000003u64 << 48;

const PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_NO_REMOTE_MASK                      : u64 = 0x00000003u64 << 52;
const PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_NO_REMOTE_DEFER                     : u64 = 0x00000000u64 << 52;
const PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_NO_REMOTE_ALWAYS_ON                 : u64 = 0x00000001u64 << 52;
const PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_NO_REMOTE_ALWAYS_OFF                : u64 = 0x00000002u64 << 52;
const PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_NO_REMOTE_RESERVED                  : u64 = 0x00000003u64 << 52;

const PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_NO_LOW_LABEL_MASK                   : u64 = 0x00000003u64 << 56;
const PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_NO_LOW_LABEL_DEFER                  : u64 = 0x00000000u64 << 56;
const PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_NO_LOW_LABEL_ALWAYS_ON              : u64 = 0x00000001u64 << 56;
const PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_NO_LOW_LABEL_ALWAYS_OFF             : u64 = 0x00000002u64 << 56;
const PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_NO_LOW_LABEL_RESERVED               : u64 = 0x00000003u64 << 56;

const PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_PREFER_SYSTEM32_MASK                : u64 = 0x00000003u64 << 60;
const PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_PREFER_SYSTEM32_DEFER               : u64 = 0x00000000u64 << 60;
const PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_PREFER_SYSTEM32_ALWAYS_ON           : u64 = 0x00000001u64 << 60;
const PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_PREFER_SYSTEM32_ALWAYS_OFF          : u64 = 0x00000002u64 << 60;
const PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_PREFER_SYSTEM32_RESERVED            : u64 = 0x00000003u64 << 60;
