//#[doc(alias = "__fastfail")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/cpp/intrinsics/fastfail)\]
/// __fastfail
///
pub fn __fastfail(code: impl Into<FAST_FAIL>) -> ! {
    let code = code.into().0;
    unsafe {
        #[cfg(target_arch = "x86_64")] {
            core::arch::asm!("int 29h", in("rax") code);
            core::hint::unreachable_unchecked()
        }
        #[cfg(target_arch = "x86")] {
            core::arch::asm!("int 29h", in("ecx") code);
            core::hint::unreachable_unchecked()
        }
        #[cfg(target_arch = "arm")] {
            core::arch::asm!(".2byte 0xDEFB", in("r0") code);
            core::hint::unreachable_unchecked()
        }
        #[cfg(target_arch = "aarch64")] {
            core::arch::asm!(".2byte 0xF003", in("x0") code);
            core::hint::unreachable_unchecked()
        }
        // I've intentionally avoided refactoring out the common `core::hint::unreachable_unchecked()` call:
        // unsupported architectures will trigger a desired error here by reaching the end of a non-returning function
    }
}



/// \[[microsoft.com](https://learn.microsoft.com/en-us/cpp/intrinsics/fastfail)\]
/// FAST\_FAIL\_\* constants for \_\_fastfail(code)
///
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, bytemuck::Pod, Default, bytemuck::Zeroable, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct FAST_FAIL(pub u32);

impl From<u32> for FAST_FAIL { fn from(v: u32) -> Self { Self(v) } }
impl From<FAST_FAIL> for u32 { fn from(v: FAST_FAIL) -> Self { v.0 } }

impl FAST_FAIL {
    // C:\Program Files (x86)\Windows Kits\10\Include\10.0.22621.0\um\winnt.h

