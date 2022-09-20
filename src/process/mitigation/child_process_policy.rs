use super::*;
use crate::*;
use bytemuck::*;
use winapi::um::winnt::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntddk/ns-ntddk-_process_mitigation_child_process_policy)\]
/// ~ [PROCESS_MITIGATION_CHILD_PROCESS_POLICY]
///
/// Lock down the process's ability to create child processes.
///
/// Aside from fork bombs, malicious code might spawn child processes in an attempt to bypass mitigations of the current process
/// (blocked system calls, disabled non-system fonts, restrictive DLL settings, blocked dynamic code, job restrictions, etc.)
///
/// Overlapping functionality with:
/// *   [process::ThreadAttributeRef::child_process_policy]\([process::creation::child_process::RESTRICTED]\) (applied by parent process)
/// *   [job::object::BasicLimitInformation::active_process_limit] (applied to jobs, which are escapeable!)
///
/// ### References
/// *   chromium/docs/design/sandbox.md
///     *   [The Job Object](https://github.com/chromium/chromium/blob/main/docs/design/sandbox.md#the-job-object)
///     *   [Extra Disable Child Process Creation](https://github.com/chromium/chromium/blob/main/docs/design/sandbox.md#extra-disable-child-process-creation)
/// *   [Mitigating arbitrary native code execution in Microsoft Edge](https://blogs.windows.com/msedgedev/2017/02/23/mitigating-arbitrary-native-code-execution/)
#[derive(Clone, Copy, Debug)]
#[derive(Zeroable)]
pub struct ChildProcessPolicy {
    /// **Consider enabling**: Prevents the creation of child processes.
    ///
    /// This will cause most uses of e.g. [`std::process::Command`] to fail, as well as windows API calls doing similar.
    /// There are, of course, many legitimate use cases for child processes - including respawning your current process with more secure on-startup settings - making this a "sometimes" feature.
    pub no_child_process_creation:          bool,

    /// **Disable for security:**  Weaken [`no_child_process_creation`](Self::no_child_process_creation) so it logs child process creation (to the Event Log?) instead of refusing to create them.
    pub audit_no_child_process_creation:    bool,

    /// **Disable for security:**  Weaken [`no_child_process_creation`](Self::no_child_process_creation)?
    ///
    /// I *think* this is mainly for querying to see if the parent process set [process::creation::child_process::OVERRIDE]?
    /// The documentation is not super amazing, but talks about this perhaps being somehow involved in one process creating child processes at the behest of a third process?
    pub allow_secure_process_creation:      bool,

    #[doc(hidden)] pub _reserved_flags: ()
}

impl ChildProcessPolicy {
    /// Configure the strictest options at this time.
    ///
    /// If new safety-enhancing fields are added, they will *not* be enabled - instead, a new fn (e.g. `strict_v2()`) will be introduced,
    /// and this one will be marked as deprecated (and possibly `#[cfg(...)]`ed out with the right `--cfg` flag for auditing purpouses.)
    pub fn strict_v1() -> Self {
        Self {
            no_child_process_creation:          true,
            audit_no_child_process_creation:    false,
            allow_secure_process_creation:      false,
            _reserved_flags:                    (),
        }
    }
}

unsafe impl GetPolicy for PROCESS_MITIGATION_CHILD_PROCESS_POLICY {
    type Raw = Self;
    fn ty() -> process::mitigation::Policy { process::ChildProcessPolicy }
    fn from_policy(p: Self::Raw) -> Self { p }
}

impl SetPolicy for PROCESS_MITIGATION_CHILD_PROCESS_POLICY {
    fn into_policy(self) -> Self::Raw { self }
}

unsafe impl GetPolicy for ChildProcessPolicy {
    type Raw = PROCESS_MITIGATION_CHILD_PROCESS_POLICY;
    fn ty() -> process::mitigation::Policy { process::ChildProcessPolicy }
    fn from_policy(p: Self::Raw) -> Self { p.into() }
}

impl SetPolicy for ChildProcessPolicy {
    fn into_policy(self) -> Self::Raw { self.into() }
}

impl From<ChildProcessPolicy> for PROCESS_MITIGATION_CHILD_PROCESS_POLICY {
    fn from(i: ChildProcessPolicy) -> Self {
        let mut o = Self::default();
        o.set_NoChildProcessCreation        (i.no_child_process_creation        as u32);
        o.set_AuditNoChildProcessCreation   (i.audit_no_child_process_creation  as u32);
        o.set_AllowSecureProcessCreation    (i.allow_secure_process_creation    as u32);
        o
    }
}

impl From<PROCESS_MITIGATION_CHILD_PROCESS_POLICY> for ChildProcessPolicy {
    fn from(i: PROCESS_MITIGATION_CHILD_PROCESS_POLICY) -> Self {
        let mut o = Self::zeroed();
        o.no_child_process_creation         = i.NoChildProcessCreation()        != 0;
        o.audit_no_child_process_creation   = i.AuditNoChildProcessCreation()   != 0;
        o.allow_secure_process_creation     = i.AllowSecureProcessCreation()    != 0;
        o
    }
}
