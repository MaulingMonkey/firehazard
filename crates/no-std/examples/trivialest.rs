//! Build with:
//! ```text
//! cargo +nightly -Zbuild-std=core,alloc -Zbuild-std-features=compiler-builtins-mem b --example trivialest
//! ```

#![no_std]
#![no_main]

fn run() {
}



#[no_mangle] extern "C" fn mainCRTStartup() -> i32 { run(); 0 }
#[no_mangle] extern "C" fn main() -> i32 { 0 } // unused but required?
#[no_mangle] extern "C" fn __CxxFrameHandler3() -> ! { __fastfail(0xC44) } // workaround precompiled libcore using panic = "unwind" per https://github.com/rust-lang/rust/issues/45492#issuecomment-470577653
#[no_mangle] extern "C" fn __imp_RaiseException() -> ! { __fastfail(0xF9) } // thumv7a-pc-windows-msvc
#[panic_handler] fn panic(_: &core::panic::PanicInfo) -> ! { __fastfail(0xFA71C) }
#[no_mangle] extern "C" fn fmaf(_: f32, _: f32, _: f32) -> f32 { __fastfail(0x3A7F) } // I can't figure out how to get `-Zbuild-std` to resolve these symbols via libm
#[no_mangle] extern "C" fn fma (_: f64, _: f64, _: f64) -> f64 { __fastfail(0x3A7F) } // I can't figure out how to get `-Zbuild-std` to resolve these symbols via libm



/// \[[microsoft.com](https://learn.microsoft.com/en-us/cpp/intrinsics/fastfail)\]
fn __fastfail(code: u32) -> ! {
    unsafe {
        #[cfg(target_arch = "x86_64")] {
            core::arch::asm!("int 29h", in("rax") u64::from(code));
            core::hint::unreachable_unchecked()
        }
        #[cfg(target_arch = "x86")] {
            core::arch::asm!("int 29h", in("ecx") code);
            core::hint::unreachable_unchecked()
        }
        #[cfg(target_arch = "arm")] {
            core::arch::asm!(".2byte 0xDEFB", in("r0") code);
            core::hint::unreachable_unchecked()
        }
        #[cfg(target_arch = "aarch64")] {
            core::arch::asm!(".2byte 0xF003", in("x0") code);
            core::hint::unreachable_unchecked()
        }
    }
}
