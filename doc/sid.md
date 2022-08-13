A security identifier (SID) is a unique value of variable length used to identify a trustee / user / group.

## Examples
| SID                                               | Label                                             | Category |
| ------------------------------------------------- | ------------------------------------------------- | -------- |
| S-1-5-21-2440711095-4246273057-2830868914-1001    | MaulingMonkey                                     | User
| S-1-5-21-2440711095-4246273057-2830868914-513     | None                                              | Group ("Primary" group at that?)
| S-1-0-0                                           | NULL SID                                          | ???
| S-1-1-0                                           | Everyone                                          | Group
| S-1-5-114                                         | Local account and member of Administrators group  | Group (Disabled by Default)
| S-1-5-21-2440711095-4246273057-2830868914-1002    | docker-users                                      | Group
| S-1-5-32-544                                      | Administrators                                    | Group (Disabled by Default)
| S-1-5-32-559                                      | Performance Log Users                             | Group
| S-1-5-32-545                                      | Users                                             | Group
| S-1-5-4                                           | INTERACTIVE                                       | Session Tag?
| S-1-2-1                                           | CONSOLE LOGON                                     | Session Tag?
| S-1-5-11                                          | Authenticated Users                               | Session Tag?
| S-1-5-15                                          | This Organization                                 | Group
| S-1-5-113                                         | Local account                                     | Group
| S-1-5-5-0-280282025                               | LogonSessionId_0_280282025                        | Session
| S-1-2-0                                           | LOCAL                                             | Group?
| S-1-5-64-10                                       | NTLM Authentication                               | Session Tag?
| S-1-16-0                                          | Untrusted Mandatory Level                         | Integrity Level
| S-1-16-4096                                       | Low Mandatory Level                               | Integrity Level
| S-1-16-8192                                       | Medium Mandatory Level                            | Integrity Level
| S-1-15-2-1                                        | ALL APPLICATION PACKAGES                          | Software Group
| S-1-15-2-2                                        | ALL RESTRICTED APPLICATION PACKAGES               | Software Group

| SID | Desc |
| --- | ---- |
| S-1-15-3-1024-3424233489-972189580-2057154623-747635277-1604371224-316187997-3786583170-1043257646    | [Chrome Installer SID](https://chromium.googlesource.com/chromium/src/+/refs/heads/main/chrome/installer/setup/install_worker.cc#75)
| S-1-15-3-1024-2302894289-466761758-1166120688-1039016420-2430351297-4240214049-4028510897-3317428798  | [Chrome Installer SID](https://chromium.googlesource.com/chromium/src/+/refs/heads/main/chrome/installer/setup/install_worker.cc#75)

| Category              | Desc |
| --------------------- | ---- |
| User                  | An individual user.  Included in `TokenUser`, `TokenOwner`, and `TokenGroupsAndPrivileges`, but not `TokenGroups`.
| Group                 | A group of accounts
| Session               | A unique, temporary, never-reused, logged in user's session
| Integrity Level       | <https://en.wikipedia.org/wiki/Mandatory_Integrity_Control>
| Software Group        | A group used to specify types of software that can access said resource (e.g. Windows Store Apps)
| Capability            | <https://docs.microsoft.com/en-us/windows/win32/secauthz/capability-sid-constants>
| AppContainer/Package  | An individual windows store app container/package<br><https://docs.microsoft.com/en-us/windows/win32/secauthz/implementing-an-appcontainer><br><https://docs.microsoft.com/en-us/windows/win32/api/userenv/nf-userenv-deriveappcontainersidfromappcontainername><br><https://medium.com/that-feeling-when-it-is-compiler-fault/appcontainers-for-windows-8-what-are-they-and-how-can-you-create-them-e5970a28eea4>

## References
*   [Security Identifiers](https://docs.microsoft.com/en-us/windows/win32/secauthz/security-identifiers)
*   [SID structure (winnt.h)](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-sid)
*   [Well-known SIDs](https://docs.microsoft.com/en-us/windows/win32/secauthz/well-known-sids)
