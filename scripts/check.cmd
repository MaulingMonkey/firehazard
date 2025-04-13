@pushd "%~dp0.." && setlocal

@if "%~1" EQU "src\appcontainer\appcontainer_funcs.rs" goto :test
@if "%~1" EQU "crates\no-std\examples\trivial.rs" goto :trivial
@if "%~1" EQU "examples\spam_dbg.rs" goto :spam_dbg

@cargo check --workspace --all-targets --all-features
@if ERRORLEVEL 1 goto :die
@cargo test
@if ERRORLEVEL 1 goto :die

:die
@popd && endlocal && exit /b %ERRORLEVEL%



:test
cargo test
@goto :die

:trivial
cd crates\no-std
cargo +nightly build --example trivial --release -Zbuild-std=core --target=x86_64-pc-windows-msvc
@if ERRORLEVEL 1 goto :die
cargo          build --example trivial --release
@goto :die

:spam_dbg
cargo run --example spam_dbg
@goto :die
