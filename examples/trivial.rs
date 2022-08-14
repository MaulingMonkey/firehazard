#![cfg_attr(nightly, feature(lang_items, core_intrinsics))]
#![cfg_attr(nightly, no_std)]
#![cfg_attr(nightly, no_main)]

#[cfg(not(nightly))] fn main() {}

#[cfg(nightly)] mod nightly {
    #[no_mangle] pub extern fn mainCRTStartup() -> i32 { 42 }
    #[no_mangle] pub extern fn main() -> i32 { 0 } // unused?
    #[lang = "eh_personality"] extern fn rust_eh_personality() {}
    #[lang = "panic_impl"] extern fn rust_begin_panic(_: &core::panic::PanicInfo) -> ! { core ::intrinsics::abort() }
}
