:: See usage: https://github.com/rust-lang/rustup/blob/4031dd731c5f21287539f639c087e42da78f991d/rustup-init.sh#L40-L66
curl https://static.rust-lang.org/rustup/dist/aarch64-pc-windows-msvc/rustup-init.exe --output rustup-init.exe
rustup-init.exe -y --profile minimal --default-toolchain 1.81.0 -t aarch64-pc-windows-msvc
del rustup-init.exe
