\[[en.wikipedia.org](https://en.wikipedia.org/wiki/Mandatory_Integrity_Control)\] Integrity Levels

Windows Vista introduced Integrity levels, of which 5 are named:

| Integrity     | SID (Decimal) | SID (Hex)     | Notes                             |
| ------------- | ------------- | ------------- | --------------------------------- |
| [`Untrusted`] | S-1-16-0      | S-1-16-0x0000 | Trouble loading system DLLs       |
| [`Low`]       | S-1-16-4096   | S-1-16-0x1000 |                                   |
| [`Medium`]    | S-1-16-8192   | S-1-16-0x2000 | Typical user process              |
| [`High`]      | S-1-16-12288  | S-1-16-0x3000 | Typical elevated/admin process    |
| [`System`]    | S-1-16-16384  | S-1-16-0x4000 |

This provides various protections:
*   Lower IL processes generally can't read from or send messages to Higher IL processes
*   Various file/registry stuff is blocked from [`Low`]/[`Untrusted`] Integrity Levels
*   [`Untrusted`] is incredibly restricted: executables linking Rust's `std` can't even reach their entry point due to blocked access to DLLs and `/KnownDlls`.

## Notes
*   You cannot raise the integrity level of a running process, even temporarilly from a parent process with [`set_thread_token`] at or above the integrity level you're raising to.
    *   The process will die with exit code `0xC00000A5` / `STATUS_BAD_IMPERSONATION_LEVEL` if you try anyways.
*   To query the token's integrity level, the handle must have [`token::QUERY`] access.
*   You *can lower* the integrity level of a running process by modifying it's primary token.
    *   The handle must have [`token::ADJUST_DEFAULT`] access.
    *   [`create_process_as_user_w`] clones the passed-in impersonation token, so you can't propigate chances through that handle after process creation.
    *   If the child is self-modifying, it will likely need to open the token before calling [`revert_to_self`].
    ```cpp
    HANDLE token;
    OpenProcessToken(child.hProcess, TOKEN_ADJUST_DEFAULT, &token);
    SetTokenInformation(token, TokenIntegrityLevel, ...);
    ResumeThread(child.hThread);
    ```
*   Rust's `std` requires [`Low`] to initialize sanely.  In particular, it uses these for hashmap seeding / DoS resistance:
    *   `bcrypt.dll` / `BCryptGenRandom`, which fail during `DllInit` / cause the process to die with `ERROR_DLL_INIT_FAILED` if initialized under [`Untrusted`] integrity.
    *   `cryptbase.dll` for `SystemFunction036` / `RtlGenRandom`, which - not being part of `/KnownDlls` - will fail to load under [`Untrusted`] integrity...  I think?
*   A bare bones no-imports executable can initialize at [`Untrusted`] integrity.
    *   You have access to `ntdll.dll` at this integrity level.
    *   Anything else is probably a bit sketchy, although some `/KnownDlls` functions might work?

## References
*   <https://en.wikipedia.org/wiki/Mandatory_Integrity_Control>
*   <https://docs.microsoft.com/en-us/previous-versions/windows/internet-explorer/ie-developer/platform-apis/ms537319(v=vs.85)>
*   <https://stackoverflow.com/questions/34500133/setting-up-a-chromium-like-sandbox-error-0xc00000a5>
*   <https://stackoverflow.com/questions/44027935/windows-process-with-untrusted-integrity-level/44032406>
