use super::*;
use crate::*;
use bytemuck::*;
use winapi::um::winnt::*;



#[doc(alias = "PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-process_mitigation_binary_signature_policy)\]
/// ≈ PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY
///
/// Control who must sign binaries/images/dlls in order for them to be loadable.
/// This presumably can only apply to newly loaded DLLs - existing DLLs and the executable were already loaded.
///
/// ### References
/// *   [chromium/docs/design/sandbox.md: Disable Loading of Unsigned Code (CIG)](https://github.com/chromium/chromium/blob/main/docs/design/sandbox.md#disable-loading-of-unsigned-code-cig)
/// *   [Mitigating arbitrary native code execution in Microsoft Edge](https://blogs.windows.com/msedgedev/2017/02/23/mitigating-arbitrary-native-code-execution/)
///
#[derive(Clone, Copy, Debug)]
#[derive(Zeroable)]
pub struct BinarySignaturePolicy {
    /// **Consider enabling**: Prevents loading (additional?) non-Microsoft DLLs.
    ///
    /// Given that most pure-Rust code is typically *statically* linked, there's a good chance you can use this option even with 3rd party crates.
    /// Bindings around C or C++ APIs are more likely to dynamically link, and some "pure" Rust crates might have dynamic linking options for plugins or build times (e.g. [Bevy](https://bevyengine.org/learn/book/getting-started/setup/#enable-fast-compiles-optional)).
    /// In these cases, you may not be able to use this option.
    ///
    /// Other possible incompatabilities:
    /// *   Your own custom DLLs / Plugins
    /// *   Debuggers
    /// *   Profilers
    /// *   Fuzzers
    /// *   Security products
    /// *   Input Method Editors
    /// *   Binaries Microsoft forgot to sign
    /// *   Microsoft binaries that have been modified by third parties (hooking shenannigans?)
    /// *   Drop-in replacements for Microsoft binaries (e.g. 3rd party xinput DLLs?)
    /// *   Third party COM servers (ActiveX, OLE, ...?)
    /// *   Anti-cheat
    pub microsoft_signed_only:  bool,

    /// **Consider enabling**: Prevents loading (additional?) DLLs not signed by "The Windows Store".
    ///
    /// I'm not sure if that means signed for distribution through the Microsoft store, or if locally signed for development installation is OK.
    /// Appears to cause `ERROR_INVALID_PARAMETER` if used outside of an .appx / AppContainer, even in audit mode?
    /// I currently lack a proper app container test, so I'm not sure if this blocks locally signed .appx images or not.
    /// Might need to disable this for unsigned or locally signed local builds.
    pub store_signed_only:      bool,

    /// **Consider enabling**: Prevents loading (additional?) DLLs not signed by one of:
    /// *   Microsoft
    /// *   "The Windows Store"
    /// *   "Windows Hardware Quality Labs (WHQL)"
    ///
    /// Weaker than [`microsoft_signed_only`](Self::microsoft_signed_only) or [`store_signed_only`](Self::store_signed_only), and possibly redundant if either of those are set?
    pub mitigation_opt_in:      bool,

    /// **Disable for security:**  Weaken [`microsoft_signed_only`](Self::microsoft_signed_only) so it logs non-Microsoft DLLs that are loaded (to the Event Log?) instead of refusing to load them.
    pub audit_microsoft_signed_only: bool,

    /// **Disable for security:**  Weaken [`store_signed_only`](Self::store_signed_only) so it logs non-store DLLs that are loaded (to the Event Log?) instead of refusing to load them.
    pub audit_store_signed_only: bool,

    #[doc(hidden)] pub _reserved_flags: ()
}

unsafe impl GetPolicy for PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY {
    type Raw = Self;
    fn ty() -> process::mitigation::Policy { process::SignaturePolicy }
    fn from_policy(p: Self::Raw) -> Self { p }
}

impl SetPolicy for PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY {
    fn into_policy(self) -> Self::Raw { self }
}

unsafe impl GetPolicy for BinarySignaturePolicy {
    type Raw = PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY;
    fn ty() -> process::mitigation::Policy { process::SignaturePolicy }
    fn from_policy(p: Self::Raw) -> Self { p.into() }
}

impl SetPolicy for BinarySignaturePolicy {
    fn into_policy(self) -> Self::Raw { self.into() }
}

impl From<BinarySignaturePolicy> for PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY {
    fn from(i: BinarySignaturePolicy) -> Self {
        let mut o = Self::default();
        o.set_MicrosoftSignedOnly   (i.microsoft_signed_only    as u32);
        o.set_StoreSignedOnly       (i.store_signed_only        as u32);
        o.set_MitigationOptIn       (i.mitigation_opt_in        as u32);
        o.Flags |= (i.audit_microsoft_signed_only as u32) << 3;
        o.Flags |= (i.audit_store_signed_only     as u32) << 4;
        o
    }
}

impl From<PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY> for BinarySignaturePolicy {
    fn from(i: PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY) -> Self {
        let mut o = Self::zeroed();
        o.microsoft_signed_only = i.MicrosoftSignedOnly()   != 0;
        o.store_signed_only     = i.StoreSignedOnly()       != 0;
        o.mitigation_opt_in     = i.MitigationOptIn()       != 0;
        o.audit_microsoft_signed_only   = i.Flags & (1 << 3) != 0;
        o.audit_store_signed_only       = i.Flags & (1 << 4) != 0;
        o
    }
}
