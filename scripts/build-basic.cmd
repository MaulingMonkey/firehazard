@pushd "%~dp0.." && setlocal



cargo build --manifest-path crates/no-std/Cargo.toml --release --example trivial
@if ERRORLEVEL 1 goto :die

cargo test --workspace
@if ERRORLEVEL 1 goto :die

cargo build --examples
@if ERRORLEVEL 1 goto :die

target\debug\examples\max_sandbox.exe
@if ERRORLEVEL 1 goto :die



:die
@if ERRORLEVEL 1 echo BUILD ERRORS BUILD ERRORS BUILD ERRORS BUILD ERRORS BUILD ERRORS
@popd && endlocal && exit /b %ERRORLEVEL%
