use super::*;
use crate::prelude::*;
use bytemuck::*;
use winapi::um::winnt::*;



#[doc(alias = "PROCESS_MITIGATION_DEP_POLICY")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-process_mitigation_dep_policy)\]
/// ≈ PROCESS_MITIGATION_DEP_POLICY
///
/// Data Execution Prevention (DEP) policies, preventing the execution of arbitrary data.
/// This makes the exploitation of buffer overflows for code execution vulnerabilities more difficult.
/// On modern systems and build setups, this is configured for you by default.
///
/// ### Example
///
/// ```
/// # use firehazard::*;
/// # use winapi::shared::winerror::ERROR_NOT_SUPPORTED;
/// #
/// // Already enabled:
/// let dep : process::mitigation::DepPolicy = get_process_mitigation_policy(get_current_process()).unwrap();
/// assert!(dep.enable);
/// assert!(dep.disable_atl_thunk_emulation);
/// assert!(dep.permanent);
///
/// // Cannot modify:
/// assert_eq!(ERROR_NOT_SUPPORTED, set_process_mitigation_policy(dep).unwrap_err());
/// ```
///
/// ### References
/// *   [learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/memory/data-execution-prevention)
/// *   [en.wikipedia.org](https://en.wikipedia.org/wiki/Executable_space_protection#Windows)
///
#[derive(Clone, Copy, Debug)]
#[derive(Zeroable)]
pub struct DepPolicy {
    /// **Enable Always:**
    /// Prevents the execution of pages not marked executable.
    /// The only reason you'd wish to disable this, is to work with legacy code that can't be fixed/updated, that fails to properly mark pages as executable before executing them.
    ///
    pub enable:                         bool,

    /// **Enable Always:**
    /// Disables some (security weakening?) workarounds for bugs in the ActiveX Template Library (version 7.1 and earlier) where it fails to properly mark pages as executable before executing them.
    /// Such emulation is unnecessary for ATL 8.0+ (released with Visual Studio 2005), or in the very likely case where you're not using ATL at all.
    /// The only reason you'd wish to disable this, is to work with legacy code that can't be fixed/updated, that uses ATL 7.1 or earlier.
    ///
    pub disable_atl_thunk_emulation:    bool,

    /// **Enable Always:**
    /// Prevents disabling DEP at a later point in time.
    /// The only reason you'd wish to disable this, is to work with legacy code that can't be fixed/updated, that fails to properly mark pages as executable before executing them.
    ///
    pub permanent:                      bool,

    #[doc(hidden)] pub _reserved_flags: ()
}

unsafe impl GetPolicy for PROCESS_MITIGATION_DEP_POLICY {
    type Raw = Self;
    fn ty() -> process::mitigation::Policy { process::DEPPolicy }
    fn from_policy(p: Self::Raw) -> Self { p }
}

impl SetPolicy for PROCESS_MITIGATION_DEP_POLICY {
    fn into_policy(self) -> Self::Raw { self }
}

unsafe impl GetPolicy for DepPolicy {
    type Raw = PROCESS_MITIGATION_DEP_POLICY;
    fn ty() -> process::mitigation::Policy { process::DEPPolicy }
    fn from_policy(p: Self::Raw) -> Self { p.into() }
}

impl SetPolicy for DepPolicy {
    fn into_policy(self) -> Self::Raw { self.into() }
}

impl From<DepPolicy> for PROCESS_MITIGATION_DEP_POLICY {
    fn from(i: DepPolicy) -> Self {
        let mut o = Self::default();
        o.set_Enable                    (i.enable                       as u32);
        o.set_DisableAtlThunkEmulation  (i.disable_atl_thunk_emulation  as u32);
        o.Permanent = i.permanent as _;
        o
    }
}

impl From<PROCESS_MITIGATION_DEP_POLICY> for DepPolicy {
    fn from(i: PROCESS_MITIGATION_DEP_POLICY) -> Self {
        let mut o = Self::zeroed();
        o.enable                        = i.Enable()                    != 0;
        o.disable_atl_thunk_emulation   = i.DisableAtlThunkEmulation()  != 0;
        o.permanent                     = i.Permanent                   != 0;
        o
    }
}
