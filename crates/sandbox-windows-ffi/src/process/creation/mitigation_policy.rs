//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-updateprocthreadattribute)\] UpdateProcThreadAttribute value constants for use with<br>
//! PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY



/// "legacy" creation mitigation policy options
///
/// Windows 7+
pub mod legacy {
    #[cfg(doc)] use crate::*;

    /// Enables data execution prevention (DEP) for the child process.
    /// For more information, see [Data Execution Prevention](https://docs.microsoft.com/en-us/windows/desktop/Memory/data-execution-prevention).
    pub const DEP_ENABLE            : u64 = 0x01;

    /// Enables DEP-ATL thunk emulation for the child process.
    /// DEP-ATL thunk emulation causes the system to intercept NX faults that originate from the Active Template Library (ATL) thunk layer.
    /// This value can be specified only with [process::creation::mitigation_policy::DEP_ENABLE].
    pub const DEP_ATL_THUNK_ENABLE  : u64 = 0x02;

    /// Enables structured exception handler overwrite protection (SEHOP) for the child process.
    /// SEHOP blocks exploits that use the structured exception handler (SEH) overwrite technique.
    pub const SEHOP_ENABLE          : u64 = 0x04;
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
    pub const MASK                  : u64 = 0x00000003 << 8;
    pub const DEFER                 : u64 = 0x00000000 << 8;
    pub const ALWAYS_ON             : u64 = 0x00000001 << 8;
    pub const ALWAYS_OFF            : u64 = 0x00000002 << 8;
    pub const ALWAYS_ON_REQ_RELOCS  : u64 = 0x00000003 << 8;
}

/// Heap terminate on Corruption
///
/// Note that 'always off' does not override the default opt-in for binaries with current subsystem versions set in the image header.
///
/// Heap terminate on corruption is user mode enforced.
///
/// Windows 8+
pub mod heap_terminate {
    pub const MASK                  : u64 = 0x00000003 << 12;
    pub const DEFER                 : u64 = 0x00000000 << 12;
    pub const ALWAYS_ON             : u64 = 0x00000001 << 12;
    pub const ALWAYS_OFF            : u64 = 0x00000002 << 12;
    pub const RESERVED              : u64 = 0x00000003 << 12;
}

/// Bottom up randomization (includes stack randomization)
///
/// (e.g. randomization of the lowest user address.)
///
/// Windows 8+
pub mod bottom_up_aslr {
    pub const MASK                  : u64 = 0x00000003 << 16;
    pub const DEFER                 : u64 = 0x00000000 << 16;
    pub const ALWAYS_ON             : u64 = 0x00000001 << 16;
    pub const ALWAYS_OFF            : u64 = 0x00000002 << 16;
    pub const RESERVED              : u64 = 0x00000003 << 16;
}

/// High entropy bottom up randomization.
/// Note that high entropy bottom up randomization is effective if and only if bottom up ASLR is also enabled.
///
/// N.B.  High entropy mode is only meaningful for native 64-bit processes.
/// In high entropy mode, up to 1TB of bottom up variance is enabled.
///
/// Windows 8+
pub mod high_entropy_aslr {
    pub const MASK                  : u64 = 0x00000003 << 20;
    pub const DEFER                 : u64 = 0x00000000 << 20;
    pub const ALWAYS_ON             : u64 = 0x00000001 << 20;
    pub const ALWAYS_OFF            : u64 = 0x00000002 << 20;
    pub const RESERVED              : u64 = 0x00000003 << 20;
}

/// Handle checking enforcement options.
///
/// Handle checking enforcement causes an exception to be raised immediately on a
/// bad handle reference, versus simply returning a failure status from the handle reference.
///
/// Windows 8+
pub mod strict_handle_checks {
    pub const MASK                  : u64 = 0x00000003 << 24;
    pub const DEFER                 : u64 = 0x00000000 << 24;
    pub const ALWAYS_ON             : u64 = 0x00000001 << 24;
    pub const ALWAYS_OFF            : u64 = 0x00000002 << 24;
    pub const RESERVED              : u64 = 0x00000003 << 24;
}

/// Win32k system call disable prevents a process from making Win32k calls.
///
/// Windows 8+
pub mod win32k_system_call_disable {
    pub const MASK                  : u64 = 0x00000003 << 28;
    pub const DEFER                 : u64 = 0x00000000 << 28;
    pub const ALWAYS_ON             : u64 = 0x00000001 << 28;
    pub const ALWAYS_OFF            : u64 = 0x00000002 << 28;
    pub const RESERVED              : u64 = 0x00000003 << 28;
}