    // Some of these are marked "Telemetry, nonfatal", which is at odds with `__fastfail`'s noreturn marker.
    #[doc(alias = "FAST_FAIL_LEGACY_GS_VIOLATION"               )] #[doc = "FAST_FAIL_LEGACY_GS_VIOLATION"              ] pub const LEGACY_GS_VIOLATION                 : Self = Self(  0);
    #[doc(alias = "FAST_FAIL_VTGUARD_CHECK_FAILURE"             )] #[doc = "FAST_FAIL_VTGUARD_CHECK_FAILURE"            ] pub const VTGUARD_CHECK_FAILURE               : Self = Self(  1);
    #[doc(alias = "FAST_FAIL_STACK_COOKIE_CHECK_FAILURE"        )] #[doc = "FAST_FAIL_STACK_COOKIE_CHECK_FAILURE"       ] pub const STACK_COOKIE_CHECK_FAILURE          : Self = Self(  2);
    #[doc(alias = "FAST_FAIL_CORRUPT_LIST_ENTRY"                )] #[doc = "FAST_FAIL_CORRUPT_LIST_ENTRY"               ] pub const CORRUPT_LIST_ENTRY                  : Self = Self(  3);
    #[doc(alias = "FAST_FAIL_INCORRECT_STACK"                   )] #[doc = "FAST_FAIL_INCORRECT_STACK"                  ] pub const INCORRECT_STACK                     : Self = Self(  4);
    #[doc(alias = "FAST_FAIL_INVALID_ARG"                       )] #[doc = "FAST_FAIL_INVALID_ARG"                      ] pub const INVALID_ARG                         : Self = Self(  5);
    #[doc(alias = "FAST_FAIL_GS_COOKIE_INIT"                    )] #[doc = "FAST_FAIL_GS_COOKIE_INIT"                   ] pub const GS_COOKIE_INIT                      : Self = Self(  6);
    #[doc(alias = "FAST_FAIL_FATAL_APP_EXIT"                    )] #[doc = "FAST_FAIL_FATAL_APP_EXIT"                   ] pub const FATAL_APP_EXIT                      : Self = Self(  7);
    #[doc(alias = "FAST_FAIL_RANGE_CHECK_FAILURE"               )] #[doc = "FAST_FAIL_RANGE_CHECK_FAILURE"              ] pub const RANGE_CHECK_FAILURE                 : Self = Self(  8);
    #[doc(alias = "FAST_FAIL_UNSAFE_REGISTRY_ACCESS"            )] #[doc = "FAST_FAIL_UNSAFE_REGISTRY_ACCESS"           ] pub const UNSAFE_REGISTRY_ACCESS              : Self = Self(  9);
    #[doc(alias = "FAST_FAIL_GUARD_ICALL_CHECK_FAILURE"         )] #[doc = "FAST_FAIL_GUARD_ICALL_CHECK_FAILURE"        ] pub const GUARD_ICALL_CHECK_FAILURE           : Self = Self( 10);
    #[doc(alias = "FAST_FAIL_GUARD_WRITE_CHECK_FAILURE"         )] #[doc = "FAST_FAIL_GUARD_WRITE_CHECK_FAILURE"        ] pub const GUARD_WRITE_CHECK_FAILURE           : Self = Self( 11);
    #[doc(alias = "FAST_FAIL_INVALID_FIBER_SWITCH"              )] #[doc = "FAST_FAIL_INVALID_FIBER_SWITCH"             ] pub const INVALID_FIBER_SWITCH                : Self = Self( 12);
    #[doc(alias = "FAST_FAIL_INVALID_SET_OF_CONTEXT"            )] #[doc = "FAST_FAIL_INVALID_SET_OF_CONTEXT"           ] pub const INVALID_SET_OF_CONTEXT              : Self = Self( 13);
    #[doc(alias = "FAST_FAIL_INVALID_REFERENCE_COUNT"           )] #[doc = "FAST_FAIL_INVALID_REFERENCE_COUNT"          ] pub const INVALID_REFERENCE_COUNT             : Self = Self( 14);
    #[doc(alias = "FAST_FAIL_INVALID_JUMP_BUFFER"               )] #[doc = "FAST_FAIL_INVALID_JUMP_BUFFER"              ] pub const INVALID_JUMP_BUFFER                 : Self = Self( 18);
    #[doc(alias = "FAST_FAIL_MRDATA_MODIFIED"                   )] #[doc = "FAST_FAIL_MRDATA_MODIFIED"                  ] pub const MRDATA_MODIFIED                     : Self = Self( 19);
    #[doc(alias = "FAST_FAIL_CERTIFICATION_FAILURE"             )] #[doc = "FAST_FAIL_CERTIFICATION_FAILURE"            ] pub const CERTIFICATION_FAILURE               : Self = Self( 20);
    #[doc(alias = "FAST_FAIL_INVALID_EXCEPTION_CHAIN"           )] #[doc = "FAST_FAIL_INVALID_EXCEPTION_CHAIN"          ] pub const INVALID_EXCEPTION_CHAIN             : Self = Self( 21);
    #[doc(alias = "FAST_FAIL_CRYPTO_LIBRARY"                    )] #[doc = "FAST_FAIL_CRYPTO_LIBRARY"                   ] pub const CRYPTO_LIBRARY                      : Self = Self( 22);
    #[doc(alias = "FAST_FAIL_INVALID_CALL_IN_DLL_CALLOUT"       )] #[doc = "FAST_FAIL_INVALID_CALL_IN_DLL_CALLOUT"      ] pub const INVALID_CALL_IN_DLL_CALLOUT         : Self = Self( 23);
    #[doc(alias = "FAST_FAIL_INVALID_IMAGE_BASE"                )] #[doc = "FAST_FAIL_INVALID_IMAGE_BASE"               ] pub const INVALID_IMAGE_BASE                  : Self = Self( 24);
    #[doc(alias = "FAST_FAIL_DLOAD_PROTECTION_FAILURE"          )] #[doc = "FAST_FAIL_DLOAD_PROTECTION_FAILURE"         ] pub const DLOAD_PROTECTION_FAILURE            : Self = Self( 25);
    #[doc(alias = "FAST_FAIL_UNSAFE_EXTENSION_CALL"             )] #[doc = "FAST_FAIL_UNSAFE_EXTENSION_CALL"            ] pub const UNSAFE_EXTENSION_CALL               : Self = Self( 26);
    #[doc(alias = "FAST_FAIL_DEPRECATED_SERVICE_INVOKED"        )] #[doc = "FAST_FAIL_DEPRECATED_SERVICE_INVOKED"       ] pub const DEPRECATED_SERVICE_INVOKED          : Self = Self( 27);
    #[doc(alias = "FAST_FAIL_INVALID_BUFFER_ACCESS"             )] #[doc = "FAST_FAIL_INVALID_BUFFER_ACCESS"            ] pub const INVALID_BUFFER_ACCESS               : Self = Self( 28);
    #[doc(alias = "FAST_FAIL_INVALID_BALANCED_TREE"             )] #[doc = "FAST_FAIL_INVALID_BALANCED_TREE"            ] pub const INVALID_BALANCED_TREE               : Self = Self( 29);
    #[doc(alias = "FAST_FAIL_INVALID_NEXT_THREAD"               )] #[doc = "FAST_FAIL_INVALID_NEXT_THREAD"              ] pub const INVALID_NEXT_THREAD                 : Self = Self( 30);
    #[doc(alias = "FAST_FAIL_GUARD_ICALL_CHECK_SUPPRESSED"      )] #[doc = "FAST_FAIL_GUARD_ICALL_CHECK_SUPPRESSED"     ] pub const GUARD_ICALL_CHECK_SUPPRESSED        : Self = Self( 31);
    #[doc(alias = "FAST_FAIL_APCS_DISABLED"                     )] #[doc = "FAST_FAIL_APCS_DISABLED"                    ] pub const APCS_DISABLED                       : Self = Self( 32);
    #[doc(alias = "FAST_FAIL_INVALID_IDLE_STATE"                )] #[doc = "FAST_FAIL_INVALID_IDLE_STATE"               ] pub const INVALID_IDLE_STATE                  : Self = Self( 33);
    #[doc(alias = "FAST_FAIL_MRDATA_PROTECTION_FAILURE"         )] #[doc = "FAST_FAIL_MRDATA_PROTECTION_FAILURE"        ] pub const MRDATA_PROTECTION_FAILURE           : Self = Self( 34);
    #[doc(alias = "FAST_FAIL_UNEXPECTED_HEAP_EXCEPTION"         )] #[doc = "FAST_FAIL_UNEXPECTED_HEAP_EXCEPTION"        ] pub const UNEXPECTED_HEAP_EXCEPTION           : Self = Self( 35);
    #[doc(alias = "FAST_FAIL_INVALID_LOCK_STATE"                )] #[doc = "FAST_FAIL_INVALID_LOCK_STATE"               ] pub const INVALID_LOCK_STATE                  : Self = Self( 36);
    #[doc(alias = "FAST_FAIL_GUARD_JUMPTABLE"                   )] #[doc = "FAST_FAIL_GUARD_JUMPTABLE"                  ] pub const GUARD_JUMPTABLE                     : Self = Self( 37);
    #[doc(alias = "FAST_FAIL_INVALID_LONGJUMP_TARGET"           )] #[doc = "FAST_FAIL_INVALID_LONGJUMP_TARGET"          ] pub const INVALID_LONGJUMP_TARGET             : Self = Self( 38);
    #[doc(alias = "FAST_FAIL_INVALID_DISPATCH_CONTEXT"          )] #[doc = "FAST_FAIL_INVALID_DISPATCH_CONTEXT"         ] pub const INVALID_DISPATCH_CONTEXT            : Self = Self( 39);
    #[doc(alias = "FAST_FAIL_INVALID_THREAD"                    )] #[doc = "FAST_FAIL_INVALID_THREAD"                   ] pub const INVALID_THREAD                      : Self = Self( 40);
    #[doc(alias = "FAST_FAIL_INVALID_SYSCALL_NUMBER"            )] #[doc = "FAST_FAIL_INVALID_SYSCALL_NUMBER"           ] pub const INVALID_SYSCALL_NUMBER              : Self = Self( 41);
    #[doc(alias = "FAST_FAIL_INVALID_FILE_OPERATION"            )] #[doc = "FAST_FAIL_INVALID_FILE_OPERATION"           ] pub const INVALID_FILE_OPERATION              : Self = Self( 42);
    #[doc(alias = "FAST_FAIL_LPAC_ACCESS_DENIED"                )] #[doc = "FAST_FAIL_LPAC_ACCESS_DENIED"               ] pub const LPAC_ACCESS_DENIED                  : Self = Self( 43);
    #[doc(alias = "FAST_FAIL_GUARD_SS_FAILURE"                  )] #[doc = "FAST_FAIL_GUARD_SS_FAILURE"                 ] pub const GUARD_SS_FAILURE                    : Self = Self( 44);
    #[doc(alias = "FAST_FAIL_LOADER_CONTINUITY_FAILURE"         )] #[doc = "FAST_FAIL_LOADER_CONTINUITY_FAILURE"        ] pub const LOADER_CONTINUITY_FAILURE           : Self = Self( 45);
    #[doc(alias = "FAST_FAIL_GUARD_EXPORT_SUPPRESSION_FAILURE"  )] #[doc = "FAST_FAIL_GUARD_EXPORT_SUPPRESSION_FAILURE" ] pub const GUARD_EXPORT_SUPPRESSION_FAILURE    : Self = Self( 46);
    #[doc(alias = "FAST_FAIL_INVALID_CONTROL_STACK"             )] #[doc = "FAST_FAIL_INVALID_CONTROL_STACK"            ] pub const INVALID_CONTROL_STACK               : Self = Self( 47);
    #[doc(alias = "FAST_FAIL_SET_CONTEXT_DENIED"                )] #[doc = "FAST_FAIL_SET_CONTEXT_DENIED"               ] pub const SET_CONTEXT_DENIED                  : Self = Self( 48);
    #[doc(alias = "FAST_FAIL_INVALID_IAT"                       )] #[doc = "FAST_FAIL_INVALID_IAT"                      ] pub const INVALID_IAT                         : Self = Self( 49);
    #[doc(alias = "FAST_FAIL_HEAP_METADATA_CORRUPTION"          )] #[doc = "FAST_FAIL_HEAP_METADATA_CORRUPTION"         ] pub const HEAP_METADATA_CORRUPTION            : Self = Self( 50);
    #[doc(alias = "FAST_FAIL_PAYLOAD_RESTRICTION_VIOLATION"     )] #[doc = "FAST_FAIL_PAYLOAD_RESTRICTION_VIOLATION"    ] pub const PAYLOAD_RESTRICTION_VIOLATION       : Self = Self( 51);
    #[doc(alias = "FAST_FAIL_LOW_LABEL_ACCESS_DENIED"           )] #[doc = "FAST_FAIL_LOW_LABEL_ACCESS_DENIED"          ] pub const LOW_LABEL_ACCESS_DENIED             : Self = Self( 52);
    #[doc(alias = "FAST_FAIL_ENCLAVE_CALL_FAILURE"              )] #[doc = "FAST_FAIL_ENCLAVE_CALL_FAILURE"             ] pub const ENCLAVE_CALL_FAILURE                : Self = Self( 53);
    #[doc(alias = "FAST_FAIL_UNHANDLED_LSS_EXCEPTON"            )] #[doc = "FAST_FAIL_UNHANDLED_LSS_EXCEPTON"           ] pub const UNHANDLED_LSS_EXCEPTON              : Self = Self( 54);
    #[doc(alias = "FAST_FAIL_ADMINLESS_ACCESS_DENIED"           )] #[doc = "FAST_FAIL_ADMINLESS_ACCESS_DENIED"          ] pub const ADMINLESS_ACCESS_DENIED             : Self = Self( 55);
    #[doc(alias = "FAST_FAIL_UNEXPECTED_CALL"                   )] #[doc = "FAST_FAIL_UNEXPECTED_CALL"                  ] pub const UNEXPECTED_CALL                     : Self = Self( 56);
    #[doc(alias = "FAST_FAIL_CONTROL_INVALID_RETURN_ADDRESS"    )] #[doc = "FAST_FAIL_CONTROL_INVALID_RETURN_ADDRESS"   ] pub const CONTROL_INVALID_RETURN_ADDRESS      : Self = Self( 57);
    #[doc(alias = "FAST_FAIL_UNEXPECTED_HOST_BEHAVIOR"          )] #[doc = "FAST_FAIL_UNEXPECTED_HOST_BEHAVIOR"         ] pub const UNEXPECTED_HOST_BEHAVIOR            : Self = Self( 58);
    #[doc(alias = "FAST_FAIL_FLAGS_CORRUPTION"                  )] #[doc = "FAST_FAIL_FLAGS_CORRUPTION"                 ] pub const FLAGS_CORRUPTION                    : Self = Self( 59);
    #[doc(alias = "FAST_FAIL_VEH_CORRUPTION"                    )] #[doc = "FAST_FAIL_VEH_CORRUPTION"                   ] pub const VEH_CORRUPTION                      : Self = Self( 60);
    #[doc(alias = "FAST_FAIL_ETW_CORRUPTION"                    )] #[doc = "FAST_FAIL_ETW_CORRUPTION"                   ] pub const ETW_CORRUPTION                      : Self = Self( 61);
    #[doc(alias = "FAST_FAIL_RIO_ABORT"                         )] #[doc = "FAST_FAIL_RIO_ABORT"                        ] pub const RIO_ABORT                           : Self = Self( 62);
    #[doc(alias = "FAST_FAIL_INVALID_PFN"                       )] #[doc = "FAST_FAIL_INVALID_PFN"                      ] pub const INVALID_PFN                         : Self = Self( 63);
    #[doc(alias = "FAST_FAIL_GUARD_ICALL_CHECK_FAILURE_XFG"     )] #[doc = "FAST_FAIL_GUARD_ICALL_CHECK_FAILURE_XFG"    ] pub const GUARD_ICALL_CHECK_FAILURE_XFG       : Self = Self( 64);
    #[doc(alias = "FAST_FAIL_CAST_GUARD"                        )] #[doc = "FAST_FAIL_CAST_GUARD"                       ] pub const CAST_GUARD                          : Self = Self( 65);
    #[doc(alias = "FAST_FAIL_HOST_VISIBILITY_CHANGE"            )] #[doc = "FAST_FAIL_HOST_VISIBILITY_CHANGE"           ] pub const HOST_VISIBILITY_CHANGE              : Self = Self( 66);
    #[doc(alias = "FAST_FAIL_KERNEL_CET_SHADOW_STACK_ASSIST"    )] #[doc = "FAST_FAIL_KERNEL_CET_SHADOW_STACK_ASSIST"   ] pub const KERNEL_CET_SHADOW_STACK_ASSIST      : Self = Self( 67);
    #[doc(alias = "FAST_FAIL_PATCH_CALLBACK_FAILED"             )] #[doc = "FAST_FAIL_PATCH_CALLBACK_FAILED"            ] pub const PATCH_CALLBACK_FAILED               : Self = Self( 68);
    #[doc(alias = "FAST_FAIL_NTDLL_PATCH_FAILED"                )] #[doc = "FAST_FAIL_NTDLL_PATCH_FAILED"               ] pub const NTDLL_PATCH_FAILED                  : Self = Self( 69);
    #[doc(alias = "FAST_FAIL_INVALID_FLS_DATA"                  )] #[doc = "FAST_FAIL_INVALID_FLS_DATA"                 ] pub const INVALID_FLS_DATA                    : Self = Self( 70);
    #[doc(alias = "FAST_FAIL_INVALID_FAST_FAIL_CODE"            )] #[doc = "FAST_FAIL_INVALID_FAST_FAIL_CODE"           ] pub const INVALID_FAST_FAIL_CODE              : Self = Self(0xFFFFFFFF);
}

