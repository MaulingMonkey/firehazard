## Security
* [x] SID lockdown
* [X] Privileges lockdown
* [X] Integrity level lockdown
* [ ] AppContainer s?
    * [ ] Low box token?
    * [ ] Less Privileged App Container (LPAC)
* [ ] Job object lockdown
* [X] Alternative desktop object
* [ ] Alternative console object?
* [ ] Process mitigation policies
    * [ ] Apply to max_sandbox
    * [X] ASLR
    * [X] Heap corruption terminate
    * [X] Strict handle checks
    * [X] ProcessSystemCallDisablePolicy
    * [X] ProcessExtensionPointDisablePolicy
    * [X] ProcessFontDisablePolicy
    * [X] ProcessSignaturePolicy
    * [X] ProcessImageLoadPolicy
    * [ ] Control Flow Guard
    * [ ] CET Shadow Stack
* [ ] SYSTEM_MANDATORY_LABEL_NO_WRITE_UP \| SYSTEM_MANDATORY_LABEL_NO_READ_UP \| SYSTEM_MANDATORY_LABEL_NO_EXECUTE_UP
* [ ] `HANDLE` auditing
    * [ ] [GetProcessHandleCount](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocesshandlecount)
    * [ ] [NtQuerySystemInformation](https://docs.microsoft.com/en-us/windows/win32/api/winternl/nf-winternl-ntquerysysteminformation)(SystemHandleInformation, ...)
* [ ] [Private object namespaces](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-createprivatenamespacea) for e.g. jobs?
* [ ] Scrub environment

<https://github.com/chromium/chromium/blob/main/docs/design/sandbox.md>

## Bugs/Caveats
* [ ] Doubled-up SIDs in procexp with some uses of create restricted token?
* [ ] Issues lowering token integrity in multithreaded debug mode?
* [ ] Thread pause/unpause dance doesn't do what I think it does?

## Refactoring
* [ ] High level sandboxing API
* [ ] Well known SID constants (inc. chromium installer SIDs?)

## Research
* [ ] Temporary sandboxing SIDs?
* [ ] Injecting code into early DLL init / modifying imports?
* [ ] Breakpoint APIs?
* [ ] Parse PEs for forbidden imports (would be a debugging feature, not a security feature)
* [ ] WASM -> heavily sandboxed x64 PE compiler
* [ ] Inject info into sandboxed child process with:
    * [ ] VirtualAllocEx
    * [ ] Thread injection to call API?
