use super::*;
use crate::prelude::*;
use bytemuck::*;
use winapi::um::winnt::*;



#[doc(alias = "PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-process_mitigation_control_flow_guard_policy)\]
/// ≈ PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY
///
/// Function pointer validation settings.
///
/// Read/Write:
/// *   [`strict_mode`](Self::strict_mode): Ban loading DLLs that aren't CFG-aware
/// *   [`enable_xfg_audit_mode`](Self::enable_xfg_audit_mode):  Audit XFG (via the Event Log?) instead of enforcing function signatures?
///
/// Read-only:
/// *   [`enable_control_flow_guard`](Self::enable_control_flow_guard): Validate function pointers point at functions.
/// *   [`enable_export_suppression`](Self::enable_export_suppression): Don't allow dll exports as function pointer targets by default.
/// *   [`enable_xfg`](Self::enable_xfg)?: Validate function pointers point at functions **of the correct signature**.
///
/// ### References
/// *   [Advancing Windows Security (BlueHat Shanghai 2019)](https://query.prod.cms.rt.microsoft.com/cms/api/am/binary/RE37dMC)
/// *   [Control-flow integrity](https://en.wikipedia.org/wiki/Control-flow_integrity) (en.wikipedia.org)
///
#[derive(Clone, Copy, Debug)]
#[derive(Zeroable)]
pub struct ControlFlowGuardPolicy {
    /// **Read Only.**
    /// Control Flow Guard is enabled for the process.
    /// CFG-aware code will validate function pointers point to actual functions before calling them.
    ///
    /// This field cannot be changed via [`set_process_mitigation_policy`].
    /// However, it should be sufficient to build the executable with CFG enabled:
    ///
    /// ```toml
    /// # .cargo/config.toml
    /// [target]
    /// x86_64-pc-windows-msvc.rustflags = ["-Ccontrol-flow-guard=checks"]
    /// i686-pc-windows-msvc.rustflags   = ["-Ccontrol-flow-guard=checks"]
    /// ```
    ///
    /// In which case you could use this field to validate the executable was indeed
    /// built/launched with CFG enabled if you were feeling sufficiently paranoid:
    ///
    /// ```no_run
    /// use firehazard::*;
    /// use firehazard::process::mitigation::*;
    /// let cfg : ControlFlowGuardPolicy = get_process_mitigation_policy(get_current_process()).unwrap();
    /// assert!(cfg.enable_control_flow_guard, "See docs for setting up .cargo/config.toml to enable CFG");
    /// // sadly, this won't work for `cargo test` - rustflags not picked up?
    /// ```
    ///
    /// ### See Also
    /// *   [`process::creation::mitigation_policy::control_flow_guard::ALWAYS_ON`] (allows a parent process to enable CFG even for non-CFG-aware binaries?)
    /// *   [Control Flow Guard](https://learn.microsoft.com/en-us/windows/win32/secbp/control-flow-guard) (learn.microsoft.com)
    /// *   [Control-flow integrity: Microsoft Control Flow Guard](https://en.wikipedia.org/wiki/Control-flow_integrity#Microsoft_Control_Flow_Guard) (en.wikipedia.org)
    ///
    pub enable_control_flow_guard:  bool,

    /// **Read Only.**
    /// Treat exported functions (DLL-exported?) as invalid indirect call targets by default.
    /// Exported functions become valid call targets if referenced via [`GetProcAddress`](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getprocaddress).
    /// Cannot be changed via [`set_process_mitigation_policy`].
    ///
    /// I believe there's [PE metadata](https://learn.microsoft.com/en-us/windows/win32/secbp/pe-metadata#export-suppression) to forcibly enable this,
    /// but I can't seem to find build options in rustc nor MSVC to enable this by default.
    ///
    /// ### See Also
    /// *   [`process::creation::mitigation_policy::control_flow_guard::EXPORT_SUPPRESSION`] (allows a parent process to enable this at launch time)
    ///
    pub enable_export_suppression:  bool,

