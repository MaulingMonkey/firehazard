//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_basic_ui_restrictions)\] JOB_OBJECT_UILIMIT_*

#[allow(unused_imports)] use crate::*;
use core::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_basic_ui_restrictions)\] DWORD/[u32]: [job::object::BasicUiRestrictions] flags mask
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct FlagsMask(u32);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_basic_ui_restrictions)\] DWORD/[u32]: [job::object::BasicUiRestrictions] flags
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Flags(u32);

flags!(impl .. for Flags(u32) - FlagsMask);

impl Flags {
    /// ### Safety
    /// *   Some APIs might theoretically assume flags are a valid?
    pub const unsafe fn from_unchecked(flags: u32) -> Self { Self(flags) }
}

impl Debug for Flags {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        use winapi::um::winnt::*;
        flags!(self.0, fmt, "0x{:X}", [
            JOB_OBJECT_UILIMIT_DESKTOP,
            JOB_OBJECT_UILIMIT_DISPLAYSETTINGS,
            JOB_OBJECT_UILIMIT_EXITWINDOWS,
            JOB_OBJECT_UILIMIT_GLOBALATOMS,
            JOB_OBJECT_UILIMIT_HANDLES,
            JOB_OBJECT_UILIMIT_READCLIPBOARD,
            JOB_OBJECT_UILIMIT_SYSTEMPARAMETERS,
            JOB_OBJECT_UILIMIT_WRITECLIPBOARD,
            JOB_OBJECT_UILIMIT_IME,
        ])
    }
}

/// Forbids calling [create_desktop_a] or [switch_desktop].
pub const DESKTOP : Flags = Flags(winapi::um::winnt::JOB_OBJECT_UILIMIT_DESKTOP);

/// Forbids calling [ChangeDisplaySettings].
///
/// [ChangeDisplaySettings]:    https://docs.microsoft.com/en-us/windows/desktop/api/winuser/nf-winuser-changedisplaysettingsa
pub const DISPLAYSETTINGS : Flags = Flags(winapi::um::winnt::JOB_OBJECT_UILIMIT_DISPLAYSETTINGS);

/// Forbids calling [ExitWindows] or [ExitWindowsEx].
///
/// [ExitWindows]:      https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-exitwindows
/// [ExitWindowsEx]:    https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-exitwindowsex
pub const EXITWINDOWS : Flags = Flags(winapi::um::winnt::JOB_OBJECT_UILIMIT_EXITWINDOWS);

/// Forbids accessing global [atoms]. When this flag is used, the job has its own atom table.
///
/// [atoms]:    https://docs.microsoft.com/en-us/windows/win32/dataxchg/about-atom-tables
pub const GLOBALATOMS : Flags = Flags(winapi::um::winnt::JOB_OBJECT_UILIMIT_GLOBALATOMS);

/// Forbids using USER handles owned by processes not associated with the same job.
pub const HANDLES : Flags = Flags(winapi::um::winnt::JOB_OBJECT_UILIMIT_HANDLES);

/// Forbids reading data from the clipboard.
pub const READCLIPBOARD : Flags = Flags(winapi::um::winnt::JOB_OBJECT_UILIMIT_READCLIPBOARD);

/// Forbids calling [SystemParametersInfo].
///
/// [SystemParametersInfo]: https://docs.microsoft.com/en-us/windows/desktop/api/winuser/nf-winuser-systemparametersinfoa
pub const SYSTEMPARAMETERS : Flags = Flags(winapi::um::winnt::JOB_OBJECT_UILIMIT_SYSTEMPARAMETERS);

/// Forbids writing data to the clipboard.
pub const WRITECLIPBOARD : Flags = Flags(winapi::um::winnt::JOB_OBJECT_UILIMIT_WRITECLIPBOARD);

pub const NONE : Flags = Flags(winapi::um::winnt::JOB_OBJECT_UILIMIT_NONE);
pub const IME : Flags = Flags(JOB_OBJECT_UILIMIT_IME);
pub const ALL : FlagsMask = FlagsMask(JOB_OBJECT_UILIMIT_ALL);

// C:\Program Files (x86)\Windows Kits\10\Include\10.0.22621.0\um\winnt.h
const JOB_OBJECT_UILIMIT_IME : u32 = 0x00000100;
const JOB_OBJECT_UILIMIT_ALL : u32 = 0x000001FF;
