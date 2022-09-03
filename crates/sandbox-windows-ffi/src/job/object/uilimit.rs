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
        ])
    }
}

/// Prevents processes associated with the job from creating desktops and switching desktops using the [create_desktop_a] and [switch_desktop] functions.
pub const DESKTOP : Flags = Flags(winapi::um::winnt::JOB_OBJECT_UILIMIT_DESKTOP);

/// Prevents processes associated with the job from calling the [ChangeDisplaySettings](https://docs.microsoft.com/en-us/windows/desktop/api/winuser/nf-winuser-changedisplaysettingsa) function.
pub const DISPLAYSETTINGS : Flags = Flags(winapi::um::winnt::JOB_OBJECT_UILIMIT_DISPLAYSETTINGS);

/// Prevents processes associated with the job from calling the ExitWindows or ExitWindowsEx function.
pub const EXITWINDOWS : Flags = Flags(winapi::um::winnt::JOB_OBJECT_UILIMIT_EXITWINDOWS);

/// Prevents processes associated with the job from accessing global atoms. When this flag is used, each job has its own atom table.
pub const GLOBALATOMS : Flags = Flags(winapi::um::winnt::JOB_OBJECT_UILIMIT_GLOBALATOMS);

/// Prevents processes associated with the job from using USER handles owned by processes not associated with the same job.
pub const HANDLES : Flags = Flags(winapi::um::winnt::JOB_OBJECT_UILIMIT_HANDLES);

/// Prevents processes associated with the job from reading data from the clipboard.
pub const READCLIPBOARD : Flags = Flags(winapi::um::winnt::JOB_OBJECT_UILIMIT_READCLIPBOARD);

/// Prevents processes associated with the job from changing system parameters by using the [SystemParametersInfo](https://docs.microsoft.com/en-us/windows/desktop/api/winuser/nf-winuser-systemparametersinfoa) function.
pub const SYSTEMPARAMETERS : Flags = Flags(winapi::um::winnt::JOB_OBJECT_UILIMIT_SYSTEMPARAMETERS);

/// Prevents processes associated with the job from writing data to the clipboard.
pub const WRITECLIPBOARD : Flags = Flags(winapi::um::winnt::JOB_OBJECT_UILIMIT_WRITECLIPBOARD);
