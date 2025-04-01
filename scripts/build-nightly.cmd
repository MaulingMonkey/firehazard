@pushd "%~dp0.." && setlocal
@set RUSTUP_TOOLCHAIN=nightly
@set CARGO_TARGET_DIR=target\nightly



:: Missing symbols: memset, memcpy, memcmp, __chkstk, _fltused
::cargo build --example trivial
::@if ERRORLEVEL 1 goto :die

@set CARGO_TARGET_DIR=crates\no-std\target\nightly
cargo build --manifest-path crates\no-std\Cargo.toml --release --examples --target=x86_64-pc-windows-msvc
@if ERRORLEVEL 1 goto :die
@set CARGO_TARGET_DIR=target\nightly

@setlocal
@call "C:\Program Files (x86)\Microsoft Visual Studio\2017\Community\VC\Auxiliary\Build\vcvars64.bat" >NUL
@echo on
@dumpbin /NOLOGO /IMPORTS crates\no-std\target\nightly\x86_64-pc-windows-msvc\release\examples\trivial.exe
@endlocal

crates\no-std\target\nightly\x86_64-pc-windows-msvc\release\examples\trivial.exe
@if ERRORLEVEL 1 goto :die

cargo build --release --workspace --examples --target=x86_64-pc-windows-msvc
@if ERRORLEVEL 1 goto :die

target\nightly\x86_64-pc-windows-msvc\release\examples\max_sandbox.exe
@if ERRORLEVEL 1 goto :die



:die
@if ERRORLEVEL 1 echo BUILD ERRORS BUILD ERRORS BUILD ERRORS BUILD ERRORS BUILD ERRORS
@popd && endlocal && exit /b %ERRORLEVEL%
