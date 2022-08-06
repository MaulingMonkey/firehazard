## Overview

[Privileges](https://docs.microsoft.com/en-us/windows/win32/secauthz/privilege-constants#constants) are referenced either by:
*   String label (e.g. [`"SeShutdownPrivilege"`](https://docs.microsoft.com/en-us/windows/win32/secauthz/privilege-constants#constants))
*   `u64` LUID (Locally Unique ID)

Convert with:
*   [`LookupPrivilegeValue`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-lookupprivilegevaluea): `&str -> LUID`
*   [`LookupPrivilegeName`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-lookupprivilegenamea): `LUID -> &str`

An access token has main states for any given privilege:
*   `Enabled`   (e.g. `ExitWindowsEx` will succeed)
*   `Disabled`  (e.g. `ExitWindowsEx` will fail... unless you enable the privilege first, as `shutdown /s /t 0` does.  This is a guard against accidents, not malicious code!)
*   `Removed`   (e.g. `ExitWindowsEx` will fail, and the process can't re-enable the privilege, so even `shutdown /s /t 0` should fail.)
*   <https://superuser.com/questions/1254253/how-am-i-able-to-shutdown-the-system-when-i-dont-have-seshutdownprivilege>



## Example

Privileges from my machine on a fairly typical non-elevated admin user:
```text
src\bin\tests.rs:29 t.get_token_privileges().unwrap().privileges() = [
    LuidAndAttributes { luid: PrivilegeLuid(0x13 "SeShutdownPrivilege"), attributes: 0x00000000 },
    LuidAndAttributes { luid: PrivilegeLuid(0x17 "SeChangeNotifyPrivilege"), attributes: 0x00000003 },
    LuidAndAttributes { luid: PrivilegeLuid(0x19 "SeUndockPrivilege"), attributes: 0x00000000 },
    LuidAndAttributes { luid: PrivilegeLuid(0x21 "SeIncreaseWorkingSetPrivilege"), attributes: 0x00000000 },
    LuidAndAttributes { luid: PrivilegeLuid(0x22 "SeTimeZonePrivilege"), attributes: 0x00000000 },
]
```

Notes:
*   Many privileges default to 0 attributes
*   SeChangeNotifyPrivilege defaults to `SE_PRIVILEGE_ENABLED_BY_DEFAULT | SE_PRIVILEGE_ENABLED`

| Privilege                     | Desc |
| ----------------------------- | ---- |
| SeShutdownPrivilege           | Shut down the system |
| SeChangeNotifyPrivilege       | Receive notifications of changes to files or directories. This privilege also causes the system to skip all traversal access checks. It is enabled by default for all users.
| SeUndockPrivilege             | Undock a laptop / remove computer from docking station.
| SeIncreaseWorkingSetPrivilege | Allocate more memory for applications that run in the context of users.
| SeTimeZonePrivilege           | Adjust the time zone associated with the computer's internal clock.

## References
*   <https://docs.microsoft.com/en-us/windows/win32/secauthz/privilege-constants>

## Constants
From `C:\Program Files (x86)\Windows Kits\10\Include\10.0.17134.0\um\winnt.h`:
```cpp
#define SE_PRIVILEGE_ENABLED_BY_DEFAULT (0x00000001L)
#define SE_PRIVILEGE_ENABLED            (0x00000002L)
#define SE_PRIVILEGE_REMOVED            (0X00000004L)
#define SE_PRIVILEGE_USED_FOR_ACCESS    (0x80000000L)
```
```cpp
#define SE_CREATE_TOKEN_NAME                         TEXT("SeCreateTokenPrivilege")
#define SE_ASSIGNPRIMARYTOKEN_NAME                   TEXT("SeAssignPrimaryTokenPrivilege")
#define SE_LOCK_MEMORY_NAME                          TEXT("SeLockMemoryPrivilege")
#define SE_INCREASE_QUOTA_NAME                       TEXT("SeIncreaseQuotaPrivilege")
#define SE_UNSOLICITED_INPUT_NAME                    TEXT("SeUnsolicitedInputPrivilege")
#define SE_MACHINE_ACCOUNT_NAME                      TEXT("SeMachineAccountPrivilege")
#define SE_TCB_NAME                                  TEXT("SeTcbPrivilege")
#define SE_SECURITY_NAME                             TEXT("SeSecurityPrivilege")
#define SE_TAKE_OWNERSHIP_NAME                       TEXT("SeTakeOwnershipPrivilege")
#define SE_LOAD_DRIVER_NAME                          TEXT("SeLoadDriverPrivilege")
#define SE_SYSTEM_PROFILE_NAME                       TEXT("SeSystemProfilePrivilege")
#define SE_SYSTEMTIME_NAME                           TEXT("SeSystemtimePrivilege")
#define SE_PROF_SINGLE_PROCESS_NAME                  TEXT("SeProfileSingleProcessPrivilege")
#define SE_INC_BASE_PRIORITY_NAME                    TEXT("SeIncreaseBasePriorityPrivilege")
#define SE_CREATE_PAGEFILE_NAME                      TEXT("SeCreatePagefilePrivilege")
#define SE_CREATE_PERMANENT_NAME                     TEXT("SeCreatePermanentPrivilege")
#define SE_BACKUP_NAME                               TEXT("SeBackupPrivilege")
#define SE_RESTORE_NAME                              TEXT("SeRestorePrivilege")
#define SE_SHUTDOWN_NAME                             TEXT("SeShutdownPrivilege")
#define SE_DEBUG_NAME                                TEXT("SeDebugPrivilege")
#define SE_AUDIT_NAME                                TEXT("SeAuditPrivilege")
#define SE_SYSTEM_ENVIRONMENT_NAME                   TEXT("SeSystemEnvironmentPrivilege")
#define SE_CHANGE_NOTIFY_NAME                        TEXT("SeChangeNotifyPrivilege")
#define SE_REMOTE_SHUTDOWN_NAME                      TEXT("SeRemoteShutdownPrivilege")
#define SE_UNDOCK_NAME                               TEXT("SeUndockPrivilege")
#define SE_SYNC_AGENT_NAME                           TEXT("SeSyncAgentPrivilege")
#define SE_ENABLE_DELEGATION_NAME                    TEXT("SeEnableDelegationPrivilege")
#define SE_MANAGE_VOLUME_NAME                        TEXT("SeManageVolumePrivilege")
#define SE_IMPERSONATE_NAME                          TEXT("SeImpersonatePrivilege")
#define SE_CREATE_GLOBAL_NAME                        TEXT("SeCreateGlobalPrivilege")
#define SE_TRUSTED_CREDMAN_ACCESS_NAME               TEXT("SeTrustedCredManAccessPrivilege")
#define SE_RELABEL_NAME                              TEXT("SeRelabelPrivilege")
#define SE_INC_WORKING_SET_NAME                      TEXT("SeIncreaseWorkingSetPrivilege")
#define SE_TIME_ZONE_NAME                            TEXT("SeTimeZonePrivilege")
#define SE_CREATE_SYMBOLIC_LINK_NAME                 TEXT("SeCreateSymbolicLinkPrivilege")
#define SE_DELEGATE_SESSION_USER_IMPERSONATE_NAME    TEXT("SeDelegateSessionUserImpersonatePrivilege")

// begin_ntosifs

//
// List Of String Capabilities.
//
#define SE_ACTIVATE_AS_USER_CAPABILITY L"activateAsUser"
#define SE_CONSTRAINED_IMPERSONATION_CAPABILITY L"constrainedImpersonation"
#define SE_SESSION_IMPERSONATION_CAPABILITY L"sessionImpersonation"
#define SE_MUMA_CAPABILITY L"muma"
#define SE_DEVELOPMENT_MODE_NETWORK_CAPABILITY L"developmentModeNetwork"
```