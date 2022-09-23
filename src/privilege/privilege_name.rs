use abistr::CStrNonNull;

use crate::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/secauthz/privilege-constants)\]
/// Privilege name, referencing a [privilege](https://learn.microsoft.com/en-us/windows/win32/secauthz/privilege-constants#constants) such as `"SeShutdownPrivilege"`
#[derive(Clone, Copy, Debug)] #[repr(transparent)] pub struct Name(CStrNonNull<'static>);

impl Name {
    /// [`lookup_privilege_value_a`] a known privilege identifier.
    ///
    /// ### Errors
    /// *   `ERROR_NO_SUCH_PRIVILEGE`   if `name` doesn't name a known privilege in this version of Windows
    /// *   `ERROR_INVALID_HANDLE`      on some permission lookup errors (e.g. if the current process's token was restricted, and excluded [`sid::builtin::alias::USERS`](crate::sid::builtin::alias::USERS))
    pub fn luid(self) -> Result<privilege::Luid, Error> {
        privilege::lookup_privilege_value_a(self.0)
    }
}

impl TryFrom<Name> for privilege::Luid {
    type Error = crate::Error;
    fn try_from(n: Name) -> Result<Self, Self::Error> { n.luid() }
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/secauthz/privilege-constants)\]
/// SE_*_NAME
pub mod name {
    use super::*;
    macro_rules! constants { ($(
        pub const $id:ident = $name:tt $(/ $luid:literal)?;
    )*) => {
        $(
            pub const $id : Name = Name(abistr::cstr!($name));
        )*
        #[test] fn constants() {
            $(
                let _luid = $id.luid().expect($name);
                $(
                    assert_eq!(_luid, privilege::Luid::from($luid), "{} had unexpected value", $name);
                )?
            )*
        }
    }}

    // C:\Program Files (x86)\Windows Kits\10\Include\10.0.22621.0\um\winnt.h
    // Line 11396 .. 11431
    // https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-lsad/1a92af76-d45f-42c3-b67c-f1dc61bd6ee1
    // https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntifs/ns-ntifs-_se_exports
    constants! {
        // no privilege for Luid::from(1)
        pub const CREATE_TOKEN                      = "SeCreateTokenPrivilege"                      / 2;
        pub const ASSIGNPRIMARYTOKEN                = "SeAssignPrimaryTokenPrivilege"               / 3;
        pub const LOCK_MEMORY                       = "SeLockMemoryPrivilege"                       / 4;
        pub const INCREASE_QUOTA                    = "SeIncreaseQuotaPrivilege"                    / 5;
        // pub const UNSOLICITED_INPUT                 = "SeUnsolicitedInputPrivilege"; // ERROR_NO_SUCH_PRIVILEGE - "The privilege that is required to read unsolicited input from a terminal device. This privilege is obsolete and unused. It has no effect on the system."
        pub const MACHINE_ACCOUNT                   = "SeMachineAccountPrivilege"                   / 6;
        pub const TCB                               = "SeTcbPrivilege"                              / 7;
        pub const SECURITY                          = "SeSecurityPrivilege"                         / 8;
        pub const TAKE_OWNERSHIP                    = "SeTakeOwnershipPrivilege"                    / 9;
        pub const LOAD_DRIVER                       = "SeLoadDriverPrivilege"                       / 10;
        pub const SYSTEM_PROFILE                    = "SeSystemProfilePrivilege"                    / 11;
        pub const SYSTEMTIME                        = "SeSystemtimePrivilege"                       / 12;
        pub const PROF_SINGLE_PROCESS               = "SeProfileSingleProcessPrivilege"             / 13;
        pub const INC_BASE_PRIORITY                 = "SeIncreaseBasePriorityPrivilege"             / 14;
        pub const CREATE_PAGEFILE                   = "SeCreatePagefilePrivilege"                   / 15;
        pub const CREATE_PERMANENT                  = "SeCreatePermanentPrivilege"                  / 16;
        pub const BACKUP                            = "SeBackupPrivilege"                           / 17;
        pub const RESTORE                           = "SeRestorePrivilege"                          / 18;
        pub const SHUTDOWN                          = "SeShutdownPrivilege"                         / 19;
        pub const DEBUG                             = "SeDebugPrivilege"                            / 20;
        pub const AUDIT                             = "SeAuditPrivilege"                            / 21;
        pub const SYSTEM_ENVIRONMENT                = "SeSystemEnvironmentPrivilege"                / 22; // misdocumented as "SeSystemEnvironment" by https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-lsad/1a92af76-d45f-42c3-b67c-f1dc61bd6ee1
        pub const CHANGE_NOTIFY                     = "SeChangeNotifyPrivilege"                     / 23;
        pub const REMOTE_SHUTDOWN                   = "SeRemoteShutdownPrivilege"                   / 24;
        pub const UNDOCK                            = "SeUndockPrivilege"                           / 25;
        pub const SYNC_AGENT                        = "SeSyncAgentPrivilege"                        / 26;
        pub const ENABLE_DELEGATION                 = "SeEnableDelegationPrivilege"                 / 27;
        pub const MANAGE_VOLUME                     = "SeManageVolumePrivilege"                     / 28;
        pub const IMPERSONATE                       = "SeImpersonatePrivilege"                      / 29;
        pub const CREATE_GLOBAL                     = "SeCreateGlobalPrivilege"                     / 30;
        pub const TRUSTED_CREDMAN_ACCESS            = "SeTrustedCredManAccessPrivilege"             / 31;
        pub const RELABEL                           = "SeRelabelPrivilege"                          / 32;
        pub const INC_WORKING_SET                   = "SeIncreaseWorkingSetPrivilege"               / 33;
        pub const TIME_ZONE                         = "SeTimeZonePrivilege"                         / 34;
        pub const CREATE_SYMBOLIC_LINK              = "SeCreateSymbolicLinkPrivilege"               / 35;
        pub const DELEGATE_SESSION_USER_IMPERSONATE = "SeDelegateSessionUserImpersonatePrivilege"   / 36; // undocumented by https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-lsad/1a92af76-d45f-42c3-b67c-f1dc61bd6ee1
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
