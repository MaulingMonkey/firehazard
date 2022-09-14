\[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/secauthz/privilege-constants)\]
Privilege related types and functions

## Overview

[Privileges](https://docs.microsoft.com/en-us/windows/win32/secauthz/privilege-constants#constants) are referenced either by:
*   String label (e.g. [`"SeShutdownPrivilege"`](https://docs.microsoft.com/en-us/windows/win32/secauthz/privilege-constants#constants))
*   64-bit integer [`privilege::Luid`] (Locally Unique ID)

Convert with:
*   [`lookup_privilege_value_a`]: <code>&[str] -> [Luid]</code>
*   [`lookup_privilege_name_a`]: <code>[Luid] -> [String](std::string::String)</code>

An access token has 3 possible states for any given privilege.  Using `SeShutdownPrivilege` as an example:

| State     | Behavior |
| --------- | -------- |
| Enabled   | `ExitWindowsEx` will succeed.
| Disabled  | `ExitWindowsEx` will fail.  The process **can** re-enable the privilege first, as `shutdown /s /t 0` does.<br>This is a guard against accidents, not malicious code!
| Removed   | `ExitWindowsEx` will fail.  The process can't re-enable the privilege, so even `shutdown /s /t 0` fails.



## Example

Privileges from my machine on a fairly typical non-elevated admin user:
```text
src\bin\tests.rs:29 t.get_token_privileges().unwrap().privileges() = [
    LuidAndAttributes { luid: privilege::Luid(0x13 "SeShutdownPrivilege"), attributes: 0x00000000 },
    LuidAndAttributes { luid: privilege::Luid(0x17 "SeChangeNotifyPrivilege"), attributes: 0x00000003 },
    LuidAndAttributes { luid: privilege::Luid(0x19 "SeUndockPrivilege"), attributes: 0x00000000 },
    LuidAndAttributes { luid: privilege::Luid(0x21 "SeIncreaseWorkingSetPrivilege"), attributes: 0x00000000 },
    LuidAndAttributes { luid: privilege::Luid(0x22 "SeTimeZonePrivilege"), attributes: 0x00000000 },
]
```

Notes:
*   Many privileges default to 0 attributes
*   SeChangeNotifyPrivilege defaults to `SE_PRIVILEGE_ENABLED_BY_DEFAULT | SE_PRIVILEGE_ENABLED`

| Privilege                     | Desc |
| ----------------------------- | ---- |
| SeShutdownPrivilege           | Shut down the system |
| SeChangeNotifyPrivilege       | Receive notifications of changes to files or directories.<br>Also skip all traversal access checks.<br>Enabled by default for all users.
| SeUndockPrivilege             | Undock a laptop / remove computer from docking station.
| SeIncreaseWorkingSetPrivilege | Allocate more memory for applications that run in the context of users.
| SeTimeZonePrivilege           | Adjust the time zone associated with the computer's internal clock.

## References
*   [docs.microsoft.com: Privilege Constants (Authorization)](https://docs.microsoft.com/en-us/windows/win32/secauthz/privilege-constants)
*   [Stack Overflow: How am I able to shutdown the system when I don't have SeShutdownPrivilege](https://superuser.com/questions/1254253/how-am-i-able-to-shutdown-the-system-when-i-dont-have-seshutdownprivilege)
*   `#define SE_*` in `C:\Program Files (x86)\Windows Kits\10\Include\10.0.17134.0\um\winnt.h`
