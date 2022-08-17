#![cfg_attr(nightly, feature(lang_items, core_intrinsics))]
#![cfg_attr(nightly, no_std)]
#![cfg_attr(nightly, no_main)]

#[cfg(not(nightly))] fn main() { sandbox() }

#[cfg(nightly)] mod nightly {

    #[no_mangle] pub extern fn mainCRTStartup() -> i32 { crate::sandbox(); 0 }
    #[no_mangle] pub extern fn main() -> i32 { 0 } // unused?
    #[lang = "eh_personality"] extern fn rust_eh_personality() {}
    #[lang = "panic_impl"] extern fn rust_begin_panic(_: &core::panic::PanicInfo) -> ! { core ::intrinsics::abort() }
}

fn sandbox() {
    unsafe { winapi::um::debugapi::OutputDebugStringA("sandbox\0".as_ptr().cast()) }
    #[cfg(not(nightly))] std::thread::sleep(std::time::Duration::from_secs(1));
}