    /// **Consider enabling:**  Refuse to load any new DLLs that weren't built with CFG.
    ///
    /// Cannot be disabled once enabled.
    /// Previously loaded DLLs are presumably unaffected.
    /// Might block DLLs that weren't built with XFG if [`enable_xfg`](Self::enable_xfg) is enabled?
    /// Since most pure-Rust code is typically *statically* linked, there's a good chance you can use this option even with 3rd party crates without hassle.
    /// Rust-based plugins/DLLs may need to be built with CFG enabled:
    ///
    /// ```toml
    /// # .cargo/config.toml
    /// [target]
    /// x86_64-pc-windows-msvc.rustflags = ["-Ccontrol-flow-guard=checks"]
    /// i686-pc-windows-msvc.rustflags   = ["-Ccontrol-flow-guard=checks"]
    /// ```
    ///
    /// ### See Also
    /// *   [`process::creation::mitigation_policy2::strict_control_flow_guard::ALWAYS_ON`] (allows a parent process to enable this at launch time)
    /// *   [`process::mitigation::BinarySignaturePolicy::microsoft_signed_only`] - another DLL-blocking security policy, with a list of third party DLLs that could be affected.
    ///
    pub strict_mode:                bool,

    /// **Read Only?**
    /// eXtended Flow Guard is enabled for the process.
    /// XFG-aware code will validate function pointers point to actual functions, **of the correct type/signature,** before calling them.
    ///
    /// At the time of writing this doc comment (2022-09-21), this may be too new to enable:
    /// *   rustc 1.63.0 doesn't appear to have XFG support yet
    /// *   Windows 10.0.19043.2006 doesn't appear to have XFG support yet (causes ERROR_INVALID_PARAMETER if you attempt to enable it)
    /// *   Windows 11 or insider builds might have more luck?
    /// Additionally, if this works like [`enable_control_flow_guard`](Self::enable_control_flow_guard), this might be a read-only setting.
    /// Of course, you're free to experiment and try enabling this option anyways!
    ///
    /// ### See Also
    /// *   [`process::creation::mitigation_policy2::xtended_control_flow_guard::ALWAYS_ON`] (allows a parent process to enable this at launch time)
    /// *   [Exploit Development: Between a Rock and a (Xtended Flow) Guard Place: Examining XFG](https://connormcgarr.github.io/examining-xfg/) (github.io)
    /// *   [Control-flow integrity: Microsoft eXtended Flow Guard](https://en.wikipedia.org/wiki/Control-flow_integrity#Microsoft_eXtended_Flow_Guard) (en.wikipedia.org)
    ///
    pub enable_xfg: bool,

    /// **Disable for security:**  Weaken [`enable_xfg`](Self::enable_xfg) so it logs (to the Event Log?) instead of protecting.
    ///
    /// I'm not sure if this means logging DLLs that would otherwise be blocked from loading, or if it means logging function signature check failures.
    ///
    pub enable_xfg_audit_mode: bool,

    #[doc(hidden)] pub _reserved_flags: ()
}

unsafe impl GetPolicy for PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY {
    type Raw = Self;
    fn ty() -> process::mitigation::Policy { process::ControlFlowGuardPolicy }
    fn from_policy(p: Self::Raw) -> Self { p }
}

impl SetPolicy for PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY {
    fn into_policy(self) -> Self::Raw { self }
}

unsafe impl GetPolicy for ControlFlowGuardPolicy {
    type Raw = PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY;
    fn ty() -> process::mitigation::Policy { process::ControlFlowGuardPolicy }
    fn from_policy(p: Self::Raw) -> Self { p.into() }
}

impl SetPolicy for ControlFlowGuardPolicy {
    fn into_policy(self) -> Self::Raw { self.into() }
}

impl From<ControlFlowGuardPolicy> for PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY {
    fn from(i: ControlFlowGuardPolicy) -> Self {
        let mut o = Self::default();
        o.set_EnableControlFlowGuard    (i.enable_control_flow_guard    as u32);
        o.set_EnableExportSuppression   (i.enable_export_suppression    as u32);
        o.set_StrictMode                (i.strict_mode                  as u32);
        o.Flags |= (i.enable_xfg            as u32) << 3;
        o.Flags |= (i.enable_xfg_audit_mode as u32) << 4;
        o
    }
}

impl From<PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY> for ControlFlowGuardPolicy {
    fn from(i: PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY) -> Self {
        let mut o = Self::zeroed();
        o.enable_control_flow_guard = i.EnableControlFlowGuard()    != 0;
        o.enable_export_suppression = i.EnableExportSuppression()   != 0;
        o.strict_mode               = i.StrictMode()                != 0;
        o.enable_xfg                = i.Flags & (1 << 3) != 0;
        o.enable_xfg_audit_mode     = i.Flags & (1 << 4) != 0;
        o
    }
}
