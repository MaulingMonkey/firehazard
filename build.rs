use std::env;
use std::io::Read;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn main() {
    let mut v = String::new();
    Command::new("rustc").arg("--version").stdout(Stdio::piped()).spawn().unwrap().stdout.unwrap().read_to_string(&mut v).unwrap();
    let std = env::var_os("CARGO_FEATURE_STD").is_some();

    if std { println!("cargo:rustc-cfg=std"); }

    if std::env::var_os("CARGO_CFG_TARGET_ENV").map_or(false, |env| env == "msvc") { msvc_only() }
}

fn msvc_only() {
    let windows = std::env::var_os("CARGO_CFG_WINDOWS").is_some(); // else non-windows MSVC target (MacOS / System 9?)

    // https://en.wikipedia.org/wiki/Microsoft_Visual_Studio#History
    let vsv = std::env::var("VisualStudioVersion").unwrap_or_default();
    let (vs_major, _vs_minor_etc) = vsv.split_once('.').unwrap_or((vsv.as_str(), ""));
    let vs_major = vs_major.parse().unwrap_or_else(|_|{
        let pf86 = PathBuf::from(std::env::var_os("ProgramFiles(x86)").unwrap());
        let mvs = pf86.join("Microsoft Visual Studio");
        if      mvs.join("2022").exists()   { 17 }
        else if mvs.join("2019").exists()   { 16 }
        else if mvs.join("2017").exists()   { 15 }
        else                                {  0 }
    });
    let _vs_2017 = vs_major >= 15;
    let _vs_2019 = vs_major >= 16;
    let _vs_2022 = vs_major >= 17;

    println!("cargo:rustc-link-arg=/WX"); // Error out on linker warnings

    if windows {
        // UWP / Microsoft Store isolation
        // Not 100% sure what the linker flag alone does.
        // https://learn.microsoft.com/en-us/cpp/build/reference/appcontainer-windows-store-app
        println!("cargo:rustc-link-arg=/APPCONTAINER");
    }

    if false { // demo only for now
        // Delay Load Import
        // Might be useful for DLLs that cause explosions (`user32.dll`?)
        // https://learn.microsoft.com/en-us/cpp/build/reference/delayload-delay-load-import
        println!("cargo:rustc-link-arg=/DELAYLOAD:advapi32.dll");   // SystemFunction036 aka RtlGenRandom
        println!("cargo:rustc-link-arg=/DELAYLOAD:bcrypt.dll");     // BCryptGenRandom
        println!("cargo:rustc-link-arg=delayimp.lib");
    }

    if _vs_2019 {
        // CET Shadow Stack compatible.  Requires VS2019+
        // https://learn.microsoft.com/en-us/cpp/build/reference/cetcompat
        println!("cargo:rustc-link-arg=/CETCOMPAT");
        println!("cargo:rustc-cfg=cet"); // enforce CET in examples
    }

    // Enable Control Guard Checks.  Already enabled via .cargo/config.toml
    // https://learn.microsoft.com/en-us/cpp/build/reference/guard-enable-guard-checks
    //println!("cargo:rustc-link-arg=/GUARD:CF");

    // https://learn.microsoft.com/en-us/cpp/build/reference/manifest-create-side-by-side-assembly-manifest
    //println!("cargo:rustc-link-arg=/MANIFEST");

    // https://learn.microsoft.com/en-us/cpp/build/reference/manifestfile-name-manifest-file
    //println!("cargo:rustc-link-arg=/MANIFESTFILE:some.manifest");

    // Use inferred sanitizer libs - requires rustc support
    // https://learn.microsoft.com/en-us/cpp/build/reference/inferasanlibs
    // https://github.com/rust-lang/rust/pull/89369
    //println!("cargo:rustc-link-arg=/INFERASANLIBS");
}
