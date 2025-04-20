//! \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/winstation/desktops)\]
//! Desktop APIs
//!
//! Desktop objects are both a user interface element, and a security boundary.
//! Windows and processes on the same desktop can interact with each other by sending messages and inspecting each other's windows and handles by default, as well as sharing clipboards and other UI state.
//! Such access can also be somewhat restricted by:
//! *   [`job::object::uilimit`]s
//! *   [AppContainers](https://learn.microsoft.com/en-us/windows/win32/secauthz/appcontainer-isolation#window-isolation)
//! *   [Low Integrity Levels](https://github.com/chromium/chromium/blob/main/docs/design/sandbox.md#the-integrity-levels)
//!
//! Desktop objects are used by:
//! *   Windows itself, for secure password entry / login.
//! *   Windows itself, for multi-user terminal services server systems.
//! *   [Sysinternals Desktops](https://learn.microsoft.com/en-us/sysinternals/downloads/desktops)
//! *   [Chrome](https://github.com/chromium/chromium/blob/main/docs/design/sandbox.md#the-alternate-desktop) uses a non-visible desktop for sandboxing code that uses `user32.dll`
//! *   [KeePass Password Safe's Secure Desktop Option](https://keepass.info/help/kb/sec_desk.html) (opt-in for secure password entry)
//!
//! Desktop objects are *not* used by Windows 10+ "virtual" desktops:
//! *   `Ctrl`+`Win`+`D`:   Create
//! *   `Ctrl`+`Win`+`F4`:  Close
//! *   `Ctrl`+`Win`+`🠞`:   Switch Right
//! *   `Ctrl`+`Win`+`🠜`:   Switch Left
//!
//! Specifying an invalid or inaccessible [`process::StartupInfo::desktop`] will cause `user32.dll`, if referenced, to `STATUS_DLL_INIT_FAILED` before main is reached.
//! The default permissions of [`create_desktop_a`] may require [`sid::integrity::Medium`] to access, whereas the built-in desktop might only require [`sid::integrity::Low`].
//! Using `set_thread_desktop` / [`with_thread_desktop`] will *not* set the desktop of processes spawned by said thread.
//!
//! On my system, by default, the following desktops exist for the logged in user:
//! *   `WinSta0\Default` (the current/default/real desktop of said window station)
//! *   `Service-0x0-21cd8$\Default` (interactive services desktop)
//!
//! Other desktops owned by the system / WinLogin may not be visible to enumeration.
//!
//!
//!
//! ## Visible Desktops
//!
//! You can trivially [`create_desktop_a`] and [`switch_desktop`] to a new desktop.
//! Such new desktops lack an associated `explorer.exe` process / shell by default, meaning:
//! *   No desktop background
//! *   No start menu / taskbar
//! *   Most [General Windows Shortcuts](https://ss64.com/nt/syntax-keyboard.html) aren't in use or available to conflict with your own shortcuts.
//!     The following still work however:
//!     *   `Ctrl`+`Shift`+`Esc`:   Task Manager (lets you run commands in a pinch via the File menu)
//!     *   `Ctrl`+`Alt`+`Del`:     Windows Security Screen
//!
//! Windows 10 is pretty good about not letting you get stuck on the desktop.
//! It appears that if there is no process running on the desktop, and no process has open handles to said desktop,
//! you'll be booted back to the `Default` desktop and said desktop will be destroyed,
//! without the need for any watchdog processes or other cleanup.
//!
//! Of course, if you leak or hold onto a desktop handle from a long running process without any visible UI on said desktop,
//! the user might need to hunt down the offending process to kill it.
//!
//!
//!
//! ### See Also
//! *   [mod `winsta`](crate::winsta)   for window station APIs and references

#[allow(unused_imports)] use crate::prelude::*;

mod desktop_access_rights;              pub use desktop_access_rights::*;
mod desktop_flags;                      pub use desktop_flags::*;
mod desktop_handles;                    pub use desktop_handles::*;

pub use funcs::*;
#[path = "desktop_funcs.rs"] pub(crate) mod funcs;
