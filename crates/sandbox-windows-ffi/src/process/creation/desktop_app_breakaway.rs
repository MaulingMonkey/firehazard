//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-updateprocthreadattribute)\] UpdateProcThreadAttribute value constants for use with<br>
//! PROC_THREAD_ATTRIBUTE_DESKTOP_APP_POLICY
//!
//! Windows 10 version 1703 +



/// The process being created will create any child processes outside of the desktop app runtime environment.
/// This behavior is the default for processes for which no policy has been set.
pub const ENABLE_PROCESS_TREE    : u32 = 0x01;

/// The process being created will create any child processes inside of the desktop app runtime environment.
/// This policy is inherited by the descendant processes until it is overridden by creating a process with PROCESS_CREATION_DESKTOP_APP_BREAKAWAY_ENABLE_PROCESS_TREE.
pub const DISABLE_PROCESS_TREE   : u32 = 0x02;

/// The process being created will run inside the desktop app runtime environment.
/// This policy applies only to the process being created, not its descendants.
pub const OVERRIDE               : u32 = 0x04;
