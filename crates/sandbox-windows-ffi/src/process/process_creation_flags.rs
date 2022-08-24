use core::convert::Infallible;
use core::fmt::{self, Debug, Formatter};
use core::ops::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/procthread/process-creation-flags)\] Process creation flags mask
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct CreationFlagsMask(u32);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/procthread/process-creation-flags)\] Process creation flags
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct CreationFlags(u32);

impl CreationFlagsMask {
    pub fn as_u32(self) -> u32 { self.0 }
}

impl CreationFlags {
    /// ### Safety
    /// *   Some APIs might theoretically assume access rights are a valid?
    pub const unsafe fn from_unchecked(rights: u32) -> Self { Self(rights) }

    pub fn as_u32(self) -> u32 { self.0 }
}

impl Debug for CreationFlags {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        let mut v = self.0;
        if v == 0 { return write!(fmt, "0") }

        macro_rules! v { ($e:expr) => {{
            const E : u32 = $e.0;
            if v & E != 0 {
                write!(fmt, "{}", stringify!($e))?;
                v &= !E;
                if v != 0 { write!(fmt, " | ")?; }
            }
        }}}

        v!(DEBUG_PROCESS                    );
        v!(DEBUG_ONLY_THIS_PROCESS          );
        v!(CREATE_SUSPENDED                 );
        v!(DETACHED_PROCESS                 );
        v!(CREATE_NEW_CONSOLE               );
        v!(CREATE_NEW_PROCESS_GROUP         );
        v!(CREATE_UNICODE_ENVIRONMENT       );
        v!(CREATE_SEPARATE_WOW_VDM          );
        v!(CREATE_SHARED_WOW_VDM            );
        v!(INHERIT_PARENT_AFFINITY          );
        v!(CREATE_PROTECTED_PROCESS         );
        v!(EXTENDED_STARTUPINFO_PRESENT     );
        v!(CREATE_SECURE_PROCESS            );
        v!(CREATE_BREAKAWAY_FROM_JOB        );
        v!(CREATE_PRESERVE_CODE_AUTHZ_LEVEL );
        v!(CREATE_DEFAULT_ERROR_MODE        );
        v!(CREATE_NO_WINDOW                 );

        if v != 0 { write!(fmt, "0x{:X}", v)? }

        Ok(())
    }
}

impl From<()> for CreationFlags { fn from(_: ()) -> Self { Self(0) } }
impl From<Option<Infallible>> for CreationFlags { fn from(_: Option<Infallible>) -> Self { Self(0) } }
impl From<CreationFlags> for u32 { fn from(ar: CreationFlags) -> Self { ar.as_u32() } }

impl BitAnd         for CreationFlags { type Output = Self; fn bitand(self, rhs: Self) -> Self::Output { Self(self.as_u32() & rhs.as_u32()) } }
impl BitXor         for CreationFlags { type Output = Self; fn bitxor(self, rhs: Self) -> Self::Output { Self(self.as_u32() ^ rhs.as_u32()) } }
impl BitOr          for CreationFlags { type Output = Self; fn bitor (self, rhs: Self) -> Self::Output { Self(self.as_u32() | rhs.as_u32()) } }
impl BitAndAssign   for CreationFlags { fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.as_u32() } }
impl BitXorAssign   for CreationFlags { fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.as_u32() } }
impl BitOrAssign    for CreationFlags { fn bitor_assign (&mut self, rhs: Self) { self.0 |= rhs.as_u32() } }

impl Not                                for CreationFlags     { type Output = CreationFlagsMask; fn not(self) -> Self::Output { CreationFlagsMask(!self.as_u32()) } }
impl BitAnd<CreationFlagsMask>          for CreationFlags     { type Output = CreationFlags; fn bitand(self, rhs: CreationFlagsMask) -> CreationFlags { CreationFlags(self.as_u32() & rhs.as_u32()) } }
impl BitAnd<CreationFlags>              for CreationFlagsMask { type Output = CreationFlags; fn bitand(self, rhs: CreationFlags    ) -> CreationFlags { CreationFlags(self.as_u32() & rhs.as_u32()) } }
impl BitAndAssign<CreationFlagsMask>    for CreationFlags     { fn bitand_assign(&mut self, rhs: CreationFlagsMask) { self.0 &= rhs.as_u32() } }

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

/// [StartupInfoExW] is passed to `startup_info`, allowing the use of a [process::ThreadAttributeList].
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
