pub(crate) fn debug<T>(fmt: &mut core::fmt::Formatter, module: &str, name: &str, handle: core::ptr::NonNull<T>) -> core::fmt::Result {
    write!(fmt, "{module}::{name}(")?;
    match handle.as_ptr() as isize {
        // N.B. these are semi-ambiguous: C:\Program Files (x86)\Windows Kits\10\Include\10.0.22621.0\um\winnt.h:
        // #define MEMORY_CURRENT_PARTITION_HANDLE         ((HANDLE) (LONG_PTR) -1)
        // #define MEMORY_SYSTEM_PARTITION_HANDLE          ((HANDLE) (LONG_PTR) -2)
        // #define MEMORY_EXISTING_VAD_PARTITION_HANDLE    ((HANDLE) (LONG_PTR) -3)
        // presumably these handles would be blocked from being convertable to generic handle::Psuedo handles.
        -1  => write!(fmt, "-1 aka GetCurrentProcess()"),
        -2  => write!(fmt, "-2 aka GetCurrentThread()"),
        -3  => write!(fmt, "-3 aka GetCurrentSession()"),               // https://stackoverflow.com/a/45632388/953531
        -4  => write!(fmt, "-4 aka GetCurrentProcessToken()"),
        -5  => write!(fmt, "-5 aka GetCurrentThreadToken()"),
        -6  => write!(fmt, "-6 aka GetCurrentThreadEffectiveToken()"),
        o   => write!(fmt, "0x{:08x}", o as usize),
    }?;
    write!(fmt, ")")
}

#[test] fn verify_debug_values() {
    assert_eq!(-1, get_current_process()                .as_handle() as isize);
    assert_eq!(-2, get_current_thread()                 .as_handle() as isize);
    assert_eq!(-4, get_current_process_token()          .as_handle() as isize);
    assert_eq!(-5, get_current_thread_token()           .as_handle() as isize);
    assert_eq!(-6, get_current_thread_effective_token() .as_handle() as isize);
}
