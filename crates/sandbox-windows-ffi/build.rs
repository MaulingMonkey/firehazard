use std::env;
use std::io::Read;
use std::process::{Command, Stdio};

fn main() {
    let mut v = String::new();
    Command::new("rustc").arg("--version").stdout(Stdio::piped()).spawn().unwrap().stdout.unwrap().read_to_string(&mut v).unwrap();
    let nightly = v.contains("-nightly (");
    let std     = env::var_os("CARGO_FEATURE_STD").is_some();

    if nightly          { println!("cargo:rustc-cfg=nightly"); }
    if std              { println!("cargo:rustc-cfg=std"); }
    if nightly && !std  { println!("cargo:rustc-cfg=nostd"); }
}
