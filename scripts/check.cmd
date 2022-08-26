@pushd "%~dp0.." && setlocal

@if "%~1" EQU "crates\sandbox-windows-ffi\examples\trivial.rs" goto :trivial

@cargo check --workspace --all-targets
@if ERRORLEVEL 1 goto :die

:die
@popd && endlocal && exit /b %ERRORLEVEL%



:trivial
cargo +nightly build --example trivial --release --no-default-features -Zbuild-std --target=x86_64-pc-windows-msvc
@if ERRORLEVEL 1 goto :die
cargo          build --example trivial --release --no-default-features
@if ERRORLEVEL 1 goto :die
cargo          build --example trivial --features std
@goto :die
