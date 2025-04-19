use super::*;
use crate::*;
use bytemuck::*;
use winapi::um::winnt::*;



#[doc(alias = "PROCESS_MITIGATION_ASLR_POLICY")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-process_mitigation_aslr_policy)\]
/// ≈ PROCESS_MITIGATION_ASLR_POLICY
///
/// Enable [Address Space Layout Randomization](https://en.wikipedia.org/wiki/Address_space_layout_randomization).
/// This presumably can only apply to newly loaded DLLs - existing DLLs and the executable were already loaded.
/// The good news is that this is mostly redundant, as [`/DYNAMICBASE`] and [`/HIGHENTROPYVA`] are enabled by default on modern linkers where applicable.
/// But, on the off chance you delay-load older and more legacy DLLs, this might help a little.
///
/// [`/DYNAMICBASE`]:   https://learn.microsoft.com/en-us/cpp/build/reference/dynamicbase-use-address-space-layout-randomization
/// [`/HIGHENTROPYVA`]: https://learn.microsoft.com/en-us/cpp/build/reference/highentropyva-support-64-bit-aslr
///
#[derive(Clone, Copy, Debug)]
#[derive(Zeroable)]
pub struct AslrPolicy {
    /// **Enable Always:**
    /// Randomizes the base "bottom" address used by VirtualAlloc etc. by default (e.g. when not combined with [`MEM_TOP_DOWN`].)
    /// Noop for 32-bit code, so limited virtual address space shouldn't be a concern.
    /// For 64-bit code, implied by `link.exe`'s [`/DYNAMICBASE`], which is enabled by default.
    ///
    /// This may also (vastly?) improve the entropy of forced regular pseudo-ASLR on some systems?
    /// *   <https://blog.didierstevens.com/2011/09/01/bottom-up-randomization-saves-mandatory-aslr/>
    /// *   <https://blog.didierstevens.com/2011/08/16/so-how-good-is-pseudo-aslr/>
    ///
    /// [`/DYNAMICBASE`]:   https://learn.microsoft.com/en-us/cpp/build/reference/dynamicbase-use-address-space-layout-randomization
    ///
    pub enable_bottom_up_randomization:     bool,

    /// **Enable Always:**
    /// Force relocation of "legacy" images not built with [`/DYNAMICBASE`].
    /// Since `link.exe` enables [`/DYNAMICBASE`] by default, this probably doesn't do much.
    ///
    /// This might break some badwrong DLLs that really really wanted to load at a specific address.
    ///
    /// Such DLLs were already broken, and should be fixed instead of disabling forced relocation:
    /// *   Another DLL or the EXE might've loaded with a conflicting address already
    /// *   VirtualAlloc could've allocated at the DLL's preferred address.
    ///
    /// It's also possible for there to be a performance hit when loading DLLs, and reduced memory sharing from pointer fixups modifying DLL code.
    /// To fix such performance issues, consider rebuilding said DLLs with [`/DYNAMICBASE`].
    ///
    /// [`/DYNAMICBASE`]:   https://learn.microsoft.com/en-us/cpp/build/reference/dynamicbase-use-address-space-layout-randomization
    ///
    pub enable_force_relocate_images:       bool,

    /// **Enable Always:**
    /// Increase ASLR entropy.
    /// Implied by `link.exe`'s [`/HIGHENTROPYVA`], which is enabled by default.
    ///
    /// This might break some badwrong XP-era code that made stupid assumptions about VirtualAlloc giving you contiguous memory blocks when called repeatedly in quick succession.
    ///
    /// Such code was already broken, and should be fixed instead of disabling high entropy:
    /// *   Memory fragmentation might mean the next free block isn't contiguous
    /// *   Allocations on another thread might steal the page
    /// *   This high entropy option might be enabled.
    ///
    /// [`/HIGHENTROPYVA`]: https://learn.microsoft.com/en-us/cpp/build/reference/highentropyva-support-64-bit-aslr
    ///
    pub enable_high_entropy:                bool,

    /// **Enable Always:**
    /// Blocks loading of DLLs that not only don't have a dynamic base, but had their relocation information stripped too.
    ///
    /// Such DLLs were already broken, and should be fixed instead of weaking this prohibition:
    /// *   Another DLL or the EXE might've loaded with a conflicting address already
    /// *   VirtualAlloc could've allocated at the DLL's preferred address.
    ///
    pub disallow_stripped_images:           bool,

    #[doc(hidden)] pub _reserved_flags: ()
}

impl AslrPolicy {
    /// Configure the strictest options at this time.
    ///
    /// If new safety-enhancing fields are added, they will *not* be enabled - instead, a new fn (e.g. `strict_v2()`) will be introduced,
    /// and this one will be marked as deprecated (and possibly `#[cfg(...)]`ed out with the right `--cfg` flag for auditing purpouses.)
    ///
    pub fn strict_v1() -> Self {
        Self {
            enable_bottom_up_randomization: true,
            enable_force_relocate_images:   true,
            enable_high_entropy:            true,
            disallow_stripped_images:       true,
            _reserved_flags:                ()
        }
    }
}

unsafe impl GetPolicy for PROCESS_MITIGATION_ASLR_POLICY {
    type Raw = Self;
    fn ty() -> process::mitigation::Policy { process::ASLRPolicy }
    fn from_policy(p: Self::Raw) -> Self { p }
}

impl SetPolicy for PROCESS_MITIGATION_ASLR_POLICY {
    fn into_policy(self) -> Self::Raw { self }
}

unsafe impl GetPolicy for AslrPolicy {
    type Raw = PROCESS_MITIGATION_ASLR_POLICY;
    fn ty() -> process::mitigation::Policy { process::ASLRPolicy }
    fn from_policy(p: Self::Raw) -> Self { p.into() }
}

impl SetPolicy for AslrPolicy {
    fn into_policy(self) -> Self::Raw { self.into() }
}

impl From<AslrPolicy> for PROCESS_MITIGATION_ASLR_POLICY {
    fn from(i: AslrPolicy) -> Self {
        let mut o = Self::default();
        o.set_EnableBottomUpRandomization   (i.enable_bottom_up_randomization   as u32);
        o.set_EnableForceRelocateImages     (i.enable_force_relocate_images     as u32);
        o.set_EnableHighEntropy             (i.enable_high_entropy              as u32);
        o.set_DisallowStrippedImages        (i.disallow_stripped_images         as u32);
        o
    }
}

impl From<PROCESS_MITIGATION_ASLR_POLICY> for AslrPolicy {
    fn from(i: PROCESS_MITIGATION_ASLR_POLICY) -> Self {
        let mut o = Self::zeroed();
        o.enable_bottom_up_randomization    = i.EnableBottomUpRandomization()   != 0;
        o.enable_force_relocate_images      = i.EnableForceRelocateImages()     != 0;
        o.enable_high_entropy               = i.EnableHighEntropy()             != 0;
        o.disallow_stripped_images          = i.DisallowStrippedImages()        != 0;
        o
    }
}
