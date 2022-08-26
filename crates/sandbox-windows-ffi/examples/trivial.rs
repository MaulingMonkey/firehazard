#![cfg_attr(nostd, no_std)]
#![cfg_attr(nostd, no_main)]

use sandbox_windows_ffi::*;
use abistr::*;

#[cfg(not(nostd))] fn main() { run() }
#[cfg(nostd)] #[no_mangle] extern fn mainCRTStartup() -> i32 { run(); 0 }
#[cfg(nostd)] #[no_mangle] extern fn main() -> i32 { 0 } // unused but required?
#[cfg(nostd)] #[no_mangle] extern fn __CxxFrameHandler3() -> ! { exit_process(0xC44) } // workaround precompiled libcore using panic = "unwind" per https://github.com/rust-lang/rust/issues/45492#issuecomment-470577653
#[cfg(nostd)] #[panic_handler] fn panic(_: &core::panic::PanicInfo) -> ! { exit_process(0xFA71C) }

fn run() {
    revert_to_self().unwrap();
    output_debug_string_a(cstr!("sandbox"));
    sleep_ms(1_000);
}