/// Extension point disable allows a process to opt-out of loading various arbitrary extension point DLLs.
///
/// Windows 8+
pub mod extension_point_disable {
    pub const MASK                  : u64 = 0x00000003u64 << 32;
    pub const DEFER                 : u64 = 0x00000000u64 << 32;
    pub const ALWAYS_ON             : u64 = 0x00000001u64 << 32;
    pub const ALWAYS_OFF            : u64 = 0x00000002u64 << 32;
    pub const RESERVED              : u64 = 0x00000003u64 << 32;
}

/// Dynamic code
///
/// Windows 8.1+
pub mod prohibit_dynamic_code {
    pub const MASK                      : u64 = 0x00000003u64 << 36;
    pub const DEFER                     : u64 = 0x00000000u64 << 36;
    pub const ALWAYS_ON                 : u64 = 0x00000001u64 << 36;
    pub const ALWAYS_OFF                : u64 = 0x00000002u64 << 36;
    pub const ALWAYS_ON_ALLOW_OPT_OUT   : u64 = 0x00000003u64 << 36;
}

/// Control Flow Guard (CFG) allows indirect control transfers to be checked at runtime.
///
/// Windows 8.1+
pub mod control_flow_guard {
    pub const MASK                  : u64 = 0x00000003u64 << 40;
    pub const DEFER                 : u64 = 0x00000000u64 << 40;
    pub const ALWAYS_ON             : u64 = 0x00000001u64 << 40;
    pub const ALWAYS_OFF            : u64 = 0x00000002u64 << 40;
    pub const EXPORT_SUPPRESSION    : u64 = 0x00000003u64 << 40;
}

/// Module signature options.  When enabled, this option will block mapping of non-microsoft binaries.
///
/// Windows 8.1+
pub mod block_non_microsoft_binaries {
    pub const MASK                  : u64 = 0x00000003u64 << 44;
    pub const DEFER                 : u64 = 0x00000000u64 << 44;
    pub const ALWAYS_ON             : u64 = 0x00000001u64 << 44;
    pub const ALWAYS_OFF            : u64 = 0x00000002u64 << 44;
    pub const ALLOW_STORE           : u64 = 0x00000003u64 << 44;
}

/// Font Disable Policy: Blocks loading non-system fonts.
///
/// Windows 10+
pub mod font_disable {
    pub const MASK                  : u64 = 0x00000003u64 << 48;
    pub const DEFER                 : u64 = 0x00000000u64 << 48;
    pub const ALWAYS_ON             : u64 = 0x00000001u64 << 48;
    pub const ALWAYS_OFF            : u64 = 0x00000002u64 << 48;
    pub const AUDIT_NONSYSTEM_FONTS : u64 = 0x00000003u64 << 48;
}
#[doc(hidden)] pub use font_disable::AUDIT_NONSYSTEM_FONTS; // not prefixed with "FONT_DISABLE_" like the rest of the font disabling constants

/// Block mapping of images from remote devices.
///
/// Windows 10+
pub mod image_load_no_remote {
    pub const MASK                  : u64 = 0x00000003u64 << 52;
    pub const DEFER                 : u64 = 0x00000000u64 << 52;
    pub const ALWAYS_ON             : u64 = 0x00000001u64 << 52;
    pub const ALWAYS_OFF            : u64 = 0x00000002u64 << 52;
    pub const RESERVED              : u64 = 0x00000003u64 << 52;
}

/// Block mapping of images that have the low mandatory label.
///
/// Windows 10+
pub mod image_load_no_low_label {
    pub const MASK                  : u64 = 0x00000003u64 << 56;
    pub const DEFER                 : u64 = 0x00000000u64 << 56;
    pub const ALWAYS_ON             : u64 = 0x00000001u64 << 56;
    pub const ALWAYS_OFF            : u64 = 0x00000002u64 << 56;
    pub const RESERVED              : u64 = 0x00000003u64 << 56;
}

/// Prefer loading images from the System32 folder over images in the application directory.
///
/// Windows 10+
pub mod image_load_prefer_system32 {
    pub const MASK                  : u64 = 0x00000003u64 << 60;
    pub const DEFER                 : u64 = 0x00000000u64 << 60;
    pub const ALWAYS_ON             : u64 = 0x00000001u64 << 60;
    pub const ALWAYS_OFF            : u64 = 0x00000002u64 << 60;
    pub const RESERVED              : u64 = 0x00000003u64 << 60;
}
