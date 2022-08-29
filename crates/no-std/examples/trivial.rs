#![no_std]
#![no_main]

use sandbox_windows_ffi::*;
use abistr::*;

#[no_mangle] extern fn mainCRTStartup() -> i32 { run(); 0 }
#[no_mangle] extern fn main() -> i32 { 0 } // unused but required?
#[no_mangle] extern fn __CxxFrameHandler3() -> ! { exit_process(0xC44) } // workaround precompiled libcore using panic = "unwind" per https://github.com/rust-lang/rust/issues/45492#issuecomment-470577653
#[panic_handler] fn panic(_: &core::panic::PanicInfo) -> ! { exit_process(0xFA71C) }

fn run() {
    revert_to_self().unwrap();
    output_debug_string_a(cstr!("sandbox"));
    sleep_ms(1_000);
}
