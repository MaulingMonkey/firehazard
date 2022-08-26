//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-updateprocthreadattribute)\] UpdateProcThreadAttribute value constants for use with<br>
//! PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY

use crate::process::creation::MitigationPolicyFlags2;



/// Loader Integrity Continuity mitigation policy options.
/// This mitigation enforces OS signing levels for depenedent module loads.
///
/// Windows 10+
pub mod loader_integrity_continuity {
    use super::*;
    pub const MASK          : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000003u64 << 4);
    pub const DEFER         : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000000u64 << 4);
    pub const ALWAYS_ON     : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000001u64 << 4);
    pub const ALWAYS_OFF    : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000002u64 << 4);
    pub const AUDIT         : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000003u64 << 4);
}

/// Strict Control Flow Guard (CFG) mitigation policy options.
/// This mitigation requires all images that load in the process to be instrumented by CFG.
///
/// Windows 10+
pub mod strict_control_flow_guard {
    use super::*;
    pub const MASK          : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000003u64 << 8);
    pub const DEFER         : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000000u64 << 8);
    pub const ALWAYS_ON     : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000001u64 << 8);
    pub const ALWAYS_OFF    : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000002u64 << 8);
    pub const RESERVED      : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000003u64 << 8);
}

/// Module tampering mitigation policy options.
///
/// Windows 10+
pub mod module_tampering_protection {
    use super::*;
    pub const MASK          : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000003u64 << 12);
    pub const DEFER         : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000000u64 << 12);
    pub const ALWAYS_ON     : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000001u64 << 12);
    pub const ALWAYS_OFF    : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000002u64 << 12);
    pub const NOINHERIT     : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000003u64 << 12);
}


/// Restricted indirect branch prediction mitigation policy options.
///
/// Windows 10+
pub mod restrict_indirect_branch_prediction {
    use super::*;
    pub const MASK          : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000003u64 << 16);
    pub const DEFER         : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000000u64 << 16);
    pub const ALWAYS_ON     : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000001u64 << 16);
    pub const ALWAYS_OFF    : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000002u64 << 16);
    pub const RESERVED      : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000003u64 << 16);
}

/// Allow a broker to downgrade the dynamic code policy for a process.
///
/// Windows 10+
pub mod allow_downgrade_dynamic_code_policy {
    use super::*;
    pub const MASK          : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000003u64 << 20);
    pub const DEFER         : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000000u64 << 20);
    pub const ALWAYS_ON     : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000001u64 << 20);
    pub const ALWAYS_OFF    : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000002u64 << 20);
    pub const RESERVED      : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000003u64 << 20);
}

/// Memory Disambiguation mitigation
///
/// Windows 10+
pub mod speculative_store_bypass_disable {
    use super::*;
    pub const MASK          : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000003u64 << 24);
    pub const DEFER         : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000000u64 << 24);
    pub const ALWAYS_ON     : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000001u64 << 24);
    pub const ALWAYS_OFF    : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000002u64 << 24);
    pub const RESERVED      : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000003u64 << 24);
}

/// User-mode shadow stack mitigation
///
/// Windows 10+
pub mod cet_user_shadow_stacks {
    use super::*;
    pub const MASK          : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000003u64 << 28);
    pub const DEFER         : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000000u64 << 28);
    pub const ALWAYS_ON     : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000001u64 << 28);
    pub const ALWAYS_OFF    : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000002u64 << 28);
    pub const STRICT_MODE   : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000003u64 << 28);
}

/// User-mode CET set context instruction pointer validation mitigation policy options.
pub mod user_cet_set_context_ip_validation {
    use super::*;
    pub const MASK          : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000003u64 << 32);
    pub const DEFER         : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000000u64 << 32);
    pub const ALWAYS_ON     : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000001u64 << 32);
    pub const ALWAYS_OFF    : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000002u64 << 32);
    pub const RELAXED_MODE  : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000003u64 << 32);
}

/// Block non-CET/non-EHCONT binaries mitigation policy options.
pub mod block_non_cet_binaries {
    use super::*;
    pub const MASK          : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000003u64 << 36);
    pub const DEFER         : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000000u64 << 36);
    pub const ALWAYS_ON     : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000001u64 << 36);
    pub const ALWAYS_OFF    : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000002u64 << 36);
    pub const NON_EHCONT    : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000003u64 << 36);
}

/// XFG mitigation policy options.
pub mod xtended_control_flow_guard {
    use super::*;
    pub const MASK          : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000003u64 << 40);
    pub const DEFER         : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000000u64 << 40);
    pub const ALWAYS_ON     : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000001u64 << 40);
    pub const ALWAYS_OFF    : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000002u64 << 40);
    pub const RESERVED      : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000003u64 << 40);
}

/// ARM64 user-mode per-process instruction pointer authentication mitigation policy options.
pub mod pointer_auth_user_ip {
    use super::*;
    pub const MASK          : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000003u64 << 44);
    pub const DEFER         : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000000u64 << 44);
    pub const ALWAYS_ON     : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000001u64 << 44);
    pub const ALWAYS_OFF    : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000002u64 << 44);
    pub const RESERVED      : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000003u64 << 44);
}

/// CET-related dynamic code validation data APIs out-of-proc mitigation policy options.
pub mod cet_dynamic_apis_out_of_proc_only {
    use super::*;
    pub const MASK          : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000003u64 << 48);
    pub const DEFER         : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000000u64 << 48);
    pub const ALWAYS_ON     : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000001u64 << 48);
    pub const ALWAYS_OFF    : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000002u64 << 48);
    pub const RESERVED      : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000003u64 << 48);
}

/// Restrict core sharing policy options.
pub mod restrict_core_sharing {
    use super::*;
    pub const MASK          : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000003u64 << 52);
    pub const DEFER         : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000000u64 << 52);
    pub const ALWAYS_ON     : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000001u64 << 52);
    pub const ALWAYS_OFF    : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000002u64 << 52);
    pub const RESERVED      : MitigationPolicyFlags2 = MitigationPolicyFlags2(0x00000003u64 << 52);
}
