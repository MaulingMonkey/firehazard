## Security
* [x] SID lockdown
* [X] Privileges lockdown
* [X] Integrity level lockdown
* [ ] AppContainer s?
    * [ ] Low box token?
    * [ ] Less Privileged App Container (LPAC)
* [ ] Job object lockdown
* [ ] Alternative desktop object
* [ ] Alternative console object?
* [ ] Process mitigation policies
    * [ ] ASLR
        * [ ] Basic
        * [ ] Bottom-up?
        * [ ] High-entropy
    * [ ] Heap corruption terminate
    * [ ] Strict handle checks
    * [ ] ProcessSystemCallDisablePolicy
    * [ ] ProcessExtensionPointDisablePolicy
    * [ ] ProcessFontDisablePolicy
    * [ ] ProcessSignaturePolicy
    * [ ] ProcessImageLoadPolicy
    * [ ] Control Flow Guard
    * [ ] CET Shadow Stack
* [ ] `HANDLE` auditing
    * [ ] [GetProcessHandleCount](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocesshandlecount)
    * [ ] [NtQuerySystemInformation](https://docs.microsoft.com/en-us/windows/win32/api/winternl/nf-winternl-ntquerysysteminformation)(SystemHandleInformation, ...)

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
