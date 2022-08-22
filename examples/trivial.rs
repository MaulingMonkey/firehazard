#![cfg_attr(nightly, feature(lang_items, core_intrinsics))]
#![cfg_attr(nightly, no_std)]
#![cfg_attr(nightly, no_main)]

use sandbox::windows::ffi::*;
use abistr::*;

#[cfg(not(nightly))] fn main() { run() }

#[cfg(nightly)] mod nightly {
    #[no_mangle] pub extern fn mainCRTStartup() -> i32 { crate::run(); 0 }
    #[no_mangle] pub extern fn main() -> i32 { 0 } // unused but required?
    #[lang = "eh_personality"] extern fn rust_eh_personality() {}
    #[lang = "panic_impl"] extern fn rust_begin_panic(_: &core::panic::PanicInfo) -> ! { core ::intrinsics::abort() }
}

fn run() {
    revert_to_self().unwrap();
    output_debug_string_a(cstr!("sandbox"));
    sleep_ms(1_000);
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-sleep)\] Sleep
fn sleep_ms(ms: u32) { unsafe { winapi::um::synchapi::Sleep(ms) } }
