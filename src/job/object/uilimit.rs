//! \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_basic_ui_restrictions)\]
//! JOB_OBJECT_UILIMIT_*

#[allow(unused_imports)] use crate::prelude::*;
use core::fmt::{self, Debug, Formatter};



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_basic_ui_restrictions)\]
/// DWORD/[u32]: [job::object::BasicUiRestrictions] flags mask
///
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct FlagsMask(u32);

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_basic_ui_restrictions)\]
/// DWORD/[u32]: [job::object::BasicUiRestrictions] flags
///
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



#[doc(alias = "JOB_OBJECT_UILIMIT_DESKTOP")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_basic_ui_restrictions)\]
/// JOB_OBJECT_UILIMIT_DESKTOP
/// <br>
/// Forbids calling [create_desktop_a] or [switch_desktop].
/// <br><br>
///
pub const DESKTOP : Flags = Flags(winapi::um::winnt::JOB_OBJECT_UILIMIT_DESKTOP);



#[doc(alias = "JOB_OBJECT_UILIMIT_DISPLAYSETTINGS")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_basic_ui_restrictions)\]
/// JOB_OBJECT_UILIMIT_DISPLAYSETTINGS
/// <br>
/// Forbids calling [ChangeDisplaySettings].
/// <br><br>
///
/// [ChangeDisplaySettings]:    https://learn.microsoft.com/en-us/windows/desktop/api/winuser/nf-winuser-changedisplaysettingsa
///
pub const DISPLAYSETTINGS : Flags = Flags(winapi::um::winnt::JOB_OBJECT_UILIMIT_DISPLAYSETTINGS);



#[doc(alias = "JOB_OBJECT_UILIMIT_EXITWINDOWS")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_basic_ui_restrictions)\]
/// JOB_OBJECT_UILIMIT_EXITWINDOWS
/// <br>
/// Forbids calling [ExitWindows] or [ExitWindowsEx].
/// <br><br>
///
/// [ExitWindows]:      https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-exitwindows
/// [ExitWindowsEx]:    https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-exitwindowsex
///
pub const EXITWINDOWS : Flags = Flags(winapi::um::winnt::JOB_OBJECT_UILIMIT_EXITWINDOWS);



#[doc(alias = "JOB_OBJECT_UILIMIT_GLOBALATOMS")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_basic_ui_restrictions)\]
/// JOB_OBJECT_UILIMIT_GLOBALATOMS
/// <br>
/// Forbids accessing global [atoms]. When this flag is used, the job has its own atom table.
/// <br><br>
///
/// [atoms]:    https://learn.microsoft.com/en-us/windows/win32/dataxchg/about-atom-tables
///
pub const GLOBALATOMS : Flags = Flags(winapi::um::winnt::JOB_OBJECT_UILIMIT_GLOBALATOMS);



#[doc(alias = "JOB_OBJECT_UILIMIT_HANDLES")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_basic_ui_restrictions)\]
/// JOB_OBJECT_UILIMIT_HANDLES
/// <br>
/// Forbids using USER handles owned by processes not associated with the same job.
/// <br><br>
///
pub const HANDLES : Flags = Flags(winapi::um::winnt::JOB_OBJECT_UILIMIT_HANDLES);



#[doc(alias = "JOB_OBJECT_UILIMIT_READCLIPBOARD")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_basic_ui_restrictions)\]
/// JOB_OBJECT_UILIMIT_READCLIPBOARD
/// <br>
/// Forbids reading data from the clipboard.
/// <br><br>
///
pub const READCLIPBOARD : Flags = Flags(winapi::um::winnt::JOB_OBJECT_UILIMIT_READCLIPBOARD);



#[doc(alias = "JOB_OBJECT_UILIMIT_SYSTEMPARAMETERS")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_basic_ui_restrictions)\]
/// JOB_OBJECT_UILIMIT_SYSTEMPARAMETERS
/// <br>
/// Forbids calling [SystemParametersInfo].
/// <br><br>
///
/// [SystemParametersInfo]: https://learn.microsoft.com/en-us/windows/desktop/api/winuser/nf-winuser-systemparametersinfoa
///
pub const SYSTEMPARAMETERS : Flags = Flags(winapi::um::winnt::JOB_OBJECT_UILIMIT_SYSTEMPARAMETERS);



#[doc(alias = "JOB_OBJECT_UILIMIT_WRITECLIPBOARD")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_basic_ui_restrictions)\]
/// JOB_OBJECT_UILIMIT_WRITECLIPBOARD
/// <br>
/// Forbids writing data to the clipboard.
/// <br><br>
///
pub const WRITECLIPBOARD : Flags = Flags(winapi::um::winnt::JOB_OBJECT_UILIMIT_WRITECLIPBOARD);



#[doc(alias = "JOB_OBJECT_UILIMIT_NONE")]
/// \[<strike>microsoft.com</strike>\]
/// JOB_OBJECT_UILIMIT_NONE
/// <br>
/// Presumably doesn't forbid anything.
/// <br><br>
///
pub const NONE : Flags = Flags(winapi::um::winnt::JOB_OBJECT_UILIMIT_NONE);



#[doc(alias = "JOB_OBJECT_UILIMIT_IME")]
/// \[<strike>microsoft.com</strike>\]
/// JOB_OBJECT_UILIMIT_IME
/// <br>
/// Presumably forbids calling [some Imm* function(s)](https://learn.microsoft.com/en-us/windows/win32/intl/input-method-manager-functions) dealing with [Input Method Editor](https://en.wikipedia.org/wiki/Input_method)s?
/// <br><br>
///
pub const IME : Flags = Flags(JOB_OBJECT_UILIMIT_IME);



#[doc(alias = "JOB_OBJECT_UILIMIT_ALL")]
/// \[<strike>microsoft.com</strike>\]
/// JOB_OBJECT_UILIMIT_ALL
/// <br>
/// Presumably forbids all the calls specified by the other constants in this module.
/// <br><br>
///
pub const ALL : FlagsMask = FlagsMask(JOB_OBJECT_UILIMIT_ALL);



// C:\Program Files (x86)\Windows Kits\10\Include\10.0.22621.0\um\winnt.h
const JOB_OBJECT_UILIMIT_IME : u32 = 0x00000100;
const JOB_OBJECT_UILIMIT_ALL : u32 = 0x000001FF;
