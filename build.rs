use std::io::Read;
use std::process::{Command, Stdio};

fn main() {
    let mut v = String::new();
    Command::new("rustc").arg("--version").stdout(Stdio::piped()).spawn().unwrap().stdout.unwrap().read_to_string(&mut v).unwrap();
    if v.contains("-nightly (") { println!("cargo:rustc-cfg=nightly"); }
}
