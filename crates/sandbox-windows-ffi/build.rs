fn main() {
    if std::env::var_os("CARGO_FEATURE_STD").is_some() { println!("cargo:rustc-cfg=std"); }
}