impl core::fmt::Debug for FAST_FAIL {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        let s = match *self {
            FAST_FAIL::LEGACY_GS_VIOLATION              => "LEGACY_GS_VIOLATION",
            FAST_FAIL::VTGUARD_CHECK_FAILURE            => "VTGUARD_CHECK_FAILURE",
            FAST_FAIL::STACK_COOKIE_CHECK_FAILURE       => "STACK_COOKIE_CHECK_FAILURE",
            FAST_FAIL::CORRUPT_LIST_ENTRY               => "CORRUPT_LIST_ENTRY",
            FAST_FAIL::INCORRECT_STACK                  => "INCORRECT_STACK",
            FAST_FAIL::INVALID_ARG                      => "INVALID_ARG",
            FAST_FAIL::GS_COOKIE_INIT                   => "GS_COOKIE_INIT",
            FAST_FAIL::FATAL_APP_EXIT                   => "FATAL_APP_EXIT",
            FAST_FAIL::RANGE_CHECK_FAILURE              => "RANGE_CHECK_FAILURE",
            FAST_FAIL::UNSAFE_REGISTRY_ACCESS           => "UNSAFE_REGISTRY_ACCESS",
            FAST_FAIL::GUARD_ICALL_CHECK_FAILURE        => "GUARD_ICALL_CHECK_FAILURE",
            FAST_FAIL::GUARD_WRITE_CHECK_FAILURE        => "GUARD_WRITE_CHECK_FAILURE",
            FAST_FAIL::INVALID_FIBER_SWITCH             => "INVALID_FIBER_SWITCH",
            FAST_FAIL::INVALID_SET_OF_CONTEXT           => "INVALID_SET_OF_CONTEXT",
            FAST_FAIL::INVALID_REFERENCE_COUNT          => "INVALID_REFERENCE_COUNT",
            FAST_FAIL::INVALID_JUMP_BUFFER              => "INVALID_JUMP_BUFFER",
            FAST_FAIL::MRDATA_MODIFIED                  => "MRDATA_MODIFIED",
            FAST_FAIL::CERTIFICATION_FAILURE            => "CERTIFICATION_FAILURE",
            FAST_FAIL::INVALID_EXCEPTION_CHAIN          => "INVALID_EXCEPTION_CHAIN",
            FAST_FAIL::CRYPTO_LIBRARY                   => "CRYPTO_LIBRARY",
            FAST_FAIL::INVALID_CALL_IN_DLL_CALLOUT      => "INVALID_CALL_IN_DLL_CALLOUT",
            FAST_FAIL::INVALID_IMAGE_BASE               => "INVALID_IMAGE_BASE",
            FAST_FAIL::DLOAD_PROTECTION_FAILURE         => "DLOAD_PROTECTION_FAILURE",
            FAST_FAIL::UNSAFE_EXTENSION_CALL            => "UNSAFE_EXTENSION_CALL",
            FAST_FAIL::DEPRECATED_SERVICE_INVOKED       => "DEPRECATED_SERVICE_INVOKED",
            FAST_FAIL::INVALID_BUFFER_ACCESS            => "INVALID_BUFFER_ACCESS",
            FAST_FAIL::INVALID_BALANCED_TREE            => "INVALID_BALANCED_TREE",
            FAST_FAIL::INVALID_NEXT_THREAD              => "INVALID_NEXT_THREAD",
            FAST_FAIL::GUARD_ICALL_CHECK_SUPPRESSED     => "GUARD_ICALL_CHECK_SUPPRESSED",
            FAST_FAIL::APCS_DISABLED                    => "APCS_DISABLED",
            FAST_FAIL::INVALID_IDLE_STATE               => "INVALID_IDLE_STATE",
            FAST_FAIL::MRDATA_PROTECTION_FAILURE        => "MRDATA_PROTECTION_FAILURE",
            FAST_FAIL::UNEXPECTED_HEAP_EXCEPTION        => "UNEXPECTED_HEAP_EXCEPTION",
            FAST_FAIL::INVALID_LOCK_STATE               => "INVALID_LOCK_STATE",
            FAST_FAIL::GUARD_JUMPTABLE                  => "GUARD_JUMPTABLE",
            FAST_FAIL::INVALID_LONGJUMP_TARGET          => "INVALID_LONGJUMP_TARGET",
            FAST_FAIL::INVALID_DISPATCH_CONTEXT         => "INVALID_DISPATCH_CONTEXT",
            FAST_FAIL::INVALID_THREAD                   => "INVALID_THREAD",
            FAST_FAIL::INVALID_SYSCALL_NUMBER           => "INVALID_SYSCALL_NUMBER",
            FAST_FAIL::INVALID_FILE_OPERATION           => "INVALID_FILE_OPERATION",
            FAST_FAIL::LPAC_ACCESS_DENIED               => "LPAC_ACCESS_DENIED",
            FAST_FAIL::GUARD_SS_FAILURE                 => "GUARD_SS_FAILURE",
            FAST_FAIL::LOADER_CONTINUITY_FAILURE        => "LOADER_CONTINUITY_FAILURE",
            FAST_FAIL::GUARD_EXPORT_SUPPRESSION_FAILURE => "GUARD_EXPORT_SUPPRESSION_FAILURE",
            FAST_FAIL::INVALID_CONTROL_STACK            => "INVALID_CONTROL_STACK",
            FAST_FAIL::SET_CONTEXT_DENIED               => "SET_CONTEXT_DENIED",
            FAST_FAIL::INVALID_IAT                      => "INVALID_IAT",
            FAST_FAIL::HEAP_METADATA_CORRUPTION         => "HEAP_METADATA_CORRUPTION",
            FAST_FAIL::PAYLOAD_RESTRICTION_VIOLATION    => "PAYLOAD_RESTRICTION_VIOLATION",
            FAST_FAIL::LOW_LABEL_ACCESS_DENIED          => "LOW_LABEL_ACCESS_DENIED",
            FAST_FAIL::ENCLAVE_CALL_FAILURE             => "ENCLAVE_CALL_FAILURE",
            FAST_FAIL::UNHANDLED_LSS_EXCEPTON           => "UNHANDLED_LSS_EXCEPTON",
            FAST_FAIL::ADMINLESS_ACCESS_DENIED          => "ADMINLESS_ACCESS_DENIED",
            FAST_FAIL::UNEXPECTED_CALL                  => "UNEXPECTED_CALL",
            FAST_FAIL::CONTROL_INVALID_RETURN_ADDRESS   => "CONTROL_INVALID_RETURN_ADDRESS",
            FAST_FAIL::UNEXPECTED_HOST_BEHAVIOR         => "UNEXPECTED_HOST_BEHAVIOR",
            FAST_FAIL::FLAGS_CORRUPTION                 => "FLAGS_CORRUPTION",
            FAST_FAIL::VEH_CORRUPTION                   => "VEH_CORRUPTION",
            FAST_FAIL::ETW_CORRUPTION                   => "ETW_CORRUPTION",
            FAST_FAIL::RIO_ABORT                        => "RIO_ABORT",
            FAST_FAIL::INVALID_PFN                      => "INVALID_PFN",
            FAST_FAIL::GUARD_ICALL_CHECK_FAILURE_XFG    => "GUARD_ICALL_CHECK_FAILURE_XFG",
            FAST_FAIL::CAST_GUARD                       => "CAST_GUARD",
            FAST_FAIL::HOST_VISIBILITY_CHANGE           => "HOST_VISIBILITY_CHANGE",
            FAST_FAIL::KERNEL_CET_SHADOW_STACK_ASSIST   => "KERNEL_CET_SHADOW_STACK_ASSIST",
            FAST_FAIL::PATCH_CALLBACK_FAILED            => "PATCH_CALLBACK_FAILED",
            FAST_FAIL::NTDLL_PATCH_FAILED               => "NTDLL_PATCH_FAILED",
            FAST_FAIL::INVALID_FLS_DATA                 => "INVALID_FLS_DATA",
            FAST_FAIL::INVALID_FAST_FAIL_CODE           => "INVALID_FAST_FAIL_CODE",
            other                                       => return write!(fmt, "FAST_FAIL_??? ({})", other.0),
        };
        write!(fmt, "FAST_FAIL_{s}")
    }
}

tests! {
    // __fastfail shows up as STATUS_STACK_BUFFER_OVERRUN (0xC0000409)
    #[test] #[isolate = winresult::STATUS::STACK_BUFFER_OVERRUN] fn __fastfail_9001()           { __fastfail(9001) }
    #[test] #[isolate = winresult::STATUS::STACK_BUFFER_OVERRUN] fn __fastfail_fatal_app_exit() { __fastfail(FAST_FAIL::FATAL_APP_EXIT) }
}
