#[doc(alias = "ExitThread")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-exitthread)\]
/// ExitThread
///
/// ### Safety
///
/// The thread will exit without unwinding the stack or terminating the process.
/// This is fundamentally unsound when combined with <code>std::thread::[scope](std::thread::scope)(...)</code>.
/// Alternatives that are more likely to be sound include exiting from the entire process, panicing, or returning from the thread's function.
///
/// It may or may not be undefined behavior when called from *any* [`std`]-spawned thread,
/// if e.g. the underlying machinery to support <code>std::thread::[JoinHandle](std::thread::JoinHandle)::[join](std::thread::JoinHandle::join)</code>
/// doesn't handle threads dying in a manner other than the expected methods of returning or panicing.
///
/// If you *must* use this function, I strongly urge you to audit [`std`]'s thread spawning mechanism,
/// [creating a `firehazard` issue](https://github.com/MaulingMonkey/firehazard/issues) to affirm or deny the middle safety warning above,
/// and to drop as much of the thread's stack as you can before exiting.
/// <code>std::thread::[scope](std::thread::scope)(...)</code> may not be the only bit of code making assumptions about stack unwinding.
///
/// ### Examples
///
/// ```no_run
/// # #[cfg(std)] std::thread::spawn(||{
/// # use firehazard::*;
/// #
/// let counter = std::sync::atomic::AtomicUsize::new(0);
///
/// std::thread::scope(|scope| {
///     let _thread = scope.spawn(||{
///         loop { counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed); }
///     });
///
///     // UNDEFINED BEHAVIOR: `counter` will go out of scope without waiting for `_thread`
///     // The test executable will likely cause an access violation and exit 0xC0000005,
///     // but the exact behavior of undefined behavior is, of course, not guaranteed.
///     //
///     unsafe { exit_thread(42) };
/// });
/// #
/// # }).join().unwrap();
/// ```
///
pub unsafe fn exit_thread(exit_code: u32) -> ! {
    unsafe { winapi::um::processthreadsapi::ExitThread(exit_code) };
    if cfg!(debug_assertions) {
        panic!("undefined behavior: ExitThread returned, but the caller's thread should've exited!");
    } else {
        unsafe { core::hint::unreachable_unchecked() }
    }
}
