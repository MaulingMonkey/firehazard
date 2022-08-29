@pushd "%~dp0.." && setlocal

@if "%~1" EQU "crates\no-std\examples\trivial.rs" goto :trivial

@cargo check --workspace --all-targets --all-features
@if ERRORLEVEL 1 goto :die

:die
@popd && endlocal && exit /b %ERRORLEVEL%



:trivial
cd crates\no-std
cargo +nightly build --example trivial --release -Zbuild-std=core --target=x86_64-pc-windows-msvc
@if ERRORLEVEL 1 goto :die
cargo          build --example trivial --release
@goto :die
