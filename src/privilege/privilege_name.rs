use crate::*;

use abistr::CStrNonNull;
use winapi::um::winnt::LUID;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/secauthz/privilege-constants)\]
/// Privilege name, referencing a [privilege](https://learn.microsoft.com/en-us/windows/win32/secauthz/privilege-constants#constants) such as `"SeShutdownPrivilege"`
///
#[derive(Clone, Copy, Debug)] pub struct Name {
    name:       CStrNonNull<'static>,
    hardcoded:  privilege::Luid,
}

impl Name {
    #[doc(alias = "LookupPrivilegeValueA")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-lookupprivilegevaluea)\]
    /// LookupPrivilegeValueA
    ///
    /// Lookup a [`privilege::Luid`](crate::privilege::Luid) by it's string identifier.
    ///
    /// ### Errors
    /// *   `ERROR_NO_SUCH_PRIVILEGE`   if `name` doesn't name a known privilege in this version of Windows
    /// *   `ERROR_INVALID_HANDLE`      on some permission lookup errors (e.g. if the current process's token was restricted, and excluded [`sid::builtin::alias::USERS`](crate::sid::builtin::alias::USERS))
    ///
    pub fn lookup_luid(self) -> Result<privilege::Luid, Error> {
        privilege::lookup_privilege_value_a(self.name)
    }

    /// A hardcoded [`privilege::Luid`] value.
    ///
    /// The fact that privilege luid values are documented in the
    /// [Local Security Authority (Domain Policy) Remote Protocol](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-lsad/1a92af76-d45f-42c3-b67c-f1dc61bd6ee1)
    /// / RPC documentation lends credence to the idea that these LUIDs are stable - however, this isn't a guarantee that I've found explicitly spelled out anywhere.
    /// Additionally, there's no guarantee that these LUIDs are explicitly supported on whatever version of Windows you're running.
    ///
    pub const fn hardcoded_luid(self) -> privilege::Luid { self.hardcoded }
}

