# Launch Permissions

## Debugging

Launching under a restricted token can cause failures early enough in process startup / pre-main to make debugging a giantic pain.

I recommend:
1.  Launch the child process using `CREATE_SUSPENDED`.
2.  Launch Visual Studio
3.  Set a breakpoint by function name (`Ctrl`+`B`) on `LdrInitializeThunk`.  This won't help for everything, but it will work on a typical Rust exe.
4.  Attach the debugger via `Debug` > `Attach to Process...` (`Ctrl`+`Alt`+`P`)
5.  Set a watch expression: `*(unsigned int *)LdrpDebugFlags = ~0`
6.  Continue execution

On subsequent sessions, if VS was stlil running, you can simplify this to repeating steps 1/5/6:
1.  Launch the child process using `CREATE_SUSPENDED`.
2.  Re-evaluate the watch expression: `*(unsigned int *)LdrpDebugFlags = ~0`
3.  Continue execution

Alternatively, if you only care about seeing debugger spam and don't care about the breakpoints, open up `gflags` / `Global Flags (x64)` and:
1.  In the `Image File` tab
2.  Enter `tests.exe` and refresh
3.  Check `Show loader snaps`
4.  `Apply` before switching tabs!
5.  Run under a debugger or DbgView (output *won't* show in the vanilla terminal as it's done via `OutputDebugString`/`DebugPrint`/equivalents!)

This *seems* to suffice for subprocess debug spam output...

## Deny Group SIDs / Restriction SIDs

Launching even a noop no-std no dll imports executable will fail if using fully restricted group SIDs or restriction SIDs when trying to read `/KnownDlls`:
```
'tests.exe' (Win32): Loaded 'C:\local\win32_security_playground\target\debug\tests.exe'. Symbols loaded.
'tests.exe' (Win32): Loaded 'C:\Windows\System32\ntdll.dll'. Symbols loaded.
622c:1418 @ 294557937 - LdrpInitializeProcess - ERROR: Failed to open \KnownDlls with status 0xc0000022
tests.exe has triggered a breakpoint.

622c:1418 @ 294562875 - _LdrpInitialize - ERROR: Process initialization failed with status 0xc0000022
tests.exe has triggered a breakpoint.

622c:1418 @ 294565062 - LdrpInitializationFailure - ERROR: Process initialization failed with status 0xc0000022
tests.exe has triggered a breakpoint.

The thread 0x1418 has exited with code -1073741790 (0xc0000022).
The program '[25132] tests.exe' has exited with code -1073741790 (0xc0000022).
```
At:
```
ntdll.dll!NtOpenDirectoryObject()
ntdll.dll!LdrpInitializeProcess()
ntdll.dll!_LdrpInitialize()
ntdll.dll!LdrpInitialize()
ntdll.dll!LdrInitializeThunk()
```

## `Untrusted`-friendly `ntdll.dll`-only executable:

```cpp
#define DBG 1
#include <minidrv.h>

extern "C" int entry() {
    DbgPrint("Hello, world!\n");
    return 0;
}
```
```
cl /c /nologo main.cpp && link main.obj /ENTRY:entry /SUBSYSTEM:console /DEBUG:full ntdll.lib && dumpbin /all main.exe
```

## Additional Debugging Errata
*   WinDbg can't seem to handle unsuspending a process that was created suspended and then attached to (too early?)
*   VS/VSC lack great spots to breakpoint.  `LdrInitializeThunk` is my best find so far, but it doesn't work on my 32-bit C++ binaries (different loader?)

#### Breakpointabless
*   `LdrInitializeThunk` - works for rust exe, but not for `cl.exe` built standalone exe?
*   `LdrpInitializationFailure`
*   `NtCreateFile` - can interfere with debugger?
*   `NtOpenDirectoryObject` - common? failure point
*   `NtTerminateProcess`
*   `RaiseException`
*   `RtlExitUserProcess`
*   `RtlExitUserThread`
*   `RtlUserThreadStart`
*   `rust_panic`

#### Variables of Note
*   `(unsigned int *)LdrpDebugFlags` (set to `0xFFFFFFFF` for DbgView/Output spam + breakpoints + ???)
*   `ShowSnaps` (an alias of LdrpDebugFlags)
*   `RtlpStaticDebugInfo` (see [ReactOS](https://github.com/reactos/reactos/blob/934e5212e4cf36fa1467a30fab3e82ce46208ad9/sdk/lib/rtl/critical.c#L22)?)

#### Tools to remember the existence of
*   DbgView.exe (Sysinternals, DebugView)
*   Procmon.exe (Sysinternals)
*   `C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\gflags.exe`

#### References
*   [Tracing LoadLibrary](http://www.lopezruiz.net/2021/09/08-tracing-loadlibrary.htm)
*   [A catalog of NTDLL kernel mode to user mode callbacks, part 6: LdrInitializeThunk](http://www.nynaeve.net/?p=205)
*   [Entry Point Not Found, and other DLL Loading Problems](https://ofekshilon.com/2013/06/22/entry-point-not-found-and-other-dll-loading-problems/)
*   [The End of PPLdump](https://itm4n.github.io/the-end-of-ppldump/)
*   [Injecting Code into Windows Protected Processes using COM - Part 1](https://googleprojectzero.blogspot.com/2018/10/injecting-code-into-windows-protected.html)
*   [Listing KnownDLLs](https://lucasg.github.io/2017/06/07/listing-known-dlls/)
*   [NoDebugInherit related Stack Overflow](https://stackoverflow.com/questions/48577688/how-to-use-visual-studio-debugging-sub-process)
*   [Anti-Debug: Debug Flags](https://anti-debug.checkpoint.com/techniques/debug-flags.html)
*   <https://lucasg.github.io/2016/11/27/How-to-Create-and-Debug-a-Process-on-Windows/>
*   <https://twitter.com/fuzzysec/status/1166102117460979717>
*   <https://twitter.com/FuzzySec/status/1166102117460979717/photo/1>
