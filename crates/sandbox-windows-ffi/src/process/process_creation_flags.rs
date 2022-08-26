#[allow(unused_imports)] use crate::*;
use core::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/procthread/process-creation-flags)\] Process creation flags mask
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct CreationFlagsMask(u32);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/procthread/process-creation-flags)\] Process creation flags
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct CreationFlags(u32);

flags!(impl .. for CreationFlags(u32) - CreationFlagsMask { });

impl CreationFlags {
    /// ### Safety
    /// *   Some APIs might theoretically assume access rights are a valid?
    pub const unsafe fn from_unchecked(rights: u32) -> Self { Self(rights) }
}

impl Debug for CreationFlags {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        const CREATE_SECURE_PROCESS : u32 = 0x00400000;
        use winapi::um::winbase::*;
        flags!(self.0, fmt, "0x{:X}", [
            DEBUG_PROCESS,
            DEBUG_ONLY_THIS_PROCESS,
            CREATE_SUSPENDED,
            DETACHED_PROCESS,
            CREATE_NEW_CONSOLE,
            CREATE_NEW_PROCESS_GROUP,
            CREATE_UNICODE_ENVIRONMENT,
            CREATE_SEPARATE_WOW_VDM,
            CREATE_SHARED_WOW_VDM,
            INHERIT_PARENT_AFFINITY,
            CREATE_PROTECTED_PROCESS,
            EXTENDED_STARTUPINFO_PRESENT,
            CREATE_SECURE_PROCESS,
            CREATE_BREAKAWAY_FROM_JOB,
            CREATE_PRESERVE_CODE_AUTHZ_LEVEL,
            CREATE_DEFAULT_ERROR_MODE,
            CREATE_NO_WINDOW,
        ])
    }
}

/// Make the current process a debugger of the process being launched and it's children.
pub const DEBUG_PROCESS                     : CreationFlags = CreationFlags(winapi::um::winbase::DEBUG_PROCESS                      ); // 0x00000001

/// Make the current process a debugger of the process being launched (but not it's children.)
pub const DEBUG_ONLY_THIS_PROCESS           : CreationFlags = CreationFlags(winapi::um::winbase::DEBUG_ONLY_THIS_PROCESS            ); // 0x00000002

/// Start the primary thread of the new process in a suspended state - it will not run until [resume_thread] is called.
pub const CREATE_SUSPENDED                  : CreationFlags = CreationFlags(winapi::um::winbase::CREATE_SUSPENDED                   ); // 0x00000004

/// Detatch the new child process from the parent's console window.
pub const DETACHED_PROCESS                  : CreationFlags = CreationFlags(winapi::um::winbase::DETACHED_PROCESS                   ); // 0x00000008

/// Detatch the new child process from the parent's console window and create a new one.
pub const CREATE_NEW_CONSOLE                : CreationFlags = CreationFlags(winapi::um::winbase::CREATE_NEW_CONSOLE                 ); // 0x00000010

/// Create a new process group (used for `Ctrl`+`Break` reception / disables `Ctrl`+`C`)
pub const CREATE_NEW_PROCESS_GROUP          : CreationFlags = CreationFlags(winapi::um::winbase::CREATE_NEW_PROCESS_GROUP           ); // 0x00000200

/// The environment block `environment` uses Unicode (e.g. UTF16) characters
///
/// If not set, the environment block `environment` uses ANSI (e.g. ASCII? Windows-1251? UTF8?) characters instead.
pub const CREATE_UNICODE_ENVIRONMENT        : CreationFlags = CreationFlags(winapi::um::winbase::CREATE_UNICODE_ENVIRONMENT         ); // 0x00000400

/// Use a separate, private [NTVDM](https://docs.microsoft.com/en-us/windows/compatibility/ntvdm-and-16-bit-app-support) process for 16-bit executables.
pub const CREATE_SEPARATE_WOW_VDM           : CreationFlags = CreationFlags(winapi::um::winbase::CREATE_SEPARATE_WOW_VDM            ); // 0x00000800

/// (Re)use a shared, common [NTVDM](https://docs.microsoft.com/en-us/windows/compatibility/ntvdm-and-16-bit-app-support) process for 16-bit executables.
pub const CREATE_SHARED_WOW_VDM             : CreationFlags = CreationFlags(winapi::um::winbase::CREATE_SHARED_WOW_VDM              ); // 0x00001000

/// Inherit the parent's cpu core affinities.
pub const INHERIT_PARENT_AFFINITY           : CreationFlags = CreationFlags(winapi::um::winbase::INHERIT_PARENT_AFFINITY            ); // 0x00010000

/// Run specially signed Microsoft binaries (media foundation, audio engine, windows error reporting, and system) as a
/// "protected process" - the system guards the process and it's threads (mostly for DRM purpouses?)
///
/// For more information, see [Overview of the Protected Media Path](https://docs.microsoft.com/en-us/windows/win32/medfound/protected-media-path).
pub const CREATE_PROTECTED_PROCESS          : CreationFlags = CreationFlags(winapi::um::winbase::CREATE_PROTECTED_PROCESS           ); // 0x00040000

/// [process::StartupInfoExW] is passed to `startup_info`, allowing the use of a [process::ThreadAttributeList].
///
/// The configuration of said thread attribute list can result in [create_process_as_user_w] returning various error codes.
pub const EXTENDED_STARTUPINFO_PRESENT      : CreationFlags = CreationFlags(winapi::um::winbase::EXTENDED_STARTUPINFO_PRESENT       ); // 0x00080000

/// Spawn as a [Isolated User Mode (IUM) Process / Trustlet](https://docs.microsoft.com/en-us/windows/win32/procthread/isolated-user-mode--ium--processes) ?
pub const CREATE_SECURE_PROCESS             : CreationFlags = CreationFlags(0x00400000                                              ); // 0x00400000

/// Don't associate the new child process with the parent process's job, if any.
///
/// (The job must have JOB_OBJECT_LIMIT_BREAKAWAY_OK set.)
pub const CREATE_BREAKAWAY_FROM_JOB         : CreationFlags = CreationFlags(winapi::um::winbase::CREATE_BREAKAWAY_FROM_JOB          ); // 0x01000000

/// Bypass AppLocker & SAFER ?
///
/// > Allows the caller to execute a child process that bypasses the process restrictions that would normally be applied automatically to the process.
pub const CREATE_PRESERVE_CODE_AUTHZ_LEVEL  : CreationFlags = CreationFlags(winapi::um::winbase::CREATE_PRESERVE_CODE_AUTHZ_LEVEL   ); // 0x02000000

/// Don't inherit the [error mode](https://docs.microsoft.com/en-us/windows/win32/debug/error-mode) of the calling process.
pub const CREATE_DEFAULT_ERROR_MODE         : CreationFlags = CreationFlags(winapi::um::winbase::CREATE_DEFAULT_ERROR_MODE          ); // 0x04000000

/// Don't spawn a console window for the child process.
pub const CREATE_NO_WINDOW                  : CreationFlags = CreationFlags(winapi::um::winbase::CREATE_NO_WINDOW                   ); // 0x08000000
