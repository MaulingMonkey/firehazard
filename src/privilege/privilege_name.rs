use abistr::CStrNonNull;

use crate::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/secauthz/privilege-constants)\]
/// Privilege name, referencing a [privilege](https://docs.microsoft.com/en-us/windows/win32/secauthz/privilege-constants#constants) such as `"SeShutdownPrivilege"`
#[derive(Clone, Copy, Debug)] #[repr(transparent)] pub struct Name(CStrNonNull<'static>);

impl Name {
    pub fn luid(self) -> privilege::Luid {
        privilege::lookup_privilege_value_a(self.0).unwrap()
    }
}

impl From<Name> for privilege::Luid { fn from(n: Name) -> Self { n.luid() } }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/secauthz/privilege-constants)\]
/// SE_*_NAME
pub mod name {
    use super::*;
    macro_rules! name { ( $name:tt ) => { Name(abistr::cstr!($name)) } }

    // C:\Program Files (x86)\Windows Kits\10\Include\10.0.22621.0\um\winnt.h
    // Line 11396 .. 11431
    pub const CREATE_TOKEN                      : Name = name!("SeCreateTokenPrivilege");
    pub const ASSIGNPRIMARYTOKEN                : Name = name!("SeAssignPrimaryTokenPrivilege");
    pub const LOCK_MEMORY                       : Name = name!("SeLockMemoryPrivilege");
    pub const INCREASE_QUOTA                    : Name = name!("SeIncreaseQuotaPrivilege");
    pub const UNSOLICITED_INPUT                 : Name = name!("SeUnsolicitedInputPrivilege");
    pub const MACHINE_ACCOUNT                   : Name = name!("SeMachineAccountPrivilege");
    pub const TCB                               : Name = name!("SeTcbPrivilege");
    pub const SECURITY                          : Name = name!("SeSecurityPrivilege");
    pub const TAKE_OWNERSHIP                    : Name = name!("SeTakeOwnershipPrivilege");
    pub const LOAD_DRIVER                       : Name = name!("SeLoadDriverPrivilege");
    pub const SYSTEM_PROFILE                    : Name = name!("SeSystemProfilePrivilege");
    pub const SYSTEMTIME                        : Name = name!("SeSystemtimePrivilege");
    pub const PROF_SINGLE_PROCESS               : Name = name!("SeProfileSingleProcessPrivilege");
    pub const INC_BASE_PRIORITY                 : Name = name!("SeIncreaseBasePriorityPrivilege");
    pub const CREATE_PAGEFILE                   : Name = name!("SeCreatePagefilePrivilege");
    pub const CREATE_PERMANENT                  : Name = name!("SeCreatePermanentPrivilege");
    pub const BACKUP                            : Name = name!("SeBackupPrivilege");
    pub const RESTORE                           : Name = name!("SeRestorePrivilege");
    pub const SHUTDOWN                          : Name = name!("SeShutdownPrivilege");
    pub const DEBUG                             : Name = name!("SeDebugPrivilege");
    pub const AUDIT                             : Name = name!("SeAuditPrivilege");
    pub const SYSTEM_ENVIRONMENT                : Name = name!("SeSystemEnvironmentPrivilege");
    pub const CHANGE_NOTIFY                     : Name = name!("SeChangeNotifyPrivilege");
    pub const REMOTE_SHUTDOWN                   : Name = name!("SeRemoteShutdownPrivilege");
    pub const UNDOCK                            : Name = name!("SeUndockPrivilege");
    pub const SYNC_AGENT                        : Name = name!("SeSyncAgentPrivilege");
    pub const ENABLE_DELEGATION                 : Name = name!("SeEnableDelegationPrivilege");
    pub const MANAGE_VOLUME                     : Name = name!("SeManageVolumePrivilege");
    pub const IMPERSONATE                       : Name = name!("SeImpersonatePrivilege");
    pub const CREATE_GLOBAL                     : Name = name!("SeCreateGlobalPrivilege");
    pub const TRUSTED_CREDMAN_ACCESS            : Name = name!("SeTrustedCredManAccessPrivilege");
    pub const RELABEL                           : Name = name!("SeRelabelPrivilege");
    pub const INC_WORKING_SET                   : Name = name!("SeIncreaseWorkingSetPrivilege");
    pub const TIME_ZONE                         : Name = name!("SeTimeZonePrivilege");
    pub const CREATE_SYMBOLIC_LINK              : Name = name!("SeCreateSymbolicLinkPrivilege");
    pub const DELEGATE_SESSION_USER_IMPERSONATE : Name = name!("SeDelegateSessionUserImpersonatePrivilege");
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
