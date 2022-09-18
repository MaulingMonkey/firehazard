//! \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-updateprocthreadattribute)\] UpdateProcThreadAttribute value constants for use with<br>
//! PROC_THREAD_ATTRIBUTE_CHILD_PROCESS_POLICY
//!
//! Windows 10+

use crate::process::creation::ChildProcessPolicyFlags;
use core::fmt::{self, Debug, Formatter};



/// The process being created is not allowed to create child processes.
/// This restriction becomes a property of the token as which the process runs.
/// It should be noted that this restriction is only effective in sandboxed applications (such as AppContainer) which ensure privileged process handles are not accessible to the process.
/// For example, if a process restricting child process creation is able to access another process handle with PROCESS_CREATE_PROCESS or PROCESS_VM_WRITE access rights, then it may be possible to bypass the child process restriction.
pub const RESTRICTED    : ChildProcessPolicyFlags = ChildProcessPolicyFlags(PROCESS_CREATION_CHILD_PROCESS_RESTRICTED);

/// The process being created is allowed to create a child process, if it would otherwise be restricted.
/// You can only specify this value if the process that is creating the new process is not restricted.
pub const OVERRIDE      : ChildProcessPolicyFlags = ChildProcessPolicyFlags(PROCESS_CREATION_CHILD_PROCESS_OVERRIDE);

impl Debug for ChildProcessPolicyFlags {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        flags!(self.0, fmt, "0x{:02X}", [
            PROCESS_CREATION_CHILD_PROCESS_RESTRICTED,
            PROCESS_CREATION_CHILD_PROCESS_OVERRIDE,
        ])
    }
}

const PROCESS_CREATION_CHILD_PROCESS_RESTRICTED : u32 = 0x01;
const PROCESS_CREATION_CHILD_PROCESS_OVERRIDE   : u32 = 0x02;