impl TryFrom<Name> for privilege::Luid {
    type Error = crate::Error;
    fn try_from(n: Name) -> Result<Self, Self::Error> { n.lookup_luid() }
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/secauthz/privilege-constants)\]
/// SE_*_NAME
///
pub mod name {
    use super::*;
    macro_rules! constants { ($(
        $(#[$($attr:tt)*])*
        pub const $id:ident = $name:tt / $luid:literal;
    )*) => {
        $(
            $(#[$($attr)*])*
            #[doc = concat!(stringify!($name))]
            ///
            pub const $id : Name = Name {
                name:       abistr::cstr!($name),
                hardcoded:  privilege::Luid(Luid(LUID { HighPart: 0, LowPart: $luid })),
            };
        )*
        #[test] fn constants() {
            $(
                assert_eq!($id.lookup_luid().expect($name), $id.hardcoded_luid(), "{} had unexpected value", $name);
            )*
        }
    }}

    // C:\Program Files (x86)\Windows Kits\10\Include\10.0.22621.0\um\winnt.h
    // Line 11396 .. 11431
    // https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-lsad/1a92af76-d45f-42c3-b67c-f1dc61bd6ee1
    // https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntifs/ns-ntifs-_se_exports
    constants! {
        // no privilege for Luid::from(1)
        #[doc(alias = "SE_CREATE_TOKEN_NAME")]                          pub const CREATE_TOKEN                      = "SeCreateTokenPrivilege"                      / 2;
        #[doc(alias = "SE_ASSIGNPRIMARYTOKEN_NAME"  )]                  pub const ASSIGNPRIMARYTOKEN                = "SeAssignPrimaryTokenPrivilege"               / 3;
        #[doc(alias = "SE_LOCK_MEMORY_NAME")]                           pub const LOCK_MEMORY                       = "SeLockMemoryPrivilege"                       / 4;
        #[doc(alias = "SE_INCREASE_QUOTA_NAME")]                        pub const INCREASE_QUOTA                    = "SeIncreaseQuotaPrivilege"                    / 5;
        //doc(alias = "SE_UNSOLICITED_INPUT_NAME")]                     pub const UNSOLICITED_INPUT                 = "SeUnsolicitedInputPrivilege"; // ERROR_NO_SUCH_PRIVILEGE - "The privilege that is required to read unsolicited input from a terminal device. This privilege is obsolete and unused. It has no effect on the system."
        #[doc(alias = "SE_MACHINE_ACCOUNT_NAME")]                       pub const MACHINE_ACCOUNT                   = "SeMachineAccountPrivilege"                   / 6;
        #[doc(alias = "SE_TCB_NAME")]                                   pub const TCB                               = "SeTcbPrivilege"                              / 7;
        #[doc(alias = "SE_SECURITY_NAME")]                              pub const SECURITY                          = "SeSecurityPrivilege"                         / 8;
        #[doc(alias = "SE_TAKE_OWNERSHIP_NAME")]                        pub const TAKE_OWNERSHIP                    = "SeTakeOwnershipPrivilege"                    / 9;
        #[doc(alias = "SE_LOAD_DRIVER_NAME")]                           pub const LOAD_DRIVER                       = "SeLoadDriverPrivilege"                       / 10;
        #[doc(alias = "SE_SYSTEM_PROFILE_NAME")]                        pub const SYSTEM_PROFILE                    = "SeSystemProfilePrivilege"                    / 11;
        #[doc(alias = "SE_SYSTEMTIME_NAME")]                            pub const SYSTEMTIME                        = "SeSystemtimePrivilege"                       / 12;
        #[doc(alias = "SE_PROF_SINGLE_PROCESS_NAME")]                   pub const PROF_SINGLE_PROCESS               = "SeProfileSingleProcessPrivilege"             / 13;
        #[doc(alias = "SE_INC_BASE_PRIORITY_NAME")]                     pub const INC_BASE_PRIORITY                 = "SeIncreaseBasePriorityPrivilege"             / 14;
        #[doc(alias = "SE_CREATE_PAGEFILE_NAME")]                       pub const CREATE_PAGEFILE                   = "SeCreatePagefilePrivilege"                   / 15;
        #[doc(alias = "SE_CREATE_PERMANENT_NAME")]                      pub const CREATE_PERMANENT                  = "SeCreatePermanentPrivilege"                  / 16;
        #[doc(alias = "SE_BACKUP_NAME")]                                pub const BACKUP                            = "SeBackupPrivilege"                           / 17;
        #[doc(alias = "SE_RESTORE_NAME")]                               pub const RESTORE                           = "SeRestorePrivilege"                          / 18;
        #[doc(alias = "SE_SHUTDOWN_NAME")]                              pub const SHUTDOWN                          = "SeShutdownPrivilege"                         / 19;
        #[doc(alias = "SE_DEBUG_NAME")]                                 pub const DEBUG                             = "SeDebugPrivilege"                            / 20;
        #[doc(alias = "SE_AUDIT_NAME")]                                 pub const AUDIT                             = "SeAuditPrivilege"                            / 21;
        #[doc(alias = "SE_SYSTEM_ENVIRONMENT_NAME")]                    pub const SYSTEM_ENVIRONMENT                = "SeSystemEnvironmentPrivilege"                / 22; // misdocumented as "SeSystemEnvironment" by https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-lsad/1a92af76-d45f-42c3-b67c-f1dc61bd6ee1
        #[doc(alias = "SE_CHANGE_NOTIFY_NAME")]                         pub const CHANGE_NOTIFY                     = "SeChangeNotifyPrivilege"                     / 23;
        #[doc(alias = "SE_REMOTE_SHUTDOWN_NAME")]                       pub const REMOTE_SHUTDOWN                   = "SeRemoteShutdownPrivilege"                   / 24;
        #[doc(alias = "SE_UNDOCK_NAME")]                                pub const UNDOCK                            = "SeUndockPrivilege"                           / 25;
        #[doc(alias = "SE_SYNC_AGENT_NAME")]                            pub const SYNC_AGENT                        = "SeSyncAgentPrivilege"                        / 26;
        #[doc(alias = "SE_ENABLE_DELEGATION_NAME")]                     pub const ENABLE_DELEGATION                 = "SeEnableDelegationPrivilege"                 / 27;
        #[doc(alias = "SE_MANAGE_VOLUME_NAME")]                         pub const MANAGE_VOLUME                     = "SeManageVolumePrivilege"                     / 28;
        #[doc(alias = "SE_IMPERSONATE_NAME")]                           pub const IMPERSONATE                       = "SeImpersonatePrivilege"                      / 29;
        #[doc(alias = "SE_CREATE_GLOBAL_NAME")]                         pub const CREATE_GLOBAL                     = "SeCreateGlobalPrivilege"                     / 30;
        #[doc(alias = "SE_TRUSTED_CREDMAN_ACCESS_NAME")]                pub const TRUSTED_CREDMAN_ACCESS            = "SeTrustedCredManAccessPrivilege"             / 31;
        #[doc(alias = "SE_RELABEL_NAME")]                               pub const RELABEL                           = "SeRelabelPrivilege"                          / 32;
        #[doc(alias = "SE_INC_WORKING_SET_NAME")]                       pub const INC_WORKING_SET                   = "SeIncreaseWorkingSetPrivilege"               / 33;
        #[doc(alias = "SE_TIME_ZONE_NAME")]                             pub const TIME_ZONE                         = "SeTimeZonePrivilege"                         / 34;
        #[doc(alias = "SE_CREATE_SYMBOLIC_LINK_NAME")]                  pub const CREATE_SYMBOLIC_LINK              = "SeCreateSymbolicLinkPrivilege"               / 35;
        #[doc(alias = "SE_DELEGATE_SESSION_USER_IMPERSONATE_NAME")]     pub const DELEGATE_SESSION_USER_IMPERSONATE = "SeDelegateSessionUserImpersonatePrivilege"   / 36; // undocumented by https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-lsad/1a92af76-d45f-42c3-b67c-f1dc61bd6ee1
    }
}

#[cfg(nope)] pub mod capability { // XXX: do these "string capabilities" also count as privileges?  N.B. suffix _CAPABILITY, not _NAME?
    // #define SE_ACTIVATE_AS_USER_CAPABILITY L"activateAsUser"
    // #define SE_CONSTRAINED_IMPERSONATION_CAPABILITY L"constrainedImpersonation"
    // #define SE_SESSION_IMPERSONATION_CAPABILITY L"sessionImpersonation"
    // #define SE_MUMA_CAPABILITY L"muma"
    // #define SE_DEVELOPMENT_MODE_NETWORK_CAPABILITY L"developmentModeNetwork"
    // #define SE_LEARNING_MODE_LOGGING_CAPABILITY L"learningModeLogging"
    // #define SE_PERMISSIVE_LEARNING_MODE_CAPABILITY L"permissiveLearningMode"
    // #define SE_APP_SILO_VOLUME_ROOT_MINIMAL_CAPABILITY L"isolatedWin32-volumeRootMinimal"
    // #define SE_APP_SILO_PROFILES_ROOT_MINIMAL_CAPABILITY L"isolatedWin32-profilesRootMinimal"
    // #define SE_APP_SILO_USER_PROFILE_MINIMAL_CAPABILITY L"isolatedWin32-userProfileMinimal"
    // #define SE_APP_SILO_PRINT_CAPABILITY L"isolatedWin32-print"
}
